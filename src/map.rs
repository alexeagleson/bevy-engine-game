use super::Rect;
use std::cmp::{max, min};

use rand::prelude::*;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;
const MAX_ROOMS: i32 = 30;
const MIN_SIZE: i32 = 6;
const MAX_SIZE: i32 = 10;


#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH) + x as usize
}

pub type Map = Vec<TileType>;

pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; WIDTH * HEIGHT];

    // Make the boundaries walls
    for x in 0..WIDTH as i32 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, HEIGHT as i32 - 1)] = TileType::Wall;
    }
    for y in 0..HEIGHT as i32 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(WIDTH as i32 - 1, y)] = TileType::Wall;
    }

    map
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < WIDTH * HEIGHT {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < WIDTH * HEIGHT {
            map[idx] = TileType::Floor;
        }
    }
}

/// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
/// This gives a handful of random rooms and corridors joining them together.
pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Map) {
    let mut map = vec![TileType::Wall; WIDTH * HEIGHT];

    let mut rooms: Vec<Rect> = Vec::new();



    let mut rng = rand::thread_rng();

    for _i in 0..MAX_ROOMS {
        let w = rng.gen_range(MIN_SIZE..MAX_SIZE);
        let h = rng.gen_range(MIN_SIZE..MAX_SIZE);
        let x = rng.gen_range(1..WIDTH as i32 - w - 1) - 1;
        let y = rng.gen_range(1..HEIGHT as i32 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rng.gen_range(0..2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }

    (rooms, map)
}

pub fn draw_map(map: &[TileType]) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                print!(".");
                // ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                print! {"#"}
                // ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
            println!("");
        }
    }
} 

pub fn is_blocked(x: i32, y: i32, map: &Map) -> bool {
    match map[xy_idx(x, y)] {
        TileType::Floor => false,
        TileType::Wall => true
    }
}