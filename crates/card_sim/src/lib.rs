pub mod agent_action;
mod board;
mod board_state;
mod card;
mod deck;
mod effect;
mod field;
mod hand;
mod query;
mod render;
mod slot;
mod tree;

pub use board::*;
pub use board_state::*;
pub use card::*;
pub use deck::*;
pub use effect::*;
pub use field::*;
pub use hand::*;
pub use query::*;
pub use render::*;
pub use slot::*;
pub use tree::*;

use agent_action::{summon_packet_system, AgentSummonEvent};
use bevy_replicon::prelude::{AppRuleExt, ChannelKind, ClientEventAppExt};
use epithet::net::NetState;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.replicate::<Board>();
        app.replicate::<OnHand>();
        app.replicate::<OnField>();
        app.replicate::<AgentOwned>();
        app.replicate::<OnSlot>();
        app.replicate::<BoardSlot>();

        app.add_mapped_client_event::<AgentSummonEvent>(ChannelKind::Ordered);
        app.observe(board_agent_removed_observer);

        app.add_systems(
            Update,
            summon_packet_system.run_if(in_state(NetState::Server)),
        );

        #[cfg(feature = "render")]
        {
            app.init_resource::<CardAssets>();
            app.add_systems(Startup, setup);
            app.observe(added_on_hand_observer);
        }
    }
}

#[cfg(feature = "render")]
fn setup(
    mut card_assets: ResMut<CardAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    card_assets.face_mesh = meshes.add(Plane3d::new(
        Vec3::Z,
        Vec2::new(CARD_WIDTH / 2.0, CARD_HEIGHT / 2.0),
    ));
    card_assets.back_material = mats.add(StandardMaterial {
        base_color_texture: Some(assets.load("cards/art.png")),
        unlit: true,
        ..default()
    });
    card_assets.deck_mesh = meshes.add(Cuboid::new(CARD_WIDTH, 0.04, CARD_HEIGHT));
    card_assets.deck_material = mats.add(StandardMaterial {
        base_color_texture: Some(assets.load("cards/art.png")),
        unlit: true,
        ..default()
    });
    card_assets.insert_art(
        CardId(0),
        mats.add(StandardMaterial {
            base_color_texture: Some(assets.load("cards/art3.png")),
            unlit: true,
            ..default()
        }),
    );
    card_assets.insert_art(
        CardId(1),
        mats.add(StandardMaterial {
            base_color_texture: Some(assets.load("cards/art5.png")),
            unlit: true,
            ..default()
        }),
    );
    card_assets.insert_art(
        CardId(2),
        mats.add(StandardMaterial {
            base_color_texture: Some(assets.load("cards/art4.png")),
            unlit: true,
            ..default()
        }),
    );
    card_assets.insert_art(
        CardId(3),
        mats.add(StandardMaterial {
            base_color_texture: Some(assets.load("cards/art6.png")),
            unlit: true,
            ..default()
        }),
    );
}
