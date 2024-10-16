use bevy::ecs::entity::MapEntities;
pub use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Board, BoardStage, EffectEvent, Tree};

#[derive(Reflect, Serialize, Deserialize, Debug)]
pub struct BoardState {
    /// The agents playing on the board sorted by their turn order
    pub(crate) agents: Vec<Entity>,
    pub(crate) current_turn_agent: Option<Entity>,

    ///allow us to track the index of the current turn agent, used to determine the next turn agent on stage change
    pub(crate) current_turn_agent_index: usize,

    pub(crate) stage: BoardStage,

    #[serde(skip)]
    #[reflect(ignore)]
    pub(crate) current_tree: Option<Tree>,
}

impl BoardState {
    pub fn new(agents: Vec<Entity>) -> Self {
        Self {
            current_turn_agent: None,
            current_turn_agent_index: 0,
            stage: BoardStage::Start,
            current_tree: None,
            agents,
        }
    }

    pub fn game_start(&mut self) {
        self.current_turn_agent = Some(self.agents[0]);
    }

    pub fn trigger_effect(&mut self, card_entity: Entity) {
        if let Some(ref mut tree) = self.current_tree {
            tree.push_card(card_entity);
        } else {
            self.current_tree = Some(Tree::new(card_entity));
        }
    }

    pub fn get_current_turn_agent(&self) -> &Option<Entity> {
        &self.current_turn_agent
    }
}

impl Board {
    pub fn add_agent(&mut self, agent: Entity) {
        self.state.agents.push(agent);
    }
}

impl MapEntities for BoardState {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.agents = self
            .agents
            .iter()
            .map(|entity| entity_mapper.map_entity(*entity))
            .collect();
        self.current_turn_agent = self
            .current_turn_agent
            .map(|entity| entity_mapper.map_entity(entity));
    }
}

pub fn board_effect_observer(trigger: Trigger<EffectEvent>, mut boards: Query<&mut Board>) {
    let board = trigger.entity();

    if let Ok(mut board) = boards.get_mut(board) {
        board.trigger_effect(trigger.event().0);
    }
}
