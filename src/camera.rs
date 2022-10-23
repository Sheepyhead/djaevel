use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn);
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
