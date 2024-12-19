use bevy::{
    asset::AssetServer,
    audio::AudioSource,
    prelude::*
};

#[derive(Resource)]
pub struct GameTextures {
    pub background: Handle<Image>,
    pub card: Handle<Image>,
    pub window: Handle<Image>,
    pub close: Handle<Image>,
}

impl GameTextures {
    pub fn is_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.is_loaded(self.background.id())
        && asset_server.is_loaded(self.card.id())
        && asset_server.is_loaded(self.window.id())
        && asset_server.is_loaded(self.close.id())
    }
}

#[derive(Resource)]
pub struct GameSounds {
    pub move_card: Handle<AudioSource>,
    pub move_to_stock: Handle<AudioSource>,
}

impl GameSounds {
    pub fn is_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.is_loaded(self.move_card.id())
        && asset_server.is_loaded(self.move_to_stock.id())
    }
}

#[derive(Resource)]
pub struct GameFonts {
    pub dot_gothic: Handle<Font>,
}

impl GameFonts {
    pub fn is_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        asset_server.is_loaded(self.dot_gothic.id())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Resource)]
pub enum GameDifficulty {
    Easy,
    Hard,
}
