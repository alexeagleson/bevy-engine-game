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
        debug, error, info, trace, warn, App, Bundle, Changed, Commands, Entity, IntoSystem,
        Labels, ParallelSystemDescriptorCoercion, Query, Res, ResMut, With, Without,
    },
};

mod system_draw_map;

mod components;
use components::Position;

mod map;
use crossterm::{
    cursor, execute,
    style::{self, Color, Stylize},
    QueueableCommand, Result,
};
pub use map::*;

mod render;
use render::{render_entities_system, Render};

mod rect;
use rand::{
    prelude::{IteratorRandom, SliceRandom},
    Rng,
};
pub use rect::Rect;

use pathfinding::prelude::{absdiff, astar};
use rltk::BaseMap;
use system_draw_map::{draw_full_map, draw_map_changes};

struct Name(String);

struct Human;

struct Goblin;

struct Hp(i32);

trait Death {
    fn is_dead(&self) -> bool;
}

struct Path(Vec<(i32, i32)>, usize);

struct Target(Option<(Position, Entity)>);

struct Damage(i32);

impl Death for Hp {
    fn is_dead(&self) -> bool {
        self.0 <= 0
    }
}

struct Food;

struct Hunger(i32);

#[derive(Bundle)]
struct CreatureBundle {
    name: Name,
    hp: Hp,
    damage: Damage,
    render: render::Render,
    hunger: Hunger,
}

pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

use rltk::{field_of_view, Point};

fn draw_fov(
    // mut commands: Commands,
    viewshed_query: Query<&Viewshed>,
    // mut goblin_query: Query<(Entity, &mut Hp, &Goblin)>,
    map: Res<map::Map>,
) {
    let mut stdout = stdout();
    stdout
        .queue(style::SetForegroundColor(style::Color::Magenta))
        .unwrap();

    for viewshed in viewshed_query.iter() {
        for point in viewshed.visible_tiles.iter() {
            let tile = map.tiles[map.xy_idx(point.x, point.y)];

            stdout
                .queue(cursor::MoveTo(
                    point.x.try_into().unwrap(),
                    point.y.try_into().unwrap(),
                ))
                .unwrap()
                .queue(style::Print(tile_to_char(&tile)))
                .unwrap();
        }
    }
}

fn calculate_viewshed(
    // mut commands: Commands,
    mut viewshed_query: Query<(&mut Viewshed, &Position)>,
    // mut goblin_query: Query<(Entity, &mut Hp, &Goblin)>,
    map: Res<map::Map>,
) {
    for (mut viewshed, position) in viewshed_query.iter_mut() {
        // viewshed.dirty = false;
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles =
            field_of_view(Point::new(position.0, position.1), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
    }
}

// This system uses a command buffer to (potentially) add a new player to our game on each
// iteration. Normal systems cannot safely access the World instance directly because they run in
// parallel. Our World contains all of our components, so mutating arbitrary parts of it in parallel
// is not thread safe. Command buffers give us the ability to queue up changes to our World without
// directly accessing it
fn spawn_humans(mut commands: Commands) {
    for _ in 1..=10 {
        commands
            .spawn_bundle(CreatureBundle {
                name: Name(String::from("Human")),
                damage: Damage(0),
                hp: Hp(20),
                render: Render {
                    colour: Color::Green,
                    char: "@".to_string(),
                },
                hunger: Hunger(0),
            })
            .insert(Human)
            .insert(Target(None))
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 3,
                dirty: true,
            });
        // .insert(ReadyForPath(true));
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
                render: Render {
                    colour: Color::Red,
                    char: "G".to_string(),
                },
                hunger: Hunger(100),
            })
            .insert(Goblin);
        // .insert(ReadyForPath(true));
    }
}

fn spawn_food(mut commands: Commands) {
    for _ in 1..=10 {
        commands
            .spawn()
            .insert(Render {
                colour: Color::Yellow,
                char: "F".to_string(),
            })
            .insert(Food);
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
fn count_goblins(human_query: Query<&Human>, goblin_query: Query<&Goblin>, log: Res<Vec<String>>) {
    let num_goblins = goblin_query.iter().len();
    let num_humans = human_query.iter().len();

    info!("Goblins remaining: {} ", num_goblins);
    info!("Humans remaining: {} ", num_humans);
    // info!("Scroll name: {} ", make_scroll_name());

    for text in log.iter() {
        info!("{}", text);
    }
}

fn place_creatures(
    mut commands: Commands,
    creature_query: Query<Entity, Without<Position>>,
    map: Res<map::Map>,
) {
    let mut rng = rand::thread_rng();

    for entity in creature_query.iter() {
        let room_option = map.rooms.choose(&mut rng);
        if let Some(room) = room_option {
            let room_centre = room.center();

            commands
                .entity(entity)
                .insert(Position(room_centre.0, room_centre.1));
        }
    }
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

fn move_path(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &Name, &mut Position, &mut Path, &mut Target)>,
    map: Res<map::Map>,
    mut log: ResMut<Vec<String>>,
    mut changed_positions: ResMut<Vec<Position>>,
) {
    for (entity, name, mut position, mut path, mut target) in creature_query.iter_mut() {
        let idx = path.1;
        if path.0.len() > idx {
            let (next_x, next_y) = path.0[idx];
            changed_positions.push(position.clone());
            position.0 = next_x;
            position.1 = next_y;
            path.1 += 1; // Move to next path index
        } else {
            if let Some(target) = &target.0 {
                commands.entity(target.1).despawn();
                // log.push(format!("{} eats some food", name.0));
            }

            target.0 = None;
        }
    }
}

fn create_paths(
    mut commands: Commands,
    mut creature_query: Query<(Entity, &Position, &mut Target)>,
    food_query: Query<(Entity, &Food, &Position)>,
    map: Res<map::Map>,
) {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();

    for (entity, position, mut target) in creature_query.iter_mut() {
        if target.0.is_none() {
            // let room_option = rooms.choose(&mut rng).unwrap();

            // let GOAL: (i32, i32) = room_option.center();

            let random_food = food_query.iter().choose(&mut rng);

            if let Some((food_ent, _food, food_pos)) = random_food {
                // let new_x = position.0 + rng.gen_range(-1..=1);
                // let new_y = position.1 + rng.gen_range(-1..=1);

                // let mut fff: Vec<Position> = Vec::with_capacity(4);

                let result = astar(
                    &(position.0, position.1),
                    |&(x, y)| {
                        vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                            .into_iter()
                            .filter(|&(x, y)| map.is_opaque(map.xy_idx(x, y)) == false)
                            .map(|p| (p, 1))
                    },
                    |&(x, y)| absdiff(x, food_pos.0) + absdiff(y, food_pos.1),
                    |&p| p.0 == food_pos.0 && p.1 == food_pos.1,
                );
                // assert_eq!(result.expect("no path found").1, 4);

                if let Some(fff) = result {
                    let (ttt, yyy) = fff;

                    commands.entity(entity).insert(Path(ttt, 0));

                    target.0 = Some((food_pos.clone(), food_ent));
                }
            }
        }
    }
}

fn flush_stdout() {
    let mut stdout = stdout();
    stdout.queue(style::ResetColor).unwrap().flush();
}

// Our Bevy app's entry point
fn main() {
    // let (rooms, map) = new_map_rooms_and_corridors();
    let map: Map = Map::new_map_rooms_and_corridors();
    let log: Vec<String> = Vec::new();
    let changed_map_positions: Vec<Position> = Vec::new();
    // Bevy apps are created using the builder pattern. We use the builder to add systems,
    // resources, and plugins to our app
    App::build()
        .insert_resource(log)
        .insert_resource(map)
        // .insert_resource(rooms)
        .insert_resource(changed_map_positions)
        // Resources can be added to our app like this
        // .insert_resource(State { counter: 0 })
        // Some systems are configured by adding their settings as a resource
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(300)))
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
        .add_startup_system(spawn_food.system().label("spawn"))
        .add_startup_system(draw_full_map.system())
        .add_system(place_creatures.system())
        .add_system(
            draw_map_changes
                .system()
                .label("draw_map_changes")
                .after("move_creatures"),
        )
        .add_system(humans_fight_goblins.system())
        // .add_system(render_map.system().label("render_map"))
        // .add_system(
        //     count_goblins
        //         .system()
        //         .label("count_goblins")
        //         .after("render_map"),
        // )
        .add_system(
            render_entities_system
                .system()
                .label("draw_entities")
                .after("draw_map_changes")
                .after("calculate_viewshed"),
        )
        .add_system(create_paths.system().label("create paths"))
        .add_system(
            move_path
                .system()
                .label("move_creatures")
                .after("create paths"),
        )
        .add_system(
            flush_stdout
                .system()
                .label("flush_stdout")
                .after("draw_entities")
                .after("calculate_viewshed"),
        )
        .add_system(
            calculate_viewshed
                .system()
                .label("calculate_viewshed")
                .after("move_creatures"),
        )
        .add_system(
            draw_fov
                .system()
                .after("calculate_viewshed")
                .after("draw_map_changes")
                .before("draw_entities")
                .before("flush_stdout"),
        )
        .run();
}
