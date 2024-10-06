mod card;
mod board;
mod render;
mod query;
mod hand;

pub use card::*;
pub use board::*;
pub use render::*;
pub use query::*;
pub use hand::*;

use bevy::{app::{Plugin, Startup}, asset::{AssetServer, Assets}, math::{Vec2, Vec3}, pbr::StandardMaterial, prelude::{default, Mesh, Plane3d, Res, ResMut}};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CardAssets>();

        app.add_systems(Startup, setup);
        app.observe(added_on_hand_observer);
    }
}

fn setup(mut card_assets: ResMut<CardAssets>, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>, assets: Res<AssetServer>) {
    card_assets.face_mesh = meshes.add(Plane3d::new(Vec3::Y, Vec2::new(0.63, 0.88)));
    card_assets.back_material = mats.add(StandardMaterial {
        base_color_texture: Some(assets.load("cards/art.png")),
        unlit: true,
        ..default()
    });
    card_assets.insert_art(CardId(0), mats.add(StandardMaterial {
        base_color_texture: Some(assets.load("cards/art2.png")),
        unlit: true,
        ..default()
    }));
}