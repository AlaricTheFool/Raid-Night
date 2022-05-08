use crate::prelude::*;

#[system]
pub fn clear_bg() {
    clear_background(DARKBROWN);
}

#[system]
pub fn draw_grid(#[resource] grid: &BattleGrid) {
    grid.draw_from_top_left();
}

#[system(for_each)]
pub fn draw_grid_pieces(
    coords: &Coordinate,
    name: &Name,
    color: &Color,
    #[resource] grid: &BattleGrid,
) {
    let tl = grid.get_cell_tl(coords.x, coords.y);

    draw_rectangle(
        tl.x + 4.0,
        tl.y + 4.0,
        grid.grid_size - 8.0,
        grid.grid_size - 8.0,
        *color,
    );

    draw_text(
        &name.val[0..1],
        tl.x + (grid.grid_size * 0.5),
        tl.y + grid.grid_size - 8.0,
        48.0,
        BLACK,
    );
}

#[system]
#[read_component(Name)]
pub fn draw_turn_tracker(ecs: &SubWorld, #[resource] turn_tracker: &TurnTracker) {
    let origin = Vec2::new(1000., 64.);
    draw_rectangle(origin.x, origin.y, 150.0, 200.0, GRAY);

    draw_text(
        &format!("{:?}", turn_tracker.turn_state),
        origin.x,
        origin.y + 32.0,
        32.0,
        BLACK,
    );

    turn_tracker
        .combatants
        .iter()
        .enumerate()
        .for_each(|(idx, combatant)| {
            let name = match ecs
                .entry_ref(combatant.entity)
                .unwrap()
                .get_component::<Name>()
            {
                Ok(name) => format!("[{}] {}", combatant.init, name.val),

                _ => "???".to_string(),
            };

            let display_text = match idx == turn_tracker.current_combatant {
                true => format!("> {name}"),

                false => format!("  {name}"),
            };
            draw_text(
                &display_text,
                origin.x,
                60.0 + origin.y + idx as f32 * 32.0,
                24.,
                BLACK,
            );
        });
}

#[system(for_each)]
#[read_component(Coordinate)]
#[read_component(Color)]
pub fn draw_declared_moves(
    ecs: &mut SubWorld,
    _msg: &Message,
    src: &Source,
    mv: &Move,
    #[resource] grid: &BattleGrid,
) {
    let mut entity_query = <(Entity, &Color, &Coordinate)>::query();

    entity_query
        .iter(ecs)
        .filter(|(entity, _, _)| **entity == src.entity)
        .for_each(|(_, color, coord)| {
            let mut current_coord = *coord;
            mv.dirs.iter().for_each(|dir| {
                let new_coord = current_coord + Coordinate::from(*dir);

                let line_start = grid.get_cell_center(current_coord.x, current_coord.y);
                let line_end = grid.get_cell_center(new_coord.x, new_coord.y);

                draw_line(
                    line_start.x,
                    line_start.y,
                    line_end.x,
                    line_end.y,
                    4.0,
                    *color,
                );

                current_coord = new_coord;
            });
        });
}
