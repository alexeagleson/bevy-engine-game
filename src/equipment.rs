use std::collections::HashSet;

use bevy::prelude::{Commands, Entity, Query, ResMut, With, Without};
use crossterm::style::Color;
use rand::Rng;

use crate::{
    combat::{get_weapon, EquippedWeapon, Equips, Power, Weapon},
    components::{Name, Severity, SeverityLevel},
    path::Moves,
    position::{distance2d_pythagoras_squared, Position},
    render::Render,
};

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
                // let mut equipped_weapon_type =
                //     if let Some(subject_equipped_weapon) = &mut subject_equipped_weapon_type {
                //         subject_equipped_weapon.0
                //     } else {
                //         Weapon::Unarmed
                //     };

                let equipped_weapon = get_weapon(subject_equipped_weapon);
                // let target_weapon = get_weapon_from_weapon_type(&target_weapon_type);

                if equipped_weapon.get_power() < target_weapon.get_power()
                    && picked_up_entities.contains(&target_entity) == false
                {
                    commands
                        .entity(subject_entity)
                        .insert(EquippedWeapon(target_weapon.clone()));

                    commands.entity(target_entity).despawn();

                    picked_up_entities.insert(target_entity);

                    if let Some(subject_equipped_weapon) = subject_equipped_weapon {
                        let dropped_weapon = subject_equipped_weapon.0.clone();
                        commands
                            .spawn()
                            .insert(dropped_weapon)
                            .insert(Name(String::from("WEAPON")))
                            .insert(Render {
                                colour: Color::Cyan,
                                char: "W".to_string(),
                            })
                            .insert(subject_position.clone());
                    }

                    log.push(format!(
                        "{} got a new weapon calledddddddddddddddd {}",
                        subject_name.0, target_name.0
                    ));
                }
                // match subject_aggression.get_severity() {
                //     SeverityLevel::Moderate | SeverityLevel::Max => {
                //         let roll = rng.gen_range(1..=20);
                //         match roll + subject_combat_stats.attack_bonus {
                //             _roll if _roll >= target_combat_stats.armour_class => {
                //                 let weapon = if let Some(equipped_weapon) = subject_equipped_weapon
                //                 {
                //                     equipped_weapon.0
                //                 } else {
                //                     &UNARMED
                //                 };

                //                 let damage = calculate_damage(weapon);

                //                 target_hp.0 = target_hp.0 - damage;
                //                 log.push(format!(
                //                 "{} attacks {} for {} damage! ({}+{} attack roll against {} AC for {}d{} damage)",
                //                 subject_name.0,
                //                 target_name.0,
                //                 damage,
                //                 roll,
                //                 subject_combat_stats.attack_bonus,
                //                 target_combat_stats.armour_class,
                //                 &weapon.die_num,
                //                 weapon.die_size
                //             ));
                //             }
                //             _ => {
                //                 log.push(format!(
                //                     "{} attacks {} but misses! ({}+{} attack roll against {} AC)",
                //                     subject_name.0,
                //                     target_name.0,
                //                     roll,
                //                     subject_combat_stats.attack_bonus,
                //                     target_combat_stats.armour_class,
                //                 ));
                //             }
                //         }
                //     }
                //     SeverityLevel::Min => {
                //         log.push(format!(
                //             "{} shouts a friendly greeting to {}",
                //             subject_name.0, target_name.0
                //         ));
                //     }
                // }
            }
        }
    }
}
