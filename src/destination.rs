use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    combat::Damage,
    components::Name,
    components::{Goblin, Human},
    fov::Viewshed,
    map::Map,
    position::Position,
};
pub struct Wandering;

pub struct Destination(pub Position);

pub fn set_destination(
    mut commands: Commands,
    subject_query: Query<
        (Entity, &Name, Option<&Wandering>, Option<&Viewshed>),
        (With<Position>, With<Human>),
    >,
    target_query: Query<(Entity, &Name, &Position), With<Goblin>>,
    map: Res<Map>,
    mut log: ResMut<Vec<String>>,
) {
    let mut rng = rand::thread_rng();

    for (subject_entity, subject_name, subject_wandering, subject_viewshed) in subject_query.iter()
    {
        if let Some(subject_viewshed) = subject_viewshed {
            for point in subject_viewshed.visible_tiles.iter() {
                for (target_entity, target_name, target_position) in target_query.iter() {
                    if subject_entity == target_entity {
                        continue;
                    }
                    // If valid target in current viewsehd tile
                    if target_position.0 == point.x && target_position.1 == point.y {
                        commands
                            .entity(subject_entity)
                            .insert(Position(target_position.0, target_position.1))
                            .remove::<Wandering>();

                        log.push(format!(
                            "{}'s destination is {}",
                            subject_name.0, target_name.0
                        ));
                        continue;
                    }
                }
            }
        }

        // If it reaches here, no enemy is seen in viewshed (or they have no viewshed)
        // Set a random room if they are not already wandering
        if subject_wandering.is_none() {
            let room = map.rooms.choose(&mut rng).unwrap();
            let room_centre = room.center();
            commands
                .entity(subject_entity)
                .insert(Position(room_centre.0, room_centre.1))
                .insert(Wandering);

            log.push(format!("{}'s destination is a random room", subject_name.0));
        }
    }
}
