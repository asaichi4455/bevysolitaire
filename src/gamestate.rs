use bevy::{
    app::{App, Plugin, Startup},
    prelude::*,
};
use crate::{
    game::{
        CancelNewGameEvent,
        DealEvent,
        DifficultySelectEvent,
        GameClearEvent,
        LoadEvent,
        NewGameEvent,
        PrepareEvent,
    },
    GameState,
};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_system);
    }
}

fn init_system(
    mut commands: Commands,
) {
    commands.add_observer(on_end_load);
    commands.add_observer(on_select_difficulty);
    commands.add_observer(on_end_prepare);
    commands.add_observer(on_end_deal);
    commands.add_observer(on_click_new_game);
    commands.add_observer(on_cancel_new_game);
    commands.add_observer(on_game_clear);
}

fn on_end_load(
    _trigger: Trigger<LoadEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::SelectDifficulty);
}

fn on_select_difficulty(
    _trigger: Trigger<DifficultySelectEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Prepare);
}

fn on_end_prepare(
    _trigger: Trigger<PrepareEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Deal);
}

fn on_end_deal(
    _trigger: Trigger<DealEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Play);
}

fn on_click_new_game(
    _trigger: Trigger<NewGameEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::NewGame);
}

fn on_cancel_new_game(
    _trigger: Trigger<CancelNewGameEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Play);
}

fn on_game_clear(
    _trigger: Trigger<GameClearEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::GameClear);
}
