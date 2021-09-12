use bevy::prelude::{Commands, Entity, Or, Query, ResMut, With, Without};
use crossterm::style::Color;

use crate::{
    components::{Goblin, HasSeverity, Human, Name, Severity},
    hunger::Hunger,
    position::{distance2d_pythagoras_squared, Position},
    render::Render,
};

pub struct Aggression(pub i32);

impl HasSeverity for Aggression {
    fn get_severity(&self) -> Severity {
        if self.0 >= 100 {
            return Severity::Max;
        } else if self.0 > 0 {
            return Severity::Moderate;
        }
        Severity::Min
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

pub fn fight_nearby_goblins(
    human_query: Query<(&Damage, &Name, &Position, &Aggression), With<Human>>,
    mut goblin_query: Query<(&mut Hp, &Position), (With<Goblin>, Without<Dead>)>,
    mut log: ResMut<Vec<String>>,
) {
    for (human_damage, human_name, human_position, human_aggression) in human_query.iter() {
        if human_aggression.get_severity() == Severity::Moderate
            || human_aggression.get_severity() == Severity::Max
        {
            for (mut goblin_hp, goblin_position) in goblin_query.iter_mut() {
                if distance2d_pythagoras_squared(human_position, goblin_position) <= 1.0 {
                    goblin_hp.0 = goblin_hp.0 - human_damage.0;
                    log.push(format!(
                        "{} attacks goblin for {} damage",
                        human_name.0, human_damage.0
                    ));
                }
            }
        }
    }
}

pub fn death(
    mut commands: Commands,
    mut query: Query<(Entity, &Hp, &mut Render), Without<Dead>>,
    mut log: ResMut<Vec<String>>,
) {
    for (entity, hp, mut render) in query.iter_mut() {
        if hp.is_dead() {
            render.char = "%".to_string();
            render.colour = Color::Red;
            commands.entity(entity).insert(Dead);
            log.push(format!("Entity dies"));
        }
    }
}
