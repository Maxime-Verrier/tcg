use bevy::{
    asset::Handle,
    core::Name,
    ecs::system::EntityCommands,
    math::Quat,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        default, BuildChildren, Bundle, Component, InheritedVisibility, Mesh, Resource, Transform,
        ViewVisibility, Visibility,
    },
    utils::HashMap,
};
use epithet::utils::LevelEntity;

pub const CARD_WIDTH: f32 = 0.063;
pub const CARD_HEIGHT: f32 = 0.088;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CardId(pub u32);

#[derive(Component, Default)]
pub struct Card(pub CardId);

#[derive(Bundle)]
pub struct CardBundle {
    pub card: Card,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub level_entity: LevelEntity,
    pub name: Name,
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
        }
    }
}

#[cfg(feature = "render")]
#[derive(Resource, Default)]
pub struct CardAssets {
    pub face_mesh: Handle<Mesh>,
    pub back_material: Handle<StandardMaterial>,
    pub deck_mesh: Handle<Mesh>,
    pub deck_material: Handle<StandardMaterial>,
    arts: HashMap<CardId, Handle<StandardMaterial>>,
}

#[cfg(feature = "render")]
impl CardAssets {
    pub fn new(
        face_mesh: Handle<Mesh>,
        back_material: Handle<StandardMaterial>,
        deck_mesh: Handle<Mesh>,
        deck_material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            face_mesh,
            back_material,
            deck_mesh,
            deck_material,
            arts: HashMap::default(),
        }
    }

    #[cfg(feature = "render")]
    pub fn insert_art(&mut self, id: CardId, material: Handle<StandardMaterial>) {
        self.arts.insert(id, material);
    }

    #[cfg(feature = "render")]
    pub fn insert_card_render(&self, commands: &mut EntityCommands, id: &CardId) {
        commands.with_children(|parent| {
            parent
                .spawn(PbrBundle {
                    material: self.back_material.clone(),
                    mesh: self.face_mesh.clone(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(PbrBundle {
                        mesh: self.face_mesh.clone(),
                        material: self.arts.get(id).unwrap().clone(), //TODO handle unwrap
                        transform: Transform::IDENTITY
                            .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
                        ..default()
                    });
                });
        });
    }
}
