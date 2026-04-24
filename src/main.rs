mod listener;
mod ui;

use anyhow::Result;
use gtk4::Application;
use gtk4::prelude::*;

use crate::ui::VolumeBar;

fn main() -> Result<()> {
    let app = Application::builder()
        .application_id("com.asfy.vol")
        .build();

    app.connect_activate(|app| {
        let volume_bar = VolumeBar::new(app);
        volume_bar.listen();
    });

    app.run();

    Ok(())
}
