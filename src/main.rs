mod cleanup;
mod combat;
mod components;
mod fov;
mod hunger;
mod map;
mod path;
mod position;
mod rect;
mod render;
mod spawner;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::{App, IntoSystem, ParallelSystemDescriptorCoercion},
};

use cleanup::despawn;
use combat::{death, fight_nearby_goblins};
use crossterm::{style::ResetColor, QueueableCommand};

use fov::{calculate_viewshed, clear_viewshed, draw_viewshed};
use hunger::eat_nearby_food;
use map::{draw_full_map, draw_map_changes, Map};

use path::{create_path, move_path};
use position::{assign_positions, Position};
use render::render_entities_system;

use spawner::{spawn_food, spawn_goblins, spawn_humans};

fn flush_stdout() {
    let mut stdout = stdout();
    stdout.queue(ResetColor).unwrap().flush();
}

// Our Bevy app's entry point
fn main() {
    let map: Map = Map::new_map_rooms_and_corridors();
    let log: Vec<String> = Vec::new();
    let changed_map_positions: Vec<Position> = Vec::new();

    // Bevy apps are created using the builder pattern. We use the builder to add systems,
    // resources, and plugins to our app
    App::build()
        .insert_resource(log)
        .insert_resource(map)
        .insert_resource(changed_map_positions)
        // Some systems are configured by adding their settings as a resource
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(400)))
        .insert_resource(ReportExecutionOrderAmbiguities)
        // Plugins are just a grouped set of app builder calls (just like we're doing here).
        // We could easily turn our game into a plugin, but you can check out the plugin example for
        // that :) The plugin below runs our app's "system schedule" once every 5 seconds
        // (configured above).
        .add_plugin(LogPlugin::default())
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
        // We can check our systems for execution order ambiguities by examining the output produced
        // in the console by using the `LogPlugin` and adding the following Resource to our App :)
        // Be aware that not everything reported by this checker is a potential problem, you'll have
        // to make that judgement yourself.
        // This call to run() starts the app we just built!
        // Startup systems
        .add_startup_system(spawn_humans.system())
        .add_startup_system(spawn_goblins.system())
        .add_startup_system(spawn_food.system())
        .add_startup_system(draw_full_map.system())
        // initialize
        .add_system(assign_positions.system().label("initialize"))
        .add_system(create_path.system().label("initialize"))
        // before_move
        .add_system(
            eat_nearby_food
                .system()
                .label("before_move")
                .after("initialize"),
        )
        .add_system(
            fight_nearby_goblins
                .system()
                .label("before_move")
                .after("initialize"),
        )
        // move
        .add_system(move_path.system().label("move").after("before_move"))
        // draw_map_changes
        .add_system(
            draw_map_changes
                .system()
                .label("draw_map_changes")
                .after("move"),
        )
        .add_system(
            clear_viewshed
                .system()
                .label("clear_viewshed")
                .after("draw_map_changes"),
        )
        .add_system(
            calculate_viewshed
                .system()
                .label("calculate_viewshed")
                .after("clear_viewshed"),
        )
        // draw_viewshed
        .add_system(
            draw_viewshed
                .system()
                .label("draw_viewshed")
                .after("calculate_viewshed"),
        )
        // draw_entities
        .add_system(
            render_entities_system
                .system()
                .label("draw_entities")
                .after("draw_viewshed"),
        )
        // cleanup_entities
        .add_system(
            death
                .system()
                .label("cleanup_entities")
                .after("draw_entities"),
        )
        .add_system(
            despawn
                .system()
                .label("cleanup_entities")
                .after("draw_entities"),
        )
        // flush_stdout
        .add_system(
            flush_stdout
                .system()
                .label("flush_stdout")
                .after("draw_entities"),
        )
        .run();
}
