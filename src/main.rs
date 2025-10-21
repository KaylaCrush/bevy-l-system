use bevy::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy_prototype_lyon::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

// Step size and turn angle
const STEP_SIZE: f32 = 20.0;
const TURN_ANGLE: f32 = 90.0;

// Green color
const GREEN: Srgba = bevy::color::palettes::css::GREEN;

// Component for L-System plant
#[derive(Component)]
struct Plant {
    axiom: String,
    current_string: String,
    rules: Vec<(char, String)>,
    step_size: f32,
    angle: f32,
    iteration: usize,
    max_iterations: usize,
    root: Vec2,
}

impl Plant {
    fn finished(&self) -> bool {
        self.iteration >= self.max_iterations
    }

    fn reset(&mut self) {
        self.current_string = self.axiom.clone();
        self.iteration = 0;
    }
}

// Turtle for drawing
#[derive(Clone, Copy)]
struct Turtle {
    pos: Vec2,
    angle: f32,
}

// Line segment
struct LineSegment {
    start: Vec2,
    end: Vec2,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin))
        .add_plugins( EguiPlugin::default())
        .add_systems(Startup, (setup_camera, setup_plant))
        .add_systems(Update,
            (
                draw_lsystem,
                lsystem_step_system.run_if(input_just_pressed(KeyCode::Space)),
            )
        )
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}

// Camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Sample4));
}

// Spawn initial plant
fn setup_plant(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single().expect("Window error");
    let bot_mid = Vec2::new(0.0, -window.height() / 2.0 + 20.0);

    let axiom = "F+F+F+F".to_string();

    commands.spawn((
        Plant {
            axiom: axiom.clone(),
            current_string: axiom.clone(),
            rules: vec![('F', "F+F-F-F+F".to_string())],
            step_size: STEP_SIZE,
            angle: TURN_ANGLE,
            iteration: 0,
            max_iterations: 3,
            root: bot_mid,
        },
        Transform::from_translation(Vec3::new(bot_mid.x, bot_mid.y, 0.0)),
        GlobalTransform::default(),
        Visibility::default(),
    ));
}

// L-System iteration system
fn lsystem_step_system(mut query: Query<&mut Plant>) {
    for mut plant in &mut query {
        let mut next = String::new();
        for c in plant.current_string.chars() {
            if let Some((_, replacement)) = plant.rules.iter().find(|(ch, _)| *ch == c) {
                next.push_str(replacement);
            } else {
                next.push(c);
            }
        }

        plant.current_string = next;
        plant.iteration += 1;

        // Optional scaling
        let growth_factor: f32 = 4.0;
        plant.step_size = (100.0) / growth_factor.powi(plant.max_iterations as i32);
    }
}

fn draw_lsystem(mut commands: Commands, plants: Query<(Entity, &Plant)>) {
    for (entity, plant) in &plants {
        commands.entity(entity).despawn_children();

        let path = interpret_lsystem_to_path(&plant.current_string, plant.step_size, plant.angle, plant.root);

        commands.spawn((
            ShapeBuilder::with(&path)
                .stroke((GREEN, 3.0))
                .build(),
            ChildOf(entity),
        ));
    }
}

fn interpret_lsystem_to_path(lsystem_string: &str, step_size: f32, turn_angle: f32, root: Vec2) -> ShapePath {
    let mut turtle = Turtle { pos: Vec2::ZERO, angle: 90.0 };
    let mut stack = Vec::new();
    let mut path = ShapePath::new();

    // Reassign after move_to
    path = path.move_to(turtle.pos);

    for c in lsystem_string.chars() {
        match c {
            'F' => {
                let rad = turtle.angle.to_radians();
                let new_pos = turtle.pos + Vec2::new(rad.cos(), rad.sin()) * step_size;
                path = path.line_to(new_pos); // reassign after line_to
                turtle.pos = new_pos;
            }
            '+' => turtle.angle -= turn_angle,
            '-' => turtle.angle += turn_angle,
            '[' => stack.push(turtle),
            ']' => {
                turtle = stack.pop().unwrap();
                path = path.move_to(turtle.pos); // reassign after move_to
            }
            _ => {}
        }
    }
    path
}

fn ui_system(mut contexts: EguiContexts, mut query: Query<&mut Plant>) -> Result {
    egui::Window::new("Plant Settings").show(contexts.ctx_mut()?, |ui| {
        for mut plant in query.iter_mut() {
            ui.label("Adjust step size and turn angle:");
            ui.add(
                egui::Slider::new(&mut plant.step_size, 1.0..=50.0)
                    .text("Step Size")
            );
            ui.add(
                egui::Slider::new(&mut plant.angle, 0.0..=180.0)
                    .text("Turn Angle")
            );

            ui.separator();

            // Axiom input
            // Axiom input
            ui.label("Axiom:");
            ui.text_edit_singleline(&mut plant.axiom);

            ui.separator();

            // Rules
            ui.label("Rules (format: F -> F+F-F):");
            let mut remove_index: Option<usize> = None;
            for (i, rule) in plant.rules.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let mut input_char = rule.0.to_string();
                    let mut input_string = rule.1.clone();

                    ui.text_edit_singleline(&mut input_char);
                    ui.label("->");
                    ui.text_edit_singleline(&mut input_string);

                    // Update the rule if changed
                    if let Some(ch) = input_char.chars().next() {
                        rule.0 = ch;
                    }
                    rule.1 = input_string;

                    if ui.button("X").clicked() {
                        remove_index = Some(i);
                    }
                });
            }

            // Remove rule if requested
            if let Some(i) = remove_index {
                plant.rules.remove(i);
            }

            // Add new rule
            if ui.button("Add Rule").clicked() {
                plant.rules.push(('X', "".to_string()));
            }

            ui.separator();

            if ui.button("Reset L-System").clicked() {
                plant.reset();
            }

        }
    });
    Ok(())
}
