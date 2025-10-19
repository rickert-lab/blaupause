/*
blaupause - The copy assistant.
Copyright (C) 2025 The Regents of the University of Colorado

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, version 3.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see <https://www.gnu.org/licenses/>.

Author:     Christian Rickert <christian.rickert@cuanschutz.edu>
Date:       2025-04-30
Version:    0.1
*/

fn main() -> eframe::Result {
    let fixed_size = [591.0, 365.0];
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_max_inner_size([f32::INFINITY, fixed_size[1]])
            .with_min_inner_size(fixed_size)
            .with_inner_size(fixed_size)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "blaupause",
        native_options,
        Box::new(|_cc| Ok(Box::new(blaupause::BlaupauseApp::new()))),
    )
}
