use std::collections::HashMap;
use crate::cards::Shoe;
use crate::cards::Card;

pub struct Player {
    name: String,
    chips: HashMap<u32, u32>, // Map of chip denomination to quantity
    hand: HashMap<Vec<Card>, HashMap<u32, u32>> // Map of hand to bet (denomination to quantity)
}

impl Player {
    pub fn new(name: String, starting_chips: HashMap<u32, u32>) -> Self {
        Self { name, chips: starting_chips, hand: HashMap::new() }
    }

    pub fn total_value(&self) -> u32 {
        self.chips.iter().map(|(denom, count)| denom * count).sum()
    }

    pub fn has_chips(&self, amount: u32) -> bool {
        self.total_value() >= amount
    }
}


pub struct Dealer {
    dealer_hand: Vec<Card>,
}

pub struct BlackjackGame {
    dealer: Dealer,
    players: Vec<Player>,
    chip_denominations: Vec<u32>, // e.g., [1, 5, 10, 25, 100, 500, 1000]
    shoe: Shoe,
}

impl BlackjackGame {
    pub fn new(players: Vec<Player>, chip_denominations: Vec<u32>, num_decks: usize) -> Self {
        let shoe = Shoe::new(num_decks);
        Self {
            dealer: Dealer { dealer_hand: vec![] },
            players,
            chip_denominations,
            shoe,
        }
    }
}