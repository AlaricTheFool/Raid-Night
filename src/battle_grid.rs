use crate::prelude::*;

pub struct BattleGrid {
    pub screen_pos: Vec2,
    pub width: i32,
    pub height: i32,
    pub grid_size: f32,
    pub line_width: f32,
}

impl BattleGrid {
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
        None
    }

    pub fn is_cell_in_bounds(&self, coord: Coordinate) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }
}
