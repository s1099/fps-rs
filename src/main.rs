mod components;
use bevy::app::Update;
use bevy::color::palettes::tailwind;
use bevy::color::Color;
use bevy::pbr::PointLight;
use bevy::prelude::Startup;
use bevy::render::view::RenderLayers;
use bevy::transform::components::Transform;
use bevy::{
    app::App,
    asset::{AssetServer, Assets},
    color::Srgba,
    ecs::system::{Commands, Res, ResMut},
    math::primitives::{Cuboid, Plane3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    render::mesh::{Mesh, Mesh3d, Meshable},
    utils::default,
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, components::player::spawn_player))
        .add_systems(
            Update,
            (
                components::player::move_player,
                components::player::player_movement,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("#e1ed5f").unwrap().into(),
            ..default()
        })),
    ));

    for x in -2..3 {
        for z in -2..3 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::default())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Srgba::hex("#61cbe8").unwrap().into(),
                    ..default()
                })),
                Transform::from_xyz(x as f32 * 2.0, 0.5, z as f32 * 2.0),
            ));
        }
    }

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
