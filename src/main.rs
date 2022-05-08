use crate::prelude::*;
mod battle_grid;
mod card_data;
mod card_zones;
mod components;
mod coordinate;
mod direction;
mod game_state;
mod move_action;
mod systems;
mod turn_tracker;

mod prelude {

    pub use crate::battle_grid::*;
    pub use crate::card_data::*;
    pub use crate::card_zones::*;
    pub use crate::components::*;
    pub use crate::coordinate::*;
    pub use crate::direction::*;
    pub use crate::game_state::*;
    pub use crate::move_action::*;
    pub use crate::systems::*;
    pub use crate::turn_tracker::*;
    pub use ::rand::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use macroquad::prelude::*;
}

struct State {
    world: World,
    resources: Resources,
    game_initialization_schedule: Schedule,
    start_of_round_schedule: Schedule,
    declare_phase_schedule: Schedule,
    resolve_phase_schedule: Schedule,
}

impl State {
    fn new() -> Self {
        let world = World::default();
        let mut resources = Resources::default();

        resources.insert(GameState::Initialization);
        resources.insert(CardDB::new());
        resources.insert(CardZones::new());

        resources.insert(BattleGrid::new());

        resources.insert(TurnTracker::new());
        resources.insert(0.0 as f32);

        Self {
            world,
            resources,
            game_initialization_schedule: build_game_initialization_schedule(),
            start_of_round_schedule: build_start_of_round_schedule(),
            declare_phase_schedule: build_declare_phase_schedule(),
            resolve_phase_schedule: build_resolve_phase_schedule(),
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Raid Night".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::new();

    state.world.push((
        Player,
        Initiative {
            init_mod: 0,
            priority: 100,
        },
        Name {
            val: "You".to_string(),
        },
    ));

    (0..3).for_each(|num| {
        let (name, init_mod, (y, x), color) = match num {
            0 => ("Fighter", -1, (1, 2), ORANGE),
            1 => ("Cleric", 0, (0, 1), BLUE),
            2 => ("Wizard", 1, (0, 3), RED),
            _ => ("Hero", 0, (0, 0), PINK),
        };

        state.world.push((
            Enemy,
            Initiative {
                init_mod,
                priority: 0,
            },
            Name {
                val: name.to_string(),
            },
            Coordinate { x, y },
            color,
            ActionPoints::new(3),
            TileStatus::Occupied,
        ));
    });

    loop {
        let game_state = state.resources.get::<GameState>().unwrap().clone();

        match game_state {
            GameState::Initialization => {
                state
                    .game_initialization_schedule
                    .execute(&mut state.world, &mut state.resources);
            }

            GameState::Combat => {
                let turn_tracker = state.resources.get::<TurnTracker>().unwrap().clone();

                match turn_tracker.turn_state {
                    TurnState::StartOfRound => {
                        state
                            .start_of_round_schedule
                            .execute(&mut state.world, &mut state.resources);
                    }

                    TurnState::DeclarePhase => {
                        state
                            .declare_phase_schedule
                            .execute(&mut state.world, &mut state.resources);
                    }

                    TurnState::ResolvePhase => {
                        state
                            .resolve_phase_schedule
                            .execute(&mut state.world, &mut state.resources);
                    }
                }
            }
        }

        next_frame().await
    }
}
