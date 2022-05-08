use crate::prelude::*;
use rusqlite::Connection;

pub struct CardDB {
    connection: Connection,
}

impl CardDB {
    pub fn new() -> Self {
        CardDB {
            connection: Connection::open("./assets/cards.db").unwrap(),
        }
    }

    pub fn get_card_from_id(&mut self, id: i32) -> CardData {
        let mut stmt = self
            .connection
            .prepare(&format!(
                "SELECT CardID, Name, Cost, Effects FROM Cards WHERE CardID={}",
                id
            ))
            .unwrap();

        let mut card_iter = stmt
            .query_map([], |row| {
                Ok(CardData {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    cost: row.get(2).unwrap(),
                    effects: row.get(3).unwrap(),
                })
            })
            .unwrap();

        card_iter.nth(0).unwrap().unwrap()
    }

    pub fn draw_random(&mut self, count: i32) -> Result<Vec<CardData>, rusqlite::Error> {
        let mut stmt = self
            .connection
            .prepare(&format!(
                "SELECT CardID, Name, Cost, Effects FROM Cards ORDER BY RANDOM() LIMIT {}",
                count
            ))
            .unwrap();

        let card_iter = stmt
            .query_map([], |row| {
                Ok(CardData {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                    cost: row.get(2).unwrap(),
                    effects: row.get(3).unwrap(),
                })
            })
            .unwrap();

        card_iter.collect::<Result<Vec<CardData>, rusqlite::Error>>()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct CardData {
    id: i32,
    pub name: String,
    cost: Option<i32>,
    effects: String,
}

impl CardData {
    pub fn spawn_as_entity(&self, commands: &mut CommandBuffer) -> Result<Entity, String> {
        let card_effects =
            get_card_effects_from_text(self.effects.to_owned()).expect("Failed to parse Card Text");

        let entity = commands.push((
            (),
            Card {
                name: self.name.to_owned(),
            },
        ));

        if let Some(cost) = self.cost {
            commands.add_component(entity, Cost { amount: cost });
        }

        card_effects.iter().for_each(|effect| match effect {
            _ => {
                eprintln!(
                    "Unimplemented Card Effect!\n{:?}\nIn Card: {}",
                    effect, self.name
                )
            }
        });

        Ok(entity)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Target {
    Vertical(i32),
    Horizontal(i32),
}

#[derive(Debug, Clone, PartialEq)]
enum CardEffect {
    DealDamage(i32),
    Block(i32),
    DefineTarget(Target),
    Unimplemented(String),
}

impl CardEffect {
    fn from_string(val: String) -> Result<Self, String> {
        if let Some(open_paren_pos) = val.find("(") {
            if let Some(close_paren_pos) = val.find(")") {
                if close_paren_pos > open_paren_pos {
                    let keyword = val[0..open_paren_pos].to_string();
                    let params_str = val[open_paren_pos + 1..close_paren_pos].to_string();

                    let mut params = params_str.split(",").into_iter().map(|val| {
                        val.trim()
                            .parse::<i32>()
                            .expect(&format!("Error extracting params from string: {val:?}"))
                    });

                    match keyword.as_str() {
                        "deal" => return Ok(CardEffect::DealDamage(params.nth(0).unwrap())),
                        "block" => return Ok(CardEffect::Block(params.nth(0).unwrap())),
                        "vertical" => {
                            return Ok(CardEffect::DefineTarget(Target::Vertical(
                                params.nth(0).unwrap(),
                            )))
                        }

                        "horizontal" => {
                            return Ok(CardEffect::DefineTarget(Target::Horizontal(
                                params.nth(0).unwrap(),
                            )))
                        }

                        "target_bottom_rows" => {
                            return Ok(CardEffect::Unimplemented("TargetBottomRows".to_string()));
                        }

                        _ => return Err(format!("could not parse command: {}", keyword)),
                    }
                }
            }
        }

        Err(format!("Could not successfully parse string: {}", val))
    }
}

fn get_card_effects_from_text(val: String) -> Result<Vec<CardEffect>, String> {
    let effects = val
        .split(';')
        .map(|command| command.trim())
        .filter(|command| *command != "")
        .map(|command| CardEffect::from_string(command.to_string()))
        .collect::<Result<Vec<CardEffect>, String>>();

    effects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_by_id() {
        let mut db = CardDB::new();

        let actual = db.get_card_from_id(1);
        let expected = CardData {
            id: 1,
            name: "Firey Breath".to_string(),
            cost: Some(1),
            effects: "vertical(1);\ndeal(1);".to_string(),
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_deal_damage_string() {
        let amount = 4;
        let effect_string = format!("deal({})", amount);
        let actual = CardEffect::from_string(effect_string).unwrap();

        let expected = CardEffect::DealDamage(amount);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_multiple_commands_in_string() {
        let expected = Ok(vec![
            CardEffect::DealDamage(3),
            CardEffect::Block(4),
            CardEffect::DefineTarget(Target::Horizontal(1)),
        ]);
        let effect_string = "deal(3);block(4)\n;\nhorizontal(1);";

        let actual = get_card_effects_from_text(effect_string.to_string());

        assert_eq!(actual, expected)
    }
}
