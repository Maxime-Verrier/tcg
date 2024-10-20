mod visibility;

pub use visibility::*;

use bevy::{prelude::*, utils::HashMap};
use bevy_replicon::{
    core::Replicated,
    prelude::{AppRuleExt, ChannelKind, ServerEventAppExt},
    server::ServerSet,
};
use epithet::utils::LevelEntity;
use serde::{Deserialize, Serialize};

use crate::{EffectId, EffectInstance, Effects};

pub fn card_plugin(app: &mut App) {
    app.replicate::<Card>();

    app.add_mapped_server_event::<CardAttributePacket>(ChannelKind::Ordered);
    app.add_systems(Update, card_visibility_observer.before(ServerSet::Send));
    app.add_systems(Update, on_card_visibility_event);

    app.replicate::<Card>();
}

pub const CARD_WIDTH: f32 = 0.063;
pub const CARD_HEIGHT: f32 = 0.088;

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CardId(pub u32);

#[derive(Component, Serialize, Deserialize)]
pub struct Card;

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct CardAttribute {
    pub id: CardId,
}

impl CardAttribute {
    pub fn new(id: CardId) -> Self {
        Self { id }
    }
}

#[derive(Resource)]
pub struct CardRegistry {
    cards: HashMap<CardId, CardData>,
}

pub struct CardData {
    pub name: String,
    pub description: String,
    pub effects: Vec<EffectId>,
}

impl CardData {
    pub fn create_instance(&self) -> (CardBundle, Effects) {
        (
            CardBundle {
                name: Name::new(self.name.clone()),
                //TODO copy id and any needed data
                ..default()
            },
            Effects::new(
                self.effects
                    .iter()
                    .map(|effect_id| EffectInstance::new(*effect_id))
                    .collect(),
            ),
        )
    }
}

pub trait CardPluginExt {
    fn add_cards(&mut self) -> &mut Self;
}

impl CardPluginExt for App {
    fn add_cards(&mut self) -> &mut Self {
        let card_registry = self.world_mut().get_resource_mut::<CardRegistry>();

        self
    }
}

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub card_attribute: CardAttribute,
    pub card_visibility: CardVisibility,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub level_entity: LevelEntity,
    pub name: Name,
    pub replicate: Replicated,
}

impl Default for CardBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Card"),
            card: Card,
            card_attribute: CardAttribute::new(CardId(0)),
            card_visibility: CardVisibility::new(vec![], false),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            level_entity: LevelEntity,
            replicate: Replicated,
        }
    }
}

pub fn create_card_test(commands: &mut Commands) {
    commands.spawn(CardBundle::default());
}
