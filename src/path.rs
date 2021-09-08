use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Without};
use pathfinding::prelude::{absdiff, astar};
use rand::prelude::SliceRandom;
use rltk::BaseMap;

use crate::{combat::Dead, map::Map, position::Position};

pub struct Path(pub Vec<(i32, i32)>, pub usize);

pub fn create_path(
    mut commands: Commands,
    mut query: Query<(Entity, &Position), Without<Path>>,
    map: Res<Map>,
) {
    let mut rng = rand::thread_rng();

    for (entity, position) in query.iter_mut() {
        let room = map.rooms.choose(&mut rng).unwrap();
        let room_centre = room.center();

        let result = astar(
            &(position.0, position.1),
            |&(x, y)| {
                vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                    .into_iter()
                    .filter(|&(x, y)| map.is_opaque(map.xy_idx(x, y)) == false)
                    .map(|p| (p, 1))
            },
            |&(x, y)| absdiff(x, room_centre.0) + absdiff(y, room_centre.1),
            |&p| p.0 == room_centre.0 && p.1 == room_centre.1,
        );
        // assert_eq!(result.expect("no path found").1, 4);

        if let Some(result) = result {
            let (path, _cost) = result;

            commands.entity(entity).insert(Path(path, 0));
        }
    }
}

pub fn move_path(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &mut Position, &mut Path), Without<Dead>>,
    mut changed_positions: ResMut<Vec<Position>>,
) {
    for (entity, mut position, mut path) in creature_query.iter_mut() {
        let idx = path.1;
        if path.0.len() > idx {
            let (next_x, next_y) = path.0[idx];
            changed_positions.push(position.clone());
            position.0 = next_x;
            position.1 = next_y;
            path.1 += 1; // Move to next path index
        } else {
            commands.entity(entity).remove::<Path>();
        }
    }
}
