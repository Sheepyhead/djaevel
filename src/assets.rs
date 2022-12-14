use bevy::{asset::LoadState, prelude::*};
use iyes_loopless::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(LoadingState::Loading)
            .add_enter_system(LoadingState::Loading, load)
            .add_system(check_load.run_in_state(LoadingState::Loading))
            .add_exit_system(LoadingState::Loading, load_texture_atlases);
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum LoadingState {
    Loading,
    Ready,
}

#[derive(Deref)]
struct LoadedAssets(Vec<HandleUntyped>);

fn load(mut commands: Commands, ass: Res<AssetServer>) {
    commands.insert_resource(LoadedAssets(ass.load_folder("").unwrap()));
}

fn check_load(mut commands: Commands, ass: Res<AssetServer>, loaded: Res<LoadedAssets>) {
    let finished = loaded
        .iter()
        .all(|handle| matches!(ass.get_load_state(handle), LoadState::Loaded));
    if finished {
        commands.insert_resource(NextState(LoadingState::Ready));
    }
}

fn load_texture_atlases(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut texture_atlantes: ResMut<Assets<TextureAtlas>>,
) {
    let handle = ass.load("Pixel Crawler 1.1/Heroes/Knight/Idle/Idle-Sheet.png");
    let idle = texture_atlantes.add(TextureAtlas::from_grid(handle, Vec2::splat(32.0), 4, 1));
    let handle = ass.load("Pixel Crawler 1.1/Heroes/Knight/Run/Run-Sheet.png");
    let run = texture_atlantes.add(TextureAtlas::from_grid_with_padding(
        handle,
        Vec2::splat(32.0),
        6,
        2,
        Vec2::new(32.0, 0.0),
        Vec2::new(16.0, 0.0),
    ));
    commands.insert_resource(PlayerSprites { idle, run });

    let handle =
        ass.load("generic-rpg-pack_v0.4_(alpha-release)_vacaroxa/rpg-pack/mobs/slime-orange.png");
    let slime = texture_atlantes.add(TextureAtlas::from_grid(handle, Vec2::new(16.0, 24.0), 4, 1));
    commands.insert_resource(EnemySprites { slime });
}

pub struct PlayerSprites {
    pub idle: Handle<TextureAtlas>,
    pub run: Handle<TextureAtlas>,
}

pub struct EnemySprites {
    pub slime: Handle<TextureAtlas>,
}
