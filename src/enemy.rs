use crate::{
    animation::AnimateSprite,
    assets::{EnemySprites, LoadingState},
    player::Player,
};
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.5, true)))
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(LoadingState::Ready)
                    .with_system(spawn)
                    .with_system(act)
                    .into(),
            );
    }
}

#[derive(DerefMut, Deref)]
struct SpawnTimer(Timer);

fn spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    sprite: Res<EnemySprites>,
) {
    if spawn_timer.tick(time.delta()).just_finished() {
        commands.spawn_bundle(EnemyBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: sprite.slime.clone(),
                transform: Transform::from_xyz(100.0, 100.0, 0.0),
                ..default()
            },
            animate: AnimateSprite {
                timer: Timer::from_seconds(0.2, true),
                sequence: (0..4).collect(),
                ..default()
            },
            ..default()
        });
    }
}

#[derive(Component, Default)]
struct Enemy;

#[derive(Bundle, Default)]
struct EnemyBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    animate: AnimateSprite,
    enemy: Enemy,
}

fn act(
    time: Res<Time>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Query<&GlobalTransform, With<Player>>,
) {
    let player = player.single();
    let delta = time.delta_seconds();
    for mut enemy in enemies.iter_mut() {
        let direction = (player.translation() - enemy.translation).normalize_or_zero();
        enemy.translation += direction * delta * 80.0;
    }
}
