#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use anyhow::Result;
use simple_logger::SimpleLogger;
use eframe::egui;

mod core;
mod ui;

use crate::ui::app::PhotoOrganizerApp;


fn main() -> Result<()> {
    // Add panic hook for GUI error reporting on Windows
    #[cfg(not(debug_assertions))]
    std::panic::set_hook(Box::new(|info| {
        let msg = format!("Application crashed: {}", info);
        rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title("Photo & Video Organizer Error")
            .set_description(&msg)
            .show();
    }));

    SimpleLogger::new().init().unwrap();
    
    // For now, launch GUI.
    
    let icon_data = include_bytes!("../assets/icon.png");
    let icon = image::load_from_memory(icon_data)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = icon.dimensions();
    let icon_data = egui::IconData {
        rgba: icon.into_raw(),
        width,
        height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_icon(std::sync::Arc::new(icon_data)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Photo & Video Organizer",
        options,
        Box::new(|cc| Ok(Box::new(PhotoOrganizerApp::new(cc)))),
    ).map_err(|e| anyhow::anyhow!("Eframe error: {}", e))?;

    Ok(())
}
