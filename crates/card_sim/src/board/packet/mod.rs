mod join;
mod stage;
mod summon;

pub use join::*;
pub use stage::*;
pub use summon::*;

use bevy::prelude::*;
use bevy_replicon::prelude::{
    server_or_singleplayer, ChannelKind, ClientEventAppExt, ServerEventAppExt,
};

pub(crate) fn board_packet_plugin(app: &mut App) {
    app.add_mapped_client_event::<ClientJoinBoardRequestPacket>(ChannelKind::Ordered);
    app.add_mapped_server_event::<ClientJoinedBoardPacket>(ChannelKind::Ordered);
    app.add_mapped_client_event::<AgentSummonEvent>(ChannelKind::Ordered);
    app.add_mapped_client_event::<StageChangePacket>(ChannelKind::Ordered);

    app.add_systems(
        Update,
        (summon_packet_system, stage_client_stage_packet_system).run_if(server_or_singleplayer),
    );

    app.add_systems(Update, player_join_packet_system);
    app.add_systems(Update, player_joined_packet_system);
}
