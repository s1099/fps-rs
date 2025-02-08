use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, pbr::NotShadowCaster,
    prelude::*, render::view::RenderLayers,
};
use std::f32::consts::FRAC_PI_2;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    camera_sensitivity: CameraSensitivity,
    movement_speed: MovementSpeed,
    transform: Transform,
    visibility: Visibility,
    collider: Collider,
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            camera_sensitivity: CameraSensitivity::default(),
            movement_speed: MovementSpeed(5.0),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            visibility: Visibility::default(),
            collider: Collider::capsule(0.5, 1.2),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

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
        .spawn(PlayerBundle::default())
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

pub mod systems {
    use super::*;

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
        mut player: Query<(&mut Transform, &MovementSpeed, &mut LinearVelocity), With<Player>>,
    ) {
        let Ok((mut transform, speed, mut lin_velocity)) = player.get_single_mut() else {
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

        if keys.just_pressed(KeyCode::Space) {
            lin_velocity.y = 3.3;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        if direction.length_squared() > 0.0 {
            let movement = direction.normalize() * speed.0;
            lin_velocity.x = movement.x;
            lin_velocity.z = movement.z;
        } else {
            lin_velocity.x = lerp(lin_velocity.x, 0.0, 0.2);
            lin_velocity.z = lerp(lin_velocity.z, 0.0, 0.2);
        }
    }
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}
