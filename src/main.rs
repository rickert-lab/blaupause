#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let fixed_size = [600.0, 370.0];
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_max_inner_size([f32::INFINITY, fixed_size[1]])
            .with_inner_size(fixed_size)
            .with_min_inner_size(fixed_size)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "blaupause",
        native_options,
        Box::new(|cc| Ok(Box::new(blaupause::BlaupauseApp::new(cc)))),
    )
}
