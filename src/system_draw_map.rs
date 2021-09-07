use std::convert::TryInto;
use std::io::{stdout, Write};

use bevy::prelude::{Res, ResMut};
use crossterm::{cursor, style, QueueableCommand};
use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};

use crate::components::Position;

use super::map::{draw_map, tile_to_char, Map};

pub fn draw_full_map(map: Res<Map>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    draw_map(&map);
}

pub fn draw_map_changes(map: Res<Map>, mut changed_positions: ResMut<Vec<Position>>) {
    let mut stdout = stdout();

    stdout
        .queue(style::SetForegroundColor(style::Color::White))
        .unwrap();

    for position in changed_positions.iter() {
        let tile = map.tiles[map.xy_idx(position.0, position.1)];

        stdout
            .queue(cursor::MoveTo(
                position.0.try_into().unwrap(),
                position.1.try_into().unwrap(),
            ))
            .unwrap()
            .queue(style::Print(tile_to_char(&tile)))
            .unwrap();
    }

    changed_positions.clear();
}
