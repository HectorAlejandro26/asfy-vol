use crate::audio::wp_listener::{CurrentSink, watch_volume_changes};
use glib::clone;
use gtk4::cairo::{RectangleInt, Region};
use gtk4::gdk::Display;
use gtk4::{Align, Application, ApplicationWindow, Box, Label, Orientation};
use gtk4::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, prelude::*};
use gtk4::{ProgressBar, style_context_add_provider_for_display};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;

const STYLESHEET: &str = include_str!("../../css/style.css");

struct IconRange {
    symbol: char,
    weight: f64,
}

const VOLUME_ICONS: [IconRange; 3] = [
    IconRange {
        symbol: '\u{f026}',
        weight: 0.15,
    },
    IconRange {
        symbol: '\u{f027}',
        weight: 0.425,
    },
    IconRange {
        symbol: '\u{f028}',
        weight: 0.425,
    },
];

pub struct VolumeBar {
    window: ApplicationWindow,
    label: Label,
    progress_bar: ProgressBar,
    timeout_id: Rc<RefCell<Option<glib::SourceId>>>,
}

impl VolumeBar {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Asfy volume")
            .default_width(200)
            .default_height(60)
            .decorated(false)
            .resizable(false)
            .build();

        let label = Label::builder()
            .label(" ")
            .width_chars(1)
            .halign(Align::End)
            .build();
        let progress_bar = ProgressBar::builder()
            .hexpand(true)
            .margin_end(15)
            .valign(Align::Center)
            .build();
        let inner_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .valign(Align::Center)
            .build();

        inner_box.append(&progress_bar);
        inner_box.append(&label);

        let container = Box::builder()
            .css_classes(["volume-container"])
            .halign(Align::Center)
            .valign(Align::Center)
            .margin_top(25)
            .margin_bottom(25)
            .margin_start(25)
            .margin_end(25)
            .build();
        container.append(&inner_box);
        window.set_child(Some(&container));

        Self::apply_css();
        Self::setup_layer_shell(&window);

        Self {
            window,
            label,
            progress_bar,
            timeout_id: Rc::new(RefCell::new(None)),
        }
    }

    fn apply_css() {
        let provider = CssProvider::new();
        provider.load_from_string(STYLESHEET);
        if let Some(display) = Display::default() {
            style_context_add_provider_for_display(
                &display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            )
        }
    }

    fn setup_layer_shell(window: &ApplicationWindow) {
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::None);
        window.set_anchor(Edge::Bottom, true);
        window.set_margin_bottom(50);

        window.connect_realize(|win| {
            if let Some(surface) = win.surface() {
                let reg = Some(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
                surface.set_input_region(reg);
            }
        });
    }

    pub fn listen(&self) {
        let (sender, receiver) = async_channel::unbounded();
        watch_volume_changes(sender);

        let window = self.window.clone();
        let label = self.label.clone();
        let progress_bar = self.progress_bar.clone();
        let timeout_id = self.timeout_id.clone();

        glib::MainContext::default().spawn_local(clone!(
            #[weak]
            window,
            #[weak]
            label,
            #[weak]
            progress_bar,
            #[strong]
            timeout_id,
            async move {
                while let Ok(sink) = receiver.recv().await {
                    Self::update_ui(&window, &label, &progress_bar, sink);
                    Self::manage_timeout(&window, &timeout_id);
                }
            }
        ));
    }

    fn update_ui(
        window: &ApplicationWindow,
        label: &Label,
        progress_bar: &ProgressBar,
        sink: CurrentSink,
    ) {
        let fraction = sink.volume.clamp(0.0, 1.0);
        progress_bar.set_fraction(fraction);

        if sink.muted {
            label.set_text("");
            window.add_css_class("muted");
        } else {
            label.set_text(&format!("{}", map_vol_icon(sink.volume)));
            window.remove_css_class("muted");
        }
        window.present();
    }

    fn manage_timeout(
        window: &ApplicationWindow,
        timeout_id: &Rc<RefCell<Option<glib::SourceId>>>,
    ) {
        if let Some(id) = timeout_id.borrow_mut().take() {
            id.remove();
        }

        let new_id = glib::timeout_add_seconds_local(
            2,
            clone!(
                #[weak]
                window,
                #[strong]
                timeout_id,
                #[upgrade_or]
                glib::ControlFlow::Break, // Que hacer si la ventana muere
                move || {
                    window.set_visible(false);
                    *timeout_id.borrow_mut() = None;
                    glib::ControlFlow::Break
                }
            ),
        );
        *timeout_id.borrow_mut() = Some(new_id);
    }
}

fn map_vol_icon(val: f64) -> char {
    let mut threshold = 0_f64;
    for icon in &VOLUME_ICONS {
        threshold += icon.weight;
        if val <= threshold {
            return icon.symbol;
        }
    }
    VOLUME_ICONS.last().map(|i| i.symbol).unwrap_or(' ')
}
