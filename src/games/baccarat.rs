use std::collections::HashMap;
use crate::bet::Bet;
use crate::cards::{Card, Shoe, Rank as RankTrait, BaccaratRank as BaccaratRankTrait};
use crate::player::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BacRank {
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

impl RankTrait for BacRank {
    fn all() -> &'static [Self] {
        &[
            BacRank::Two,
            BacRank::Three,
            BacRank::Four,
            BacRank::Five,
            BacRank::Six,
            BacRank::Seven,
            BacRank::Eight,
            BacRank::Nine,
            BacRank::Ten,
            BacRank::Jack,
            BacRank::Queen,
            BacRank::King,
            BacRank::Ace,
        ]
    }

    fn display(&self) -> &'static str {
        match self {
            BacRank::Two => "2",
            BacRank::Three => "3",
            BacRank::Four => "4",
            BacRank::Five => "5",
            BacRank::Six => "6",
            BacRank::Seven => "7",
            BacRank::Eight => "8",
            BacRank::Nine => "9",
            BacRank::Ten => "10",
            BacRank::Jack => "J",
            BacRank::Queen => "Q",
            BacRank::King => "K",
            BacRank::Ace => "A",
        }
    }

    fn is_face(&self) -> bool {
        matches!(self, BacRank::Jack | BacRank::Queen | BacRank::King)
    }
}

impl BaccaratRankTrait for BacRank {
    fn baccarat_value(&self) -> u8 {
        match self {
            BacRank::Two => 2,
            BacRank::Three => 3,
            BacRank::Four => 4,
            BacRank::Five => 5,
            BacRank::Six => 6,
            BacRank::Seven => 7,
            BacRank::Eight => 8,
            BacRank::Nine => 9,
            BacRank::Ten | BacRank::Jack | BacRank::Queen | BacRank::King => 0,
            BacRank::Ace => 1,
        }
    }
}

pub enum BaccaratBet {
    Player,
    Banker,
    Tie,
    PlayerPair,
    BankerPair,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winner {
    Player,
    Banker,
    Tie,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CoupResult {
    pub winner: Winner,
    pub player_pair: bool,
    pub banker_pair: bool,
}

impl CoupResult {
    pub fn label(&self) -> char {
        match self.winner {
            Winner::Player => 'P',
            Winner::Banker => 'B',
            Winner::Tie => 'T',
        }
    }
}

pub struct Hand {
    cards: Vec<Card<BacRank>>
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn value(&self) -> u8 {
        let total: u8 = self.cards.iter().map(|card| card.rank.baccarat_value()).sum();
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
    shoe: Shoe<BacRank>,
    player_hand: Hand,
    banker_hand: Hand,
    history: Vec<CoupResult>,
}

impl BaccaratGame {
    pub fn new(players: Vec<Player>) -> Self {
        BaccaratGame {
            players: players.into_iter().map(PlayerSeat::new).collect(),
            shoe: Shoe::<BacRank>::new(6),
            player_hand: Hand::new(),
            banker_hand: Hand::new(),
            history: Vec::new(),
        }
    }

    // Play a single round, record the result, and return it
    pub fn play(&mut self) -> CoupResult {
        // Clear previous hands and shuffle for this round
        self.player_hand.cards.clear();
        self.banker_hand.cards.clear();
        self.shoe.shuffle();

        // deal initial cards
        for _ in 0..2 {
            self.player_hand.cards.push(self.shoe.deal().unwrap());
            self.banker_hand.cards.push(self.shoe.deal().unwrap());
        }

        // Track initial pairs (only first two cards count)
        let player_pair = {
            let a = self.player_hand.cards.first().map(|c| c.rank);
            let b = self.player_hand.cards.get(1).map(|c| c.rank);
            a.is_some() && a == b
        };
        let banker_pair = {
            let a = self.banker_hand.cards.first().map(|c| c.rank);
            let b = self.banker_hand.cards.get(1).map(|c| c.rank);
            a.is_some() && a == b
        };

        // Evaluate hands
        let player_value = self.player_hand.value();
        let banker_value = self.banker_hand.value();

        println!("Player hand value: {}", player_value);
        println!("Banker hand value: {}", banker_value);

        // Natural check: if either is 8 or 9, both stand
        if !(player_value == 8 || player_value == 9 || banker_value == 8 || banker_value == 9) {
            // Player third-card rule
            let mut player_third_val: Option<u8> = None;
            if player_value <= 5 {
                let c = self.shoe.deal().unwrap();
                let c_val = c.rank.baccarat_value();
                player_third_val = Some(c_val);
                self.player_hand.cards.push(c);
                println!("Player draws a third card");
            } else {
                println!("Player stands");
            }

            // Banker third-card rules
            let b_val = self.banker_hand.value();
            let banker_draw = if player_third_val.is_none() {
                // If player stood, banker draws on 0-5, stands on 6-7
                b_val <= 5
            } else {
                // Use the banker drawing table based on player's third card
                let v = player_third_val.unwrap();
                match b_val {
                    0..=2 => true,
                    3 => v != 8,
                    4 => (2..=7).contains(&v),
                    5 => (4..=7).contains(&v),
                    6 => v == 6 || v == 7,
                    _ => false, // 7 stands; 8-9 already handled by natural check
                }
            };

            if banker_draw {
                let c = self.shoe.deal().unwrap();
                self.banker_hand.cards.push(c);
                println!("Banker draws a third card");
            } else {
                println!("Banker stands");
            }
        } else {
            println!("Natural - no more cards drawn");
        }

        // Final evaluation and outcome
        let final_player = self.player_hand.value();
        let final_banker = self.banker_hand.value();
        println!("Final Player value: {}", final_player);
        println!("Final Banker value: {}", final_banker);

        let winner = if final_player > final_banker {
            Winner::Player
        } else if final_player < final_banker {
            Winner::Banker
        } else {
            Winner::Tie
        };

        let result = CoupResult {
            winner,
            player_pair,
            banker_pair,
        };

        println!("Outcome: {} (pairs: P={} B={})", result.label(), result.player_pair, result.banker_pair);

        // Save result to history and return it
        self.history.push(result);
        result
    }

    // Play n rounds, collecting their results in history
    pub fn play_n(&mut self, rounds: usize) {
        for _ in 0..rounds {
            let _ = self.play();
        }
    }

    // Bead-plate style grid (top-to-bottom, left-to-right), limited to the last n results.
    pub fn bead_plate_grid(&self, rows: usize, n: usize) -> Vec<Vec<Option<CoupResult>>> {
        let rows = rows.max(1);
        let hist_len = self.history.len();
        let take = n.min(hist_len);
        let slice = &self.history[hist_len - take..];

        let cols = take.div_ceil(rows);
        let mut grid: Vec<Vec<Option<CoupResult>>> = vec![vec![None; cols]; rows];
        for (i, &r) in slice.iter().enumerate() {
            let row = i % rows;
            let col = i / rows;
            grid[row][col] = Some(r);
        }
        grid
    }

    // Render a simple bead-plate string with P/B/T labels ('.' for empty)
    pub fn bead_plate_string(&self, rows: usize, n: usize) -> String {
        let grid = self.bead_plate_grid(rows, n);
        let mut out = String::new();
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let ch = grid[row][col].map(|r| r.label()).unwrap_or('.');
                out.push(ch);
                if col + 1 < grid[row].len() {
                    out.push(' ');
                }
            }
            if row + 1 < grid.len() {
                out.push('\n');
            }
        }
        out
    }

    // Access the raw history (e.g., for analytics)
    pub fn history(&self) -> &Vec<CoupResult> {
        &self.history
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::Suit;
    use super::*;
    #[test]
    fn test_hand_value() {
        let hand = Hand::new();
        assert_eq!(hand.value(), 0);
    }
    #[test]
    fn test_hand_value_ace() {
        let mut hand = Hand::new();
        hand.cards.push(Card::new(Suit::Hearts, BacRank::Ace));
        assert_eq!(hand.value(), 1);
    }
    #[test]
    fn test_hand_value_ace_pair() {
        let mut hand = Hand::new();
        hand.cards.push(Card::new(Suit::Hearts, BacRank::Ace));
        hand.cards.push(Card::new(Suit::Diamonds, BacRank::Ace));
        assert_eq!(hand.value(), 2);
    }

    #[test]
    fn test_baccarat_game_play() {
        let player1 = Player::new("Player 1");
        let player2 = Player::new("Player 2");
        let mut game = BaccaratGame::new(vec![player1, player2]);
        let result = game.play();
        assert!(matches!(result.winner, Winner::Player | Winner::Banker | Winner::Tie));
    }

    #[test]
    fn test_baccarat_game_play_n() {
        let player1 = Player::new("Player 1");
        let player2 = Player::new("Player 2");
        let mut game = BaccaratGame::new(vec![player1, player2]);
        game.play_n(50);
        assert_eq!(game.history.len(), 50);
        assert!(matches!(game.history[0].winner, Winner::Player | Winner::Banker | Winner::Tie));
        // print the bead plate
        println!("{}", game.bead_plate_string(5, 50));
    }
}