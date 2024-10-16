mod board;
mod card;
mod effect;
mod query;
pub mod packets;

use bevy::prelude::Query;
pub use board::*;
pub use card::*;
pub use effect::*;
pub use query::*;
use epithet::agent::AgentManager;

use bevy_replicon::{
    client::ClientSet,
    core::replication_registry::rule_fns::RuleFns,
    prelude::{server_or_singleplayer, AppRuleExt, ChannelKind, ClientEventAppExt, ServerEventAppExt}, server::ServerSet,
};
use packets::{board_agents_plugin, summon_packet_system, AgentSummonEvent};

pub struct CardSimPlugin;

fn test(boards: Query<&Board>) {
    for board in boards.iter() {
        //        println!("{:?}", board.lookup.on_hand_lookup);
    }
}
impl Plugin for CardSimPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, test.after(ClientSet::Receive));
        app.add_plugins(board_agents_plugin);

        app.register_type::<Board>();
        app.register_type::<OnSlot>();
        app.register_type::<AgentManager>();

        app.add_mapped_client_event::<StageChangePacket>(ChannelKind::Ordered);
        app.replicate_with::<Board>(
            RuleFns::default_mapped().with_in_place(Board::board_in_place_as_deserialize),
        );
        app.replicate_mapped::<OnBoard>();
        app.replicate::<OnHand>();
        app.replicate::<OnField>();
        app.replicate_mapped::<AgentOwned>();
        app.replicate_mapped::<OnSlot>();
        app.replicate_mapped::<BoardSlot>();
        app.replicate::<Card>();

        app.add_mapped_server_event::<CardAttributePacket>(ChannelKind::Ordered);
        app.add_systems(Update, card_visibility_observer.before(ServerSet::Send));
        app.add_systems(Update, on_card_visibility_event);

        app.add_mapped_client_event::<AgentSummonEvent>(ChannelKind::Ordered);
        app.observe(board_agent_removed_observer);

        app.add_systems(Update, summon_packet_system.run_if(server_or_singleplayer));
        app.add_systems(
            Update,
            stage_client_stage_packet_system.run_if(server_or_singleplayer),
        );
    }
}
