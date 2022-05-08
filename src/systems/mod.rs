use crate::prelude::*;

mod draw;

pub fn build_start_of_round_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(draw::clear_bg_system())
        .add_thread_local(draw::draw_grid_system())
        .add_thread_local(draw::draw_turn_tracker_system())
        .add_thread_local(draw::draw_grid_pieces_system())
        .add_thread_local(draw::draw_declared_moves_system())
        .flush()
        .add_system(roll_initiative_system())
        .add_system(clear_round_messages_system())
        .add_system(update_tile_statuses_system())
        .flush()
        .add_system(end_turn_system())
        .build()
}

pub fn build_declare_phase_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(draw::clear_bg_system())
        .add_thread_local(draw::draw_grid_system())
        .add_thread_local(draw::draw_turn_tracker_system())
        .add_thread_local(draw::draw_grid_pieces_system())
        .add_thread_local(draw::draw_declared_moves_system())
        .flush()
        .add_system(declare_ai_action_system())
        .flush()
        .add_system(end_turn_system())
        .build()
}

pub fn build_resolve_phase_schedule() -> Schedule {
    Schedule::builder()
        .add_thread_local(draw::clear_bg_system())
        .add_thread_local(draw::draw_grid_system())
        .add_thread_local(draw::draw_turn_tracker_system())
        .add_thread_local(draw::draw_grid_pieces_system())
        .add_thread_local(draw::draw_declared_moves_system())
        .flush()
        .add_system(resolve_moves_system())
        .flush()
        .add_system(end_turn_system())
        .build()
}

#[system(for_each)]
fn roll_initiative(entity: &Entity, init: &Initiative, #[resource] turn_tracker: &mut TurnTracker) {
    let init_roll = thread_rng().gen_range(1..=10) + init.init_mod;
    turn_tracker.register_combatant(entity, init_roll, init.priority);
}

#[system]
#[read_component(Player)]
#[read_component(Message)]
#[read_component(Source)]
#[read_component(ActionDeclarationFinished)]
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
                let mut msg_query = <(&Message, &Source, &ActionDeclarationFinished)>::query();

                if msg_query
                    .iter(ecs)
                    .filter(|(_, src, _)| src.entity == turn_tracker.get_current_combatant().entity)
                    .nth(0)
                    .is_some()
                {
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

#[system]
#[read_component(Enemy)]
#[read_component(Coordinate)]
fn declare_ai_action(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] turn_tracker: &TurnTracker,
    #[resource] grid: &BattleGrid,
    #[resource] timer: &mut f32,
) {
    if let Ok(current_combatant) = ecs.entry_ref(turn_tracker.get_current_combatant().entity) {
        *timer += get_frame_time();

        if *timer >= 0.5 {
            *timer = 0.0;

            if let Ok(coord) = current_combatant.get_component::<Coordinate>() {
                let rand_walk = generate_random_walk(*coord, grid, 1, 3, None);

                commands.push((
                    (),
                    Message,
                    Source {
                        entity: turn_tracker.get_current_combatant().entity,
                    },
                    Move { dirs: rand_walk },
                    Round,
                ));
            }
            commands.push((
                (),
                Message,
                ActionDeclarationFinished,
                Source {
                    entity: turn_tracker.get_current_combatant().entity,
                },
                Round,
            ));
        }
    }
}

#[system(for_each)]
fn clear_round_messages(commands: &mut CommandBuffer, entity: &Entity, _msg: &Message, _: &Round) {
    commands.remove(*entity);
}

#[system(for_each)]
#[write_component(Coordinate)]
#[read_component(TileStatus)]
fn resolve_moves(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    m_entity: &Entity,
    _msg: &Message,
    src: &Source,
    mv: &Move,
    #[resource] grid: &mut BattleGrid,
    #[resource] turn_tracker: &TurnTracker,
) {
    let mut coord_query = <(Entity, &mut Coordinate, &TileStatus)>::query();

    if let Some(start) = coord_query
        .iter_mut(ecs)
        .filter(|(entity, _, _)| {
            **entity == src.entity && turn_tracker.get_current_combatant().entity == src.entity
        })
        .nth(0)
    {
        let mut final_location = mv
            .dirs
            .iter()
            .fold(*start.1, |accum, item| accum + Coordinate::from(*item));

        eprintln!(
            "Moving from space: {:?} to {final_location:?} with directions {:?}",
            start.1, mv.dirs
        );

        if mv
            .dirs
            .iter()
            .rev()
            .find(|dir| {
                if grid.is_cell_in_bounds(final_location)
                    && grid.get_status_at_coord(&final_location) == TileStatus::Empty
                {
                    return true;
                } else {
                    final_location -= Coordinate::from(**dir);
                    return false;
                }
            })
            .is_some()
        {
            grid.set_status_at_coord(start.1, TileStatus::Empty);
            *start.1 = final_location;
            grid.set_status_at_coord(&final_location, *start.2);
        }

        commands.remove(*m_entity);
    }
}

#[system(for_each)]
fn update_tile_statuses(
    coord: &Coordinate,
    t_status: &TileStatus,
    #[resource] grid: &mut BattleGrid,
) {
    grid.set_status_at_coord(coord, *t_status);
}
