mod render;

use card_sim::Card;
use epithet::units::UnitPluginExt;

use bevy::app::App;

pub fn card_plugin(app: &mut App) {
    app.add_unit::<Card>();

    #[cfg(feature = "render")]
    {
        use bevy::app::Startup;
        use render::{create_card_render, setup_card_assets, CardAssets};

        let create_render_system_id = app.register_system(create_card_render);

        app.bind_render::<Card>(create_render_system_id);

        app.init_resource::<CardAssets>();
        app.add_systems(Startup, setup_card_assets);
    }
}
