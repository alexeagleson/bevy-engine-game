use std::cmp::{max, min};

use bevy::prelude::{Commands, Entity, Query, Res, Without};
use rand::prelude::SliceRandom;

use crate::map::Map;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position(pub i32, pub i32);

/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
pub fn distance2d_pythagoras_squared(start: &Position, end: &Position) -> f32 {
    let dx = (max(start.0, end.0) - min(start.0, end.0)) as f32;
    let dy = (max(start.1, end.1) - min(start.1, end.1)) as f32;
    (dx * dx) + (dy * dy)
}

pub fn assign_positions(
    mut commands: Commands,
    creature_query: Query<Entity, Without<Position>>,
    map: Res<Map>,
) {
    let mut rng = rand::thread_rng();

    for entity in creature_query.iter() {
        let room_option = map.rooms.choose(&mut rng);
        if let Some(room) = room_option {
            let room_centre = room.random();

            commands
                .entity(entity)
                .insert(Position(room_centre.0, room_centre.1));
        }
    }
}
