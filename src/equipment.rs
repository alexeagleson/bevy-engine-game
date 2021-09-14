use bevy::prelude::{Commands, Entity, Query, ResMut, With, Without};
use rand::Rng;

use crate::{
    combat::{get_weapon, EquippedWeapon, Equips, Power, Weapon, UNARMED},
    components::{Name, Severity, SeverityLevel},
    path::Moves,
    position::{distance2d_pythagoras_squared, Position},
    render::Render,
};

pub fn pick_up_gear(
    mut commands: Commands,
    mut subject_query: Query<(Entity, &Name, &Position, Option<&mut EquippedWeapon>), With<Equips>>,
    target_query: Query<(&Name, &Position, &Weapon)>,
    mut log: ResMut<Vec<String>>,
) {
    let mut rng = rand::thread_rng();

    for (subject_entity, subject_name, subject_position, mut subject_equipped_weapon) in
        subject_query.iter_mut()
    {
        for (target_name, target_position, target_weapon) in target_query.iter() {
            if distance2d_pythagoras_squared(subject_position, target_position) <= 2.0 {
                let mut equipped_weapon =
                    if let Some(subject_equipped_weapon) = &mut subject_equipped_weapon {
                        subject_equipped_weapon.0
                    } else {
                        &UNARMED
                    };

                if equipped_weapon.get_power() < target_weapon.get_power() {
                    equipped_weapon = target_weapon;
                    commands.entity(subject_entity).insert(target_weapon);

                    log.push(format!(
                        "{} got a new weapon called {}",
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
