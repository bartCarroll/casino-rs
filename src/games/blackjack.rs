use std::collections::HashMap;
use crate::cards::{Card, Shoe, Rank as RankTrait, BlackjackRank as BlackjackRankTrait};
use crate::player::Player;
use crate::bet::Chip;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BJRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl RankTrait for BJRank {
    fn all() -> &'static [Self] {
        &[
            BJRank::Two,
            BJRank::Three,
            BJRank::Four,
            BJRank::Five,
            BJRank::Six,
            BJRank::Seven,
            BJRank::Eight,
            BJRank::Nine,
            BJRank::Ten,
            BJRank::Jack,
            BJRank::Queen,
            BJRank::King,
            BJRank::Ace,
        ]
    }

    fn display(&self) -> &'static str {
        match self {
            BJRank::Two => "2",
            BJRank::Three => "3",
            BJRank::Four => "4",
            BJRank::Five => "5",
            BJRank::Six => "6",
            BJRank::Seven => "7",
            BJRank::Eight => "8",
            BJRank::Nine => "9",
            BJRank::Ten => "10",
            BJRank::Jack => "J",
            BJRank::Queen => "Q",
            BJRank::King => "K",
            BJRank::Ace => "A",
        }
    }

    fn is_face(&self) -> bool {
        matches!(self, BJRank::Jack | BJRank::Queen | BJRank::King)
    }
}

impl BlackjackRankTrait for BJRank {
    fn blackjack_value(&self) -> u8 {
        match self {
            BJRank::Two => 2,
            BJRank::Three => 3,
            BJRank::Four => 4,
            BJRank::Five => 5,
            BJRank::Six => 6,
            BJRank::Seven => 7,
            BJRank::Eight => 8,
            BJRank::Nine => 9,
            BJRank::Ten | BJRank::Jack | BJRank::Queen | BJRank::King => 10,
            BJRank::Ace => 11,
        }
    }

    fn is_ace(&self) -> bool {
        matches!(self, BJRank::Ace)
    }
}

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
    cards: Vec<Card<BJRank>>,
    bet: HashMap<Chip, u32>
}

impl Hand {
    pub fn value(&self) -> u8 {
        let mut total: u8 = 0;
        let mut aces = 0;
        for card in &self.cards {
            total += card.value();
            if card.is_ace() {
                aces += 1;
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
}

impl PlayerSeat {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            hands: vec![],
        }
    }

    pub fn get_player_name(&self) -> &str{
        &self.player.name
    }
}
pub struct Dealer {
    face_down_card: Option<Card<BJRank>>,
    hand: Vec<Card<BJRank>>,
}

pub struct BlackjackGame {
    dealer: Dealer,
    players: Vec<PlayerSeat>,
    shoe: Shoe<BJRank>,
    min_bet: u32,
    max_bet: u32,
}

impl BlackjackGame {
    pub fn new(players: Vec<Player>, num_decks: usize, min_bet: u32, max_bet: u32) -> Self {
        let shoe = Shoe::<BJRank>::new(num_decks);
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
        let dealer_upcard = self.dealer.hand.first();
        let dealer_upcard_is_ace = matches!(dealer_upcard, Some(card) if card.is_ace());
        if dealer_upcard_is_ace {
            // TODO: implement insurance logic

        }
    }

}