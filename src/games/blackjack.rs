use std::collections::HashMap;
use crate::cards::Shoe;
use crate::cards::Card;
use crate::player::Player;
use crate::bet::Chip;

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


pub struct Hand {
    cards: Vec<Card>,
    bet: HashMap<Chip, u32>
}

impl Hand {
    pub fn value(&self) -> u8 {
        let mut total = 0;
        let mut aces = 0;
        for card in &self.cards {
            match card.rank {
                crate::cards::Rank::Two => total += 2,
                crate::cards::Rank::Three => total += 3,
                crate::cards::Rank::Four => total += 4,
                crate::cards::Rank::Five => total += 5,
                crate::cards::Rank::Six => total += 6,
                crate::cards::Rank::Seven => total += 7,
                crate::cards::Rank::Eight => total += 8,
                crate::cards::Rank::Nine => total += 9,
                crate::cards::Rank::Ten | crate::cards::Rank::Jack | crate::cards::Rank::Queen | crate::cards::Rank::King => total += 10,
                crate::cards::Rank::Ace => {
                    aces += 1;
                    total += 11; // initially count ace as 11
                }
            }
        }
        // adjust for aces if total > 21
        while total > 21 && aces > 0 {
            total -= 10; // count one ace as 1 instead of 11
            aces -= 1;
        }
        total
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value() == 21
    }

    pub fn is_bust(&self) -> bool {
        self.value() > 21
    }
}

pub struct PlayerSeat {
    player: Player,
    hands: Vec<Hand>, // multiple when splitting
    is_active: bool,  // whether the player is still in the round
}

impl PlayerSeat {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            hands: vec![],
            is_active: true,
        }
    }
}
pub struct Dealer {
    face_down_card: Option<Card>,
    hand: Vec<Card>,
}

pub struct BlackjackGame {
    dealer: Dealer,
    players: Vec<PlayerSeat>,
    shoe: Shoe,
    min_bet: u32,
    max_bet: u32,
}

impl BlackjackGame {
    pub fn new(players: Vec<Player>, num_decks: usize, min_bet: u32, max_bet: u32) -> Self {
        let shoe = Shoe::new(num_decks);
        let players = players.into_iter().map(PlayerSeat::new).collect();
        Self {
            dealer: Dealer { face_down_card: None, hand: vec![] },
            players,
            shoe,
            min_bet,
            max_bet,
        }
    }

    pub fn shuffle_shoe(&mut self) {
        self.shoe.shuffle();
    }

    pub fn cards_remaining(&self) -> usize {
        self.shoe.shoe.len()
    }

    pub fn place_initial_bet(&mut self, player_index: usize, bet: HashMap<Chip, u32>) -> Result<(), Error> {
        if player_index >= self.players.len() {
            return Err(Error::InsufficientChips);
        }
        let total_bet: u32 = bet.values().sum();
        if total_bet < self.min_bet || total_bet > self.max_bet {
            return Err(Error::InsufficientChips);
        }
        // Here you would also check if the player has enough chips in their wallet
        self.players[player_index].hands.push(Hand { cards: vec![], bet });
        Ok(())
    }
    pub fn deal_initial_cards(&mut self) {
        for player in &mut self.players {
            if let Some(card1) = self.shoe.deal() {
                if let Some(hand) = player.hands.get_mut(0) {
                    hand.cards.push(card1);
                }
            }
        }
        // dealer gets one face down card
        self.dealer.face_down_card = Some(self.shoe.deal().unwrap());
        for player in &mut self.players {
            if let Some(card2) = self.shoe.deal() {
                if let Some(hand) = player.hands.get_mut(0) {
                    hand.cards.push(card2);
                }
            }
        }
        // dealer gets one face up card
        if let Some(card) = self.shoe.deal() {
            self.dealer.hand.push(card);
        }
    }

    pub fn play(&mut self) {
        self.deal_initial_cards();
        // is dealer showing an Ace?
        let dealer_upcard = self.dealer.hand.get(0);
        let dealer_upcard_is_ace = matches!(dealer_upcard, Some(card) if card.rank == crate::cards::Rank::Ace);
        if dealer_upcard_is_ace {
            // TODO: implement insurance logic

        }
    }

}