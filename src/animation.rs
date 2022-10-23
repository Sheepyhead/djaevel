use bevy::prelude::*;

pub struct Animation;

impl Plugin for Animation {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprites);
    }
}

#[derive(Component, Default)]
pub struct AnimateSprite {
    pub timer: Timer,
    pub sequence: Vec<usize>,
    pub current_index: usize,
}

fn animate_sprites(
    time: Res<Time>,
    mut sprites: Query<(&mut TextureAtlasSprite, &mut AnimateSprite)>,
) {
    for (mut sprite, mut animate) in sprites.iter_mut() {
        if animate.timer.tick(time.delta()).just_finished() {
            if animate.current_index + 1 == animate.sequence.len() {
                animate.current_index = 0;
            } else {
                animate.current_index += 1;
            }
            sprite.index = animate.sequence[animate.current_index];
        }
    }
}
