pub mod menu_class;
pub mod game_class;
pub mod editor_class;

use bevy::prelude::*;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}