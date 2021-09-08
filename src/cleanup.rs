use bevy::prelude::{Commands, Entity, Query};

use crate::hunger::Food;

pub fn despawn(mut commands: Commands, query: Query<(Entity, &Food)>) {
    for (entity, food) in query.iter() {
        if food.eaten {
            commands.entity(entity).despawn();
        }
    }
}
