use std::{convert::TryInto, io::stdout};

use bevy::prelude::{Changed, Commands, Entity, Or, Query, Res, With};
use crossterm::{cursor, style, QueueableCommand};
use rltk::{field_of_view, Point};

use crate::{
    components::Goblin,
    hunger::Food,
    map::{tile_to_char, Map},
    position::Position,
};

pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

pub fn draw_viewshed(viewshed_query: Query<&Viewshed>, map: Res<Map>) {
    let mut stdout = stdout();
    stdout
        .queue(style::SetForegroundColor(style::Color::Magenta))
        .unwrap();

    for viewshed in viewshed_query.iter() {
        for point in viewshed.visible_tiles.iter() {
            let tile = map.tiles[map.xy_idx(point.x, point.y)];

            stdout
                .queue(cursor::MoveTo(
                    point.x.try_into().unwrap(),
                    point.y.try_into().unwrap(),
                ))
                .unwrap()
                .queue(style::Print(tile_to_char(&tile)))
                .unwrap();
        }
    }
}

pub fn calculate_viewshed(
    // mut commands: Commands,
    mut viewshed_query: Query<(&mut Viewshed, &Position), Changed<Position>>,
    // mut goblin_query: Query<(Entity, &mut Hp, &Goblin)>,
    map: Res<Map>,
) {
    for (mut viewshed, position) in viewshed_query.iter_mut() {
        // viewshed.dirty = false;
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles =
            field_of_view(Point::new(position.0, position.1), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
    }
}


// pub fn look_around(
//     // mut commands: Commands,
//     query: Query<(Viewshed, Option<&Hp>, Option<&Damage>)>,
//     mut log: ResMut<Vec<String>>,
// ) {
//     for (entity, hp, damage) in query.iter() {
//         // let hp = query.get_component::<Hp>(entity);

//         if let Some(hp) = hp {
//             log.push(format!("Hp: {}", hp.0))
//         }

//         if let Some(damage) = damage {
//             log.push(format!("damage: {}", damage.0))
//         }

//         // match hp {
//         //     Ok(_) => log.push("ok".to_string()),
//         //     Err(e) => log.push(format!("{:?}", e)),
//         // }
//     }
// }
