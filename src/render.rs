use std::{
    convert::TryInto,
    io::{stdout, Write},
};

use bevy::prelude::{Changed, Query};
use crossterm::{cursor, style, QueueableCommand};

use super::Position;

pub struct Render {
    pub colour: style::Color,
    pub char: String,
}

// This system updates the score for each entity with the "Player" and "Score" component.
pub fn render_entities_system(query: Query<(&Position, &Render), Changed<Position>>) {
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
