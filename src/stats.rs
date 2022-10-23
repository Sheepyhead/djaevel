use bevy::prelude::*;

pub struct Stats;

impl Plugin for Stats {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageInflicted>()
            .add_system(inflict_damage);
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

fn inflict_damage(mut events: EventReader<DamageInflicted>, mut targets: Query<&mut HitPoints>) {
    for DamageInflicted { target, amount } in events.iter() {
        if let Ok(mut hit_points) = targets.get_mut(*target) {
            if hit_points.dead() {
                continue;
            }
            info!("Target {target:?} took {amount} damage!");
            if hit_points.subtract(*amount) {
                info!("Target is dead!");
            }
        }
    }
}
