#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
fn main() -> eframe::Result {
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
