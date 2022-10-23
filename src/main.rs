use animation::Animation;
use assets::AssetsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

mod animation;
mod assets;
mod camera;
mod enemy;
mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(Animation)
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
