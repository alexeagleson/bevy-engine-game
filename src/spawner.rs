use bevy::prelude::{Bundle, Commands};
use crossterm::style::Color;

use crate::{combat::*, components::*, creature::CreatureType, equipment::{EquippedWeapon, Equips, Weapon}, fov::Viewshed, path::Moves, render::Render};

#[derive(Bundle)]
struct CreatureBundle {
    name: Name,
    hp: Hp,
    render: Render,
    moves: Moves,
    aggression: Aggression,
    creature_type: CreatureType,
    viewshed: Viewshed,
    equips: Equips,
}

fn spawn_humans(commands: &mut Commands) {
    for i in 1..=2 {
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from(format!("Human{}", i))),
                hp: Hp(15),
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
                equips: Equips,
            })
            .insert(EquippedWeapon(Weapon::Sword));
    }
}

fn spawn_goblins(commands: &mut Commands) {
    for i in 1..=2 {
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from(format!("Goblin{}", i))),
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
                equips: Equips,
            })
            .insert(EquippedWeapon(Weapon::Sword));
    }
}

fn spawn_orcs(commands: &mut Commands) {
    for i in 1..=2 {
        commands.spawn_bundle(CreatureBundle {
            name: Name(String::from(format!("Orc{}", i))),
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
            equips: Equips,
        });
    }
}

fn spawn_weapons(commands: &mut Commands) {
    for _ in 1..=2 {
        commands.spawn_bundle(Weapon::Nunchucks.get_bundle());
        commands.spawn_bundle(Weapon::Superchucks.get_bundle());
    }
}

pub fn spawn_all(mut commands: Commands) {
    spawn_humans(&mut commands);
    spawn_goblins(&mut commands);
    spawn_orcs(&mut commands);
    spawn_weapons(&mut commands);
}
