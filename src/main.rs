use bevy::{input::common_conditions::input_just_pressed, math::VectorSpace, prelude::*};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

// Our modules
mod lsystem;
mod plant;
mod draw;
mod ui;

use lsystem::{LSystem, Rule};
use plant::Plant;
use draw::draw_plant;
use ui::plant_ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, (setup_camera, spawn_example_plant))
        .add_systems(Update, (draw_plant, plant_step_system.run_if(input_just_pressed(KeyCode::Space))))
        .add_systems(EguiPrimaryContextPass, plant_ui)
        .run();
}

// Camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(200.0, 200.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    commands.spawn((
        PointLight {
            intensity: 800.0,
            range: 1000.0,
            ..default()
        },
        Transform::from_xyz(200.0, 400.0, 300.0),
        GlobalTransform::default(),
    ));
}

// Example plant
fn spawn_example_plant(mut commands: Commands) {
    // let lsystem = LSystem::new(
    //     "X",
    //     vec![
    //         Rule::new('F', "FF"),
    //         Rule::new('X', "F-[[X]+X]+F[+FX]-X"),
    //     ],
    //     22.5,
    // );
    let lsystem = LSystem::new(
    "A", // axiom
    vec![
        Rule::new('A', "[&FLA]/////[&FLA]///////[&FLA]"),
        Rule::new('F', "S/////F"),
        Rule::new('S', "FL"),
        Rule::new('L', "[^^{-f+f+f-|-f+f+f}]"),
    ],
    22.5, // delta in degrees
);


    commands.spawn((
        Plant::new(lsystem, 20.0, 3, 5.0),
        Transform::from_translation(Vec3::new(0.0,-200.0,0.0)),
        GlobalTransform::default(),
        Visibility::default(),
    ));
}


// Step system
fn plant_step_system(mut query: Query<&mut Plant>) {
    for mut plant in query.iter_mut() {
        if !plant.finished() {
            plant.step();
        }
    }
}
