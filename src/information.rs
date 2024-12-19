use bevy::{app::{App, Plugin, Update}, prelude::*};
use crate::{game::{AddScoreEvent, MoveOneStepEvent}, resources::GameFonts, GameState};

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct MovementText;

#[derive(Component)]
struct UpdateScore(i32);

#[derive(Component)]
struct UpdateMovement(u32);

#[derive(Resource)]
struct Time(f32);

#[derive(Resource)]
struct Score(i32);

#[derive(Resource)]
struct Movement(u32);

pub struct InformationPlugin;

impl Plugin for InformationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Loading), init_system)
            .add_systems(OnExit(GameState::Loading), spawn_ui)
            .add_systems(OnEnter(GameState::Prepare), reset_system)
            .add_systems(Update, update_time_system.run_if(in_state(GameState::Play)))
            .add_systems(Update, update_score_system)
            .add_systems(Update, update_movement_system);
    }
}

fn init_system(
    mut commands: Commands,
) {
    commands.insert_resource(Time(0.));
    commands.insert_resource(Score(0));
    commands.insert_resource(Movement(0));
    commands.add_observer(on_score);
    commands.add_observer(on_move_one_step);
}

fn reset_system(
    mut commands: Commands,
    mut time: ResMut<Time>,
    mut score: ResMut<Score>,
    mut movement: ResMut<Movement>,
) {
    time.0 = 0.;
    score.0 = 0;
    movement.0 = 0;

    commands.spawn(UpdateScore(0));
    commands.spawn(UpdateMovement(0));
}

fn spawn_ui(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
) {
    commands.spawn((
        Text::new("0:00:00"),
        TextFont {
            font: game_fonts.dot_gothic.clone(),
            font_size: 16.,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            width: Val::Px(130.),
            top: Val::Px(18.),
            left: Val::Px(89.),
            ..Default::default()
        },
        ZIndex(0),
        TimeText,
    ));
    commands.spawn((
        Text::new("スコア  0"),
        TextFont {
            font: game_fonts.dot_gothic.clone(),
            font_size: 16.,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            width: Val::Px(130.),
            top: Val::Px(18.),
            left: Val::Px(219.),
            ..Default::default()
        },
        ZIndex(0),
        ScoreText,
    ));
    commands.spawn((
        Text::new("移動回数  0"),
        TextFont {
            font: game_fonts.dot_gothic.clone(),
            font_size: 16.,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            width: Val::Px(130.),
            top: Val::Px(18.),
            left: Val::Px(349.),
            ..Default::default()
        },
        ZIndex(0),
        MovementText,
    ));
}

fn update_time_system(
    time: Res<bevy::prelude::Time>,
    mut res_time: ResMut<Time>,
    mut query: Query<&mut Text, With<TimeText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        res_time.0 += time.delta_secs();
        let time = res_time.0 as u32;
        let hour = time / 60 / 60;
        let minute = time / 60 % 60;
        let second = time % 60;
        text.0 = String::from(format!("{:01}:{:02}:{:02}", hour, minute, second));
    }
}

fn update_score_system(
    mut commands: Commands,
    mut res_score: ResMut<Score>,
    mut query_text: Query<&mut Text, With<ScoreText>>,
    query_update_score: Query<(Entity, &UpdateScore)>,
) {
    for (entity, update_score) in query_update_score.iter() {
        if let Ok(mut text) = query_text.get_single_mut() {
            let mut score = res_score.0 + update_score.0;
            if score < 0 {
                score = 0;
            }
            res_score.0 = score;
            text.0 = String::from(format!("スコア  {}", res_score.0));
        }
        commands.entity(entity).despawn();
    }
}

fn update_movement_system(
    mut commands: Commands,
    mut res_movement: ResMut<Movement>,
    mut query_text: Query<&mut Text, With<MovementText>>,
    query_update_movement: Query<(Entity, &UpdateMovement)>,
) {
    for (entity, update_movement) in query_update_movement.iter() {
        if let Ok(mut text) = query_text.get_single_mut() {
            res_movement.0 += update_movement.0;
            text.0 = String::from(format!("移動回数  {}", res_movement.0));
        }
        commands.entity(entity).despawn();
    }
}

fn on_score(
    trigger: Trigger<AddScoreEvent>,
    mut commands: Commands,
) {
    commands.spawn(UpdateScore(trigger.event().0));
}

fn on_move_one_step(
    _trigger: Trigger<MoveOneStepEvent>,
    mut commands: Commands,
) {
    commands.spawn(UpdateMovement(1));
}
