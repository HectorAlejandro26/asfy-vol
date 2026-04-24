use crate::listener::watch_volume_changes;
use glib::clone;
use gtk4::cairo::{RectangleInt, Region};
use gtk4::gdk::Display;
use gtk4::{Align, Application, ApplicationWindow, Box, Label, Orientation};
use gtk4::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, prelude::*};
use gtk4::{ProgressBar, style_context_add_provider_for_display};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;
struct IconRange {
    symbol: char,
    weight: f32,
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

fn map_vol_icon(val: f32) -> char {
    let mut threshold = 0.0;

    // Iteramos directamente sobre la constante
    for icon in &VOLUME_ICONS {
        threshold += icon.weight;
        if val <= threshold {
            return icon.symbol;
        }
    }

    VOLUME_ICONS.last().map(|i| i.symbol).unwrap_or(' ')
}

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

        // 2. NUEVO: Crea un contenedor que será el que realmente lleve el estilo
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

        let provider = CssProvider::new();
        provider.load_from_string(
            "
    window {
        background: transparent;
    }

    .volume-container {
        box-shadow: -5px 0 20px 2px rgba(166, 227, 161, 0.4),
                    5px 0 20px 2px rgba(137, 220, 235, 0.4);

        border: 2px solid rgba(151, 224, 198, 0.8);

        background-image: linear-gradient(45deg, 
            rgba(30, 30, 46, 0.85) 40%, 
            rgba(17, 17, 27, 0.85) 100%
        );
        
        border-radius: 15px;
        padding: 10px 20px;
    }

    label {
        color: #94e2d5;
        text-shadow: 0px 0px 5px #11111b;
    }
    window.muted label {
        color: #cba6f7;
    }
    
    
    progressbar trough {
        background-color: rgba(17, 17, 27, 0.8);
        border-radius: 10px;
        min-height: 4px;
        border: none;
    }
    progressbar progress {
        background-image: linear-gradient(90deg, 
            #89dceb 20%,
            #a6e3a1 50%, 
            #94e2d5 80%,
        );
        min-height: 4px;
        border-radius: 10px;
        border: none;
    }
    window.muted .volume-container {
        box-shadow: -5px 0 20px 2px rgba(203, 166, 247, 0.4),
                    5px 0 20px 2px rgba(245, 194, 231, 0.4);
        border: 2px solid rgba(180, 190, 254, 0.8);
    }

    window.muted progressbar progress {
        background-image: linear-gradient(90deg, 
            #cba6f7 20%, /* Mauve */
            #f5c2e7 50%, /* Pink */
            #b4befe 80%  /* Lavender */
        );
    }
    ",
        );
        if let Some(display) = Display::default() {
            style_context_add_provider_for_display(
                &display,
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            )
        }

        VolumeBar::as_layer_shell(&window);

        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Left, false);
        window.set_anchor(Edge::Top, false);
        window.set_anchor(Edge::Right, false);
        window.set_margin_bottom(50);

        Self {
            window,
            label,
            timeout_id: Rc::new(RefCell::new(None)),
            progress_bar,
        }
    }

    fn as_layer_shell(window: &ApplicationWindow) {
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::None);
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
                    let fraction = (sink.volume as f64).clamp(0.0, 1.0);
                    progress_bar.set_fraction(fraction);

                    if sink.muted {
                        label.set_text("");
                        window.add_css_class("muted");
                    } else {
                        // label.set_text(&format!("{}%", vol_pct));
                        label.set_text(&format!("{}", map_vol_icon(sink.volume)));

                        window.remove_css_class("muted");
                    };

                    window.present();

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
        ));
    }
}
