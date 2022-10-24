use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{common::Despawn, enemy::Enemy, stats::DamageInflicted};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(DaggerSkill::spawn)
            .add_system(DaggerSkill::act)
            .add_system(DaggerSkill::collide);
    }
}

#[derive(Component)]
pub struct DaggerSkill {
    pub cooldown: Timer,
}

impl DaggerSkill {
    fn spawn(
        mut commands: Commands,
        ass: Res<AssetServer>,
        time: Res<Time>,
        mut daggers: Query<(&Transform, &mut DaggerSkill)>,
        enemies: Query<&Transform, With<Enemy>>,
    ) {
        for (transform, mut dagger) in daggers.iter_mut() {
            if dagger.cooldown.tick(time.delta()).just_finished() {
                let mut max = f32::MAX;
                let mut nearest = None;
                for enemy in enemies.iter() {
                    let distance = enemy.translation.distance_squared(transform.translation);
                    if distance < max {
                        max = distance;
                        nearest = Some(enemy);
                    }
                }

                if let Some(nearest) = nearest {
                    let mut transform = *transform;
                    transform.rotation = Quat::from_rotation_arc(
                        Vec3::Y,
                        (nearest.translation - transform.translation).normalize_or_zero(),
                    );
                    commands.spawn_bundle(DaggerBundle {
                        sprite: SpriteBundle {
                            texture: ass.load("Pixel Crawler 1.1/Weapons/Wood/WoodDagger.png"),
                            transform,
                            ..default()
                        },
                        despawn: Despawn(Timer::from_seconds(5.0, false)),
                        collider: Collider::ball(10.0),
                        active_events: ActiveEvents::COLLISION_EVENTS,
                        rbody: RigidBody::KinematicPositionBased,
                        ..default()
                    });
                }
            }
        }
    }

    fn act(time: Res<Time>, mut daggers: Query<&mut Transform, With<Dagger>>) {
        for mut dagger in daggers.iter_mut() {
            let direction = dagger.local_y();
            dagger.translation += direction * time.delta_seconds() * 300.0;
        }
    }

    fn collide(
        mut commands: Commands,
        mut collision_events: EventReader<CollisionEvent>,
        mut damage_events: EventWriter<DamageInflicted>,
        enemies: Query<(), With<Enemy>>,
        daggers: Query<(), With<Dagger>>,
    ) {
        for event in collision_events.iter() {
            if let CollisionEvent::Started(c1, c2, _) = event {
                let (enemy, dagger) = if enemies.contains(*c1) && daggers.contains(*c2) {
                    (c1, c2)
                } else if enemies.contains(*c2) && daggers.contains(*c1) {
                    (c2, c1)
                } else {
                    continue;
                };
                commands.entity(*dagger).despawn_recursive();
                damage_events.send(DamageInflicted {
                    target: *enemy,
                    amount: 10,
                });
            }
        }
    }
}

#[derive(Component, Default)]
struct Dagger;

#[derive(Bundle, Default)]
struct DaggerBundle {
    #[bundle]
    sprite: SpriteBundle,
    dagger: Dagger,
    despawn: Despawn,
    collider: Collider,
    sensor: Sensor,
    active_events: ActiveEvents,
    rbody: RigidBody,
}
