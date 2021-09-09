use std::{convert::TryInto, io::stdout};

use bevy::prelude::Query;
use crossterm::{cursor, style, QueueableCommand};

use crate::position::Position;

pub struct Render {
    pub colour: style::Color,
    pub char: String,
}

// This system updates the score for each entity with the "Player" and "Score" component.
pub fn draw_entities(query: Query<(&Position, &Render)>) {
    let mut stdout = stdout();

    for (position, render) in query.iter() {
        stdout
            .queue(cursor::MoveTo(
                position.0.try_into().unwrap(),
                position.1.try_into().unwrap(),
            ))
            .unwrap()
            .queue(style::SetForegroundColor(render.colour))
            .unwrap()
            .queue(style::Print(render.char.to_string()))
            .unwrap();
    }
}
