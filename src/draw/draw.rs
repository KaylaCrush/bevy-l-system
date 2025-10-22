use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};

use crate::plant::Plant;

#[derive(Clone, Copy)]
struct Turtle3D {
    pos: Vec3,
    rot: Quat,
}

#[derive(Clone, Copy)]
struct Segment{
    start: Vec3,
    end: Vec3,
}

fn interpret_plant_string_3d(lsystem_string: &str, step_size: f32, turn_angle_deg: f32) -> Vec<Segment> {
    let mut turtle = Turtle3D {
        pos: Vec3::ZERO,
        rot: Quat::IDENTITY, // facing +Y
    };
    let mut stack = Vec::new();
    let mut segments = Vec::new();

    let turn_rad = turn_angle_deg.to_radians();

    for c in lsystem_string.chars() {
        match c {
            'F' => {
                // Move forward along local Y
                let new_pos = turtle.pos + turtle.rot * Vec3::Y * step_size;
                segments.push(Segment { start: turtle.pos, end: new_pos });
                turtle.pos = new_pos;
            }
            '+' => turtle.rot *= Quat::from_rotation_z(-turn_rad), // roll clockwise
            '-' => turtle.rot *= Quat::from_rotation_z(turn_rad),  // roll counter-clockwise
            '&' => turtle.rot *= Quat::from_rotation_x(turn_rad),  // pitch down
            '^' => turtle.rot *= Quat::from_rotation_x(-turn_rad), // pitch up
            '\\' => turtle.rot *= Quat::from_rotation_y(turn_rad), // yaw left
            '/' => turtle.rot *= Quat::from_rotation_y(-turn_rad), // yaw right
            '[' => stack.push(turtle),
            ']' => turtle = stack.pop().unwrap(),
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
