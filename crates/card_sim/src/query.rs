use bevy::prelude::Query;

use crate::{AgentOwned, Board, FieldPosition};

pub enum BoardQueryLoc {
    All,
    Deck(AgentOwned),
    Hand(AgentOwned),
    Field,
    CardSlot(FieldPosition),
    Pos(FieldPosition),
}

pub struct BoardQuery;

impl BoardQuery {
    pub fn query(board: &Board, locations: Vec<BoardQueryLoc>) {
        let mut entities = vec![];

        for location in locations {
            match location {
                BoardQueryLoc::All => {
                    entities.extend(board.get_entities().iter());
                }
                BoardQueryLoc::Deck(agent) => {
                    //entities.push(vec);
                }
                BoardQueryLoc::Hand(player) => {
                    if let Some(hand) = board.get_by_hand(player.0) {
                        entities.extend(hand.iter());
                    }
                }
                BoardQueryLoc::Field => {}
                BoardQueryLoc::Pos(pos) => {
                    if let Some(entity) = board.get_by_pos(&pos) {
                        entities.push(entity);
                    }
                }
                BoardQueryLoc::CardSlot(pos) => {
                    if let Some(entity) = board.get_by_slot(&pos) {
                        entities.push(entity);
                    }
                }
            }
        }
    }
}
