use crate::position::Position;
use crate::rect::Rect;

use rltk::{Algorithm2D, BaseMap, Point, RandomNumberGenerator, Rltk, RGB};
use std::cmp::{max, min};

use std::convert::TryInto;
use std::io::{stdout, Write};

use bevy::prelude::{Res, ResMut};
use crossterm::{cursor, style, QueueableCommand};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    // pub revealed_tiles : Vec<bool>,
    // pub visible_tiles : Vec<bool>
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
    /// This gives a handful of random rooms and corridors joining them together.
    pub fn new_map_rooms_and_corridors() -> Map {
        const WIDTH: i32 = 100;
        const HEIGHT: i32 = 40;
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut map = Map {
            tiles: vec![TileType::Wall; (WIDTH * HEIGHT) as usize],
            rooms: Vec::new(),
            width: WIDTH,
            height: HEIGHT,
            // revealed_tiles : vec![false; 80*50],
            // visible_tiles : vec![false; 80*50]
        };

        let mut rng = RandomNumberGenerator::new();

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let new_center = new_room.center();
                    let new_x = new_center.0;
                    let new_y = new_center.1;

                    let prev_center =  map.rooms[map.rooms.len() - 1].center();
                    let prev_x = prev_center.0;
                    let prev_y = prev_center.1;
                    
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn tile_to_char(tile: &TileType) -> &str {
    match tile {
        TileType::Floor => ".",
        TileType::Wall => "#",
    }
}

pub fn draw_map(map: Res<Map>) {
    let mut stdout = stdout();

    stdout.queue(cursor::MoveTo(0, 0)).unwrap();

    // let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter() {
        print!("{}", tile_to_char(&tile));

        // Move the coordinates
        x += 1;
        if x > map.width - 1 {
            x = 0;
            // y += 1;
            println!("");
        }
    }
}
