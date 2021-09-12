use bevy::prelude::{Commands, Entity, Query, Res, With, Without};
use pathfinding::prelude::{absdiff, astar};
use rand::prelude::SliceRandom;
use rltk::BaseMap;

use crate::{combat::Dead, components::Destination, map::Map, position::Position};

pub struct Moves;

pub struct Path {
    pub current: Vec<(i32, i32)>,
    pub index: usize,
}

fn generate_path(
    map: &Map,
    position: &Position,
    destination: &Position,
) -> Option<(Vec<(i32, i32)>, i32)> {
    let result = astar(
        &(position.0, position.1),
        |&(x, y)| {
            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .into_iter()
                .filter(|&(x, y)| map.is_opaque(map.xy_idx(x, y)) == false)
                .map(|p| (p, 1))
        },
        |&(x, y)| absdiff(x, destination.0) + absdiff(y, destination.1),
        |&p| p.0 == destination.0 && p.1 == destination.1,
    );
    result
}

pub fn path_to_destination(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Destination), (With<Moves>, Without<Path>)>,
    map: Res<Map>,
) {
    let mut rng = rand::thread_rng();

    for (entity, position, destination) in query.iter() {
        if destination.wandering == true {
            let room = map.rooms.choose(&mut rng).unwrap();
            let room_centre = room.center();

            let result = generate_path(&map, &position, &room_centre);

            if let Some(result) = result {
                commands.entity(entity).insert(Path { current: result.0, index: 0 } );
            }
        }
    }
}

pub fn move_path(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Position, &mut Path), (With<Moves>, Without<Dead>)>,
) {
    for (entity, mut position, mut path) in creature_query.iter_mut() {
        let idx = path.1;
        if path.0.len() > idx {
            let (next_x, next_y) = path.0[idx];
            position.0 = next_x;
            position.1 = next_y;
            path.1 += 1; // Move to next path index
        } else {
            commands.entity(entity).remove::<Path>();
        }
    }
}
