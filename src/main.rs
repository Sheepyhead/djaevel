use animation::Animation;
use assets::AssetsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;

mod animation;
mod assets;
mod camera;
mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(Animation)
        .add_plugin(AssetsPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
