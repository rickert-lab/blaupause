use rfd::FileDialog;
use std::path::PathBuf;
use std::process::Command;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BlaupauseApp {
    source_buffer: Option<PathBuf>,
    source_string: String,
    target_buffer: Option<PathBuf>,
    target_string: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for BlaupauseApp {
    fn default() -> Self {
        Self {
            // Example stuff
            source_buffer: Some(PathBuf::new()),
            source_string: format!("[source path]"),
            target_buffer: Some(PathBuf::new()),
            target_string: format!("[target path]"),
            value: 0.1,
        }
    }
}

impl BlaupauseApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for BlaupauseApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Blaupause: You copy assistant.");

            ui.separator();

            ui.horizontal(|sui| {
                sui.label("Source path: ");
                sui.add_sized(
                    sui.available_size(),
                    egui::TextEdit::multiline(&mut self.source_string),
                );
            });
            ui.vertical_centered(|sui| {
                if sui.button("Choose source...").clicked() {
                    self.source_buffer = get_folder_with_label("Choose source folder");
                    self.source_string = get_string_from_buffer(&self.source_buffer);
                };
            });
            ui.horizontal(|sui| {
                sui.label("Target path: ");
                sui.add_sized(
                    sui.available_size(),
                    egui::TextEdit::multiline(&mut self.target_string),
                );
            });

            ui.vertical_centered(|sui| {
                if sui.button("Choose target...").clicked() {
                    self.target_buffer = get_folder_with_label("Choose target folder");
                    self.target_string = get_string_from_buffer(&self.target_buffer);
                }
            });

            ui.separator();

            ui.vertical_centered(|sui| {
                if sui.button("Copy").clicked() {
                    let output = Command::new(native_copy_command())
                        .args(native_copy_args(&self.source_string, &self.target_string))
                        .output()
                        .expect("Failed to copy.");
                    println!("{:?}", output);
                };
            });

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/christianrickert/blaupause",
                "Source code."
            ));
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

#[cfg(target_os = "macos")]
fn native_copy_command() -> String {
    format!("rsync")
}
fn native_copy_args(source: &String, target: &String) -> Vec<String> {
    let mut param_vec: Vec<String> = vec!["-avP".to_string()];
    param_vec.push(source.to_string());
    param_vec.push(target.to_string());
    param_vec
}

fn get_folder_with_label(label: &str) -> Option<PathBuf> {
    FileDialog::new().set_title(label).pick_folder()
}

fn get_string_from_buffer(buffer: &Option<PathBuf>) -> String {
    match buffer {
        Some(path) => path.display().to_string(),
        None => String::new(),
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
