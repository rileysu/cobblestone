use crate::simulation::resources::message_router::MessageRouter;
use bevy::{prelude::*, time::FixedTimestep};

use self::{
    connection::{init_connection_handler, term_connection_handler},
    keep_alive::{keep_alive, KeepAliveStage},
    movement::movement_handler,
    plugin_communication::plugin_communication_handler,
};

use super::check_dependencies::check_dependency;

mod connection;
mod keep_alive;
mod movement;
mod plugin_communication;

fn get_messages(mut message_router: ResMut<MessageRouter>) {
    message_router.load_messages();
}

fn clear_messages(mut message_router: ResMut<MessageRouter>) {
    message_router.clear_all();
}

pub struct MessageHandlersPlugin;

impl Plugin for MessageHandlersPlugin {
    fn build(&self, app: &mut App) {
        check_dependency::<MessageRouter>(app);

        app.add_system_to_stage(CoreStage::PreUpdate, get_messages)
            .add_system(init_connection_handler)
            .add_system(plugin_communication_handler)
            .add_system(movement_handler)
            .add_system(term_connection_handler)
            .add_stage_after(
                CoreStage::Update,
                KeepAliveStage,
                SystemStage::parallel().with_run_criteria(FixedTimestep::step(8.0)),
            )
            .add_system_to_stage(KeepAliveStage, keep_alive)
            .add_system_to_stage(CoreStage::PostUpdate, clear_messages);
    }
}
