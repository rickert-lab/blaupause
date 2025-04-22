use rfd::FileDialog;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BlaupauseApp {
    source_buffer: Option<PathBuf>,
    source_string: String,
    source_button: String,
    target_buffer: Option<PathBuf>,
    target_string: String,
    target_button: String,
    archive_copy: bool,
    delete_copy: bool,
    validate_copy: bool,
    copy_button: String,
}

impl Default for BlaupauseApp {
    fn default() -> Self {
        Self {
            // Example stuff
            source_buffer: Some(PathBuf::new()),
            source_string: format!("[Source directory]"),
            source_button: format!(" Browse source... "),
            target_buffer: Some(PathBuf::new()),
            target_string: format!("[Target directory]"),
            target_button: format!(" Browse target... "),
            archive_copy: true,
            delete_copy: false,
            validate_copy: false,
            copy_button: format!("\n    Copy source    \n      to target!    \n"),
        }
    }
}

impl BlaupauseApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }*/
        Default::default()
    }
}

impl eframe::App for BlaupauseApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Blaupause: The copy assistant.");

            ui.separator();

            ui.horizontal(|sui| {
                sui.add_enabled(
                    false,
                    egui::TextEdit::multiline(&mut self.source_string)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                );
            });

            ui.vertical_centered(|sui| {
                if sui.button(&self.source_button).clicked() {
                    self.source_buffer = get_folder_with_label("Select source directory!");
                    self.source_string = get_string_from_buffer(&self.source_buffer);
                };
            });

            ui.separator();

            ui.horizontal(|sui| {
                sui.add_enabled(
                    false,
                    egui::TextEdit::multiline(&mut self.target_string)
                        .desired_rows(3)
                        .desired_width(f32::INFINITY),
                );
            });

            ui.vertical_centered(|sui| {
                if sui.button(&self.target_button).clicked() {
                    self.target_buffer = get_folder_with_label("Select target directory!");
                    self.target_string = get_string_from_buffer(&self.target_buffer);
                }
            });

            ui.separator();

            ui.checkbox(&mut self.archive_copy, "Archive: Keep original metadata.");
            ui.checkbox(
                &mut self.delete_copy,
                "Delete: Remove surplus target files.",
            );
            ui.checkbox(
                &mut self.validate_copy,
                "Validate: Check target files during copy.",
            );

            ui.separator();

            if is_existing_directory(&self.source_string)
                && is_existing_directory(&self.target_string)
            {
                // valid source and target directory
                ui.vertical_centered(|sui| {
                    if sui.button(&self.copy_button).clicked() {
                        Command::new(native_copy_command())
                            .args(native_copy_args(
                                &self.archive_copy,
                                &self.delete_copy,
                                &self.validate_copy,
                                &self.source_string,
                                &self.target_string,
                            ))
                            .spawn()
                            .expect("Failed to start copy command.")
                            .wait()
                            .expect("Failed to wait on copy command.");
                    };
                });
            } else {
                // source or target does not exist (yet)
                ui.vertical_centered(|sui| {
                    sui.add_enabled(false, egui::Button::new(&self.copy_button));
                });
            }

            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |sui| {
                sui.hyperlink_to(
                    "v0.1 by Christian Rickert  ",
                    "https://github.com/christianrickert/blaupause/",
                );
                egui::warn_if_debug_build(sui);
            });
        });
    }
}

#[cfg(target_os = "macos")]
fn native_copy_command() -> String {
    format!("rsync")
}

#[cfg(target_os = "macos")]
fn native_copy_args(
    archive_copy: &bool,
    delete_copy: &bool,
    validate_copy: &bool,
    source: &String,
    target: &String,
) -> Vec<String> {
    let mut param_vec: Vec<String> = vec!["-rlvP".to_string()]; // copy recursively and preserve links
    if *archive_copy {
        param_vec[0].push_str("a")
    }
    if *delete_copy {
        param_vec.push("--delete-during".to_string())
    }
    if *validate_copy {
        param_vec.push("--checksum".to_string())
    }
    param_vec.push("--bwlimit=100".to_string());
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

fn is_existing_directory(path: &String) -> bool {
    let path = Path::new(path);
    path.exists() && path.is_dir()
}
