use bevy::{
    app::{App, Plugin},
    prelude::*,
    ui::widget::NodeImageMode,
};
use crate::{
    game::{CancelNewGameEvent, DifficultySelectEvent},
    resources::{
        GameDifficulty,
        GameFonts,
        GameTextures,
    },
    GameState,
};

#[derive(Component)]
struct DifficultyButton(pub GameDifficulty);

#[derive(Component)]
pub struct UISelectDifficulty;

pub struct SelectDifficultyPlugin;

impl Plugin for SelectDifficultyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SelectDifficulty), spawn_ui)
            .add_systems(OnExit(GameState::SelectDifficulty), despawn_ui)
            .add_systems(OnEnter(GameState::GameClear), spawn_ui)
            .add_systems(OnExit(GameState::GameClear), despawn_ui)
            .add_systems(OnEnter(GameState::NewGame), spawn_ui)
            .add_systems(OnExit(GameState::NewGame), despawn_ui);
    }
}

fn spawn_ui(
    mut commands: Commands,
    state: Res<State<GameState>>,
    game_textures: Res<GameTextures>,
    game_fonts: Res<GameFonts>,
) {
    let visibility_close_button =
        if *state == GameState::SelectDifficulty || *state == GameState::GameClear {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };

    commands
        .spawn((
            Node {
                width: Val::Px(568.),
                height: Val::Px(320.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ZIndex(1),
            BackgroundColor(Color::srgba_u8(0, 0, 0, 230)),
            UISelectDifficulty,
        ))
        .with_children(|parent| {
            let slicer = TextureSlicer {
                border: BorderRect::square(7.),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.,
            };
            parent.
                spawn((
                    ImageNode {
                        image: game_textures.window.clone(),
                        image_mode: NodeImageMode::Sliced(slicer),
                        ..default()
                    },
                    Node {
                        width: Val::Px(226.),
                        height: Val::Px(128.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            ImageNode {
                                image: game_textures.close.clone(),
                                ..Default::default()
                            },
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Px(10.),
                                right: Val::Px(10.),
                                ..Default::default()
                            },
                            visibility_close_button,
                        ))
                        .observe(on_click_close);
                    parent
                        .spawn((
                            Text::new("難易度を選択"),
                            TextFont {
                                font: game_fonts.dot_gothic.clone(),
                                font_size: 16.,
                                ..Default::default()
                            },
                            TextColor(Color::srgb_u8(128, 128, 128)),
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Px(20.),
                                border: UiRect::all(Val::Px(1.)),
                                ..Default::default()
                            },
                            BorderColor(Color::srgb_u8(255, 0, 0)),
                        ));
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(100.),
                                height: Val::Px(40.),
                                top: Val::Px(30.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            DifficultyButton(GameDifficulty::Easy),
                        ))
                        .with_child((
                            Text::new("簡単"),
                            TextFont {
                                font: game_fonts.dot_gothic.clone(),
                                font_size: 16.,
                                ..Default::default()
                            },
                            TextColor(Color::BLACK),
                        ))
                        .observe(on_click_difficulty);
                    parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(100.),
                                height: Val::Px(40.),
                                top: Val::Px(30.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            DifficultyButton(GameDifficulty::Hard),
                        ))
                        .with_child((
                            Text::new("難しい"),
                            TextFont {
                                font: game_fonts.dot_gothic.clone(),
                                font_size: 16.,
                                ..Default::default()
                            },
                            TextColor(Color::BLACK),
                        ))
                        .observe(on_click_difficulty);
                });
        });
}

fn despawn_ui(
    mut commands: Commands,
    query: Query<Entity, With<UISelectDifficulty>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn on_click_close(
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    commands.trigger(CancelNewGameEvent);
}

fn on_click_difficulty(
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    query: Query<&DifficultyButton>,
) {
    if click.button != PointerButton::Primary {
        return;
    }
    
    if let Ok(button) = query.get(click.entity()) {
        match button.0 {
            GameDifficulty::Easy => {
                commands.remove_resource::<GameDifficulty>();
                commands.insert_resource(GameDifficulty::Easy);
            }
            GameDifficulty::Hard => {
                commands.remove_resource::<GameDifficulty>();
                commands.insert_resource(GameDifficulty::Hard);
            }
        }
        commands.trigger(DifficultySelectEvent);
    }
}
