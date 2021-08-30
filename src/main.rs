use std::time::Duration;

use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::{
        debug, error, info, trace, warn, App, Bundle, Commands, Entity, IntoSystem, Query, Res,
        With, Without,
    },
};

mod map;
pub use map::*;

mod rect;
pub use rect::Rect;

struct Name(String);

struct Goblin;

struct Hp(i32);

trait Death {
    fn is_dead(&self) -> bool;
}

struct Damage(i32);

impl Death for Hp {
    fn is_dead(&self) -> bool {
        self.0 <= 0
    }
}

#[derive(Bundle)]
struct CreatureBundle {
    name: Name,
    hp: Hp,
    damage: Damage,
}

// This system uses a command buffer to (potentially) add a new player to our game on each
// iteration. Normal systems cannot safely access the World instance directly because they run in
// parallel. Our World contains all of our components, so mutating arbitrary parts of it in parallel
// is not thread safe. Command buffers give us the ability to queue up changes to our World without
// directly accessing it
fn spawn_humans(mut commands: Commands) {
    for _ in 1..=1000 {
        commands.spawn_bundle(CreatureBundle {
            name: Name(String::from("Human")),
            damage: Damage(5),
            hp: Hp(20),
        });

        // spawn().insert(Creature {
        //     name: String::from("ASSMAN"),
        // });
    }
}

fn spawn_goblins(mut commands: Commands) {
    for _ in 1..=100_000 {
        // commands.spawn().insert(Bug { legs: 4 });
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from("Goblin")),
                damage: Damage(4),
                hp: Hp(15),
            })
            .insert(Goblin);
    }
}

fn humans_fight_goblins(
    mut commands: Commands,
    mut human_query: Query<(Entity, &Damage, &mut Hp), Without<Goblin>>,
    mut goblin_query: Query<(Entity, &Damage, &mut Hp), With<Goblin>>,
) {
    // let mut humans = human_query.iter().count();
    let mut goblin_iter = goblin_query.iter_mut();

    for (human_entity, human_damage, mut human_hp) in human_query.iter_mut() {
        if let Some((goblin_entity, goblin_damage, mut goblin_hp)) = goblin_iter.next() {
            // Humans attack goblins
            goblin_hp.0 = goblin_hp.0 - human_damage.0;

            match goblin_hp.is_dead() {
                true => commands.entity(goblin_entity).despawn(),
                false => {
                    human_hp.0 = human_hp.0 - goblin_damage.0;
                    match human_hp.is_dead() {
                        true => commands.entity(human_entity).despawn(),
                        false => (),
                    }
                }
            }
        }
    }
}

// This system updates the score for each entity with the "Player" and "Score" component.
fn count_goblins(goblin_query: Query<&Goblin>, map: Res<Vec<TileType>>) {
    let num_goblins = goblin_query.iter().len();
    // print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    draw_map(&map);
    info!("Goblins remaining: {} ", num_goblins);
}

// Our Bevy app's entry point
fn main() {
    let (rooms, map) = new_map_rooms_and_corridors();


    // Bevy apps are created using the builder pattern. We use the builder to add systems,
    // resources, and plugins to our app
    App::build()
        .insert_resource(map)
        // Resources can be added to our app like this
        // .insert_resource(State { counter: 0 })
        // Some systems are configured by adding their settings as a resource
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs(1)))
        // Plugins are just a grouped set of app builder calls (just like we're doing here).
        // We could easily turn our game into a plugin, but you can check out the plugin example for
        // that :) The plugin below runs our app's "system schedule" once every 5 seconds
        // (configured above).
        .add_plugin(ScheduleRunnerPlugin::default())
        // Resources that implement the Default or FromResources trait can be added like this:
        // .init_resource::<GameState>()
        // Startup systems run exactly once BEFORE all other systems. These are generally used for
        // app initialization code (ex: adding entities and resources)
        // .add_startup_system(startup_system.system())
        // my_system calls converts normal rust functions into ECS systems:
        // .add_system(print_message_system.system())
        // SYSTEM EXECUTION ORDER
        //
        // Each system belongs to a `Stage`, which controls the execution strategy and broad order
        // of the systems within each tick. Startup stages (which startup systems are
        // registered in) will always complete before ordinary stages begin,
        // and every system in a stage must complete before the next stage advances.
        // Once every stage has concluded, the main loop is complete and begins again.
        //
        // By default, all systems run in parallel, except when they require mutable access to a
        // piece of data. This is efficient, but sometimes order matters.
        // For example, we want our "game over" system to execute after all other systems to ensure
        // we don't accidentally run the game for an extra round.
        //
        // Rather than splitting each of your systems into separate stages, you should force an
        // explicit ordering between them by giving the relevant systems a label with
        // `.label`, then using the `.before` or `.after` methods. Systems will not be
        // scheduled until all of the systems that they have an "ordering dependency" on have
        // completed.
        //
        // Doing that will, in just about all cases, lead to better performance compared to
        // splitting systems between stages, because it gives the scheduling algorithm more
        // opportunities to run systems in parallel.
        // Stages are still necessary, however: end of a stage is a hard sync point
        // (meaning, no systems are running) where `Commands` issued by systems are processed.
        // This is required because commands can perform operations that are incompatible with
        // having systems in flight, such as spawning or deleting entities,
        // adding or removing resources, etc.
        //
        // add_system(system) adds systems to the UPDATE stage by default
        // However we can manually specify the stage if we want to. The following is equivalent to
        // add_system(score_system)
        // .add_system_to_stage(CoreStage::Update, score_system.system())
        // We can also create new stages. Here is what our games stage order will look like:
        // "before_round": new_player_system, new_round_system
        // "update": print_message_system, score_system
        // "after_round": score_check_system, game_over_system
        // .add_stage_before(
        //     CoreStage::Update,
        //     MyStage::BeforeRound,
        //     SystemStage::parallel(),
        // )
        // .add_stage_after(
        //     CoreStage::Update,
        //     MyStage::AfterRound,
        //     SystemStage::parallel(),
        // )
        // .add_system_to_stage(MyStage::BeforeRound, new_round_system.system())
        // .add_system_to_stage(MyStage::BeforeRound, new_player_system.system())
        // // We can ensure that game_over system runs after score_check_system using explicit ordering
        // // constraints First, we label the system we want to refer to using `.label`
        // // Then, we use either `.before` or `.after` to describe the order we want the relationship
        // .add_system_to_stage(
        //     MyStage::AfterRound,
        //     score_check_system.system().label(MyLabels::ScoreCheck),
        // )
        // .add_system_to_stage(
        //     MyStage::AfterRound,
        //     game_over_system.system().after(MyLabels::ScoreCheck),
        // )
        // // We can check our systems for execution order ambiguities by examining the output produced
        // // in the console by using the `LogPlugin` and adding the following Resource to our App :)
        // // Be aware that not everything reported by this checker is a potential problem, you'll have
        // // to make that judgement yourself.
        .add_plugin(LogPlugin::default())
        .insert_resource(ReportExecutionOrderAmbiguities)
        // This call to run() starts the app we just built!
        .add_startup_system(spawn_humans.system())
        .add_startup_system(spawn_goblins.system())
        .add_system(humans_fight_goblins.system())
        .add_system(count_goblins.system())
        // .add_system(log_system.system())
        .run();
}
