use bevy::{asset::Handle, ecs::{component::{ComponentHooks, StorageType}, query, system::{EntityCommand, EntityCommands}}, math::{IVec2, Quat}, pbr::{PbrBundle, StandardMaterial}, prelude::{default, BuildChildren, Bundle, Component, GlobalTransform, InheritedVisibility, Mesh, Plane3d, QueryState, Resource, Transform, ViewVisibility, Visibility}, utils::HashMap};

use crate::OnBoard;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CardId(pub u32);

#[derive(Component, Default)]
pub struct Card(pub CardId);

#[derive(Bundle, Default)]
pub struct CardBundle {
    pub card: Card,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility
}

#[derive(Resource, Default)]
pub struct CardAssets {
    pub(crate) face_mesh: Handle<Mesh>,
    pub(crate) back_material: Handle<StandardMaterial>,
    arts: HashMap<CardId, Handle<StandardMaterial>>
}

impl CardAssets {
    pub fn new(face_mesh: Handle<Mesh>, back_material: Handle<StandardMaterial>) -> Self {
        Self {
            face_mesh,
            back_material,
            arts: HashMap::default()
        }
    }

    pub fn insert_art(&mut self, id: CardId, material: Handle<StandardMaterial>) {
        self.arts.insert(id, material);
    }

    pub fn insert_card_render(&self, commands: &mut EntityCommands, id: &CardId) {
        commands.with_children(|parent| {
            parent
                .spawn(PbrBundle {
                    mesh: self.face_mesh.clone(),
                    material: self.arts.get(id).unwrap().clone(), //TODO handle unwrap
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(PbrBundle {
                        mesh: self.face_mesh.clone(),
                        material: self.back_material.clone(),
                        transform: Transform::IDENTITY
                            .with_rotation(Quat::from_rotation_z(std::f32::consts::PI)),
                        ..default()
                    });
                });
        });
    }
}