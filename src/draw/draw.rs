use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::plant::Plant;

// Green color
const GREEN: Srgba = bevy::color::palettes::css::GREEN;

#[derive(Clone, Copy)]
struct Turtle {
    pos: Vec2,
    angle: f32,
}

pub fn draw_plant(mut commands: Commands, plants: Query<(Entity, &Plant)>) {
    for (entity, plant) in &plants {
        // Remove existing children (previously drawn shapes)
        commands.entity(entity).despawn_children();

        // Interpret L-System string into a ShapePath
        let path = interpret_plant_string(&plant.current_string, plant.step_size, plant.lsystem.angle);

        commands.spawn((
            ShapeBuilder::with(&path)
                .stroke((GREEN, 3.0))
                .build(),
            ChildOf(entity),
        ));
    }
}

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
