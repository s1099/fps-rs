use std::f32::consts::FRAC_PI_2;
use bevy::{color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, pbr::NotShadowCaster, prelude::*, render::view::RenderLayers};
use avian3d::prelude::*;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

#[derive(Debug, Component)]
pub struct WorldModelCamera;

#[derive(Debug, Component)]
pub struct MovementSpeed(pub f32);

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.1, 0.1, 0.5));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));

    commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            MovementSpeed(5.0),
            Transform::from_xyz(0.0, 1.0, 0.0),
            Visibility::default(),
            Collider::capsule(0.5, 1.2),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ));

            parent.spawn((
                Camera3d::default(),
                Camera {
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                RenderLayers::layer(1),
            ));

            // gun in right arm
            parent.spawn((
                Mesh3d(arm),
                MeshMaterial3d(arm_material),
                Transform::from_xyz(0.2, -0.1, -0.25),
                // Ensure the arm is only rendered by the view model camera.
                RenderLayers::layer(1),
                // The arm is free-floating, so shadows would look weird.
                NotShadowCaster,
            ));
        });
}

pub fn player_view(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &MovementSpeed), With<Player>>,
) {
    let Ok((mut transform, speed)) = player.get_single_mut() else {
        return;
    };

    // forward and right directions projected onto the XZ plane
    let forward = transform.forward();
    let horizontal_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    let right = transform.right();
    let horizontal_right = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction += horizontal_forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= horizontal_forward;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= horizontal_right;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += horizontal_right;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * speed.0 * time.delta_secs();
}
