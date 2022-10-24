use animation::Animation;
use assets::AssetsPlugin;
use bevy::{prelude::*, window::close_on_esc};
use bevy_rapier2d::{prelude::*, render::RapierDebugRenderPlugin};
use camera::CameraPlugin;
use common::Common;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use stats::Stats;
use weapon::WeaponPlugin;

mod animation;
mod assets;
mod camera;
mod common;
mod enemy;
mod player;
mod stats;
mod weapon;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_system(close_on_esc)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(Animation)
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(Common)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(Stats)
        .add_plugin(WeaponPlugin)
        .run();
}
