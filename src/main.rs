#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use audio::AudioPlugin;
use bevy::{
    app::App,
    prelude::*,
    window::{Window, WindowPlugin}
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use game::{GamePlugin, GameState};
use gamestate::GameStatePlugin;
use information::InformationPlugin;
use loading::LoadingPlugin;
use selectdifficulty::SelectDifficultyPlugin;

mod audio;
mod cardlist;
mod components;
mod game;
mod gamestate;
mod information;
mod loading;
mod resources;
mod selectdifficulty;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Solitaire".into(),
                resolution: (568., 320.).into(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(EmbeddedAssetPlugin::default())
        .add_plugins(GamePlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(InformationPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(SelectDifficultyPlugin)
        .insert_state(GameState::Loading)
        .run();
}
