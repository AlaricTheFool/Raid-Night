use crate::prelude::*;
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TileStatus {
    Empty,
    Occupied,
}

pub struct BattleGrid {
    pub screen_pos: Vec2,
    pub width: i32,
    pub height: i32,
    pub grid_size: f32,
    pub line_width: f32,
    pub tile_statuses: Vec<TileStatus>,
}

impl BattleGrid {
    pub fn new() -> Self {
        let mut t_statuses = Vec::new();
        t_statuses.resize(5 * 5, TileStatus::Empty);

        Self {
            screen_pos: Vec2::new(100., 100.),
            width: 5,
            height: 5,
            grid_size: 96.,
            line_width: 4.,
            tile_statuses: t_statuses,
        }
    }

    pub fn print_pretty_statuses(&self) {
        self.tile_statuses
            .iter()
            .enumerate()
            .for_each(|(idx, status)| {
                eprint!("{status:?}");

                if idx > 0 && idx % self.width as usize == 0 {
                    eprint!("\n");
                } else {
                    eprint!(", ");
                }
            });
        eprint!("\n");
        io::stdout().flush().unwrap();
    }

    pub fn set_status_at_coord(&mut self, coord: &Coordinate, status: TileStatus) {
        self.tile_statuses[(coord.x + (coord.y * self.width)) as usize] = status;
    }

    pub fn get_status_at_coord(&self, coord: &Coordinate) -> TileStatus {
        self.tile_statuses[(coord.x + (coord.y * self.width)) as usize]
    }

    pub fn draw_from_top_left(&self) {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let tl = Vec2::new(
                    (x as f32 * (self.grid_size + self.line_width)) as f32 + self.screen_pos.x,
                    (y as f32 * (self.grid_size + self.line_width)) as f32 + self.screen_pos.y,
                );
                draw_rectangle_lines(
                    tl.x - self.line_width,
                    tl.y - self.line_width,
                    self.grid_size as f32 + self.line_width * 2.0,
                    self.grid_size as f32 + self.line_width * 2.0,
                    self.line_width * 2.0,
                    GRAY,
                );
            });
        });
    }

    pub fn get_cell_tl(&self, x: i32, y: i32) -> Vec2 {
        Vec2::new(
            self.screen_pos.x + x as f32 * (self.grid_size + self.line_width),
            self.screen_pos.y + y as f32 * (self.grid_size + self.line_width),
        )
    }

    pub fn get_cell_center(&self, x: i32, y: i32) -> Vec2 {
        self.get_cell_tl(x, y) + Vec2::new(self.grid_size * 0.5, self.grid_size * 0.5)
    }

    pub fn get_cell_at_screen_pos(&self, screen_pos: Vec2) -> Option<Coordinate> {
        let offset = screen_pos - self.screen_pos;
        if offset.x >= 0. && offset.y >= 0. {
            let closest_prev_x = (offset.x / (self.grid_size + self.line_width)).floor();
            let closest_prev_y = (offset.y / (self.grid_size + self.line_width)).floor();

            // Check in bounds
            if (closest_prev_x as i32) < self.width && (closest_prev_y as i32) < self.height {
                // Check for borders
                let cell_offset_x = offset.x - closest_prev_x * (self.grid_size + self.line_width);
                let cell_offset_y = offset.y - closest_prev_y * (self.grid_size + self.line_width);

                if cell_offset_x <= self.grid_size && cell_offset_y <= self.grid_size {
                    let coord = Coordinate {
                        x: closest_prev_x as i32,
                        y: closest_prev_y as i32,
                    };
                    if self.is_cell_in_bounds(coord) {
                        return Some(Coordinate {
                            x: closest_prev_x as i32,
                            y: closest_prev_y as i32,
                        });
                    }
                }
            }
        }
        None
    }

    pub fn is_cell_in_bounds(&self, coord: Coordinate) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_borders_dont_count_for_screen_pos() {
        let grid = BattleGrid::new();
        let tl = grid.get_cell_tl(0, 0);
        let adjusted_screen_pos = tl + (Vec2::X * (grid.grid_size + 1.0));
        eprintln!("{adjusted_screen_pos:?}");
        let cell_at_screen_pos = grid.get_cell_at_screen_pos(adjusted_screen_pos);

        assert!(cell_at_screen_pos.is_none());
    }

    #[test]
    fn test_cells_start_empty() {
        let grid = BattleGrid::new();

        (0..grid.height).for_each(|y| {
            (0..grid.width).for_each(|x| {
                assert_eq!(
                    grid.get_status_at_coord(&Coordinate { x, y }),
                    TileStatus::Empty
                );
            })
        })
    }

    #[test]
    fn test_cells_are_edited_and_checked_at_same_loc() {
        let mut grid = BattleGrid::new();
        let test_coord = Coordinate { x: 3, y: 2 };
        let other_coord = Coordinate { x: 1, y: 4 };

        grid.set_status_at_coord(&test_coord, TileStatus::Occupied);
        assert_eq!(grid.get_status_at_coord(&test_coord), TileStatus::Occupied);

        grid.set_status_at_coord(&other_coord, TileStatus::Occupied);
        assert_eq!(grid.get_status_at_coord(&test_coord), TileStatus::Occupied);

        grid.set_status_at_coord(&test_coord, TileStatus::Empty);
        assert_eq!(grid.get_status_at_coord(&test_coord), TileStatus::Empty);
    }
}
