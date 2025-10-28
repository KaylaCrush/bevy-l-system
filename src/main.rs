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
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, (setup_camera, setup_lighting, spawn_flowers))
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
}


fn setup_lighting(mut commands: Commands) {
    // Global ambient fill
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.45, 0.5, 0.55), // cool grayish-blue
        brightness: 0.35,                    // gentle fill, not too bright
        affects_lightmapped_meshes: false,
    });

    // Soft blue background (fake sky)
    commands.insert_resource(ClearColor(Color::srgb(0.78, 0.86, 0.96)));

    // "Sun" — main key light
    commands.spawn((
        DirectionalLight {
            illuminance: 45_000.0, // realistic sunlight strength
            shadows_enabled: true,
            color: Color::srgb(1.0, 0.97, 0.92), // slightly warm white
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -1.1, // downward angle
            0.8,  // azimuth
            0.0,
        )),
        //DirectionalLightShadowMap { size: 4096 }, // sharper shadows
    ));

    // Optional “sky bounce” fill light (simulates light from above)
    commands.spawn((
        DirectionalLight {
            illuminance: 1500.0,
            color: Color::srgb(0.6, 0.75, 1.0), // bluish
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            1.0,
            -0.2,
            0.0,
        )),
    ));

    // Optional warm rim light from opposite direction
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            color: Color::srgb(1.0, 0.8, 0.65),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.4,
            2.5,
            0.0,
        )),
    ));
}

fn spawn_flowers(mut commands: Commands) {
    let lsystem = LSystem::new(
        "P", // axiom
        vec![
            Rule::new('P', "N+[P+O]--//[--L]N[++L]-[PO]++PO"),
            Rule::new('N', "FS[//&&L][//^^L]FS"),
            Rule::with_probability('S', "S[//&&L][//^^L]FS", 0.33),
            Rule::with_probability('S', "SFS", 0.33),
            Rule::with_probability('S', "S", 0.33),
            Rule::new('L', "['{+f-f-f+|+f-f-f}]"),
            Rule::new('O', "[&&&D'/W////W////W////W////W]"),
            Rule::new('D', "FF"),
            Rule::new('W', "['^^^F][{&&&&-f+f|-f+f}]"),
        ],
        18.0,
    );

    let grid_size = 4;
    let spacing = 150.0; // adjust this to control distance between flowers

    for x in 0..grid_size {
        for z in 0..grid_size {
            let x_pos = (x as f32 - (grid_size as f32 - 1.0) / 2.0) * spacing;
            let z_pos = (z as f32 - (grid_size as f32 - 1.0) / 2.0) * spacing;

            commands.spawn((
                Plant::new(
                    lsystem.clone(), // clone so each flower gets its own LSystem
                    5.0,
                    5,
                    1.0,
                    vec![
                        Color::srgb(0.2, 0.7, 0.3),
                        Color::srgb(0.3, 0.8, 0.7),
                        Color::srgb(0.9, 0.0, 0.10),
                    ],
                ),
                Transform::from_translation(Vec3::new(x_pos, -200.0, z_pos)),
                GlobalTransform::default(),
                Visibility::default(),
            ));
        }
    }
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
