use crate::{components::Name, position::{distance2d_pythagoras_squared, Position}};
use bevy::prelude::{Changed, Query, ResMut};

pub struct Food {
    pub eaten: bool,
}

pub struct Hunger(pub i32);

pub trait Severity {
    fn is_critical(&self) -> bool;
    fn is_dead(&self) -> bool;
}

impl Hunger {
    fn eat(&mut self, food: &mut Food) {
        self.0 = 100;
        food.eaten = true;
    }
}

impl Severity for Hunger {
    fn is_critical(&self) -> bool {
        self.0 < 100
    }
    fn is_dead(&self) -> bool {
        self.0 <= 0
    }
}

// This system updates the score for each entity with the "Player" and "Score" component.
pub fn eat_nearby_food(
    // mut commands: Commands,
    mut hunger_query: Query<(&Name, &Position, &mut Hunger), Changed<Position>>,
    mut food_query: Query<(&mut Food, &Position)>,
    mut log: ResMut<Vec<String>>,
) {
    for (hunger_name, hunger_position, mut hunger) in hunger_query.iter_mut() {
        if hunger.is_critical() {
            for (mut food, food_position) in food_query.iter_mut() {
                if distance2d_pythagoras_squared(hunger_position, food_position) <= 1.0 {
                    if food.eaten == false {
                        hunger.eat(&mut food);
                        log.push(format!("{} eats food", hunger_name.0));
                    }
                }
            }
        }
    }
}
