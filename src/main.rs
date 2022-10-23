use animation::Animation;
use assets::AssetsPlugin;
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, render::RapierDebugRenderPlugin};
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use stats::Stats;

mod animation;
mod assets;
mod camera;
mod enemy;
mod player;
mod stats;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(Animation)
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(Stats)
        .run();
}
