use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::BoardState;

#[derive(Reflect, Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
pub enum BoardStage {
    #[default]
    Start,
    Main,
    End,
}

/// Advances the board state to the specified stage.
///
/// # Arguments
///
/// * `stage` - The target stage to advance to.
///
/// # Returns
///
/// * `bool` - Always returns `true`.
impl BoardState {
    pub fn advance_stage(&mut self, stage: BoardStage) -> bool {
        if self.agents.is_empty() {
            error!("Cannot advance stage on a board with no agents, this function should never be called in this state");
            return false;
        }

        //TODO make chain stage when the target stage result in multiple stage change to trigger effects on each stage
        if stage == BoardStage::Start {
            self.current_turn_agent_index = if self.current_turn_agent_index < self.agents.len() - 1
            {
                self.current_turn_agent_index + 1
            } else {
                0
            };
            self.current_turn_agent = Some(self.agents[self.current_turn_agent_index]);
        }
        true
    }
}
