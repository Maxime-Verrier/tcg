mod board;
mod card;
mod effect;
mod query;

pub use board::*;
pub use card::*;
pub use effect::*;
pub use query::*;

use bevy::prelude::*;
use epithet::agent::AgentManager;

pub struct CardSimPlugin;

impl Plugin for CardSimPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<AgentManager>();
        app.add_plugins((board_plugin, card_plugin));
    }
}

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub enum CardSimSet {
    NetReceive,
    Update,
    NetSend,
}
