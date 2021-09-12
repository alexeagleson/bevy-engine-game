use bevy::prelude::{Commands, Entity, Query, Res, ResMut, With, Without};
use pathfinding::prelude::{absdiff, astar};
use rand::prelude::SliceRandom;
use rltk::BaseMap;

use crate::{combat::Dead, components::Name, destination::{Destination, Wandering}, map::Map, position::Position};

pub struct Moves;

pub struct Path {
    pub current: Vec<(i32, i32)>,
    pub index: usize,
    pub destination: Position,
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
    query: Query<(Entity, &Name, &Position, &Destination, Option<&Path>), (With<Moves>, Without<Path>)>,
    map: Res<Map>,
    mut log: ResMut<Vec<String>>,
) {

    for (entity, name, position, destination, path) in query.iter() {
        if path.is_none()
            || (path.is_some()
                && path.unwrap().destination.0 != destination.0 .0
                && path.unwrap().destination.1 != destination.0 .1)
        {
            let result = generate_path(&map, &position, &destination.0);

            if let Some(result) = result {
                let last_position = result.0.last().unwrap();
                let destination = Position(last_position.0, last_position.1);
                commands.entity(entity).insert(Path {
                    current: result.0,
                    index: 0,
                    destination,
                });
                log.push(format!("{} is now moving to their destination", name.0));
            }
        }
    }
}

pub fn move_path(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Position, &mut Path), (With<Moves>, Without<Dead>)>,
) {
    for (entity, mut position, mut path) in creature_query.iter_mut() {
        if path.current.len() > path.index {
            let (next_x, next_y) = path.current[path.index];
            position.0 = next_x;
            position.1 = next_y;
            path.index += 1;
        } else {
            commands.entity(entity).remove::<Path>().insert(Wandering);
        }
    }
}
