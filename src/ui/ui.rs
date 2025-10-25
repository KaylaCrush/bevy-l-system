use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::plant::Plant;
use crate::lsystem::Rule;
use bevy_egui::egui;

pub fn plant_ui(mut contexts: EguiContexts, mut query: Query<&mut Plant>) {
    egui::Window::new("Plant Settings").show(contexts.ctx_mut().unwrap(), |ui| {
        for mut plant in query.iter_mut() {
            ui.label("Adjust step size:");
            ui.add(
                egui::Slider::new(&mut plant.step_size, 1.0..=50.0)
                    .text("Step Size")
            );

            ui.label("Root Thickness:");
            ui.add(
                egui::Slider::new(&mut plant.root_thickness, 1.0..=10.0)
                    .text("Thickness")
            );

            ui.label("Max Iterations:");
            ui.add(
                egui::Slider::new(&mut plant.max_iterations, 1..=10)
                    .text("Max Iterations")
            );

            ui.label("Axiom:");
            ui.text_edit_singleline(&mut plant.lsystem.axiom);

            ui.separator();
            ui.label("Rules (format: F -> F+F-F):");

            let mut remove_index: Option<usize> = None;

            for (i, rule) in plant.lsystem.rules.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let mut pred_char = rule.predecessor.to_string();
                    let mut succ_string = rule.successor.clone();

                    ui.text_edit_singleline(&mut pred_char);
                    ui.label("->");
                    ui.text_edit_singleline(&mut succ_string);

                    if let Some(ch) = pred_char.chars().next() {
                        rule.predecessor = ch;
                    }
                    rule.successor = succ_string;

                    if ui.button("X").clicked() {
                        remove_index = Some(i);
                    }
                });
            }

            // Remove rule if requested
            if let Some(i) = remove_index {
                plant.lsystem.rules.remove(i);
            }

            // Add new rule
            if ui.button("Add Rule").clicked() {
                plant.lsystem.rules.push(Rule::new('X', ""));
            }
            ui.separator();

            if ui.button("Reset Plant").clicked() {
                plant.reset();
            }

            ui.separator();
            ui.label("Current String:");
            ui.label(plant.current_string.clone());
        }
    });
}

pub fn palette_ui(mut contexts: EguiContexts, mut query: Query<&mut Plant>) {
    egui::Window::new("Palette Editor").show(contexts.ctx_mut().unwrap(), |ui| {
        for mut plant in query.iter_mut() {
            ui.label("Edit Plant Palette:");

            let mut remove_index: Option<usize> = None;
            let palette_len = plant.palette.len(); // capture length once here

            // Iterate by index to avoid borrowing the whole palette mutably in the closure
            for i in 0..palette_len {
                // Get mutable reference to the i-th color
                let color = &mut plant.palette[i];

                ui.horizontal(|ui| {
                    let mut r = (color.to_srgba().red * 255.0) as u8;
                    let mut g = (color.to_srgba().green * 255.0) as u8;
                    let mut b = (color.to_srgba().blue * 255.0) as u8;

                    ui.label("R:");
                    ui.add(egui::DragValue::new(&mut r).range(0..=255));
                    ui.label("G:");
                    ui.add(egui::DragValue::new(&mut g).range(0..=255));
                    ui.label("B:");
                    ui.add(egui::DragValue::new(&mut b).range(0..=255));

                    // Write back into the color
                    *color = Color::srgba_u8(r, g, b, 255);

                    // Only mark for removal; don't mutate the palette here
                    if ui.button("X").clicked() && palette_len > 1 {
                        remove_index = Some(i);
                    }
                });
            }

            // Remove outside the loop
            if let Some(i) = remove_index {
                plant.palette.remove(i);
            }

            if ui.button("Add Color").clicked() {
                plant.palette.push(Color::srgba_u8(128, 64, 0, 255));
            }

            ui.separator();
            ui.label("Tip: Palette cycles are applied with the `'` operator in the L-system.");
        }
    });
}
