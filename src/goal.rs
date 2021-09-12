use bevy::prelude::*;

use crate::{
    combat::Damage,
    components::{Goblin, Human},
    fov::Viewshed,
    hunger::{Food, Hunger},
    map::Map,
    position::Position,
    components::Name
};

#[derive(PartialEq, Debug)]
pub enum Goal {
    Wander,
    Fight,
    Eat,
}



pub fn set_goal(
    // mut commands: Commands,
    mut subject_query: Query<
        (Entity, &Name, &Viewshed, &mut Goal, Option<&Hunger>, Option<&Damage>),
        (
            With<Human>,
            With<Position>,
            Or<(With<Hunger>, With<Damage>)>,
        ),
    >,
    target_query: Query<
        (Entity, &Position, Option<&Food>, Option<&Goblin>),
        Or<(With<Food>, With<Goblin>)>,
    >,
    map: Res<Map>,
    mut log: ResMut<Vec<String>>,
) {
    for (subject_entity, subject_name, subject_viewshed, mut subject_goal, subject_hunger, subject_damage) in
        subject_query.iter_mut()
    {
        if *subject_goal == Goal::Wander {
            for tile in subject_viewshed.visible_tiles.iter() {
                for (target_entity, target_position, target_food, target_goblin) in
                    target_query.iter()
                {
                    if subject_entity == target_entity {
                        continue;
                    }

                    if tile.x == target_position.0 && tile.y == target_position.1 {
                        if let Some(subject_hunger) = subject_hunger {
                            if let Some(target_food) = target_food {
                                // if subject_hunger.is_critical() {
                                //     log.push(format!("{} is hungry and sees food", subject_name.0));
                                //     *subject_goal = Goal::Eat;
                                //     log.push(format!("{}'s goal is {:?}", subject_name.0, subject_goal));
                                // }
                            }
                        }
                    }
                }
            }
        }

        //////////////////////////
        // let hp = query.get_component::<Hp>(entity);

        // if let Some(hp) = hp {
        //     log.push(format!("Hp: {}", hp.0))
        // }

        // if let Some(damage) = damage {
        //     log.push(format!("damage: {}", damage.0))
        // }

        // match hp {
        //     Ok(_) => log.push("ok".to_string()),
        //     Err(e) => log.push(format!("{:?}", e)),
        // }
    }
}
