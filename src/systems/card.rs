use crate::prelude::*;

#[system]
pub fn render_hand(#[resource] card_zones: &CardZones) {
    render_card(
        Vec2::new(800.0, 400.0),
        "Blargasnarg".to_string(),
        69,
        "The Blargasnarg is a vicious beast that cannot be vanquished.".to_string(),
    );
}

fn render_card(pos: Vec2, card_name: String, cost: i32, card_text: String) {
    let height = 300.0;
    let width = height * 0.7;
    draw_rectangle(pos.x, pos.y, width, height, BLACK);
    let border_width = 10.0;
    let inner_tl = pos + Vec2::new(border_width, border_width);
    let inner_dimensions = Vec2::new(width, height) - (Vec2::new(border_width, border_width) * 2.0);
    draw_rectangle(
        inner_tl.x,
        inner_tl.y,
        inner_dimensions.x,
        inner_dimensions.y,
        DARKPURPLE,
    );

    // TODO: Figure out why this doesn't really work with longer titled cards.
    let mut font_size = 100;
    while measure_text(&card_name, None, font_size, 1.0).width >= inner_dimensions.x {
        font_size -= 1;
    }

    let title_text_measurement = measure_text(&card_name, None, font_size, 1.0);
    draw_text(
        &card_name,
        inner_tl.x,
        inner_tl.y + title_text_measurement.height + 1.0,
        font_size as f32,
        BLACK,
    );

    draw_text(
        &cost.to_string(),
        inner_tl.x,
        inner_tl.y + inner_dimensions.y,
        font_size as f32,
        BLACK,
    );
}
