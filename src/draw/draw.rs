use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};
use bevy_egui::egui::FontSelection;

use crate::plant::Plant;

#[derive(Clone, Copy)]
struct Turtle3D {
    pos: Vec3,
    rot: Quat,
    thickness: f32,
    color_index: usize,
}

#[derive(Clone, Copy)]
struct Segment{
    start: Vec3,
    end: Vec3,
    thickness: f32,
    color: Color,
}

#[derive(Clone)]
struct Folio{
    vertices: Vec<Vec3>,
    color: Color,
}

fn interpret_plant(lsystem_string: &str, step_size: f32, turn_angle_deg: f32, root_thickness: f32, palette: &Vec<Color>) -> (Vec<Segment>, Vec<Folio>) {
    let mut turtle = Turtle3D {
        pos: Vec3::ZERO,
        rot: Quat::IDENTITY, // facing +Y
        thickness: root_thickness,
        color_index: 0,
    };
    let mut stack = Vec::new();
    let mut segments = Vec::new();
    let mut folios: Vec<Folio> = Vec::new();
    let mut current_folio: Option<Vec<Vec3>> = None;

    let turn_rad = turn_angle_deg.to_radians();

    for c in lsystem_string.chars() {
        match c {


            '{' => {
                current_folio = Some(vec![turtle.pos]);
            }
            '}' => {
                if let Some(mut verts) = current_folio.take() {
                    // Close the shape by connecting to the first vertex if needed
                    if verts.len() >= 3 {
                        folios.push(Folio { vertices: verts, color: palette[turtle.color_index % palette.len()] });
                    }
                }
            }
            'F' => {
                // Move forward along local Y
                let new_pos = turtle.pos + turtle.rot * Vec3::Y * step_size;
                let color = palette[turtle.color_index % palette.len()];
                segments.push(Segment { start: turtle.pos, end: new_pos, thickness: turtle.thickness, color: color});
                turtle.pos = new_pos;
            }
            'f' => {
                // forward but without drawing a segment
                turtle.pos += turtle.rot * Vec3::Y * (step_size/2.0);
                if let Some(ref mut verts) = current_folio {
                    verts.push(turtle.pos);
                }
            }
            '!' => turtle.thickness *= 0.9,
            '\'' => turtle.color_index += 1,
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

    (segments, folios)
}

fn build_segment_mesh(segments: &[Segment], folios: &[Folio]) -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();

    for seg in segments {
        let dir = seg.end - seg.start;
        let length = dir.length();
        if length == 0.0 { continue; }

        let rotation = Quat::from_rotation_arc(Vec3::Y, dir.normalize());
        let translation = seg.start + dir * 0.5;
        let scale = Vec3::new(seg.thickness, length, seg.thickness);
        let transform = Mat4::from_scale_rotation_translation(scale, rotation, translation);
        let color = seg.color.to_srgba().to_f32_array();

        let mut vertex_offset = 0;

        // Define vertices for a unit rectangular prism along Y from y=-0.5..0.5
        // (you can skew or taper these later if you want more organic shapes)
        let verts = [
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new( 0.5, -0.5, -0.5),
            Vec3::new( 0.5,  0.5, -0.5),
            Vec3::new(-0.5,  0.5, -0.5),
            Vec3::new(-0.5, -0.5,  0.5),
            Vec3::new( 0.5, -0.5,  0.5),
            Vec3::new( 0.5,  0.5,  0.5),
            Vec3::new(-0.5,  0.5,  0.5),
        ];

        // 12 triangles (6 faces × 2 triangles)
        let face_indices = [
            (0, 1, 2, 3), // back
            (5, 4, 7, 6), // front
            (4, 0, 3, 7), // left
            (1, 5, 6, 2), // right
            (3, 2, 6, 7), // top
            (4, 5, 1, 0), // bottom
        ];

        let face_normals = [
            Vec3::NEG_Z, Vec3::Z, Vec3::NEG_X, Vec3::X, Vec3::Y, Vec3::NEG_Y,
        ];

        for (face_i, &(a, b, c, d)) in face_indices.iter().enumerate() {
            let normal = rotation * face_normals[face_i];
            let start_idx = positions.len() as u32;

            for &vi in &[a, b, c, d] {
                let v = transform * verts[vi].extend(1.0);
                positions.push([v.x, v.y, v.z]);
                normals.push([normal.x, normal.y, normal.z]);
                colors.push(color);
            }

            // Two triangles per face
            indices.extend_from_slice(&[
                start_idx, start_idx + 1, start_idx + 2,
                start_idx, start_idx + 2, start_idx + 3,
            ]);
        }

        vertex_offset += (verts.len() * 6) as u32; // optional tracking if you mix types
    }

    // leaves (flat double-sided quads/polygons)
    for folio in folios {
        if folio.vertices.len() < 3 {
            continue;
        }

        // Triangulate polygon (simple fan)
        let base_index = positions.len() as u32;
        let color = folio.color.to_srgba().to_f32_array();

        // Find polygon normal (approx. via cross product)
        let normal = {
            let v1 = folio.vertices[1] - folio.vertices[0];
            let v2 = folio.vertices[2] - folio.vertices[0];
            v1.cross(v2).normalize()
        };

        // Push vertices
        for v in &folio.vertices {
            positions.push([v.x, v.y, v.z]);
            normals.push([normal.x, normal.y, normal.z]);
            colors.push(color);
        }

        // Triangulate as a fan
        for i in 1..folio.vertices.len() as u32 - 1 {
            indices.extend_from_slice(&[base_index, base_index + i, base_index + i + 1]);
        }

        // Double-sided: duplicate triangles with flipped normal
        let back_base = positions.len() as u32;
        for v in &folio.vertices {
            positions.push([v.x, v.y, v.z]);
            normals.push([-normal.x, -normal.y, -normal.z]);
            colors.push(color);
        }
        for i in 1..folio.vertices.len() as u32 - 1 {
            indices.extend_from_slice(&[back_base, back_base + i + 1, back_base + i]);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
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

        let (segments, folios) = interpret_plant(&plant.current_string, plant.step_size, plant.lsystem.angle, plant.root_thickness, &plant.palette);

        let mesh = build_segment_mesh(&segments, &folios);
        let mesh_handle = meshes.add(mesh);

        commands.spawn((
            Mesh3d(mesh_handle),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.6, 0.25),
                perceptual_roughness: 0.8, // very diffuse
                metallic: 0.0,             // plants aren't metallic
                reflectance: 0.2,          // subtle highlights
                ..default()
                //base_color: Color::srgb(0.0, 1.0, 0.0),
                //cull_mode: None,
            })),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::default(),
            Name::new("PlantMesh"),
            ChildOf(entity),
        ));
    }
}
