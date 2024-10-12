mod join;
mod summon;

use bevy_replicon::{
    prelude::{ChannelKind, ClientEventAppExt},
    server::ServerSet,
};
pub use join::*;
pub use summon::*;

pub(crate) fn board_agents_plugin(app: &mut App) {
    app.add_mapped_client_event::<PlayerJoinPacket>(ChannelKind::Ordered);

    app.add_systems(Update, player_join_packet_system.after(ServerSet::Receive));
}
