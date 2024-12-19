use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::*,
};
use crate::{
    game::LoadEvent,
    resources::{
        GameFonts, GameSounds, GameTextures
    }
};
use crate::GameState;

pub const BACKGROUND: &str = "textures/background.png";
pub const CARD: &str = "textures/card.png";
pub const WINDOW: &str = "textures/background_difficulty.png";
pub const CLOSE: &str = "textures/close.png";

pub const MOVE_CARD: &str = "sounds/move_card.ogg";
pub const MOVE_TO_STOCK: &str = "sounds/move_to_stock.ogg";

pub const DOT_GOTHIC: &str = "fonts/DotGothic16-Regular.ttf";

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_system)
            .add_systems(
                Update,
                wait_loading_system.run_if(in_state(GameState::Loading))
            );
    }
}

fn load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let prefix = "embedded://";
    let game_textures = GameTextures {
        background: asset_server.load(format!("{}{}", prefix, BACKGROUND)),
        card: asset_server.load(format!("{}{}", prefix, CARD)),
        window: asset_server.load(format!("{}{}", prefix, WINDOW)),
        close: asset_server.load(format!("{}{}", prefix, CLOSE)),
    };
    commands.insert_resource(game_textures);
    
    let game_sounds = GameSounds {
        move_card: asset_server.load(format!("{}{}", prefix, MOVE_CARD)),
        move_to_stock: asset_server.load(format!("{}{}", prefix, MOVE_TO_STOCK)),
    };
    commands.insert_resource(game_sounds);
    
    let game_fonts = GameFonts {
        dot_gothic: asset_server.load(format!("{}{}", prefix, DOT_GOTHIC)),
    };
    commands.insert_resource(game_fonts);
}

fn wait_loading_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    game_sounds: Res<GameSounds>,
    game_fonts: Res<GameFonts>,
    asset_server: Res<AssetServer>,
) {
    let mut finished = true;

    if !game_textures.is_loaded(&asset_server) {
        finished = false;
    }
    if !game_sounds.is_loaded(&asset_server) {
        finished = false;
    }
    if !game_fonts.is_loaded(&asset_server) {
        finished = false;
    }

    if finished {
        commands.trigger(LoadEvent);
    }
}
