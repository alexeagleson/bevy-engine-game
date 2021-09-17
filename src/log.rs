use std::{convert::TryInto, io::stdout};

use bevy::prelude::Res;
use crossterm::{
    cursor,
    style::{self, Color},
    QueueableCommand,
};

use crate::map::Map;

pub fn draw_log(map: Res<Map>, log: Res<Vec<String>>) {
    let mut stdout = stdout();

    stdout
        .queue(style::SetForegroundColor(Color::White))
        .unwrap();

    'log_loop: for (idx, log_entry) in log.iter().rev().enumerate() {
        stdout
            .queue(cursor::MoveTo(
                (map.width + 1).try_into().unwrap(),
                (idx + 2).try_into().unwrap(),
            ))
            // .unwrap()
            // .queue(style::Print(log_entry))
            .unwrap()
            .queue(style::Print(format!("{: <1$}", log_entry, 145 - log_entry.len())))
            .unwrap();

            if idx >= (map.height - 3).try_into().unwrap() {
                break 'log_loop;
            }
    }
}
