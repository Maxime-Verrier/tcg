mod render;

use card_sim::{Card, Startup};
use epithet::units::UnitPluginExt;
pub use render::*;

use bevy::app::App;

pub fn card_plugin(app: &mut App) {
    let create_render_system_id = app.register_system(create_card_render);

    app.add_unit::<Card>();
    app.bind_render::<Card>(create_render_system_id);

    #[cfg(feature = "render")]
    {
        app.init_resource::<CardAssets>();
        app.add_systems(Startup, setup_card_assets);
    }
}
