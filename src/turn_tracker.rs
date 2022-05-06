use crate::prelude::*;
use std::cmp::Ordering;

#[derive(PartialEq, Copy, Clone)]
pub struct Combatant {
    pub init: i32,
    pub priority: i32,
    pub entity: Entity,
}

impl Combatant {
    fn compare_init(&self, other: &Combatant) -> Ordering {
        let init_comparison = self.init.cmp(&other.init).reverse();
        match init_comparison {
            Ordering::Equal => self.priority.cmp(&other.priority).reverse(),

            _ => init_comparison,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TurnState {
    StartOfRound,
    DeclarePhase,
    ResolvePhase,
}

#[derive(PartialEq, Clone)]
pub struct TurnTracker {
    pub turn_state: TurnState,
    pub current_combatant: usize,
    pub combatants: Vec<Combatant>,
}

impl TurnTracker {
    pub fn new() -> Self {
        Self {
            turn_state: TurnState::StartOfRound,
            current_combatant: 0,
            combatants: Vec::new(),
        }
    }

    fn order_combatants(&mut self) {
        self.combatants.sort_by(|a, b| a.compare_init(b));
    }

    pub fn next_turn(&mut self) {
        match self.turn_state {
            TurnState::StartOfRound => {
                self.order_combatants();
                self.jump_to_last_combatant();
                self.turn_state = TurnState::DeclarePhase;
            }

            TurnState::DeclarePhase => {
                if self.current_combatant > 0 {
                    self.current_combatant -= 1;
                } else {
                    self.turn_state = TurnState::ResolvePhase;
                    self.jump_to_first_combatant();
                }
            }

            TurnState::ResolvePhase => {
                if self.current_combatant < self.combatants.len() - 1 {
                    self.current_combatant += 1;
                } else {
                    self.turn_state = TurnState::StartOfRound;
                }
            }
        }
    }

    fn jump_to_last_combatant(&mut self) {
        self.current_combatant = self.combatants.len() - 1;
    }

    fn jump_to_first_combatant(&mut self) {
        self.current_combatant = 0;
    }

    // Adds an entity to the combatants list at the specified initiative. Or updates the initiative
    // of an already existing entity.
    pub fn register_combatant(&mut self, entity: &Entity, init: i32, priority: i32) {
        if let Some(existing) = self
            .combatants
            .iter_mut()
            .filter(|combatant| combatant.entity == *entity)
            .nth(0)
        {
            existing.init = init;
            existing.priority = priority;
        } else {
            self.combatants.push(Combatant {
                entity: *entity,
                init,
                priority,
            })
        }

        if self.turn_state != TurnState::StartOfRound {
            self.order_combatants();
        }
    }

    pub fn get_current_combatant(&self) -> Combatant {
        self.combatants[self.current_combatant]
    }
}
