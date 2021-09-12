use bevy::prelude::{Bundle, Commands};
use crossterm::style::Color;

use crate::{combat::*, components::*, destination::Wandering, fov::Viewshed, hunger::{Food, Hunger}, path::Moves, render::Render};

#[derive(Bundle)]
struct CreatureBundle {
    name: Name,
    hp: Hp,
    damage: Damage,
    render: Render,
    hunger: Hunger,
    moves: Moves,
    // goal: Goal,
    aggression: Aggression,
}

pub fn spawn_humans(mut commands: Commands) {
    for i in 1..=5 {
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from(format!("Human{}", i))),
                damage: Damage(5),
                hp: Hp(20),
                render: Render {
                    colour: Color::Green,
                    char: i.to_string(),
                },
                hunger: Hunger(100),
                moves: Moves,
                // goal: Goal::Wander,
                aggression: Aggression(100),
            })
            .insert(Human)
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 4,
                dirty: true,
            })
            .insert(Wandering);
    }
}

pub fn spawn_goblins(mut commands: Commands) {
    for _ in 1..=5 {
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from("Goblin")),
                damage: Damage(0),
                hp: Hp(15),
                render: Render {
                    colour: Color::Red,
                    char: "G".to_string(),
                },
                hunger: Hunger(0),
                moves: Moves,
                // goal: Goal::Wander,
                aggression: Aggression(0),
            })
            .insert(Goblin);
    }
}

pub fn spawn_food(mut commands: Commands) {
    for _ in 1..=10 {
        commands
            .spawn()
            .insert(Render {
                colour: Color::Yellow,
                char: "F".to_string(),
            })
            .insert(Food { eaten: false });
    }
}
