use bevy::{ecs::{component::{ComponentHooks, StorageType}, query}, math::IVec2, pbr::PbrBundle, prelude::{Component, QueryState}};

pub struct CardId(u32);

#[derive(Component)]
pub struct Card {
    id: CardId
}