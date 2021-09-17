use bevy::prelude::{Bundle, Commands};
use crossterm::style::Color;

use crate::{combat::*, components::*, creature::CreatureType, equipment::{Armour, EquippedWeapon, Equips, Shield, Weapon}, fov::Viewshed, path::Moves, render::Render};

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

pub struct Tracked;

fn spawn_humans(commands: &mut Commands) {
    for _ in 1..=4 {
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from(format!("Human"))),
                hp: Hp(15),
                render: Render {
                    colour: Color::Green,
                    char: "H".to_string(),
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
    for i in 1..=4 {
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
    commands
        .spawn_bundle(CreatureBundle {
            name: Name(String::from(format!("Special Orc"))),
            hp: Hp(15),
            render: Render {
                colour: Color::Cyan,
                char: "O".to_string(),
            },
            moves: Moves,
            aggression: Aggression(100),
            viewshed: Viewshed {
                visible_tiles: Vec::new(),
                range: 6,
            },
            creature_type: CreatureType::Orc,
            equips: Equips,
        })
        .insert(EquippedWeapon(Weapon::GreatHammer))
        .insert(Tracked);
}

fn spawn_weapons(commands: &mut Commands) {
    for _ in 1..=2 {
        commands.spawn_bundle(Weapon::Nunchucks.get_bundle());
    }
    for _ in 1..=40 {
        commands.spawn_bundle(Weapon::GreatHammer.get_bundle());
    }
}

fn spawn_armour(commands: &mut Commands) {
    for _ in 1..=6 {
        commands.spawn_bundle(Armour::ChainMail.get_bundle());
        commands.spawn_bundle(Armour::PlateMail.get_bundle());
    }
}

fn spawn_shields(commands: &mut Commands) {
    for _ in 1..=20 {
        commands.spawn_bundle(Shield::Buckler.get_bundle());
    }
}

pub fn spawn_all(mut commands: Commands) {
    spawn_humans(&mut commands);
    spawn_goblins(&mut commands);
    spawn_orcs(&mut commands);
    spawn_weapons(&mut commands);
    spawn_armour(&mut commands);
    spawn_shields(&mut commands);
}
