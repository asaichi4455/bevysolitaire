use bevy::{
    app::{App, Plugin, Startup},
    prelude::*,
};
use crate::{
    game::{GameState, MoveOneStepEvent, MoveStep},
    resources::GameSounds,
};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_system)
            .add_systems(OnEnter(GameState::Deal), deal_system);
    }
}

fn init_system(
    mut commands: Commands,
) {
    commands.add_observer(on_move_one_step);
}

fn deal_system(
    mut commands: Commands,
    game_sounds: Res<GameSounds>,
) {
    commands.spawn(AudioPlayer::new(game_sounds.move_to_stock.clone()));
}

fn on_move_one_step(
    trigger: Trigger<MoveOneStepEvent>,
    mut commands: Commands,
    game_sounds: Res<GameSounds>,
) {
    let audio_handle = match trigger.event().0 {
        MoveStep::WasteToStock => Some(game_sounds.move_to_stock.clone()),
        MoveStep::StockToWaste
        | MoveStep::WasteToPile
        | MoveStep::WasteToFoundation
        | MoveStep::PileToPile
        | MoveStep::PileToFoundation
        | MoveStep::FoundationToPile => Some(game_sounds.move_card.clone()),
        MoveStep::FaceupPile => None
    };

    if let Some(handle) = audio_handle {
        commands.spawn(AudioPlayer::new(handle));
    }
}
