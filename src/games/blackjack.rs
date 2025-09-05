use std::collections::HashMap;
use crate::cards::Shoe;
use crate::cards::Card;

pub enum GameState {
    WaitingForBets,
    Dealing,
    PlayerTurn,
    DealerTurn,
    RoundOver,
}

// define error type
#[derive(Debug)]
pub enum Error {
    InsufficientChips,
}


pub struct DealerHand{

}

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

    pub fn place_initial_bet(&mut self, bet: HashMap<u32, u32>) -> Result<(), Error> {
        let total_bet: u32 = bet.iter().map(|(denom, count)| denom * count).sum();
        if self.has_chips(total_bet) {
            for (denom, count) in bet.clone() {
                *self.chips.entry(denom).or_insert(0) -= count;
            }
            // Assuming initial bet is placed on an empty hand
            self.hand.insert(vec![], bet.clone());
            Ok(())
        } else {
            Err(Error::InsufficientChips)
        }
    }

}


pub struct Dealer {
    face_down_card: Option<Card>,
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
            dealer: Dealer { face_down_card: None, dealer_hand: vec![] },
            players,
            chip_denominations,
            shoe,
        }
    }

    pub fn shuffle_shoe(&mut self) {
        self.shoe.shuffle();
    }

    pub fn cards_remaining(&self) -> usize {
        self.shoe.shoe.len()
    }

    pub fn deal_initial_cards(&mut self) {
        for player in &mut self.players {
            // each player gets one card
        }
        // dealer gets one face down card
        self.dealer.face_down_card = Some(self.shoe.deal().unwrap());
        // players each get one more card
        // dealer gets one more card face up
        if let Some(card1) = self.shoe.deal() {
            self.dealer.dealer_hand.push(card1);
        }
        if let Some(card2) = self.shoe.deal() {
            self.dealer.face_down_card = Some(card2);
        }
    }

}