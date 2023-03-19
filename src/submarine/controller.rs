use bevy::prelude::*;

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub movement_spot: f32,
    pub no_movement_spot: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub run_speed: f32,
    pub friction: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            movement_spot: 250.0,
            no_movement_spot: 50.0,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_up: KeyCode::D,
            key_down: KeyCode::A,
            run_speed: 6.0,
            friction: 0.5,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn control_translation(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut options)) = query.get_single_mut() {
        if !options.enabled {
            return;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            /* let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            }; */
            options.velocity = axis_input.normalize() * options.run_speed; // max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }

        let forward = transform.forward();
        let right = transform.right();
        let up = transform.up();

        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * up
            + options.velocity.z * dt * forward;
    }
}

pub fn control_z_axis_rotation(
    time: Res<Time>,
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, options)) = query.get_single_mut() {
        let window = windows.single();
        if let Some(cursor_position) = window.cursor_position() {
            // TODO: implement velocity for nose up/down (y) and rotation (x)
            let x_coefficient = get_relative_motion(
                cursor_position.x,
                window.width(),
                options.movement_spot,
                options.no_movement_spot,
            );

            transform.rotation = transform
                .rotation
                .mul_quat(Quat::from_rotation_z(x_coefficient * dt));
        }
    }
}

pub fn control_x_axis_rotation(
    time: Res<Time>,
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, options)) = query.get_single_mut() {
        let window = windows.single();
        if let Some(cursor_position) = window.cursor_position() {
            // TODO: implement velocity for nose up/down (y) and rotation (x)
            let y_coefficient = get_relative_motion(
                cursor_position.y,
                window.height(),
                options.movement_spot,
                options.no_movement_spot,
            );

            transform.rotation = transform
                .rotation
                .mul_quat(Quat::from_rotation_x(y_coefficient * dt));
        }
    }
}

fn get_relative_motion(
    position: f32,
    domain: f32,
    movement_spot: f32,
    no_movement_spot: f32,
) -> f32 {
    let x_relative_position = domain * 0.5 - position;
    if x_relative_position >= movement_spot {
        1.0
    } else if x_relative_position <= movement_spot * -1.0 {
        -1.0
    } else if x_relative_position.abs() <= no_movement_spot {
        0.0
    } else if x_relative_position > 0.0 {
        (x_relative_position - no_movement_spot) / (movement_spot - no_movement_spot)
    } else {
        (x_relative_position + no_movement_spot) / (movement_spot - no_movement_spot)
    }
}
