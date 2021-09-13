use bevy::prelude::{Bundle, Commands};
use crossterm::style::Color;

use crate::{combat::*, components::*, fov::Viewshed, path::Moves, render::Render};

#[derive(Bundle)]
struct CreatureBundle {
    name: Name,
    hp: Hp,
    damage: Damage,
    render: Render,
    moves: Moves,
    aggression: Aggression,
    creature_type: CreatureType,
    viewshed: Viewshed,
}

fn spawn_humans(commands: &mut Commands) {
    for i in 1..=5 {
        commands.spawn_bundle(CreatureBundle {
            name: Name(String::from(format!("Human{}", i))),
            damage: Damage(5),
            hp: Hp(20),
            render: Render {
                colour: Color::Green,
                char: i.to_string(),
            },
            moves: Moves,
            aggression: Aggression(100),
            viewshed: Viewshed {
                visible_tiles: Vec::new(),
                range: 4,
            },
            creature_type: CreatureType::Human,
        });
    }
}

fn spawn_goblins(commands: &mut Commands) {
    for i in 1..=20 {
        commands.spawn_bundle(CreatureBundle {
            name: Name(String::from(format!("Goblin{}", i))),
            damage: Damage(1),
            hp: Hp(15),
            render: Render {
                colour: Color::Red,
                char: "G".to_string(),
            },
            moves: Moves,
            aggression: Aggression(100),
            viewshed: Viewshed {
                visible_tiles: Vec::new(),
                range: 4,
            },
            creature_type: CreatureType::Goblin,
        });
    }
}

fn spawn_orcs(commands: &mut Commands) {
    for i in 1..=4 {
        commands.spawn_bundle(CreatureBundle {
            name: Name(String::from(format!("Orc{}", i))),
            damage: Damage(10),
            hp: Hp(15),
            render: Render {
                colour: Color::Cyan,
                char: "O".to_string(),
            },
            moves: Moves,
            aggression: Aggression(100),
            viewshed: Viewshed {
                visible_tiles: Vec::new(),
                range: 4,
            },
            creature_type: CreatureType::Orc,
        });
    }
}

pub fn spawn_all(mut commands: Commands) {
    spawn_humans(&mut commands);
    spawn_goblins(&mut commands);
    spawn_orcs(&mut commands);
}
