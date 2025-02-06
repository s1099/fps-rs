mod components;
use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind,
    prelude::*,
    render::view::RenderLayers,
    window::{CursorGrabMode, WindowMode},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }), PhysicsPlugins::default()))
        .add_systems(Startup, (setup, components::player::spawn_player))
        .add_systems(
            Update,
            (
                components::player::player_view,
                components::player::player_movement,
                toggle_cursor,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut window: Single<&mut Window>,
) {
    // grab cursor
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;

    // 1080X1920
    window.resolution.set(1920.0, 1080.0);

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("#e1ed5f").unwrap().into(),
            ..default()
        })),
        RigidBody::Static,
        Collider::cuboid(20.0, 0.1, 20.0),
    ));

    // cubes
    for x in -2..3 {
        for z in -2..3 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Srgba::hex("#61cbe8").unwrap().into(),
                    ..default()
                })),
                Transform::from_xyz(x as f32 * 2.0, 0.5, z as f32 * 2.0),
                RigidBody::Dynamic,
                Collider::cuboid(1.0, 1.0, 1.0),
            ));
        }
    }

    // light
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 5.0, -0.75),
        RenderLayers::from_layers(&[0, 1]),
    ));
}

fn toggle_cursor(mut window: Single<&mut Window>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = !window.cursor_options.visible;
        window.cursor_options.grab_mode = match window.cursor_options.grab_mode {
            CursorGrabMode::None => CursorGrabMode::Locked,
            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
        };
    }
}
