use anyhow::Result;
use asfy_vol::{config::Config, ui::volume_bar::VolumeBar};
use gtk4::{Application, prelude::*};

fn main() -> Result<()> {
    let config = Config::setup(None).unwrap_or_else(|e| {
        eprintln!("Error trying to get configuration, using default: {}", e);
        Config::default()
    });
    dbg!(&config);

    let app = Application::builder()
        .application_id("com.asfy.vol")
        .build();

    app.connect_activate(move |app| {
        let volume_bar = VolumeBar::new(app, config.clone());
        volume_bar.listen();
    });

    app.run();
    Ok(())
}
