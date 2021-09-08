use bevy::prelude::{Commands, Entity, Query, With, Without};
use crossterm::style::Color;

use crate::{components::{Goblin, Human}, position::{distance2d_pythagoras_squared, Position}, render::Render};

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
    human_query: Query<(&Damage, &Position), With<Human>>,
    mut goblin_query: Query<(&mut Hp, &Position), (With<Goblin>, Without<Dead>)>,
) {
    for (human_damage, human_position) in human_query.iter() {
        for (mut goblin_hp, goblin_position) in goblin_query.iter_mut() {
            if distance2d_pythagoras_squared(human_position, goblin_position) <= 1.0 {
                goblin_hp.0 = goblin_hp.0 - human_damage.0;
                // print!("fight!")
            }
        }
    }
}

pub fn death(mut commands: Commands, mut query: Query<(Entity, &Hp, &mut Render), Without<Dead>>) {
    for (entity, hp, mut render) in query.iter_mut() {
        if hp.is_dead() {
            render.char = "%".to_string();
            render.colour = Color::Red;
            commands.entity(entity).insert(Dead);
            // print!("dead!")
        }
    }
}
