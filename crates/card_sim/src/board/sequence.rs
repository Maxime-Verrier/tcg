use std::fmt::{self, Debug, Formatter};

use bevy::prelude::*;

pub trait BoardActionRunner {
    fn execute(&self, commands: &mut Commands);
}

pub struct BoardSequence {
    pub channel_timer: Timer,

    pub runner: Box<dyn BoardActionRunner + 'static + Send + Sync>,
}

impl Debug for BoardSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoardAction")
            .field("channel_time", &self.channel_timer)
            .finish()
    }
}
