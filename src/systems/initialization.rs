use crate::prelude::*;

#[system]
pub fn build_starter_deck(
    commands: &mut CommandBuffer,
    #[resource] db: &mut CardDB,
    #[resource] card_zones: &mut CardZones,
) {
    let fire_breath_data = db.get_card_from_id(1);
    let first_card = fire_breath_data
        .spawn_as_entity(commands)
        .expect("Failed to Spawn a Card.");

    let claw_data = db.get_card_from_id(2);
    let second_card = claw_data
        .spawn_as_entity(commands)
        .expect("Failed to Spawn a Card.");

    let block_data = db.get_card_from_id(3);
    let third_card = block_data
        .spawn_as_entity(commands)
        .expect("Failed to Spawn a Card.");

    card_zones
        .hand
        .extend([first_card, second_card, third_card]);
}

#[system]
pub fn begin_combat(
    #[resource] gstate: &mut GameState,
    #[resource] turn_tracker: &mut TurnTracker,
) {
    *gstate = GameState::Combat;
    turn_tracker.turn_state = TurnState::StartOfRound;
}
