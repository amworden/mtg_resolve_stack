// src/models.rs

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CardType {
    Instant,
    Sorcery,
    Creature,
    Enchantment,
    Artifact,
    Planeswalker,
    Land,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Effect {
    DealDamage { amount: u32 },
    CounterSpell,
    // Add other effects as needed
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Card {
    name: String,
    card_type: CardType,
    effect: Effect,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResolutionResult {
    card: Card,
    result: String,
    player_health: u32,
    opponent_health: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stack {
    cards: Vec<Card>,
    player_health: u32,
    opponent_health: u32,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            cards: vec![],
            player_health: 20,
            opponent_health: 20
        }
    }

    pub fn add_to_stack(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn resolve_stack(&mut self) -> Vec<ResolutionResult> {
        self.cards.reverse(); // LIFO
        let mut resolved_cards: Vec<Card> = vec![];
        let mut countered_cards: Vec<Card> = vec![];
        let mut results: Vec<ResolutionResult> = vec![];

        for card in &self.cards {
            match &card.effect {
                Effect::CounterSpell => {
                    if let Some(target) = self.cards.iter().find(|c| !resolved_cards.contains(c) && !countered_cards.contains(c)) {
                        countered_cards.push(target.clone());
                        results.push(ResolutionResult {
                            card: card.clone(),
                            result: format!("Countered {}", target.name),
                            player_health: self.player_health,
                            opponent_health: self.opponent_health,
                        });
                    }
                }
                Effect::DealDamage { amount } => {
                    if !countered_cards.contains(card) {
                        resolved_cards.push(card.clone());
                        self.opponent_health = self.opponent_health.saturating_sub(*amount);
                        results.push(ResolutionResult {
                            card: card.clone(),
                            result: format!("Dealt {} damage", amount),
                            player_health: self.player_health,
                            opponent_health: self.opponent_health,
                        });
                    } else {
                        results.push(ResolutionResult {
                            card: card.clone(),
                            result: "Fizzled (was countered)".to_string(),
                            player_health: self.player_health,
                            opponent_health: self.opponent_health,
                        });
                    }
                }
                _ => {
                    if !countered_cards.contains(card) {
                        resolved_cards.push(card.clone());
                        results.push(ResolutionResult {
                            card: card.clone(),
                            result: "Resolved".to_string(),
                            player_health: self.player_health,
                            opponent_health: self.opponent_health,
                        });
                    } else {
                        results.push(ResolutionResult {
                            card: card.clone(),
                            result: "Fizzled (was countered)".to_string(),
                            player_health: self.player_health,
                            opponent_health: self.opponent_health,
                        });
                    }
                }
            }
        }

        self.cards.clear();
        results
    }
}
