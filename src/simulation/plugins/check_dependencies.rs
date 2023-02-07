use bevy::prelude::*;

//Panic if dependencies aren't available
pub fn check_dependency<T: Resource>(app: &mut App) {
    app.world.get_resource::<T>().expect("this plugin is missing a dependency!");
}