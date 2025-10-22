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
