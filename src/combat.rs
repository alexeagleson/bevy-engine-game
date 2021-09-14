use bevy::prelude::{Commands, Entity, Query, ResMut, Without};
use rand::Rng;

use crate::{
    components::{Name, Severity, SeverityLevel},
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

pub struct Weapon {
    die_size: i32,
    die_num: i32,
}

pub struct Equips;

pub struct EquippedWeapon(pub &'static Weapon);

pub struct Dead;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum CreatureType {
    Human,
    Goblin,
    Orc,
}

pub struct CombatStats {
    armour_class: i32,
    attack_bonus: i32,
}

impl Default for CombatStats {
    fn default() -> Self {
        CombatStats {
            armour_class: 10,
            attack_bonus: 0,
        }
    }
}

pub static UNARMED: Weapon = Weapon {
    die_num: 1,
    die_size: 3,
};

pub static SWORD: Weapon = Weapon {
    die_num: 2,
    die_size: 6,
};

pub static NUNCHUCKS: Weapon = Weapon {
    die_num: 5,
    die_size: 8,
};

pub trait Power {
    fn get_power(&self) -> i32;
}

impl Power for Weapon {
    fn get_power(&self) -> i32 {
        self.die_num * self.die_size
    }
}

fn calculate_damage(weapon: &Weapon) -> i32 {
    let mut rng = rand::thread_rng();
    let mut damage = 0;

    for _ in 0..weapon.die_num {
        damage += rng.gen_range(1..=weapon.die_size);
    }

    damage
}

pub fn get_weapon(optional_equipped: Option<&EquippedWeapon>) -> &Weapon {
    if let Some(equipped) = optional_equipped {
        equipped.0
    } else {
        &UNARMED
    }
}

pub fn fight(
    subject_query: Query<(
        &Name,
        &Position,
        &Aggression,
        &CreatureType,
        &CombatStats,
        Option<&EquippedWeapon>,
    )>,
    mut target_query: Query<
        (&mut Hp, &Name, &Position, &CreatureType, &CombatStats),
        Without<Dead>,
    >,
    mut log: ResMut<Vec<String>>,
) {
    let mut rng = rand::thread_rng();

    for (
        subject_name,
        subject_position,
        subject_aggression,
        subject_creature_type,
        subject_combat_stats,
        subject_equipped_weapon,
    ) in subject_query.iter()
    {
        for (
            mut target_hp,
            target_name,
            target_position,
            target_creature_type,
            target_combat_stats,
        ) in target_query.iter_mut()
        {
            // Same typed creatures do not attack one another
            if subject_creature_type == target_creature_type {
                continue;
            }

            if distance2d_pythagoras_squared(subject_position, target_position) <= 2.0 {
                match subject_aggression.get_severity() {
                    SeverityLevel::Moderate | SeverityLevel::Max => {
                        let roll = rng.gen_range(1..=20);
                        match roll + subject_combat_stats.attack_bonus {
                            _roll if _roll >= target_combat_stats.armour_class => {
                                let weapon = get_weapon(subject_equipped_weapon);

                                let damage = calculate_damage(weapon);

                                target_hp.0 = target_hp.0 - damage;
                                log.push(format!(
                                "{} attacks {} for {} damage! ({}+{} attack roll against {} AC for {}d{} damage)",
                                subject_name.0,
                                target_name.0,
                                damage,
                                roll,
                                subject_combat_stats.attack_bonus,
                                target_combat_stats.armour_class,
                                &weapon.die_num,
                                weapon.die_size
                            ));
                            }
                            _ => {
                                log.push(format!(
                                    "{} attacks {} but misses! ({}+{} attack roll against {} AC)",
                                    subject_name.0,
                                    target_name.0,
                                    roll,
                                    subject_combat_stats.attack_bonus,
                                    target_combat_stats.armour_class,
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
