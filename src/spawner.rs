use bevy::prelude::{Bundle, Commands};
use crossterm::style::Color;

use crate::{
    combat::*,
    components::*,
    fov::Viewshed,
    hunger::{Food, Hunger},
    render::Render,
};

#[derive(Bundle)]
struct CreatureBundle {
    name: Name,
    hp: Hp,
    damage: Damage,
    render: Render,
    hunger: Hunger,
}

pub fn spawn_humans(mut commands: Commands) {
    for _ in 1..=5 {
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from("Human")),
                damage: Damage(5),
                hp: Hp(20),
                render: Render {
                    colour: Color::Green,
                    char: "@".to_string(),
                },
                hunger: Hunger(0),
            })
            .insert(Human)
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 4,
                dirty: true,
            });
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
                hunger: Hunger(100),
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
