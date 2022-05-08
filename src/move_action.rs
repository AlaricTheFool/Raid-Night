use crate::prelude::*;

pub fn generate_random_walk(
    pos: Coordinate,
    grid: &BattleGrid,
    depth: i32,
    max_depth: i32,
    last_dir: Option<Direction>,
) -> Vec<Direction> {
    let all_dirs = Direction::all();
    let random_dir = all_dirs
        .iter()
        .filter(|dir| last_dir.is_none() || last_dir.unwrap().reverse() != **dir)
        .filter(|dir| grid.is_cell_in_bounds(pos + Coordinate::from(**dir)))
        .choose::<ThreadRng>(&mut thread_rng());

    if random_dir.is_some() {
        let chosen_dir = *random_dir.unwrap();

        let mut this_step = vec![chosen_dir];
        if depth == max_depth {
            this_step
        } else {
            let mut other_steps = generate_random_walk(
                pos + Coordinate::from(chosen_dir),
                grid,
                depth + 1,
                max_depth,
                Some(chosen_dir),
            );

            this_step.append(&mut other_steps);
            this_step
        }
    } else {
        Vec::new()
    }
}
