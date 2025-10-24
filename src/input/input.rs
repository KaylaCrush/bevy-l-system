use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion,MouseWheel};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller_system);
    }
}

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub focus: Vec3,
    pub sensitivity: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: -0.3,
            distance: 400.0,
            focus: Vec3::ZERO,
            sensitivity: 0.005,
            zoom_speed: 1.0,
            pan_speed: 1.0,
        }
    }
}

fn camera_controller_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut CameraController)>,
) {
    for (mut transform, mut controller) in &mut query {
        // --- ROTATE (Right Mouse) ---
        if mouse_input.pressed(MouseButton::Right) {
            let mut delta = Vec2::ZERO;
            for ev in mouse_motion_events.read() {
                delta += ev.delta;
            }
            controller.yaw -= delta.x * controller.sensitivity;
            controller.pitch -= delta.y * controller.sensitivity; // invert to feel natural
            controller.pitch = controller.pitch.clamp(-1.5, 1.5);
        }

        // --- ZOOM (Mouse Wheel) ---
        for ev in mouse_wheel_events.read() {
            controller.distance -= ev.y * controller.zoom_speed;
            controller.distance = controller.distance.clamp(1.0, 1000.0);
        }

        // --- PAN (WASD) ---
        let mut pan = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) { pan += Vec3::Y * controller.pan_speed; }
        if keyboard_input.pressed(KeyCode::KeyS) { pan -= Vec3::Y * controller.pan_speed; }
        if keyboard_input.pressed(KeyCode::KeyA) { pan -= transform.right() * controller.pan_speed; }
        if keyboard_input.pressed(KeyCode::KeyD) { pan += transform.right() * controller.pan_speed; }
        controller.focus += pan;

        // --- APPLY TRANSFORM ---
        let rotation = Quat::from_axis_angle(Vec3::X, controller.pitch)
            * Quat::from_axis_angle(Vec3::Y, controller.yaw);

        // Camera looks along -Z by default
        let offset = rotation * Vec3::new(0.0, 0.0, controller.distance);
        transform.translation = controller.focus + offset;
        transform.look_at(controller.focus, Vec3::Y);
    }
}
