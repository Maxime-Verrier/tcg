use bevy::ecs::entity::MapEntities;
pub use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Board, EffectEvent, Tree};

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardState {
    current_turn_agent: Entity,
    current_priority_agent: Entity,

    #[serde(skip)]
    tree: Option<Tree>,
}

impl BoardState {
    pub fn new(current_turn_agent: Entity) -> Self {
        Self {
            current_turn_agent,
            current_priority_agent: current_turn_agent,
            tree: None,
        }
    }

    pub fn trigger_effect(&mut self, card_entity: Entity) {
        if let Some(ref mut tree) = self.tree {
            tree.push_card(card_entity);
        } else {
            self.tree = Some(Tree::new(card_entity));
        }
    }
}

impl MapEntities for BoardState {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.current_turn_agent = entity_mapper.map_entity(self.current_turn_agent);
        self.current_priority_agent = entity_mapper.map_entity(self.current_priority_agent);
    }
}

pub fn board_effect_observer(trigger: Trigger<EffectEvent>, mut boards: Query<&mut Board>) {
    let board = trigger.entity();

    if let Ok(mut board) = boards.get_mut(board) {
        board.trigger_effect(trigger.event().0);
    }
}
