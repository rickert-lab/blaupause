/*
This file is part of blaupause.

Copyright (C) 2025 Christian Rickert

blaupause is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, version 3.

blaupause is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see <https://www.gnu.org/licenses/>.
*/

use rfd::FileDialog;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

#[allow(unused_imports)]
use std::ffi::OsStr;

pub struct BlaupauseApp {
    source_buffer: Option<PathBuf>,
    source_string: String,
    source_button: String,
    target_buffer: Option<PathBuf>,
    target_string: String,
    target_button: String,
    archive_copy: bool,
    delete_copy: bool,
    is_unix: bool,
    validate_copy: bool,
    copy_button: String,
}

impl Default for BlaupauseApp {
    fn default() -> Self {
        Self {
            source_buffer: Some(PathBuf::new()),
            source_string: "[Source directory]".to_string(),
            source_button: " Browse source... ".to_string(),
            target_buffer: Some(PathBuf::new()),
            target_string: "[Target directory]".to_string(),
            target_button: " Browse target... ".to_string(),
            archive_copy: false,
            is_unix: !cfg!(target_os = "windows"),
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                egui::widgets::global_theme_preference_buttons(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |sui| {
                    sui.label("Powered by egui & eframe.");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
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
            ui.add_enabled(
                self.is_unix,
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
                        // prepare command
                        let native_copy_command = native_copy_command();
                        let native_copy_args = native_copy_args(
                            &self.archive_copy,
                            &self.delete_copy,
                            &self.validate_copy,
                            &self.source_string,
                            &self.target_string,
                        );

                        // print command
                        println!(
                            "\r\n{} {}",
                            &native_copy_command,
                            &native_copy_args.join(" ")
                        );

                        // run command
                        if which(&native_copy_command).is_ok() {
                            let _ = Command::new(&native_copy_command)
                                .args(&native_copy_args)
                                .spawn()
                                .expect("Failed to run command.")
                                .wait();
                        } else {
                            eprintln!("Executable not found: {}", &native_copy_command);
                        }
                    };
                });
            } else {
                // source or target do not exist (yet)
                ui.vertical_centered(|sui| {
                    sui.add_enabled(false, egui::Button::new(&self.copy_button));
                });
            };

            ui.separator();

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |sui| {
                sui.hyperlink_to(
                    "Version 0.1 by Christian Rickert. ",
                    "https://github.com/christianrickert/blaupause/",
                );
                egui::warn_if_debug_build(sui);
            });
        });
    }
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
    let mut param_vec: Vec<String> = vec![
        "-h".to_string(), // human-readable output
        "-r".to_string(), // recursive copying
        "-l".to_string(), // preserve links
        "-v".to_string(), // verbose (summary)
        "-P".to_string(), // progress report
        "-W".to_string(), // copy entire file (faster)
    ];
    if cfg!(target_os = "linux") {
        param_vec.push("--info=progress2".to_string()); // show time remaining, v3.1.0 (2013-09-28)
    }
    if *archive_copy {
        param_vec[0].push('a'); // preserve metadata (-Dgloprt)
    }
    if *delete_copy {
        param_vec.push("--delete-during".to_string()); //receiver deletes during the transfer (sync)
    }
    if *validate_copy {
        param_vec.push("--checksum".to_string()); // skip based on checksum, not mod-time & size
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
    // we have to fix it here to avoid unfortunate situations involving '/PURGE'.
    let source_dir = Path::new(source)
        .file_name()
        .unwrap_or_else(|| OsStr::new("copy"));
    let new_target = Some(Path::new(target).join(&source_dir));
    param_vec.push(get_string_from_buffer(&new_target).to_string());
    param_vec.push("/E".to_string()); // recursive, including empty directories
    param_vec.push("/ETA".to_string()); // progress report
    param_vec.push("/MT:2".to_string()); // multi-threading
    param_vec.push("/R:0".to_string()); // no retry upon failure
    param_vec.push("/V".to_string()); // verbose (show skipped)
    if *archive_copy {
        param_vec.push("/COPYALL".to_string()); // copy file information (/copy:DATSOU)
    }
    if *delete_copy {
        param_vec.push("/PURGE".to_string()); // delete target items missing in source (sync)
    }

    param_vec
}
