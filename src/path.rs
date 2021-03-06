use bevy::prelude::{Changed, Commands, Entity, Query, Res, With, Without};
use pathfinding::prelude::{absdiff, astar};
use rand::Rng;
use rltk::BaseMap;

use crate::{
    combat::Dead, components::Name, destination::Destination, map::Map, position::Position,
};

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
    query: Query<(Entity, &Name, &Position, &Destination), (With<Moves>, Changed<Destination>)>,
    map: Res<Map>,
    // mut log: ResMut<Vec<String>>,
) {
    let mut rng = rand::thread_rng();

    for (entity, _name, position, destination) in query.iter() {
        let result = generate_path(&map, &position, &destination.position);

        if let Some(result) = result {
            let last_position = result.0.last().unwrap();
            let destination = Position(last_position.0, last_position.1);
            commands.entity(entity).insert(Path {
                current: result.0,
                index: rng.gen_range(0..=1),
                destination,
            });

            // [EXTRA DEBUG]
            // log.push(format!("{} is now moving to their new destination", name.0));
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
            commands
                .entity(entity)
                .remove::<Path>()
                .remove::<Destination>();
        }
    }
}
