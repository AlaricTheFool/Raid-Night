use crate::prelude::*;

const CARD_HEIGHT: f32 = 200.0;
const CARD_WIDTH: f32 = CARD_HEIGHT * 0.7;

const HAND_ZONE_Y: f32 = 600.0;
const HAND_ZONE_SPAN: std::ops::Range<f32> = 700.0..1200.0;
const HAND_ZONE_WIDTH: f32 = HAND_ZONE_SPAN.end - HAND_ZONE_SPAN.start;

const CARD_MAX_ARC_DOWN: f32 = 64.0;

#[system]
pub fn render_hand(#[resource] card_zones: &CardZones) {
    let cards_in_hand = 10;

    draw_debug_hand_zone();

    (0..cards_in_hand).for_each(|idx| {
        let card_pos = calculate_card_pos(idx, cards_in_hand);
        // TODO: ADJUST TO TOP LEFT WHEN YOU UNCOMMENT
        render_card(
            calculate_card_pos(idx, cards_in_hand) - Vec2::new(CARD_WIDTH / 2.0, CARD_HEIGHT / 2.0),
            "Blargasnarg".to_string(),
            69,
            "The Blargasnarg is a vicious beast that cannot be vanquished.".to_string(),
        );
    });
}

fn draw_debug_hand_zone() {
    draw_line(
        HAND_ZONE_SPAN.start,
        HAND_ZONE_Y + CARD_MAX_ARC_DOWN,
        HAND_ZONE_SPAN.end,
        HAND_ZONE_Y + CARD_MAX_ARC_DOWN,
        4.0,
        BLUE,
    );
    draw_line(
        HAND_ZONE_SPAN.start,
        HAND_ZONE_Y,
        HAND_ZONE_SPAN.end,
        HAND_ZONE_Y,
        4.0,
        BLUE,
    );
}

fn calculate_card_pos(idx_in_hand: i32, total_cards_in_hand: i32) -> Vec2 {
    let width_per_card = HAND_ZONE_WIDTH / total_cards_in_hand as f32;

    let center_idx = (total_cards_in_hand - 1) as f32 / 2.0;
    let dist_from_center_idx = (idx_in_hand as f32 - center_idx).abs();
    let dist_factor = (dist_from_center_idx / center_idx).powi(2);

    if idx_in_hand == 0 {
        eprintln!("{dist_factor:?}");
    }

    Vec2::new(
        HAND_ZONE_SPAN.start + (width_per_card * (idx_in_hand as f32 + 0.5)),
        HAND_ZONE_Y + (CARD_MAX_ARC_DOWN * dist_factor),
    )
}

fn render_card(pos: Vec2, card_name: String, cost: i32, card_text: String) {
    draw_rectangle(pos.x, pos.y, CARD_WIDTH, CARD_HEIGHT, BLACK);
    let border_width = 10.0;
    let inner_tl = pos + Vec2::new(border_width, border_width);
    let inner_dimensions =
        Vec2::new(CARD_WIDTH, CARD_HEIGHT) - (Vec2::new(border_width, border_width) * 2.0);
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
