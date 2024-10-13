mod join;
mod summon;

use bevy_replicon::{
    client::ClientSet, prelude::{ChannelKind, ClientEventAppExt, ServerEventAppExt}, server::ServerSet
};
pub use join::*;
pub use summon::*;

pub(crate) fn board_agents_plugin(app: &mut App) {
    app.add_mapped_client_event::<ClientJoinBoardRequestPacket>(ChannelKind::Ordered);
    app.add_mapped_server_event::<ClientJoinedBoardPacket>(ChannelKind::Ordered);

    app.add_systems(Update, player_join_packet_system);
    app.add_systems(
        Update,
        player_joined_packet_system
    );
}
