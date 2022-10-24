use bevy::prelude::*;

pub struct Common;

impl Plugin for Common {
    fn build(&self, app: &mut App) {
        app.add_system(timed_despawn);
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Despawn(pub Timer);

fn timed_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut despawn: Query<(Entity, &mut Despawn)>,
) {
    for (entity, mut despawn) in despawn.iter_mut() {
        if despawn.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
