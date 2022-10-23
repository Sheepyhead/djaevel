use crate::{
    animation::AnimateSprite,
    assets::{EnemySprites, LoadingState},
    player::Player,
    stats::DamageInflicted,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(0.5, true)))
            .add_event::<EnemyTouch>()
            .add_enter_system(LoadingState::Ready, spawn)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(LoadingState::Ready)
                    .with_system(act)
                    .with_system(interact)
                    .with_system(touch)
                    .with_system(touch_timer_tick)
                    .into(),
            );
    }
}

#[derive(DerefMut, Deref)]
struct SpawnTimer(Timer);

fn spawn(
    mut commands: Commands,
    // time: Res<Time>,
    // mut spawn_timer: ResMut<SpawnTimer>,
    sprite: Res<EnemySprites>,
) {
    // if spawn_timer.tick(time.delta()).just_finished() {
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
        collider: Collider::ball(10.0),
        rbody: RigidBody::Dynamic,
        axes: LockedAxes::ROTATION_LOCKED,
        damping: Damping {
            linear_damping: 10.0,
            ..default()
        },
        touch_timer: TouchTimer(Timer::from_seconds(0.5, false)),
        ..default()
    });
    // }
}

#[derive(Component, Default)]
struct Enemy;

#[derive(Bundle, Default)]
struct EnemyBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    animate: AnimateSprite,
    enemy: Enemy,
    collider: Collider,
    rbody: RigidBody,
    axes: LockedAxes,
    damping: Damping,
    touch_timer: TouchTimer,
}

#[derive(Component, Default, Deref, DerefMut)]
struct TouchTimer(Timer);

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

fn interact(
    rapier_context: Res<RapierContext>,
    mut events: EventWriter<EnemyTouch>,
    player: Query<Entity, With<Player>>,
    enemies: Query<Entity, (With<Enemy>, Without<Player>)>,
) {
    let player = player.single();

    for enemy in rapier_context
        .intersections_with(player)
        .filter(|(_, _, intersecting)| *intersecting)
        .filter_map(|(c1, c2, _)| enemies.get(c1).or(enemies.get(c2)).ok())
    {
        events.send(EnemyTouch(enemy));
    }
}

fn touch_timer_tick(time: Res<Time>, mut timers: Query<&mut TouchTimer>) {
    for mut timer in timers.iter_mut() {
        if !timer.finished() {
            timer.tick(time.delta());
        }
    }
}

struct EnemyTouch(Entity);

fn touch(
    mut reader: EventReader<EnemyTouch>,
    mut writer: EventWriter<DamageInflicted>,
    player: Query<Entity, With<Player>>,
    mut enemies: Query<&mut TouchTimer>,
) {
    let player = player.single();
    for EnemyTouch(enemy) in reader.iter() {
        if let Ok(mut timer) = enemies.get_mut(*enemy) {
            if timer.finished() {
                writer.send(DamageInflicted {
                    target: player,
                    amount: 10,
                });
                timer.reset();
            }
        }
    }
}
