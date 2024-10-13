use bevy::math::IVec3;

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
                    entities.extend(board.lookup.get_entities().iter());
                }
                BoardQueryLoc::Deck(agent) => {
                    //entities.push(vec);
                }
                BoardQueryLoc::Hand(player) => {
                    if let Some(hand) = board.lookup.get_by_hand(&player.0) {
                        entities.extend(hand.iter());
                    }
                }
                BoardQueryLoc::Field => {}
                BoardQueryLoc::OnSlot(pos) => {
                    if let Some(entity) = board.lookup.get_on_slot(&pos) {
                        entities.push(entity);
                    }
                }
            }
        }
    }
}
