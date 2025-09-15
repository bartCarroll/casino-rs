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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    PlayerBlackjack,
    DealerBlackjack,
    PlayerWin,
    DealerWin,
    Push,
    PlayerBust,
    DealerBust,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoundResult {
    pub player_index: usize,
    pub player_value: u8,
    pub dealer_value: u8,
    pub outcome: Outcome,
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
    num_decks: usize,
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
            num_decks,
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
        // Ensure each player has a base hand
        for player in &mut self.players {
            if player.hands.is_empty() {
                player.hands.push(Hand { cards: vec![], bet: HashMap::new() });
            } else {
                // Clear the first hand for a fresh round (no splits support here)
                if let Some(h) = player.hands.get_mut(0) {
                    h.cards.clear();
                }
            }
        }
        // Clear dealer state
        self.dealer.hand.clear();
        self.dealer.face_down_card = None;

        // First card to each player
        for player in &mut self.players {
            if let Some(card1) = self.shoe.deal() {
                if let Some(hand) = player.hands.get_mut(0) {
                    hand.cards.push(card1);
                }
            }
        }
        // Dealer gets one face down card
        self.dealer.face_down_card = self.shoe.deal();
        // Second card to each player
        for player in &mut self.players {
            if let Some(card2) = self.shoe.deal() {
                if let Some(hand) = player.hands.get_mut(0) {
                    hand.cards.push(card2);
                }
            }
        }
        // Dealer gets one face up card
        if let Some(card) = self.shoe.deal() {
            self.dealer.hand.push(card);
        }
    }

    // Ensure the shoe has enough cards; if not, reinitialize and shuffle
    fn ensure_shoe_capacity(&mut self, min_remaining: usize) {
        if self.cards_remaining() < min_remaining {
            self.shoe = Shoe::<BJRank>::new(self.num_decks);
            self.shoe.shuffle();
        }
    }

    // Score arbitrary cards with blackjack ace adjustments
    fn score_cards(cards: &[Card<BJRank>]) -> u8 {
        let mut total: u8 = 0;
        let mut aces = 0;
        for c in cards {
            total += c.value();
            if c.is_ace() {
                aces += 1;
            }
        }
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }
        total
    }

    pub fn play(&mut self) -> Vec<RoundResult> {
        // Fresh round setup
        self.ensure_shoe_capacity(52);
        self.deal_initial_cards();

        // Check if dealer has blackjack (with hole card)
        let mut dealer_cards: Vec<Card<BJRank>> = self.dealer.hand.clone();
        if let Some(fd) = self.dealer.face_down_card {
            dealer_cards.push(fd);
        }
        let dealer_has_blackjack = dealer_cards.len() == 2 && Self::score_cards(&dealer_cards) == 21;

        // Player turns (simple strategy: hit until 17 or more)
        for player in &mut self.players {
            if let Some(hand) = player.hands.get_mut(0) {
                // Skip action if natural blackjack
                if !hand.is_blackjack() {
                    while hand.value() < 17 {
                        if let Some(card) = self.shoe.deal() {
                            hand.cards.push(card);
                        } else {
                            break;
                        }
                        if hand.is_bust() {
                            break;
                        }
                    }
                }
            }
        }

        // Reveal dealer hole card
        if let Some(fd) = self.dealer.face_down_card.take() {
            self.dealer.hand.push(fd);
        }

        // If dealer had no blackjack, play dealer to 17+
        if !dealer_has_blackjack {
            loop {
                let v = Self::score_cards(&self.dealer.hand);
                if v >= 17 {
                    break;
                }
                if let Some(card) = self.shoe.deal() {
                    self.dealer.hand.push(card);
                } else {
                    break;
                }
            }
        }

        // Resolve outcomes per player (first hand only for now)
        let dealer_value = Self::score_cards(&self.dealer.hand);
        let mut results: Vec<RoundResult> = Vec::with_capacity(self.players.len());
        for (idx, player) in self.players.iter().enumerate() {
            let mut outcome = Outcome::Push;
            let player_value = if let Some(hand) = player.hands.get(0) {
                hand.value()
            } else {
                0
            };

            let player_blackjack = if let Some(hand) = player.hands.get(0) {
                hand.is_blackjack()
            } else {
                false
            };

            if player_blackjack && dealer_has_blackjack {
                outcome = Outcome::Push;
            } else if player_blackjack {
                outcome = Outcome::PlayerBlackjack;
            } else if dealer_has_blackjack {
                outcome = Outcome::DealerBlackjack;
            } else {
                let player_bust = if let Some(hand) = player.hands.get(0) {
                    hand.is_bust()
                } else {
                    false
                };
                let dealer_bust = dealer_value > 21;

                outcome = if player_bust {
                    Outcome::PlayerBust
                } else if dealer_bust {
                    Outcome::DealerBust
                } else if player_value > dealer_value {
                    Outcome::PlayerWin
                } else if player_value < dealer_value {
                    Outcome::DealerWin
                } else {
                    Outcome::Push
                };
            }

            results.push(RoundResult {
                player_index: idx,
                player_value,
                dealer_value,
                outcome,
            });
        }

        // Prepare players for the next round: keep hands but clear cards
        for p in &mut self.players {
            if let Some(h) = p.hands.get_mut(0) {
                h.cards.clear();
            }
        }
        // Dealer cleared next time in deal_initial_cards

        results
    }

    pub fn play_n(&mut self, rounds: usize) -> Vec<RoundResult> {
        let mut all = Vec::new();
        for _ in 0..rounds {
            let res = self.play();
            all.extend(res);
        }
        all
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_n_games() {
        let players = vec![
            Player::new("Alice"),
            Player::new("Bob"),
            Player::new("Charlie"),
            Player::new("Dave"),
        ];
        let mut game = BlackjackGame::new(players, 1, 10, 100);
        let results = game.play_n(10);
        println!("{:?}", results);
        assert_eq!(results.len(), 10 * 4); // 4 players, 10 rounds each
    }
}