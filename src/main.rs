use crate::prelude::*;

mod battle_grid;
mod systems;

mod prelude {
    #[derive(Debug)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
    }

    pub use crate::battle_grid::*;
    pub use crate::systems::*;
    pub use legion::*;
    pub use macroquad::prelude::*;
}

struct State {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl State {
    fn new() -> Self {
        let world = World::default();
        let mut resources = Resources::default();

        resources.insert(BattleGrid {
            screen_pos: Vec2::new(100., 100.),
            width: 5,
            height: 5,
            grid_size: 96.,
            line_width: 4.,
        });

        Self {
            world,
            resources,
            schedule: build_schedule(),
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
    loop {
        state
            .schedule
            .execute(&mut state.world, &mut state.resources);

        next_frame().await
    }
}
