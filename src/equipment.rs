use std::collections::HashSet;

use bevy::prelude::{Bundle, Commands, Entity, Query, ResMut, With, Without};
use crossterm::style::Color;
use rand::Rng;

use crate::{
    components::{Name, Severity, SeverityLevel},
    path::Moves,
    position::{distance2d_pythagoras_squared, Position},
    render::Render,
};

pub struct Equips;
pub struct EquippedWeapon(pub Weapon);
pub struct EquippedArmour(pub Armour);

#[derive(Clone, Debug)]
pub enum Weapon {
    Unarmed,
    Sword,
    Nunchucks,
    Superchucks,
}

pub enum Armour {
    Unarmoured,
    ChainMail,
}

pub struct WeaponStats {
    pub die_size: i32,
    pub die_num: i32,
}

pub struct ArmourStats {
    pub armour_class: i32,
}

impl Weapon {
    pub fn get_stats(&self) -> &WeaponStats {
        match self {
            Weapon::Unarmed => &WeaponStats {
                die_num: 1,
                die_size: 3,
            },
            Weapon::Sword => &WeaponStats {
                die_num: 2,
                die_size: 6,
            },
            Weapon::Nunchucks => &WeaponStats {
                die_num: 2,
                die_size: 8,
            },
            Weapon::Superchucks => &WeaponStats {
                die_num: 3,
                die_size: 20,
            },
        }
    }

    pub fn get_power(&self) -> i32 {
        let stats = self.get_stats();
        stats.die_num * stats.die_size
    }

    pub fn get_damage(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let stats = self.get_stats();
        let mut damage = 0;

        for _ in 0..stats.die_num {
            damage += rng.gen_range(1..=stats.die_size);
        }

        damage
    }

    pub fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn get_glyph(&self) -> String {
        self.get_name()
            .chars()
            .next()
            .unwrap()
            .to_ascii_lowercase()
            .to_string()
    }

    pub fn get_bundle(&self) -> WeaponBundle {
        WeaponBundle {
            name: Name(self.get_name()),
            render: Render {
                colour: Color::DarkYellow,
                char: self.get_glyph(),
            },
            weapon: self.clone(),
        }
    }
}

pub fn get_weapon(optional_equipped: Option<&EquippedWeapon>) -> &Weapon {
    if let Some(equipped) = optional_equipped {
        &equipped.0
    } else {
        &Weapon::Unarmed
    }
}

#[derive(Bundle)]
pub struct WeaponBundle {
    pub name: Name,
    render: Render,
    weapon: Weapon,
}

pub fn pick_up_gear(
    mut commands: Commands,
    subject_query: Query<(Entity, &Name, &Position, Option<&EquippedWeapon>), With<Equips>>,
    target_query: Query<(Entity, &Name, &Position, &Weapon)>,
    mut log: ResMut<Vec<String>>,
) {
    // let mut rng = rand::thread_rng();

    let mut picked_up_entities: HashSet<Entity> = HashSet::new();

    for (subject_entity, subject_name, subject_position, subject_equipped_weapon) in
        subject_query.iter()
    {
        for (target_entity, target_name, target_position, target_weapon) in target_query.iter() {
            if distance2d_pythagoras_squared(subject_position, target_position) <= 2.0 {
                let equipped_weapon = get_weapon(subject_equipped_weapon);

                if equipped_weapon.get_power() < target_weapon.get_power()
                    && picked_up_entities.contains(&target_entity) == false
                {
                    commands
                        .entity(subject_entity)
                        .insert(EquippedWeapon(target_weapon.clone()));

                    commands.entity(target_entity).despawn();

                    picked_up_entities.insert(target_entity);

                    if let Some(subject_equipped_weapon) = subject_equipped_weapon {
                        let dropped_weapon = &subject_equipped_weapon.0;

                        let weapon_bundle = dropped_weapon.get_bundle();

                        log.push(format!(
                            "{} drops {}",
                            subject_name.0, &weapon_bundle.name.0
                        ));

                        commands
                            .spawn()
                            .insert_bundle(weapon_bundle)
                            .insert(subject_position.clone());
                    }

                    log.push(format!("{} picks up {}", subject_name.0, target_name.0));
                }
            }
        }
    }
}
