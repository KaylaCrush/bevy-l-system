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

pub fn build_segment_mesh(segments: &[Segment], thickness:f32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Unit cube vertices (centered on Y-axis, from y=0 to y=1)
    // centered along Y
    let cube_positions = [
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, 0.5),
    ];
    // Cube indices
    let cube_indices: [u32; 36] = [
        0,1,2, 2,3,0,   // back
        4,5,6, 6,7,4,   // front
        0,4,7, 7,3,0,   // left
        1,5,6, 6,2,1,   // right
        3,2,6, 6,7,3,   // top
        0,1,5, 5,4,0,   // bottom
    ];

    let mut vertex_offset = 0;

    for seg in segments {
        let dir = seg.end - seg.start;
        let length = dir.length();
        if length == 0.0 { continue; }

        // Rotation to align Y-axis to the segment
        let rotation = Quat::from_rotation_arc(Vec3::Y, dir.normalize());
        let translation = seg.start + dir * 0.5; // center along segment
        let scale = Vec3::new(thickness, length, thickness);
        let transform = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        // Transform and push cube vertices
        for p in &cube_positions {
            let v = transform * p.extend(1.0);
            positions.push([v.x, v.y, v.z]);
            // simple normals from local position, rotated
            let n = rotation * p.normalize_or_zero();
            normals.push([n.x, n.y, n.z]);
        }

        // Push indices with offset
        for &i in &cube_indices {
            indices.push(vertex_offset + i);
        }

        vertex_offset += cube_positions.len() as u32;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
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

        let mesh = build_segment_mesh(&segments, plant.root_thickness);
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
