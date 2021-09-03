use std::{
    convert::TryInto,
    io::{stdout, Write},
    time::Duration,
};

use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::{
        debug, error, info, trace, warn, App, Bundle, Commands, Entity, IntoSystem,
        ParallelSystemDescriptorCoercion, Query, Res, With, Without,
    },
};

mod map;
use crossterm::{
    cursor, execute,
    style::{self, Color, Stylize},
    QueueableCommand, Result,
};
pub use map::*;

mod rect;
use rand::{prelude::SliceRandom, Rng};
pub use rect::Rect;

struct Name(String);

struct Render;

struct Human;

struct Goblin;

struct Hp(i32);

trait Death {
    fn is_dead(&self) -> bool;
}

struct Colour(Color);

struct Position {
    x: i32,
    y: i32,
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
    render: Render,
}

// This system uses a command buffer to (potentially) add a new player to our game on each
// iteration. Normal systems cannot safely access the World instance directly because they run in
// parallel. Our World contains all of our components, so mutating arbitrary parts of it in parallel
// is not thread safe. Command buffers give us the ability to queue up changes to our World without
// directly accessing it
fn spawn_humans(mut commands: Commands) {
    for _ in 1..=5 {
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from("Human")),
                damage: Damage(0),
                hp: Hp(20),
                render: Render,
            })
            .insert(Human)
            .insert(Colour(Color::Green));
    }
}

fn spawn_goblins(mut commands: Commands) {
    for _ in 1..=5 {
        // commands.spawn().insert(Bug { legs: 4 });
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from("Goblin")),
                damage: Damage(0),
                hp: Hp(15),
                render: Render,
            })
            .insert(Goblin)
            .insert(Colour(Color::Red));
    }
}

fn humans_fight_goblins(
    mut commands: Commands,
    human_query: Query<(&Damage, &Human)>,
    mut goblin_query: Query<(Entity, &mut Hp, &Goblin)>,
) {
    // let mut humans = human_query.iter().count();
    let mut goblin_iter = goblin_query.iter_mut();

    for (human_damage, _) in human_query.iter() {
        if let Some((goblin_entity, mut goblin_hp, _)) = goblin_iter.next() {
            // Humans attack goblins
            goblin_hp.0 = goblin_hp.0 - human_damage.0;

            match goblin_hp.is_dead() {
                true => commands.entity(goblin_entity).despawn(),
                false => (),
            }
        }
    }
}

fn make_scroll_name() -> String {
    let mut rng = rand::thread_rng();

    let length = 4 + rng.gen_range(1..4);
    let mut name = "Scroll of ".to_string();

    for i in 0..length {
        if i % 2 == 0 {
            name += match rng.gen_range(1..5) {
                1 => "a",
                2 => "e",
                3 => "i",
                4 => "o",
                _ => "u",
            }
        } else {
            name += match rng.gen_range(1..21) {
                1 => "b",
                2 => "c",
                3 => "d",
                4 => "f",
                5 => "g",
                6 => "h",
                7 => "j",
                8 => "k",
                9 => "l",
                10 => "m",
                11 => "n",
                12 => "p",
                13 => "q",
                14 => "r",
                15 => "s",
                16 => "t",
                17 => "v",
                18 => "w",
                19 => "x",
                20 => "y",
                _ => "z",
            }
        }
    }

    name
}

// This system updates the score for each entity with the "Player" and "Score" component.
fn count_goblins(human_query: Query<&Human>, goblin_query: Query<&Goblin>) {
    let num_goblins = goblin_query.iter().len();
    let num_humans = human_query.iter().len();

    info!("Goblins remaining: {} ", num_goblins);
    info!("Humans remaining: {} ", num_humans);
    // info!("Scroll name: {} ", make_scroll_name());
}

fn place_creatures(
    mut commands: Commands,
    creature_query: Query<Entity, Without<Position>>,
    rooms: Res<Vec<Rect>>,
) {
    let mut rng = rand::thread_rng();

    for entity in creature_query.iter() {
        let room_option = rooms.choose(&mut rng);
        if let Some(room) = room_option {
            let room_centre = room.center();

            commands.entity(entity).insert(Position {
                x: room_centre.0,
                y: room_centre.1,
            });
        }
    }
}

// This system updates the score for each entity with the "Player" and "Score" component.
fn render_map(map: Res<Vec<TileType>>) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    draw_map(&map);
}

// execute!(
//     stdout(),
//     // Blue foreground
//     SetForegroundColor(Color::Blue),
//     // Red background
//     SetBackgroundColor(Color::Red),
//     // Print text
//     Print("Blue text on Red.".to_string()),
//     // Reset to default colors
//     ResetColor
// )

// This system updates the score for each entity with the "Player" and "Score" component.
fn draw_creatures(position_query: Query<(&Position, &Colour)>) {
    let mut stdout = stdout();

    println!("HEYYYY!!!");

    for (position, colour) in position_query.iter() {
        // execute!(
        //     stdout(),
        //     // Blue foreground
        //     SetForegroundColor(Color::Blue),
        //     // Red background
        //     SetBackgroundColor(Color::Red),
        //     // Print text
        //     Print("Blue text on Red.".to_string()),
        //     // Reset to default colors
        //     ResetColor
        // );

        stdout
            .queue(cursor::MoveTo(
                position.x.try_into().unwrap(),
                position.y.try_into().unwrap(),
            ))
            .unwrap()
            // .queue(style::PrintStyledContent("█".blue()));
            .queue(style::SetForegroundColor(colour.0))
            .unwrap()
            .queue(style::Print("@".to_string()))
            .unwrap();
    }

    stdout.queue(style::ResetColor).unwrap().flush();
}

// Our Bevy app's entry point
fn main() {
    let (rooms, map) = new_map_rooms_and_corridors();

    // Bevy apps are created using the builder pattern. We use the builder to add systems,
    // resources, and plugins to our app
    App::build()
        .insert_resource(map)
        .insert_resource(rooms)
        // Resources can be added to our app like this
        // .insert_resource(State { counter: 0 })
        // Some systems are configured by adding their settings as a resource
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs(2)))
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
        .add_startup_system(spawn_humans.system().label("spawn"))
        .add_startup_system(spawn_goblins.system().label("spawn"))
        .add_system(place_creatures.system())
        .add_system(humans_fight_goblins.system())
        .add_system(render_map.system().label("render_map"))
        .add_system(
            count_goblins
                .system()
                .label("count_goblins")
                .after("render_map"),
        )
        .add_system(draw_creatures.system().after("count_goblins"))
        .run();
}
