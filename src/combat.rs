use bevy::prelude::{Commands, Entity, Query, ResMut, Without};

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

pub struct Damage(pub i32);

pub struct Dead;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum CreatureType {
    Human,
    Goblin,
    Orc,
}

pub fn fight(
    subject_query: Query<(&Damage, &Name, &Position, &Aggression, &CreatureType)>,
    mut target_query: Query<(&mut Hp, &Name, &Position, &CreatureType), Without<Dead>>,
    mut log: ResMut<Vec<String>>,
) {
    for (
        subject_damage,
        subject_name,
        subject_position,
        subject_aggression,
        subject_creature_type,
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
                        target_hp.0 = target_hp.0 - subject_damage.0;
                        log.push(format!(
                            "{} attacks {} for {} damage",
                            subject_name.0, target_name.0, subject_damage.0
                        ));
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
