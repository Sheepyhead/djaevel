use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{assets::{LoadingState, PlayerSprites}, animation::AnimateSprite};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(LoadingState::Ready, spawn);
    }
}

fn spawn(mut commands: Commands, sprites: Res<PlayerSprites>) {
    commands.spawn_bundle(PlayerBundle {
        sprite: SpriteSheetBundle {
            texture_atlas: sprites.idle.clone(),
            ..default()
        },
        animate: AnimateSprite {
            timer: Timer::from_seconds(0.2, true),
            sequence: vec![0, 1, 2, 3, 2, 1],
            ..default()
        },
        ..default()
    });
}

#[derive(Component, Default)]
struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    player: Player,
    animate: AnimateSprite,
}

