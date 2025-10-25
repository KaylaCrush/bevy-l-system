use bevy::{input::common_conditions::input_just_pressed, math::VectorSpace, prelude::*};
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

// Our modules
mod lsystem;
mod plant;
mod draw;
mod ui;
mod input;

use lsystem::{LSystem, Rule};
use plant::Plant;
use draw::draw_plant;
use ui::{plant_ui, palette_ui};
use input::{CameraController, InputPlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InputPlugin))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, (setup_camera, spawn_flower))
        .add_systems(Update, (draw_plant, plant_step_system))
        .add_systems(EguiPrimaryContextPass, (plant_ui, palette_ui))
        .run();
}

// Camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        CameraController::default(),
        Transform::from_xyz(400.0, 400.0, 400.0).looking_at(Vec3::ZERO, Vec3::Y),
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

fn spawn_flower(mut commands: Commands){
    let lsystem = LSystem::new(
    "P", // axiom
    vec![
        Rule::new('P', "N+[P+O]--//[--L]N[++L]-[PO]++PO"),
        Rule::new('N', "FS[//&&L][//^^L]FS"),
        Rule::new('S', "SFS"),
        Rule::new('L', "['{+f-ff-f+|+f-ff-f}]"),
        Rule::new('O', "[&&&D'/W////W////W////W////W]"),
        Rule::new('D', "FF"),
        Rule::new('W', "['^F][{&&&&-f+f|-f+f}]"),
    ],
    18.0
);
    commands.spawn((
        Plant::new(
            lsystem,
            2.0,
            5,
            1.0,
            vec![
                Color::srgb(0.2, 0.7, 0.3),   // green for leaves
                Color::srgb(0.9, 0.5, 0.7),   // pink flower
                Color::srgb(0.5, 0.35, 0.2),  // darker brown for pedicels
            ]),
        Transform::from_translation(Vec3::new(0.0,-200.0,0.0)),
        GlobalTransform::default(),
        Visibility::default(),
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
        Rule::new('A', "[&FL!A]/////'[&FL!A]///////'[&FL!A]"),
        Rule::new('F', "S/////F"),
        Rule::new('S', "FL"),
        Rule::new('L', "['''^^{-f+f+f-|-f+f+f}]"),
    ],
    22.5, // delta in degrees
    );

    commands.spawn((
        Plant::new(
            lsystem,
            10.0,
            7,
            5.0,
            vec![
            Color::srgba(80.0 / 255.0, 42.0 / 255.0, 20.0 / 255.0, 1.0),
            // Stem mid – medium brown
            Color::srgba(121.0 / 255.0, 76.0 / 255.0, 39.0 / 255.0, 1.0),
            // Stem tip / small branches – lighter reddish-brown
            Color::srgba(160.0 / 255.0, 96.0 / 255.0, 40.0 / 255.0, 1.0),
            // Leaf 1 – dark green
            Color::srgba(34.0 / 255.0, 139.0 / 255.0, 34.0 / 255.0, 1.0),
            // Leaf 2 – medium green
            Color::srgba(50.0 / 255.0, 205.0 / 255.0, 50.0 / 255.0, 1.0),
            // Leaf 3 – bright green
            Color::srgba(124.0 / 255.0, 252.0 / 255.0, 0.0 / 255.0, 1.0),
                // Color::srgb(0.90, 0.75, 0.50), // yellowish highlight
                // Color::srgb(0.85, 0.45, 0.55), // optional petal tint
            ]),
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
