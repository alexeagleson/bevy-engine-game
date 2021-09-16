use std::{collections::HashSet, time::Instant};

use bevy::{
    app::AppExit,
    prelude::{EventReader, EventWriter, Query, Res, ResMut, Without},
};

use crate::{EndGameEvent, combat::Dead, creature::CreatureType};

pub fn creature_type_count(
    query: Query<&CreatureType, Without<Dead>>,
    mut log: ResMut<Vec<String>>,
    mut end_game_event: EventWriter<EndGameEvent>,
    game_start_time: Res<Instant>,
) {
    let creature_set: HashSet<&CreatureType> =
        query.iter().fold(HashSet::new(), |mut acc, creature_type| {
            acc.insert(&creature_type);
            acc
        });

    let num_creatures_remaining = creature_set.len();
    match num_creatures_remaining {
        0 => {
            log.push(format!("Game over!  Everybody is dead!  Everybody loses!",));
            end_game_event.send(EndGameEvent);
        }
        1 => {
            log.push(format!(
                "Game over!  Winner: {:?}s after {} seconds",
                creature_set.iter().next().unwrap(),
                game_start_time.elapsed().as_secs()
            ));
            end_game_event.send(EndGameEvent);
        }
        _ => (),
    }
}

pub fn end_game(mut exit: EventWriter<AppExit>, mut end_game_event: EventReader<EndGameEvent>) {
    for _ in end_game_event.iter() {
        exit.send(AppExit)
    }
}
