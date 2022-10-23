use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    animation::AnimateSprite,
    assets::{LoadingState, PlayerSprites},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_enter_system(LoadingState::Ready, spawn)
            .add_system(run);
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
        input: InputManagerBundle::<Action> {
            input_map: InputMap::new([(
                VirtualDPad {
                    up: KeyCode::Comma.into(),
                    down: KeyCode::O.into(),
                    left: KeyCode::A.into(),
                    right: KeyCode::E.into(),
                },
                Action::Run,
            )])
            .build(),
            ..default()
        },
        ..default()
    });
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    player: Player,
    animate: AnimateSprite,
    #[bundle]
    input: InputManagerBundle<Action>,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Run,
}

fn run(
    time: Res<Time>,
    sprites: Option<Res<PlayerSprites>>,
    mut query: Query<
        (
            &ActionState<Action>,
            &mut Transform,
            &mut Handle<TextureAtlas>,
            &mut AnimateSprite,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    for (action_state, mut transform, mut atlas, mut animate, mut sprite) in query.iter_mut() {
        if action_state.pressed(Action::Run) {
            let axis_pair = action_state.axis_pair(Action::Run).unwrap();
            transform.translation += axis_pair.xy().extend(0.0) * time.delta_seconds() * 100.0;
            if axis_pair.x() < 0.0 {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        }
        if let Some(sprites) = &sprites {
            if action_state.just_pressed(Action::Run) {
                *atlas = sprites.run.clone();
                animate.sequence = (6..12).collect();
                animate.current_index = 0;
                sprite.index = 0;
            }
            if action_state.just_released(Action::Run) {
                *atlas = sprites.idle.clone();
                animate.sequence = (0..4).collect();
                animate.current_index = 0;
                sprite.index = 0;
            }
        }
    }
}
