use crate::prelude::*;
use std::cmp::Ordering;

pub struct Combatant {
    pub init: i32,
    pub priority: i32,
    pub entity: Entity,
}

impl Combatant {
    fn compare_init(&self, other: &Combatant) -> Ordering {
        let init_comparison = self.init.cmp(&other.init);
        match init_comparison {
            Ordering::Equal => self.priority.cmp(&other.priority),

            _ => init_comparison,
        }
    }
}

pub enum TurnState {
    StartOfRound,
    DeclarePhase,
    ResolvePhase,
}

pub struct TurnTracker {
    pub turn_state: TurnState,
    pub current_combatant: i32,
    pub combatants: Vec<Combatant>,
}

impl TurnTracker {
    fn order_combatants(&mut self) {
        self.combatants.sort_by(|a, b| a.compare_init(b));
    }
}
