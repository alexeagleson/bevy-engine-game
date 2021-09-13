use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    combat::{CreatureType, Dead},
    components::Name,
    fov::Viewshed,
    map::Map,
    path::{Moves, Path},
    position::{distance2d_pythagoras_squared, Position},
};

pub struct Destination {
    pub position: Position,
}

pub fn set_destination(
    mut commands: Commands,
    subject_query: Query<
        (
            Entity,
            &Name,
            &Position,
            &CreatureType,
            Option<&Destination>,
            Option<&Viewshed>,
        ),
        (With<Position>, With<Moves>),
    >,
    target_query: Query<(Entity, &Name, &Position, &CreatureType), Without<Dead>>,
    map: Res<Map>,
    mut log: ResMut<Vec<String>>,
) {
    let mut rng = rand::thread_rng();

    'subject_loop: for (
        subject_entity,
        subject_name,
        subject_position,
        subject_creature_type,
        subject_destination,
        subject_viewshed,
    ) in subject_query.iter()
    {
        if let Some(subject_viewshed) = subject_viewshed {
            let mut closest_target: Option<&Position> = None;
            let mut closest_distance: Option<f32> = None;

            // Only search if subject has a destination while wandering
            for point in subject_viewshed.visible_tiles.iter() {
                for (target_entity, _target_name, target_position, target_creature_type) in
                    target_query.iter()
                {
                    // Never set self as destination
                    if subject_entity == target_entity {
                        continue;
                    }

                    // Do not pursue creatures of the same type
                    if subject_creature_type == target_creature_type {
                        continue;
                    }

                    // If valid target in current viewsehd tile
                    if target_position.0 == point.x && target_position.1 == point.y {
                        let distance =
                            distance2d_pythagoras_squared(&subject_position, &target_position);

                        if let Some(closest_distance) = closest_distance {
                            if closest_distance < distance {
                                continue;
                            }
                        }

                        closest_target = Some(&target_position);
                        closest_distance = Some(distance);
                    }
                }
            }

            if let Some(closest_target) = closest_target {
                // Check if new destination is the same as the old one, if not, don't change the path
                if let Some(subject_destination) = subject_destination {
                    if distance2d_pythagoras_squared(&subject_destination.position, &closest_target)
                        == 0.0
                    {
                        continue 'subject_loop;
                    }
                }

                commands
                    .entity(subject_entity)
                    .insert(Destination {
                        position: closest_target.clone(),
                        // wandering: true,
                    })
                    .remove::<Path>();

                log.push(format!("{} has a new destination", subject_name.0));
            }
        }
        if subject_destination.is_none() {
            let room = map.rooms.choose(&mut rng).unwrap();
            let room_centre = room.center();
            commands.entity(subject_entity).insert(Destination {
                position: Position(room_centre.0, room_centre.1),
            });

            log.push(format!("{}'s destination is a random room", subject_name.0));
        }
    }
}
