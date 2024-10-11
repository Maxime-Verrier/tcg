pub mod agent_action;
mod board;
mod board_state;
mod card;
mod deck;
mod effect;
mod field;
mod hand;
mod query;
mod slot;
mod stage;
mod tree;

pub use board::*;
pub use board_state::*;
pub use card::*;
pub use deck::*;
pub use effect::*;
use epithet::agent::AgentManager;
pub use field::*;
pub use hand::*;
pub use query::*;
pub use slot::*;
pub use stage::*;
pub use tree::*;

use agent_action::{summon_packet_system, AgentSummonEvent};
use bevy_replicon::prelude::{server_or_singleplayer, AppRuleExt, ChannelKind, ClientEventAppExt};

pub struct CardSimPlugin;

impl Plugin for CardSimPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Board>();
        app.register_type::<OnSlot>();
        app.register_type::<AgentManager>();

        app.add_mapped_client_event::<StageChangePacket>(ChannelKind::Ordered);
        app.replicate::<Board>();
        app.replicate_mapped::<OnBoard>();
        app.replicate::<OnHand>();
        app.replicate::<OnField>();
        app.replicate_mapped::<AgentOwned>();
        app.replicate_mapped::<OnSlot>();
        app.replicate_mapped::<BoardSlot>();
        app.replicate::<Card>();

        app.add_mapped_client_event::<AgentSummonEvent>(ChannelKind::Ordered);
        app.observe(board_agent_removed_observer);

        app.add_systems(Update, summon_packet_system.run_if(server_or_singleplayer));
        app.add_systems(
            Update,
            stage_client_stage_packet_system.run_if(server_or_singleplayer),
        );
    }
}
