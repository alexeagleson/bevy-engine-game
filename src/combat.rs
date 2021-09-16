use bevy::prelude::{Commands, Entity, Query, ResMut, Without};
use rand::Rng;

use crate::{
    components::{Name, Severity, SeverityLevel},
    creature::CreatureType,
    equipment::{get_weapon, EquippedWeapon},
    path::Moves,
    position::{distance2d_pythagoras_squared, Position},
    render::Render,
};

pub struct Aggression(pub i32);

impl Severity for Aggression {
    fn get_severity(&self) -> SeverityLevel {
        if self.0 >= 100 {
            return SeverityLevel::Max;
        } else if self.0 > 0 {
            return SeverityLevel::Moderate;
        }
        SeverityLevel::Min
    }
}

pub struct Hp(pub i32);

pub trait Death {
    fn is_dead(&self) -> bool;
}

impl Death for Hp {
    fn is_dead(&self) -> bool {
        self.0 <= 0
    }
}

pub struct Dead;

pub fn fight(
    subject_query: Query<(
        &Name,
        &Position,
        &Aggression,
        &CreatureType,
        Option<&EquippedWeapon>,
    )>,
    mut target_query: Query<(&mut Hp, &Name, &Position, &CreatureType), Without<Dead>>,
    mut log: ResMut<Vec<String>>,
) {
    let mut rng = rand::thread_rng();

    for (
        subject_name,
        subject_position,
        subject_aggression,
        subject_creature_type,
        subject_equipped_weapon,
    ) in subject_query.iter()
    {
        for (mut target_hp, target_name, target_position, target_creature_type) in
            target_query.iter_mut()
        {
            // Same typed creatures do not attack one another
            if subject_creature_type == target_creature_type {
                continue;
            }

            if distance2d_pythagoras_squared(subject_position, target_position) <= 2.0 {
                match subject_aggression.get_severity() {
                    SeverityLevel::Moderate | SeverityLevel::Max => {
                        let roll = rng.gen_range(1..=20);
                        match roll + subject_creature_type.get_stats().attack_bonus {
                            _roll if _roll >= target_creature_type.get_stats().armour_class => {
                                // let weapon_type = get_weapon(subject_equipped_weapon);
                                let weapon = get_weapon(subject_equipped_weapon);
                                let damage = weapon.get_damage();
                                let weapon_stats = weapon.get_stats();

                                target_hp.0 = target_hp.0 - damage;

                                log.push(format!(
                                    "{} hits {} with {} for {} damage!",
                                    subject_name.0,
                                    target_name.0,
                                    weapon.get_name(),
                                    damage,
                                ));

                                log.push(format!(
                                    "(Rolled {}+{} (1d20 + AB) against {} AC for {} ({}d{}) damage)",
                                    roll,
                                    subject_creature_type.get_stats().attack_bonus,
                                    target_creature_type.get_stats().armour_class,
                                    damage,
                                    &weapon_stats.die_num,
                                    weapon_stats.die_size
                                ));
                            }
                            _ => {
                                log.push(format!(
                                    "{} attacks {} but misses! ({}+{} attack roll against {} AC)",
                                    subject_name.0,
                                    target_name.0,
                                    roll,
                                    subject_creature_type.get_stats().attack_bonus,
                                    target_creature_type.get_stats().armour_class,
                                ));
                            }
                        }
                    }
                    SeverityLevel::Min => {
                        log.push(format!(
                            "{} shouts a friendly greeting to {}",
                            subject_name.0, target_name.0
                        ));
                    }
                }
            }
        }
    }
}

pub fn death(
    mut commands: Commands,
    mut query: Query<(Entity, &Hp, &Name, &mut Render), Without<Dead>>,
    mut log: ResMut<Vec<String>>,
) {
    for (entity, hp, name, mut render) in query.iter_mut() {
        if hp.is_dead() {
            render.char = "%".to_string();
            // render.colour = Color::Red;
            commands
                .entity(entity)
                .insert(Dead)
                .remove::<Moves>()
                .remove::<Aggression>();

            log.push(format!("{} dies!", name.0));
        }
    }
}
