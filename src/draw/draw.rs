use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, VertexAttributeValues},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};

use bevy_prototype_lyon::prelude::*;
use crate::plant::Plant;


// Green color
const GREEN: Srgba = bevy::color::palettes::css::GREEN;

#[derive(Clone, Copy)]
struct Segment{
    start: Vec3,
    end: Vec3,
}

#[derive(Clone, Copy)]
struct Turtle3D {
    pos: Vec3,
    rot: Quat,
}

fn interpret_plant_string_3d(lsystem_string: &str, step_size: f32, turn_angle: f32) -> Vec<Segment> {
    let mut turtle_pos = Vec3::ZERO;
    let mut turtle_rot = Quat::from_rotation_y(0.0); // Or use Euler angles
    let mut stack = Vec::new();
    let mut segments = Vec::new();

    for c in lsystem_string.chars() {
        match c {
            'F' => {
                let new_pos = turtle_pos + turtle_rot * Vec3::Z * step_size;
                segments.push(Segment { start: turtle_pos, end: new_pos });
                turtle_pos = new_pos;
            }
            '+' => turtle_rot *= Quat::from_rotation_y(turn_angle.to_radians()),
            '-' => turtle_rot *= Quat::from_rotation_y(-turn_angle.to_radians()),
            '[' => stack.push((turtle_pos, turtle_rot)),
            ']' => {
                let (pos, rot) = stack.pop().unwrap();
                turtle_pos = pos;
                turtle_rot = rot;
            }
            _ => {}
        }
    }

    segments
}

fn build_segment_mesh(segments: &[Segment]) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();

    for (i, seg) in segments.iter().enumerate() {
        positions.push([seg.start.x, seg.start.y, seg.start.z]);
        positions.push([seg.end.x, seg.end.y, seg.end.z]);
        indices.push((i*2) as u32);
        indices.push((i*2 + 1) as u32);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}


#[derive(Clone, Copy)]
struct Turtle {
    pos: Vec2,
    angle: f32,
}

pub fn draw_plant(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    plants: Query<(Entity, &Plant)>,
) {
    for (entity, plant) in &plants {
        commands.entity(entity).despawn_children();

        let segments = interpret_plant_string_3d(&plant.current_string, plant.step_size, plant.lsystem.angle);

        let mesh = build_segment_mesh(&segments);
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 1.0, 0.0),
                ..default()
            })),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            Name::new("PlantMesh"),
            ChildOf(entity),
        ));
    }
}



// pub fn draw_plant(mut commands: Commands, plants: Query<(Entity, &Plant)>) {
//     for (entity, plant) in &plants {
//         // Remove existing children (previously drawn shapes)
//         commands.entity(entity).despawn_children();

//         // Interpret L-System string into a ShapePath
//         let path = interpret_plant_string(&plant.current_string, plant.step_size, plant.lsystem.angle);

//         commands.spawn((
//             ShapeBuilder::with(&path)
//                 .stroke((GREEN, 3.0))
//                 .build(),
//             ChildOf(entity),
//         ));
//     }
// }

fn interpret_plant_string(lsystem_string: &str, step_size: f32, turn_angle: f32) -> ShapePath {
    let mut turtle = Turtle { pos: Vec2::ZERO, angle: 90.0 };
    let mut stack = Vec::new();
    let mut path = ShapePath::new();
    path = path.move_to(turtle.pos);

    for c in lsystem_string.chars() {
        match c {
            'F' => {
                let rad = turtle.angle.to_radians();
                let new_pos = turtle.pos + Vec2::new(rad.cos(), rad.sin()) * step_size;
                path = path.line_to(new_pos);
                turtle.pos = new_pos;
            }
            '+' => turtle.angle += turn_angle,
            '-' => turtle.angle -= turn_angle,
            '[' => stack.push(turtle),
            ']' => {
                turtle = stack.pop().unwrap();
                path = path.move_to(turtle.pos);
            }
            _ => {}
        }
    }
    path
}

