use crate::prelude::*;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(clear_bg_system())
        .add_thread_local(draw_grid_system())
        .add_thread_local(draw_turn_tracker_system())
        .add_thread_local(draw_grid_pieces_system())
        .flush()
        .add_system(print_hovered_cell_system())
        .flush()
        .add_system(end_turn_system())
        .build()
}

pub fn build_start_of_round_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(clear_bg_system())
        .add_thread_local(draw_grid_system())
        .add_thread_local(draw_turn_tracker_system())
        .add_thread_local(draw_grid_pieces_system())
        .flush()
        .add_system(print_hovered_cell_system())
        .add_system(roll_initiative_system())
        .flush()
        .add_system(end_turn_system())
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

#[system(for_each)]
fn roll_initiative(entity: &Entity, init: &Initiative, #[resource] turn_tracker: &mut TurnTracker) {
    let init_roll = thread_rng().gen_range(1..=10) + init.init_mod;
    turn_tracker.register_combatant(entity, init_roll, init.priority);
}

#[system]
#[read_component(Name)]
fn draw_turn_tracker(ecs: &SubWorld, #[resource] turn_tracker: &TurnTracker) {
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

#[system]
#[read_component(Player)]
fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_tracker: &mut TurnTracker,
    #[resource] timer: &mut f32,
) {
    let turn_entity = ecs
        .entry_ref(turn_tracker.get_current_combatant().entity)
        .unwrap();

    match turn_tracker.turn_state {
        TurnState::DeclarePhase => {
            if let Ok(_player) = turn_entity.get_component::<Player>() {
                if is_key_pressed(KeyCode::Space) {
                    turn_tracker.next_turn();
                }
            } else {
                *timer += get_frame_time();

                while *timer >= 0.5 {
                    *timer -= 0.5;
                    turn_tracker.next_turn();
                }
            }
        }
        _ => {
            *timer += get_frame_time();

            while *timer >= 0.5 {
                *timer -= 0.5;
                turn_tracker.next_turn();
            }
        }
    }
}

#[system(for_each)]
fn draw_grid_pieces(
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
