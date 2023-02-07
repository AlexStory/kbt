use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub lanes: Vec<Lane>,
}

impl Board {
    pub fn new() -> Self {
        Self { lanes: vec![] }
    }

    pub fn add_lane(&mut self, lane: Lane) {
        self.lanes.push(lane);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lane {
    pub name: String,
    pub cards: Vec<Card>,
}

impl Lane {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cards: vec![],
        }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub title: String,
    pub description: Option<String>,
}

impl Card {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            description: None,
        }
    }
}
