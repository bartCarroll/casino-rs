use std::collections::HashMap;
use crate::bet::Bet;
use crate::cards::{Card, Shoe};
use crate::player::Player;

pub enum BaccaratBet {
    Player,
    Banker,
    Tie,
    PlayerPair,
    BankerPair,
}

pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn value(&self) -> u8 {
        let total: u8 = self.cards.iter().map(|card| {
            match card.rank {
                // 2-9 are worth their face value
                crate::cards::Rank::Two => 2,
                crate::cards::Rank::Three => 3,
                crate::cards::Rank::Four => 4,
                crate::cards::Rank::Five => 5,
                crate::cards::Rank::Six => 6,
                crate::cards::Rank::Seven => 7,
                crate::cards::Rank::Eight => 8,
                crate::cards::Rank::Nine => 9,
                // 10, J, Q, K are worth 0
                crate::cards::Rank::Ten | crate::cards::Rank::Jack | crate::cards::Rank::Queen | crate::cards::Rank::King => 0,
                // A is worth 1
                crate::cards::Rank::Ace => 1,
            }
        }).sum();
        total % 10 // baccarat hand values are modulo 10
    }
}

pub struct PlayerSeat {
    pub player: Player,
    pub player_bet: HashMap<BaccaratBet, Bet>
}

impl PlayerSeat {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            player_bet: HashMap::new()
        }
    }

    pub fn get_player_name(&self) -> &str{
        &self.player.name
    }
}
pub struct BaccaratGame {
    players: Vec<PlayerSeat>,
    shoe: Shoe,
    player_hand: Hand,
    banker_hand: Hand,
}

impl BaccaratGame {
    pub fn new(players: Vec<Player>) -> Self {
        BaccaratGame {
            players: players.into_iter().map(PlayerSeat::new).collect(),
            shoe: Shoe::new(6),
            player_hand: Hand::new(),
            banker_hand: Hand::new(),
        }
    }

    pub fn play(&mut self) {
        // deal cards
        self.shoe.shuffle();
        // deal initial cards
        for _ in 0..2 {
            self.player_hand.cards.push(self.shoe.deal().unwrap());
            self.banker_hand.cards.push(self.shoe.deal().unwrap());
        }

        // Evaluate hands
        let player_value = self.player_hand.value();
        let banker_value = self.banker_hand.value();

    }
}