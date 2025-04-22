use rfd::FileDialog;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

#[allow(unused_imports)]
use std::ffi::OsStr;

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
            source_string: "[Source directory]".to_string(),
            source_button: " Browse source... ".to_string(),
            target_buffer: Some(PathBuf::new()),
            target_string: "[Target directory]".to_string(),
            target_button: " Browse target... ".to_string(),
            archive_copy: true,
            delete_copy: false,
            validate_copy: false,
            copy_button: "\n    Copy source    \n      to target!    \n".to_string(),
        }
    }
}

impl BlaupauseApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
        Default::default()
    }
}

impl eframe::App for BlaupauseApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |sui| {
                    sui.hyperlink_to(
                        "v0.1 by Christian Rickert  ",
                        "https://github.com/christianrickert/blaupause/",
                    );
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
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

            let enabled = !cfg!(target_os = "windows");
            ui.add_enabled(
                enabled,
                egui::widgets::Checkbox::new(
                    &mut self.validate_copy,
                    "Validate: Check target files during copy.",
                ),
            );

            ui.separator();

            if is_existing_directory(&self.source_string)
                && is_existing_directory(&self.target_string)
            {
                // valid source and target directory
                ui.vertical_centered(|sui| {
                    if sui.button(&self.copy_button).clicked() {
                        let native_copy_command = native_copy_command();
                        let native_copy_args = native_copy_args(
                            &self.archive_copy,
                            &self.delete_copy,
                            &self.validate_copy,
                            &self.source_string,
                            &self.target_string,
                        );
                        println!(
                            "COMMAND: \"{} {}\"",
                            &native_copy_command,
                            &native_copy_args.join(" ")
                        );
                        if which(&native_copy_command).is_ok() {
                            Command::new(&native_copy_command)
                                .args(&native_copy_args)
                                .spawn()
                                .expect("Failed to start copy command.")
                                .wait()
                                .expect("Failed to wait on copy command.");
                        } else {
                            eprintln!("Executable not found: {}", &native_copy_command);
                        }
                    };
                });
            } else {
                // source or target does not exist (yet)
                ui.vertical_centered(|sui| {
                    sui.add_enabled(false, egui::Button::new(&self.copy_button));
                });
            };

            ui.separator();

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |sui| {
                sui.label("Powered by egui & eframe.");
                egui::warn_if_debug_build(sui);
            });
        });
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn native_copy_command() -> String {
    "rsync".to_string()
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn native_copy_args(
    archive_copy: &bool,
    delete_copy: &bool,
    validate_copy: &bool,
    source: &String,
    target: &String,
) -> Vec<String> {
    let mut param_vec: Vec<String> = vec!["-rlvPW".to_string()];
    if *archive_copy {
        param_vec[0].push('a');
    }
    if *delete_copy {
        param_vec.push("--delete-during".to_string());
    }
    if *validate_copy {
        param_vec.push("--checksum".to_string());
    }
    param_vec.push(source.to_string());
    param_vec.push(target.to_string());
    param_vec
}
#[cfg(target_os = "windows")]
fn native_copy_command() -> String {
    "ROBOCOPY".to_string()
}

#[cfg(target_os = "windows")]
fn native_copy_args(
    archive_copy: &bool,
    delete_copy: &bool,
    _validate_copy: &bool,
    source: &String,
    target: &String,
) -> Vec<String> {
    let mut param_vec: Vec<String> = Vec::new();
    param_vec.push(source.to_string());
    // ROBOCOPY dumps the content of the source directory into the target directory,
    // we have to fix it here to avoid unfortunate situations involving '--purge'
    let source_dir = Path::new(source)
        .file_name()
        .unwrap_or_else(|| OsStr::new("copy"));
    let new_target = Some(Path::new(target).join(&source_dir));
    param_vec.push(get_string_from_buffer(&new_target));
    param_vec.push("/e /eta /mt /v /zb".to_string());
    if *archive_copy {
        param_vec.push("/copy:DATSOU /dcopy:DATE".to_string());
    }
    if *delete_copy {
        param_vec.push("/purge".to_string());
    }

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
