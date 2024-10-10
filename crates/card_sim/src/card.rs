use bevy::{
    core::Name,
    prelude::{
        Bundle, Component, InheritedVisibility,
        ViewVisibility, Visibility,
    },
};
use bevy_replicon::core::Replicated;
use epithet::utils::LevelEntity;
use serde::{Deserialize, Serialize};

pub const CARD_WIDTH: f32 = 0.063;
pub const CARD_HEIGHT: f32 = 0.088;

#[derive(Default, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CardId(pub u32);

#[derive(Component, Default, Serialize, Deserialize)]
pub struct Card(pub CardId);

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
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
            card: Card::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            level_entity: LevelEntity,
            replicate: Replicated,
        }
    }
}
