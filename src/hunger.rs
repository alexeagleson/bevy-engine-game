use crate::position::{distance2d_pythagoras_squared, Position};
use bevy::prelude::{Changed, Query};

pub struct Food {
    pub eaten: bool,
}

pub struct Hunger(pub i32);

pub trait Severity {
    fn is_critical(&self) -> bool;
    fn is_dead(&self) -> bool;
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
    hunger_query: Query<(&Position, &Hunger), Changed<Position>>,
    mut food_query: Query<(&mut Food, &Position)>,
) {
    for (hunger_position, hunger) in hunger_query.iter() {
        if hunger.is_critical() {
            for (mut food, food_position) in food_query.iter_mut() {
                if distance2d_pythagoras_squared(hunger_position, food_position) <= 1.0 {
                    if food.eaten == false {
                        food.eaten = true;
                    }
                }
            }
        }
    }
}
