use crate::prelude::*;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(clear_bg_system())
        .add_thread_local(draw_grid_system())
        .flush()
        .add_system(print_hovered_cell_system())
        .build()
}

#[system]
fn clear_bg() {
    clear_background(DARKBROWN);
}

#[system]
fn draw_grid(#[resource] grid: &BattleGrid) {
    grid.draw_from_top_left();
}

#[system]
fn print_hovered_cell(#[resource] grid: &BattleGrid) {
    let mouse_pos = mouse_position();
    let cell = grid.get_cell_at_screen_pos(Vec2::new(mouse_pos.0, mouse_pos.1));
    eprintln!("Cell under mouse at {mouse_pos:?} is {cell:?}");
}
