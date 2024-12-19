use bevy::prelude::{Entity, Resource};
use crate::game::{CardInfo, CardSuit, CardType};

#[derive(Resource)]
pub struct CardList(pub Vec<CardInfo>);

impl CardList {
    /// カードを取得
    pub fn get(&self, entity: Entity) -> Option<&CardInfo> {
        self.0.iter().find(|card| {
            card.entity == entity
        })
    }

    /// カードを取得
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut CardInfo> {
        self.0.iter_mut().find(|card| {
            card.entity == entity
        })
    }

    /// 山札からめくるカードを取得
    pub fn get_turn_cards_mut(&mut self, num: u32) -> Option<Vec<&mut CardInfo>> {
        let mut result = Vec::new();
        for card in self.0.iter_mut().filter(|card| {
            card.card_type == CardType::Stock
        }) {
            result.push(&mut *card);
        }

        let num = num.clamp(0, num);
        result.truncate(num as usize);

        if result.len() > 0 {
            return Some(result);
        }
        None
    }

    /// 山札からめくったカードを取得
    pub fn get_waste_cards_mut(&mut self) -> Option<Vec<&mut CardInfo>> {
        let mut result = Vec::new();
        for card in self.0.iter_mut().filter(|card| {
            card.card_type == CardType::Waste
        }) {
            result.push(&mut *card);
        }

        if result.len() > 0 {
            return Some(result);
        }
        None
    }

    /// 場札カードを取得
    pub fn get_pile_cards(&self, index: u32) -> Option<Vec<&CardInfo>> {
        let mut result = Vec::new();
        for card in self.0.iter().filter(|card| {
            card.card_type == CardType::Pile(index)
        }) {
            result.push(card);
        }

        result.sort_by(|a, b| {
            a.order.cmp(&b.order)
        });

        if result.len() > 0 {
            return Some(result);
        }
        None
    }

    /// 場札カードを取得
    pub fn get_pile_cards_mut(&mut self, index: u32) -> Option<Vec<&mut CardInfo>> {
        let mut result = Vec::new();
        for card in self.0.iter_mut().filter(|card| {
            card.card_type == CardType::Pile(index)
        }) {
            result.push(&mut *card);
        }

        result.sort_by(|a, b| {
            a.order.cmp(&b.order)
        });

        if result.len() > 0 {
            return Some(result);
        }
        None
    }
    
    /// 組札カードを取得
    pub fn get_foundation_cards(&self, suit: CardSuit) -> Option<Vec<&CardInfo>> {
        let mut result = Vec::new();
        for card in self.0.iter().filter(|card| {
            card.card_type == CardType::Foundation(suit)
        }) {
            result.push(card);
        }

        result.sort_by(|a, b| {
            a.order.cmp(&b.order)
        });

        if result.len() > 0 {
            return Some(result);
        }
        None
    }

    /// 重ねられたカードを取得
    pub fn get_connected_cards(&self, target: Entity) -> Option<Vec<&CardInfo>> {
        let mut result = Vec::new();
        if let Some(target) = self.get(target) {
            for card in self.0.iter().filter(|card| {
                card.card_type == target.card_type && card.order > target.order
            }) {
                result.push(card);
            }
        }

        if result.len() > 0 {
            return Some(result);
        } 
        None
    }

    /// 重ねられたカードを取得
    pub fn get_connected_cards_mut(&mut self, target: Entity) -> Option<Vec<&mut CardInfo>> {
        let mut result = Vec::new();
        let mut card_type = CardType::Stock;
        let mut order = 0;
        if let Some(card) = self.get(target) {
            card_type = card.card_type;
            order = card.order;
        }

        if let CardType::Pile(_) = card_type {
            for card in self.0.iter_mut().filter(|card| {
                card.card_type == card_type && card.order > order
            }) {
                result.push(&mut *card);
            }
        }

        if result.len() > 0 {
            return Some(result);
        } 
        None
    }

    /// 場札に重ねられるかどうかを取得
    pub fn can_stack_pile(&self, index: u32, target: &CardInfo) -> Option<i32> {
        let mut result = None;
        if let Some(cards) = self.get_pile_cards(index as u32) {
            if let Some(card) = cards.last() {
                if (target.card_suit == CardSuit::Heart || target.card_suit == CardSuit::Diamond)
                && (card.card_suit == CardSuit::Club || card.card_suit == CardSuit::Spade) {
                    if target.card_number == card.card_number - 1 {
                        result = Some(cards.len() as i32);
                    }
                } else if (target.card_suit == CardSuit::Club || target.card_suit == CardSuit::Spade)
                && (card.card_suit == CardSuit::Heart || card.card_suit == CardSuit::Diamond) {
                    if target.card_number == card.card_number - 1 {
                        result = Some(cards.len() as i32);
                    }
                }
            }
        } else {
            if target.card_number == 13 {
                result = Some(0);
            }
        }
        result
    }

    /// 組札に重ねられるかどうかを取得
    pub fn can_stack_foundation(&self, target: &CardInfo) -> bool {
        let mut result = false;
        if let Some(cards) = self.get_foundation_cards(target.card_suit) {
            if let Some(card) = cards.last() {
                if target.card_suit == card.card_suit && target.card_number == card.card_number + 1 {
                    result = true;
                }
            }
        } else {
            if target.card_number == 1 {
                result = true;
            }
        }
        result
    }

    /// 場札の裏向きカードの枚数を取得
    pub fn num_facedown(&self, index: u32) -> u32 {
        let mut result = 0;
        if let Some(cards) = self.get_pile_cards(index) {
            result = cards.iter().filter(|card| {
                card.facedown
            }).count() as u32;
        }
        result
    }

    /// 場札の表向きカードの枚数を取得
    pub fn num_faceup(&self, index: u32) -> u32 {
        let mut result = 0;
        if let Some(cards) = self.get_pile_cards(index) {
            result = cards.iter().filter(|card| {
                !card.facedown
            }).count() as u32;
        }
        result
    }
}
