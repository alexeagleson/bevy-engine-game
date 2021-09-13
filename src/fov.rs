use std::{convert::TryInto, io::stdout};

use bevy::prelude::{Changed, Query, Res};
use crossterm::{cursor, style, QueueableCommand};
use rltk::{field_of_view, Point};

use crate::{
    map::{tile_to_char, Map},
    position::Position,
};

pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
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
    mut viewshed_query: Query<(&mut Viewshed, &Position), Changed<Position>>,
    map: Res<Map>,
) {
    for (mut viewshed, position) in viewshed_query.iter_mut() {
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles =
            field_of_view(Point::new(position.0, position.1), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
    }
}
