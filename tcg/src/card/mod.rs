mod summon;

pub use summon::*;

use bevy::app::App;

pub fn card_plugin(app: &mut App) {
    let execute_id = app.register_system(summon_action_execute);
    let cancel_id = app.register_system(summon_action_cancel);
    let finish_id = app.register_system(summon_action_finish);

    app.insert_resource(SummonActionResource {
        execute_id,
        cancel_id,
        finish_id,
    });
}
