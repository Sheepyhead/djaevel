use bevy::prelude::*;

use crate::player::Player;

pub struct Stats;

impl Plugin for Stats {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageInflicted>()
            .add_event::<Death>()
            .add_system(inflict_damage)
            .add_system(death);
    }
}

#[derive(Component)]
pub struct HitPoints {
    current: u32,
    _max: u32,
}

impl Default for HitPoints {
    fn default() -> Self {
        Self::new(100)
    }
}

impl HitPoints {
    fn dead(&self) -> bool {
        self.current == 0
    }

    fn subtract(&mut self, amount: u32) -> bool {
        self.current = self.current.saturating_sub(amount);
        self.current == 0
    }

    fn new(amount: u32) -> Self {
        Self {
            current: amount,
            _max: amount,
        }
    }
}

pub struct DamageInflicted {
    pub target: Entity,
    pub amount: u32,
}

fn inflict_damage(
    mut reader: EventReader<DamageInflicted>,
    mut writer: EventWriter<Death>,
    mut targets: Query<&mut HitPoints>,
) {
    for DamageInflicted { target, amount } in reader.iter() {
        if let Ok(mut hit_points) = targets.get_mut(*target) {
            if hit_points.dead() {
                continue;
            }
            info!("Target {target:?} took {amount} damage!");
            if hit_points.subtract(*amount) {
                writer.send(Death(*target));
            }
        }
    }
}

pub struct Death(Entity);

fn death(mut commands: Commands, mut reader: EventReader<Death>, player: Query<(), With<Player>>) {
    for Death(target) in reader.iter() {
        info!("Entity {target:?} has died");
        if player.contains(*target) {
            panic!("Player has died");
        } else {
            commands.entity(*target).despawn_recursive();
        }
    }
}
