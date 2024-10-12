use std::collections::HashMap;

use bevy::{ecs::system::EntityCommands, prelude::*, render::view::visibility};
use card_sim::{Card, CardId};

pub fn create_card_render(
    card_entity: In<Entity>,
    mut commands: Commands,
    card_assets: Res<CardAssets>,
    cards: Query<(&Card, Option<&Transform>, Option<&Visibility>)>,
) {
    if let Ok((card, transform, visibility)) = cards.get(card_entity.0) {
        let mut card_commands = commands.entity(card_entity.0);

        if transform.is_none() {
            card_commands.insert(TransformBundle::default());
        }
        if visibility.is_none() {
            card_commands.insert(VisibilityBundle::default());
        }

        card_assets.insert_card_render(&mut card_commands, &card.0);
    } else {
        error!("Could not create card render, the provided entity do not have a card component");
    }
}

#[cfg(feature = "render")]
#[derive(Resource, Default)]
pub struct CardAssets {
    pub face_mesh: Handle<Mesh>,
    pub back_material: Handle<StandardMaterial>,
    pub deck_mesh: Handle<Mesh>,
    pub deck_material: Handle<StandardMaterial>,
    pub arts: HashMap<CardId, Handle<StandardMaterial>>,
}

#[cfg(feature = "render")]
impl CardAssets {
    pub fn insert_art(&mut self, id: CardId, material: Handle<StandardMaterial>) {
        self.arts.insert(id, material);
    }

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

#[cfg(feature = "render")]
pub(crate) fn setup_card_assets(
    mut card_assets: ResMut<CardAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    use card_sim::{CARD_HEIGHT, CARD_WIDTH};

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
            base_color_texture: Some(assets.load("cards/art4.png")),
            unlit: true,
            ..default()
        }),
    );
}
