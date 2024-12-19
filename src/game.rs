use std::time::Duration;
use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    text::TextBounds,
    time::common_conditions::on_timer
};
use rand::seq::SliceRandom;
use crate::resources::{
    GameDifficulty, GameFonts, GameTextures
};
use crate::cardlist::CardList;

const POSITION_STOCK: Vec2 = Vec2::new(-258., 63.);
const POSITION_WASTE: Vec2 = Vec2::new(-258., 1.);
const POSITION_PILES: [Vec2; NUM_PILES] = [
    Vec2::new(-193., 62.),
    Vec2::new(-129., 62.),
    Vec2::new(-65., 62.),
    Vec2::new(-1., 62.),
    Vec2::new(63., 62.),
    Vec2::new(127., 62.),
    Vec2::new(191., 62.),
];
const POSITION_FOUNDATIONS: [Vec2; NUM_SUIT] = [
    Vec2::new(257., 62.),
    Vec2::new(257., 2.),
    Vec2::new(257., -58.),
    Vec2::new(257., -118.),
];
const NUM_SUIT: usize = 4;
const NUM_PILES: usize = 7;
const MAX_WASTES: u32 = 3;

const CARD_SIZE: Vec2 = Vec2::new(38., 52.);
const OFFSET_WASTE_Y: f32 = 16.;
const OFFSET_PILE_Y: f32 = 16.;
const OFFSET_PILE_Y_MIN: f32 = 6.;
const PILE_DROP_AREA: [Aabb2d; NUM_PILES] = [
    Aabb2d {
        min: Vec2::new(POSITION_PILES[0].x - CARD_SIZE.x / 2., -144.),
        max: Vec2::new(POSITION_PILES[0].x + CARD_SIZE.x / 2., 88.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_PILES[1].x - CARD_SIZE.x / 2., -144.),
        max: Vec2::new(POSITION_PILES[1].x + CARD_SIZE.x / 2., 88.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_PILES[2].x - CARD_SIZE.x / 2., -144.),
        max: Vec2::new(POSITION_PILES[2].x + CARD_SIZE.x / 2., 88.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_PILES[3].x - CARD_SIZE.x / 2., -144.),
        max: Vec2::new(POSITION_PILES[3].x + CARD_SIZE.x / 2., 88.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_PILES[4].x - CARD_SIZE.x / 2., -144.),
        max: Vec2::new(POSITION_PILES[4].x + CARD_SIZE.x / 2., 88.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_PILES[5].x - CARD_SIZE.x / 2., -144.),
        max: Vec2::new(POSITION_PILES[5].x + CARD_SIZE.x / 2., 88.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_PILES[6].x - CARD_SIZE.x / 2., -144.),
        max: Vec2::new(POSITION_PILES[6].x + CARD_SIZE.x / 2., 88.),
    },
];
const FOUNDATION_DROP_AREA: [Aabb2d; NUM_SUIT] = [
    Aabb2d {
        min: Vec2::new(POSITION_FOUNDATIONS[0].x - CARD_SIZE.x / 2., POSITION_FOUNDATIONS[0].y - CARD_SIZE.y / 2.),
        max: Vec2::new(POSITION_FOUNDATIONS[0].x + CARD_SIZE.x / 2., POSITION_FOUNDATIONS[0].y + CARD_SIZE.y / 2.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_FOUNDATIONS[1].x - CARD_SIZE.x / 2., POSITION_FOUNDATIONS[1].y - CARD_SIZE.y / 2.),
        max: Vec2::new(POSITION_FOUNDATIONS[1].x + CARD_SIZE.x / 2., POSITION_FOUNDATIONS[1].y + CARD_SIZE.y / 2.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_FOUNDATIONS[2].x - CARD_SIZE.x / 2., POSITION_FOUNDATIONS[2].y - CARD_SIZE.y / 2.),
        max: Vec2::new(POSITION_FOUNDATIONS[2].x + CARD_SIZE.x / 2., POSITION_FOUNDATIONS[2].y + CARD_SIZE.y / 2.),
    },
    Aabb2d {
        min: Vec2::new(POSITION_FOUNDATIONS[3].x - CARD_SIZE.x / 2., POSITION_FOUNDATIONS[3].y - CARD_SIZE.y / 2.),
        max: Vec2::new(POSITION_FOUNDATIONS[3].x + CARD_SIZE.x / 2., POSITION_FOUNDATIONS[3].y + CARD_SIZE.y / 2.),
    },
];

const DRAG_CARD_Z: f32 = 100.;
const DRAG_DISTANCE_THRESHOLD: f32 = 5.;

const ATLAS_INDEX_FACEDOWN: usize = 52;
const ATLAS_INDEX_STOCK_BASE: usize = 57;
const ATLAS_INDEX_FOUNDATION_BASE: [usize; NUM_SUIT] = [55, 56, 54, 53];

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Loading,
    SelectDifficulty,
    Prepare,
    Deal,
    Play,
    NewGame,
    GameClear,
}

pub enum MoveStep {
    StockToWaste,
    WasteToStock,
    WasteToPile,
    WasteToFoundation,
    PileToPile,
    PileToFoundation,
    FoundationToPile,
    FaceupPile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CardSuit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CardType {
    Stock,
    Waste,
    Pile(u32),
    Foundation(CardSuit),
}

#[derive(Clone, Copy, Component)]
pub struct Card;

pub struct CardInfo {
    pub entity: Entity,
    pub card_suit: CardSuit,
    pub card_number: u32,
    pub card_type: CardType,
    pub order: i32,
    pub facedown: bool,
    pub prev_position: Vec3,
    pub dst_position: Vec3,
    pub clickable: bool,
    pub dragging: bool,
}

impl Default for CardInfo {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            card_suit: CardSuit::Heart,
            card_number: 1,
            card_type: CardType::Stock,
            order: 0,
            facedown: true,
            prev_position: Vec3::new(0., 0., 0.),
            dst_position: Vec3::new(0., 0., 0.),
            clickable: false,
            dragging: false,
        }
    }
}

#[derive(Component)]
struct StackInfo {
    target: Entity,
    dst: CardType,
    order: i32,
}

#[derive(Component)]
struct FillWaste;

#[derive(Component)]
struct AdjustPile {
    index: u32,
}

#[derive(Component)]
struct UpdateZ {
    target: Entity,
    value: f32,
}

#[derive(Component)]
struct UpdateSprite {
    target: Entity,
}

#[derive(Component)]
struct GameClear;

#[derive(Event)]
pub struct LoadEvent;

#[derive(Event)]
pub struct DifficultySelectEvent;

#[derive(Event)]
pub struct PrepareEvent;

#[derive(Event)]
pub struct DealEvent;

#[derive(Event)]
pub struct NewGameEvent;

#[derive(Event)]
pub struct CancelNewGameEvent;

#[derive(Event)]
pub struct GameClearEvent;

#[derive(Event)]
pub struct MoveOneStepEvent(pub MoveStep);

#[derive(Event)]
pub struct AddScoreEvent(pub i32);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::Loading), init_system)
            .add_systems(OnEnter(GameState::Prepare), prepare_system)
            .add_systems(OnEnter(GameState::Deal), deal_system)
            .add_systems(
                Update,
                movement_system.run_if(
                    in_state(GameState::Deal)
                    .or(in_state(GameState::Play))
                    .or(in_state(GameState::GameClear))
                    .and(on_timer(Duration::from_secs_f32(0.03)))
                )
            )
            .add_systems(
                Update,
                stack_system.run_if(in_state(GameState::Play))
            )
            .add_systems(
                Update,
                fill_waste_system.run_if(in_state(GameState::Play))
            )
            .add_systems(
                Update,
                adjust_pile_system.run_if(in_state(GameState::Play))
            )
            .add_systems(
                Update,
                update_z_system.run_if(
                    in_state(GameState::Prepare)
                    .or(in_state(GameState::Deal)
                    .or(in_state(GameState::Play)))
                )
            )
            .add_systems(
                Update,
                update_sprite_system.run_if(
                    in_state(GameState::Prepare)
                    .or(in_state(GameState::Deal)
                    .or(in_state(GameState::Play)))
                )
            )
            .add_systems(
                Update,
                game_clear_system.run_if(in_state(GameState::Play))
            );
    }
}

/// 初期化
fn init_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    game_fonts: Res<GameFonts>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(38, 52), 8, 8, None, None);
    let layout = texture_atlases.add(texture_atlas);

    // カメラ
    commands.spawn(Camera2d);

    // 背景
    commands.spawn((
        Sprite::from_image(game_textures.background.clone()),
        Transform::from_translation(Vec3::new(0., 0., -2.)),
    ));

    // 山札ベース
    commands.spawn((
        Sprite {
            image: game_textures.card.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: ATLAS_INDEX_STOCK_BASE,
            }),
            ..Default::default()
        },
        Transform::from_translation(
            Vec3::new(POSITION_STOCK.x, POSITION_STOCK.y, -1.)
        ),
    ))
    .observe(on_click_stock_base);

    // 組札ベース
    for index in 0..NUM_SUIT {
        commands.spawn((
            Sprite {
                image: game_textures.card.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: layout.clone(),
                    index: ATLAS_INDEX_FOUNDATION_BASE[index],
                }),
                ..Default::default()
            },
            Transform::from_translation(
                Vec3::new(POSITION_FOUNDATIONS[index].x, POSITION_FOUNDATIONS[index].y, -1.)
            ),
        ));
    }

    // 新しいゲーム
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(110., 32.)),
            color: Color::Srgba(Srgba::NONE),
            ..Default::default()
        },
        Text2d::new("新しいゲーム"),
        TextFont {
            font: game_fonts.dot_gothic.clone(),
            font_size: 16.,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextBounds::from(Vec2::new(110., 32.)),
        Transform::from_translation(Vec3::new(140., -130., -1.)),
    ))
    .observe(on_click_new_game);

    // カード
    let mut card_list = Vec::new();
    for suit in [CardSuit::Heart, CardSuit::Diamond, CardSuit::Club, CardSuit::Spade] {
        for number in 1..=13 {
            let entity = commands.spawn((
                Card,
                Sprite {
                    image: game_textures.card.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: layout.clone(),
                        index: ATLAS_INDEX_FACEDOWN,
                    }),
                    ..Default::default()
                },
                Visibility::Hidden
            ))
            .observe(on_click_card)
            .observe(on_drag_start)
            .observe(on_drag)
            .observe(on_drag_end)
            .id();

            let card = CardInfo {
                entity: entity,
                card_suit: suit,
                card_number: number,
                ..Default::default()
            };
            card_list.push(card);
        }
    }

    commands.insert_resource(CardList(card_list));
}

/// シャッフルして山札に配置
fn prepare_system(
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
    mut query: Query<(&mut Transform, &mut Visibility), With<Card>>,
) {
    let mut suit_and_num = Vec::new();
    for suit in [CardSuit::Heart, CardSuit::Diamond, CardSuit::Club, CardSuit::Spade] {
        for number in 1..=13 {
            suit_and_num.push((suit, number));
        }
    }
    let mut rng = rand::thread_rng();
    suit_and_num.shuffle(&mut rng);

    let mut order = 0;
    for card in card_list.0.iter_mut() {
        if let Some((suit, number)) = suit_and_num.pop() {
            card.card_suit = suit;
            card.card_number = number;
            card.card_type = CardType::Stock;
            card.order = order;
            card.clickable = true;

            card.facedown = true;
            commands.spawn(UpdateSprite {
                target: card.entity,
            });

            let position = Vec3::new(POSITION_STOCK.x, POSITION_STOCK.y, order as f32);
            card.dst_position = position;
            if let Ok((mut transform, mut visibility)) = query.get_mut(card.entity) {
                transform.translation = position;
                *visibility = Visibility::Visible;
            }
            
            commands.spawn(UpdateZ {
                target: card.entity,
                value: order as f32,
            });

            order += 1;
        }
    }

    commands.trigger(PrepareEvent);
}

/// カードを配る
fn deal_system(
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
) {
    let mut pile_index = 0;
    let mut order = 0;
    for card in card_list.0.iter_mut() {
        card.card_type = CardType::Pile(pile_index);
        card.order = order;

        let facedown = order != pile_index as i32;
        card.facedown = facedown;
        commands.spawn(UpdateSprite {
            target: card.entity,
        });

        let position = calc_pile_position(pile_index, order, pile_index, 1);
        card.dst_position = position;
        card.clickable = !facedown;

        order += 1;
        if order > pile_index as i32 {
            pile_index += 1;
            order = 0;
        }
        if pile_index >= NUM_PILES as u32 {
            break;
        }
    }

    commands.trigger(DealEvent);
}

/// カードの移動
fn movement_system(
    mut card_list: ResMut<CardList>,
    mut query: Query<&mut Transform, With<Card>>,
) {
    for card in card_list.0.iter_mut() {
        if let Ok(transform) = query.get(card.entity) {
            let dst_position = card.dst_position;
            let mut position = transform.translation;

            if position.xy().distance(dst_position.xy()) > 1. {
                position.x = position.x.lerp(dst_position.x, 0.4);
                position.y = position.y.lerp(dst_position.y, 0.4);
                if position.xy().distance(dst_position.xy()) <= 1. {
                    (position.x, position.y, position.z) =
                        (dst_position.x, dst_position.y, dst_position.z);
                }
                if let Ok(mut transform) = query.get_mut(card.entity) {
                    transform.translation = position;
                }
            }
        }
    }
}

/// カードの移動判定
fn stack_system(
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
    difficulty: Res<GameDifficulty>,
    query: Query<(Entity, &StackInfo)>,
) {
    for (entity, stackinfo) in query.iter() {
        let mut dst_order = 0;
        let mut dst_card_type = CardType::Stock;
        let mut dst_position = Vec3::default();
        let mut src_card_type = CardType::Stock;
        let mut src_order = 0;
        let mut move_step = MoveStep::StockToWaste;
        let mut should_move = false;

        if let Some(card) = card_list.get(stackinfo.target) {
            src_card_type = card.card_type;
            src_order = card.order;

            match (&card.card_type, &stackinfo.dst) {
                // 山札からめくったカード -> 場札の移動判定
                (CardType::Waste, CardType::Pile(index)) => {
                    dst_order = stackinfo.order;
                    dst_card_type = CardType::Pile(*index);
                    let pos = calc_pile_position(
                        *index,
                        stackinfo.order,
                        card_list.num_facedown(*index),
                        card_list.num_faceup(*index)
                    );
                    dst_position = Vec3::new(pos.x, pos.y, stackinfo.order as f32);

                    move_step = MoveStep::WasteToPile;
                    should_move = true;

                    commands.spawn(FillWaste{});
                    commands.trigger(AddScoreEvent(get_score(MoveStep::WasteToPile)));
                }

                // 山札からめくったカード -> 組札の移動判定
                (CardType::Waste, CardType::Foundation(suit)) => {
                    dst_order = stackinfo.order;
                    dst_card_type = CardType::Foundation(*suit);
                    let pos = POSITION_FOUNDATIONS[*suit as usize];
                    dst_position = Vec3::new(pos.x, pos.y, stackinfo.order as f32);

                    move_step = MoveStep::WasteToFoundation;
                    should_move = true;

                    if is_game_clear(&card_list.0, num_turn_to_waste(*difficulty)) {
                        commands.spawn(GameClear{});
                    } else {
                        commands.spawn(FillWaste{});
                    }
                    commands.trigger(AddScoreEvent(get_score(MoveStep::WasteToFoundation)));
                }

                // 場札 -> 場札の移動判定
                (CardType::Pile(_), CardType::Pile(index)) => {
                    dst_order = stackinfo.order;
                    dst_card_type = CardType::Pile(*index);
                    let pos = calc_pile_position(
                        *index,
                        stackinfo.order,
                        card_list.num_facedown(*index),
                        card_list.num_faceup(*index)
                    );
                    dst_position = Vec3::new(pos.x, pos.y, stackinfo.order as f32);

                    move_step = MoveStep::PileToPile;
                    should_move = true;

                    commands.trigger(AddScoreEvent(get_score(MoveStep::PileToPile)));
                }

                // 場札 -> 組札の移動判定
                (CardType::Pile(_), CardType::Foundation(suit)) => {
                    dst_order = stackinfo.order;
                    dst_card_type = CardType::Foundation(*suit);
                    let pos = POSITION_FOUNDATIONS[*suit as usize];
                    dst_position = Vec3::new(pos.x, pos.y, stackinfo.order as f32);

                    move_step = MoveStep::PileToFoundation;
                    should_move = true;

                    if is_game_clear(&card_list.0, num_turn_to_waste(*difficulty)) {
                        commands.spawn(GameClear{});
                    }
                    commands.trigger(AddScoreEvent(get_score(MoveStep::PileToFoundation)));
                }

                // 組札 -> 場札の移動判定
                (CardType::Foundation(_), CardType::Pile(index)) => {
                    dst_order = stackinfo.order;
                    dst_card_type = CardType::Pile(*index);
                    let pos = calc_pile_position(
                        *index,
                        stackinfo.order,
                        card_list.num_facedown(*index),
                        card_list.num_faceup(*index)
                    );
                    dst_position = Vec3::new(pos.x, pos.y, stackinfo.order as f32);

                    move_step = MoveStep::FoundationToPile;
                    should_move = true;

                    commands.trigger(AddScoreEvent(get_score(MoveStep::FoundationToPile)));
                }

                (_, _) => {}
            }
        }

        // カードを移動
        if let Some(card) = card_list.get_mut(stackinfo.target) {
            if should_move {
                card.order = dst_order;
                card.card_type = dst_card_type;
                card.dst_position = dst_position;
                commands.trigger(MoveOneStepEvent(move_step));
            } else {
                card.dst_position = card.prev_position;
            }
        }

        if let CardType::Pile(index) = src_card_type {
            if let Some(mut cards) = card_list.get_pile_cards_mut(index) {
                if should_move {
                    // つながっていたカードを移動
                    let mut offset = 1;
                    for card in cards.iter_mut() {
                        if card.order > src_order {
                            card.order = dst_order + offset;
                            card.card_type = dst_card_type;
                            offset += 1;
                        }
                    }
                } else {
                    // つながっていたカードをもどす
                    for card in cards {
                        if card.order > src_order {
                            card.dst_position = card.prev_position;
                        }
                    }
                }
            }

            // 移動元の場札列の手前のカードをめくる
            if should_move {
                if let Some(mut cards) = card_list.get_pile_cards_mut(index) {
                    if let Some(last) = cards.last_mut() {
                        if last.facedown {
                            last.clickable = true;
                            last.facedown = false;
                            commands.spawn(UpdateSprite {
                                target: last.entity,
                            });
                            commands.trigger(AddScoreEvent(get_score(MoveStep::FaceupPile)));
                        }
                    }
                }
            }
        }

        // 場札の表示位置を更新
        if should_move {
            if let CardType::Pile(index) = src_card_type {
                commands.spawn(AdjustPile {
                    index: index,
                });
            }
            if let CardType::Pile(index) = dst_card_type {
                commands.spawn(AdjustPile {
                    index: index,
                });
            }
        }

        commands.entity(entity).despawn();
    }
}

/// 山札から引いたカードの補充
fn fill_waste_system(
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
    query_fill_waste: Query<Entity, With<FillWaste>>,
) {
    for entity in query_fill_waste.iter() {
        if let Some(mut waste_cards) = card_list.get_waste_cards_mut() {
            waste_cards.reverse();
            waste_cards.truncate(MAX_WASTES as usize);
            waste_cards.reverse();

            let mut order = 0;
            for card in waste_cards.iter_mut() {
                card.order = order;
                card.clickable = false;

                let mut position = POSITION_WASTE;
                position.y -= order as f32 * OFFSET_WASTE_Y;
                (card.dst_position.x, card.dst_position.y) = (position.x, position.y);

                order += 1;
            }

            if let Some(card) = waste_cards.last_mut() {
                card.clickable = true;
            }
        }
        commands.entity(entity).despawn();
    }
}

/// 場札の表示位置の調整
fn adjust_pile_system(
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
    query: Query<(Entity, &AdjustPile)>,
) {
    for (entity, adjustpile) in query.iter() {
        let num_facedown = card_list.num_facedown(adjustpile.index);
        let num_faceup = card_list.num_faceup(adjustpile.index);

        if let Some(cards) = card_list.get_pile_cards_mut(adjustpile.index) {
            for card in cards {
                let pos = calc_pile_position(
                    adjustpile.index,
                    card.order,
                    num_facedown,
                    num_faceup
                );
                card.dst_position = pos;
            }
        }
        commands.entity(entity).despawn();
    }
}

/// カード表示順の更新
fn update_z_system(
    mut commands: Commands,
    card_list: ResMut<CardList>,
    mut query_transform: Query<&mut Transform, With<Card>>,
    query_update_z: Query<(Entity, &UpdateZ)>,
) {
    for (entity, update_z) in query_update_z.iter() {
        if let Some(card) = card_list.get(update_z.target) {
            if let Ok(mut transform) = query_transform.get_mut(card.entity) {
                transform.translation.z = update_z.value;
            }
        }
        commands.entity(entity).despawn();
    }
}

/// カード画像の更新
fn update_sprite_system(
    mut commands: Commands,
    card_list: ResMut<CardList>,
    mut query_sprite: Query<&mut Sprite, With<Card>>,
    query_update_sprite: Query<(Entity, &UpdateSprite)>,
) {
    for (entity, update_sprite) in query_update_sprite.iter() {
        if let Some(card) = card_list.get(update_sprite.target) {
            if let Ok(mut sprite) = query_sprite.get_mut(card.entity) {
                if card.facedown {
                    facedown_card(sprite.texture_atlas.as_mut());
                } else {
                    faceup_card(card.card_suit, card.card_number, sprite.texture_atlas.as_mut());
                }
            }
        }
        commands.entity(entity).despawn();
    }
}

/// ゲームクリア時処理
fn game_clear_system(
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
    query: Query<Entity, With<GameClear>>,
) {
    for entity in query.iter() {
        for card in card_list.0.iter_mut() {
            card.card_type = CardType::Foundation(card.card_suit);
            card.order = card.card_number as i32;

            let position = POSITION_FOUNDATIONS[card.card_suit as usize];
            card.dst_position = Vec3::new(position.x, position.y, card.order as f32);

            commands.spawn(UpdateZ {
                target: card.entity,
                value: card.card_number as f32,
            });
        }
        commands.trigger(GameClearEvent);

        commands.entity(entity).despawn();
    }
}

/// 「新しいゲーム」クリック時処理
fn on_click_new_game(
    click: Trigger<Pointer<Click>>,
    state: Res<State<GameState>>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    if *state == GameState::Play {
        commands.trigger(NewGameEvent);
    }
}

/// 山札ベースクリック時処理
fn on_click_stock_base(
    click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    // すべて山札に戻す
    if let Some(waste_cards) = card_list.get_waste_cards_mut() {
        let mut count = 0;
        for card in waste_cards {
            card.card_type = CardType::Stock;
            card.clickable = true;
            card.order = count;

            card.facedown = true;
            commands.spawn(UpdateSprite {
                target: card.entity,
            });
            
            let pos = POSITION_STOCK;
            card.dst_position = Vec3::new(pos.x, pos.y, count as f32);
            
            count += 1;
        }

        commands.trigger(MoveOneStepEvent(MoveStep::WasteToStock));
        commands.trigger(AddScoreEvent(get_score(MoveStep::WasteToStock)));
    }
}

/// カードクリック時処理
fn on_click_card(
    click: Trigger<Pointer<Click>>,
    commands: Commands,
    card_list: ResMut<CardList>,
    difficulty: Res<GameDifficulty>,
) {
    if click.button != PointerButton::Primary {
        return;
    }
    
    if let Some(card) = card_list.get(click.entity()) {
        if card.clickable {
            match card.card_type {
                CardType::Stock => on_click_stock(commands, card_list, difficulty),
                CardType::Waste => on_click_waste(commands, card_list, click.entity()),
                CardType::Pile(_) => on_click_pile(commands, card_list, click.entity()),
                CardType::Foundation(_) => on_click_foundation(commands, card_list, click.entity()),
            }
        }
    }
}

/// 山札クリック時処理
fn on_click_stock(
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
    difficulty: Res<GameDifficulty>,
) {
    let mut num_turn_cards =
        if let Some(turn_cards) = card_list.get_turn_cards_mut(num_turn_to_waste(*difficulty)) {
            turn_cards.len() as u32
        } else {
            0
        };
    let num_waste_cards =
        if let Some(cards) = card_list.get_waste_cards_mut() {
            cards.len() as u32
        } else {
            0
        };

    // 山札を難易度に応じて1枚か3枚めくる
    if let Some(mut waste_top) = card_list.get_waste_cards_mut() {
        // すでにめくられたカードを奥に移動
        waste_top.reverse();
        waste_top.truncate(MAX_WASTES as usize);
        waste_top.reverse();

        for card in waste_top.iter_mut() {
            card.clickable = false;
        }

        let mut count = 0;
        let num_waste_top = waste_top.len() as i32;
        if num_waste_top + num_turn_cards as i32 > MAX_WASTES as i32 {
            for card in waste_top.iter_mut() {
                let mut order = count - (num_waste_top + num_turn_cards as i32 - MAX_WASTES as i32);
                order = order.clamp(0, MAX_WASTES as i32 - 1);
                card.order = order;
    
                let mut pos = POSITION_WASTE;
                pos.y -= order as f32 * OFFSET_WASTE_Y;
                card.dst_position.y = pos.y;
    
                count += 1;
            }
        }
    }

    // めくったカードを移動
    if let Some(mut turn_cards) = card_list.get_turn_cards_mut(num_turn_to_waste(*difficulty)) {
        num_turn_cards = turn_cards.len() as u32;
        let mut count = 0;
        for card in turn_cards.iter_mut() {
            card.card_type = CardType::Waste;
            card.clickable = false;
            
            let mut order = num_waste_cards.clamp(0, MAX_WASTES) as i32;
            if order + num_turn_cards as i32 - 1 > MAX_WASTES as i32 - 1 {
                order -= order + num_turn_cards as i32 - 1 - (MAX_WASTES as i32 - 1);
            }
            order += count;
            card.order = order;
            
            let mut pos = POSITION_WASTE;
            pos.y -= order as f32 * OFFSET_WASTE_Y;
            (card.dst_position.x, card.dst_position.y) = (pos.x, pos.y);

            let z = num_waste_cards as f32 + num_turn_cards as f32 + order as f32;
            card.dst_position = Vec3::new(pos.x, pos.y, z);

            commands.spawn(UpdateZ {
                target: card.entity,
                value: z,
            });

            card.facedown = false;
            commands.spawn(UpdateSprite {
                target: card.entity,
            });

            count += 1;
        }

        if let Some(card) = turn_cards.last_mut() {
            card.clickable = true;
        }

        commands.trigger(MoveOneStepEvent(MoveStep::StockToWaste));
        commands.trigger(AddScoreEvent(get_score(MoveStep::StockToWaste)));
    }
}

/// 山札から引いたカードのクリック時処理
fn on_click_waste(
    mut commands: Commands,
    card_list: ResMut<CardList>,
    card: Entity,
) {
    if let Some(card) = card_list.get(card) {
        if card.dragging {
            return;
        }

        if card_list.can_stack_foundation(card) {
            commands.spawn(StackInfo {
                target: card.entity,
                dst: CardType::Foundation(card.card_suit),
                order: card.card_number as i32,
            });
            commands.spawn(UpdateZ {
                target: card.entity,
                value: DRAG_CARD_Z,
            });
        } else {
            for index in 0..NUM_PILES {
                if let Some(order) = card_list.can_stack_pile(index as u32, card) {
                    commands.spawn(StackInfo {
                        target: card.entity,
                        dst: CardType::Pile(index as u32),
                        order: order,
                    });
                    commands.spawn(UpdateZ {
                        target: card.entity,
                        value: DRAG_CARD_Z,
                    });
                    break;
                }
            }
        }
    }
}

/// 場札クリック時処理
fn on_click_pile(
    mut commands: Commands,
    card_list: ResMut<CardList>,
    card: Entity,
) {
    if let Some(card) = card_list.get(card) {
        if card.dragging {
            return;
        }
        
        if card_list.can_stack_foundation(card) {
            if let None = card_list.get_connected_cards(card.entity) {
                commands.spawn(StackInfo {
                    target: card.entity,
                    dst: CardType::Foundation(card.card_suit),
                    order: card.card_number as i32,
                });
                commands.spawn(UpdateZ {
                    target: card.entity,
                    value: DRAG_CARD_Z,
                });
                return;
            }
        }
        for index in 0..NUM_PILES {
            if let Some(order) = card_list.can_stack_pile(index as u32, card) {
                commands.spawn(StackInfo {
                    target: card.entity,
                    dst: CardType::Pile(index as u32),
                    order: order,
                });
                commands.spawn(UpdateZ {
                    target: card.entity,
                    value: DRAG_CARD_Z,
                });
                if let Some(cards) = card_list.get_connected_cards(card.entity) {
                    for card in cards {
                        commands.spawn(UpdateZ {
                            target: card.entity,
                            value: DRAG_CARD_Z + card.order as f32,
                        });
                    }
                }
                break;
            }
        }
    }
}

/// 組札クリック時処理
fn on_click_foundation(
    mut commands: Commands,
    card_list: ResMut<CardList>,
    card: Entity,
) {
    if let Some(card) = card_list.get(card) {
        if card.dragging {
            return;
        }
        
        for index in 0..NUM_PILES {
            if let Some(order) = card_list.can_stack_pile(index as u32, card) {
                commands.spawn(StackInfo {
                    target: card.entity,
                    dst: CardType::Pile(index as u32),
                    order: order,
                });
                commands.spawn(UpdateZ {
                    target: card.entity,
                    value: DRAG_CARD_Z,
                });
                break;
            }
        }
    }
}

/// カードのドラッグ開始時処理
fn on_drag_start(
    drag: Trigger<Pointer<DragStart>>,
    mut card_list: ResMut<CardList>,
) {
    if drag.button != PointerButton::Primary {
        return;
    }
    
    if let Some(card) = card_list.get_mut(drag.entity()) {
        if !card.clickable || card.card_type == CardType::Stock {
            return;
        }
        if card.clickable {
            card.prev_position = card.dst_position;
        }
    }

    if let Some(cards) = card_list.get_connected_cards_mut(drag.entity()) {
        for card in cards {
            card.prev_position = card.dst_position;
        }
    }
}

/// カードのドラッグ中処理
fn on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut card_list: ResMut<CardList>,
    mut query: Query<&mut Transform, With<Card>>,
) {
    if drag.button != PointerButton::Primary {
        return;
    }
    
    let mut card_type = CardType::Stock;
    if let Some(card) = card_list.get(drag.entity()) {
        if !card.clickable || card.card_type == CardType::Stock {
            return;
        }
        card_type = card.card_type;
    }

    if let Some(card) = card_list.get_mut(drag.entity()) {
        if let Ok(mut transform) = query.get_mut(card.entity) {
            let mut position = transform.translation;
            position.x += drag.delta.x;
            position.y -= drag.delta.y;
            position.z = DRAG_CARD_Z + card.order as f32;
            transform.translation = position;
            card.dst_position = position;
        }
        if drag.distance.length() > DRAG_DISTANCE_THRESHOLD {
            card.dragging = true;
        }
    }

    if let CardType::Pile(_) = card_type {
        if let Some(cards) = card_list.get_connected_cards_mut(drag.entity()) {
            for card in cards {
                if let Ok(mut transform) = query.get_mut(card.entity) {
                    let mut position = transform.translation;
                    position.x += drag.delta.x;
                    position.y -= drag.delta.y;
                    position.z = DRAG_CARD_Z + card.order as f32;
                    transform.translation = position;
                    card.dst_position = position;
                }
            }
        }
    }
}

/// カードのドラッグ終了時処理
fn on_drag_end(
    drag: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    mut card_list: ResMut<CardList>,
    query: Query<&Transform, With<Card>>,
) {
    if drag.button != PointerButton::Primary {
        return;
    }

    let mut can_stack = false;
    let mut card_type = CardType::Stock;

    if let Some(card) = card_list.get(drag.entity()) {
        if !card.clickable || card.card_type == CardType::Stock {
            return;
        }

        card_type = card.card_type;

        if let Ok(transform) = query.get(card.entity) {
            let card_aabb = Aabb2d::new(transform.translation.truncate(), CARD_SIZE / 2.);

            match card.card_type {
                CardType::Stock => {}

                // 山札からめくったカードの移動
                CardType::Waste => {
                    if card_aabb.intersects(&FOUNDATION_DROP_AREA[card.card_suit as usize])
                    && card_list.can_stack_foundation(card) {
                        commands.spawn(StackInfo {
                            target: card.entity,
                            dst: CardType::Foundation(card.card_suit),
                            order: card.card_number as i32,
                        });
                        can_stack = true;
                    } else {
                        for index in 0..NUM_PILES {
                            if card_aabb.intersects(&PILE_DROP_AREA[index]) {
                                if let Some(order) = card_list.can_stack_pile(index as u32, card) {
                                    commands.spawn(StackInfo {
                                        target: card.entity,
                                        dst: CardType::Pile(index as u32),
                                        order: order,
                                    });
                                    can_stack = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                // 場札の移動
                CardType::Pile(_) => {
                    if card_aabb.intersects(&FOUNDATION_DROP_AREA[card.card_suit as usize]) {
                        if let None = card_list.get_connected_cards(card.entity) {
                            if card_list.can_stack_foundation(card) {
                                commands.spawn(StackInfo {
                                    target: card.entity,
                                    dst: CardType::Foundation(card.card_suit),
                                    order: card.card_number as i32,
                                });
                                can_stack = true;
                            }
                        }
                    } else {
                        for index in 0..NUM_PILES {
                            if card_aabb.intersects(&PILE_DROP_AREA[index]) {
                                if let Some(order) = card_list.can_stack_pile(index as u32, card) {
                                    commands.spawn(StackInfo {
                                        target: card.entity,
                                        dst: CardType::Pile(index as u32),
                                        order: order,
                                    });
                                    can_stack = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                
                // 組札の移動
                CardType::Foundation(_) => {
                    for index in 0..NUM_PILES {
                        if card_aabb.intersects(&PILE_DROP_AREA[index]) {
                            if let Some(order) = card_list.can_stack_pile(index as u32, card) {
                                commands.spawn(StackInfo {
                                    target: card.entity,
                                    dst: CardType::Pile(index as u32),
                                    order: order,
                                });
                                can_stack = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(card) = card_list.get_mut(drag.entity()) {
        if !can_stack {
            card.dst_position = card.prev_position;
        }
        card.dragging = false;
    }

    if let CardType::Pile(_) = card_type {
        if !can_stack {
            if let Some(cards) = card_list.get_connected_cards_mut(drag.entity()) {
                for card in cards {
                    card.dst_position = card.prev_position;
                }
            }
        }
    }
}

/// カード裏向き処理
fn facedown_card(atlas: Option<&mut TextureAtlas>) {
    if let Some(atlas) = atlas {
        atlas.index = ATLAS_INDEX_FACEDOWN;
    }
}

/// カード表向き処理
fn faceup_card(suit: CardSuit, number: u32, atlas: Option<&mut TextureAtlas>) {
    if let Some(atlas) = atlas {
        if let Some(index) = get_atlas_index(suit, number){
            atlas.index = index;
        }
    }
}

/// 組札の表示位置計算処理
fn calc_pile_position(
    pile_index: u32,
    order: i32,
    num_facedown: u32,
    num_faceup: u32
) -> Vec3 {
    let mut offset_facedown = OFFSET_PILE_Y;
    let mut offset_faceup = OFFSET_PILE_Y;
    let card_height =  CARD_SIZE.y;
    let pile_drop_area_height =
        (PILE_DROP_AREA[pile_index as usize].max - PILE_DROP_AREA[pile_index as usize].min).y;

    let mut height_over =
        card_height + (num_facedown + num_faceup) as f32 * OFFSET_PILE_Y - pile_drop_area_height;
    if height_over > 0. {
        offset_facedown -= (height_over / num_facedown as f32).ceil();
        offset_facedown = offset_facedown.clamp(OFFSET_PILE_Y_MIN, OFFSET_PILE_Y);
        height_over =
            card_height + num_facedown as f32 * offset_facedown + (num_faceup - 1) as f32 * offset_faceup - pile_drop_area_height;
        if height_over > 0. {
            offset_faceup -= (height_over / (num_faceup - 1) as f32).ceil();
            offset_faceup = offset_faceup.clamp(OFFSET_PILE_Y_MIN, OFFSET_PILE_Y);
        }
    }

    let xy = POSITION_PILES[pile_index as usize];
    let mut position = Vec3::new(xy.x, xy.y, order as f32);
    for i in 0..(num_facedown + num_faceup) {
        if i == order as u32 {
            break;
        }
        position.y -= if i < num_facedown {
            offset_facedown
        } else {
            offset_faceup
        };
    }
    
    position
}

/// ゲームクリア判定
pub fn is_game_clear(cards: &Vec<CardInfo>, num_turn_to_waste: u32) -> bool {
    // 裏の場札がある or 山札が残っている
    for card in cards.iter() {
        if let CardType::Pile(_) = card.card_type {
            if card.facedown {
                return false;
            }
        }
        if card.card_type == CardType::Stock {
            return false;
        }
    }

    // 山札からめくったカードの中で操作できないカードがある
    let num_waste = cards.iter().filter(|card| {
        card.card_type == CardType::Waste
    }).count() as u32;
    if num_waste > MAX_WASTES {
        return false;
    }
    if num_turn_to_waste > 1 && num_waste > 1 {
        return false;
    }

    true
}

/// スコア取得処理
fn get_score(move_step: MoveStep) -> i32 {
    match move_step {
        MoveStep::StockToWaste => 0,
        MoveStep::WasteToStock => -100,
        MoveStep::WasteToPile => 5,
        MoveStep::WasteToFoundation => 10,
        MoveStep::PileToPile => 0,
        MoveStep::PileToFoundation => 15,
        MoveStep::FoundationToPile => -15,
        MoveStep::FaceupPile => 5,
    }
}

/// 山札から引くカード枚数取得処理
pub fn num_turn_to_waste(difficulty: GameDifficulty) -> u32 {
    if difficulty == GameDifficulty::Easy {
        1
    } else {
        3
    }
}

/// カードのテクスチャアトラスインデックス取得処理
fn get_atlas_index(suit: CardSuit, number: u32) -> Option<usize> {
    match suit {
        CardSuit::Heart => {
            match number {
                1 => Some(26),
                2 => Some(27),
                3 => Some(28),
                4 => Some(29),
                5 => Some(30),
                6 => Some(31),
                7 => Some(32),
                8 => Some(33),
                9 => Some(34),
                10 => Some(35),
                11 => Some(36),
                12 => Some(37),
                13 => Some(38),
                _ => None,
            }
        },
        CardSuit::Diamond => {
            match number {
                1 => Some(39),
                2 => Some(40),
                3 => Some(41),
                4 => Some(42),
                5 => Some(43),
                6 => Some(44),
                7 => Some(45),
                8 => Some(46),
                9 => Some(47),
                10 => Some(48),
                11 => Some(49),
                12 => Some(50),
                13 => Some(51),
                _ => None,
            }
        }
        CardSuit::Club => {
            match number {
                1 => Some(13),
                2 => Some(14),
                3 => Some(15),
                4 => Some(16),
                5 => Some(17),
                6 => Some(18),
                7 => Some(19),
                8 => Some(20),
                9 => Some(21),
                10 => Some(22),
                11 => Some(23),
                12 => Some(24),
                13 => Some(25),
                _ => None,
            }
        }
        CardSuit::Spade => {
            match number {
                1 => Some(0),
                2 => Some(1),
                3 => Some(2),
                4 => Some(3),
                5 => Some(4),
                6 => Some(5),
                7 => Some(6),
                8 => Some(7),
                9 => Some(8),
                10 => Some(9),
                11 => Some(10),
                12 => Some(11),
                13 => Some(12),
                _ => None,
            }
        }
    }
}
