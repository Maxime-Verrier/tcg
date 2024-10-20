use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Board, BoardStage, Tree};

use super::{BoardActionRunner, BoardSequence};

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
    pub game_state: BoardGameState,

    #[serde(skip)]
    pub tick_triggers: Vec<(Entity, usize)>,

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
            game_state: BoardGameState::Open,
            tick_triggers: Vec::new(),
            agents,
        }
    }

    pub fn game_start(&mut self) {
        self.current_turn_agent = Some(self.agents[0]);
    }

    pub fn trigger_effect(&mut self, card_entity: Entity, effect_index: usize) {
        self.tick_triggers.push((card_entity, effect_index));
    }

    pub fn activate_effect(&mut self, card_entity: Entity) {
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

#[derive(Debug, Default)]
pub enum BoardGameState {
    #[default]
    Open,
    Sequence(BoardSequence),
    Action,
}

pub(crate) fn board_state_update(
    mut boards: Query<&mut Board>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for mut board in boards.iter_mut() {
        if let BoardGameState::Sequence(action) = &mut board.state.game_state {
            action.channel_timer.tick(time.delta());

            if action.channel_timer.finished() {
                action.runner.execute(&mut commands);
            }
        }
    }
}

impl Board {
    pub fn add_agent(&mut self, agent: Entity) {
        self.state.agents.push(agent);
    }
}
