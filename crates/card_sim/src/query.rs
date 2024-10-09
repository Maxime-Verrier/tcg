use bevy::{
    math::{IVec2, IVec3},
    prelude::Query,
};

use crate::{AgentOwned, Board};

pub enum BoardQueryLoc {
    All,
    Deck(AgentOwned),
    Hand(AgentOwned),
    Field,
    OnSlot(IVec3),
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
                BoardQueryLoc::OnSlot(pos) => {
                    if let Some(entity) = board.get_on_slot(&pos) {
                        entities.push(entity);
                    }
                }
            }
        }
    }
}
