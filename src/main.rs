use crate::prelude::*;

mod battle_grid;

mod prelude {
    #[derive(Debug)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
    }

    pub use crate::battle_grid::*;
    pub use macroquad::prelude::*;
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
    let grid = BattleGrid {
        screen_pos: Vec2::new(100., 100.),
        width: 5,
        height: 5,
        grid_size: 96.,
        line_width: 4.,
    };
    loop {
        clear_background(BLACK);

        grid.draw_from_top_left();

        let mouse_pos = mouse_position();
        let cell = grid.get_cell_at_screen_pos(Vec2::new(mouse_pos.0, mouse_pos.1));
        eprintln!("Cell under mouse as {mouse_pos:?} is {cell:?}");

        next_frame().await
    }
}
