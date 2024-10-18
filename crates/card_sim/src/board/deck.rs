use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::CardId;

#[derive(Component)]
pub struct Deck {
    // Using a vec for fasted iteration
    // Cards will be in reverse order so we can pop with O(1) cost since decks will not grow more than their original size
    cards: Vec<CardId>,
}

use rand::thread_rng;

impl Deck {
    pub fn new(cards: Vec<CardId>) -> Self {
        Self { cards }
    }

    pub fn draw(&mut self) -> Option<CardId> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}
