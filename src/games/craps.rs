use crate::dice;

#[derive(Clone)]
pub struct Player{
    pub name: String,
    pub chips: u32,
    pub is_shooter: bool,
    pub next_player: Option<Box<Player>>,
}

pub enum CrapsType {
    StandardCraps,
    CraplessCraps,
    EasyCraps,
}

pub enum GameState {
    // Initial state before the point is set
    ComeOut,
    // State when the point is set (4, 5, 6, 8, 9, or 10)
    PointSet(u8),
    // State when the player rolls the point number again
    PointWin,
    // State when the player rolls a 7 after the point is set
    Lose,
    // State when the player rolls a 2, 3, or 12 on the come-out roll
    Craps,
    // State when the player rolls a 7 or 11 on the come-out roll
    NaturalWin,
}

pub struct CrapsGame {
    pub game_type: CrapsType,
    pub game_state: GameState,
    pub shooter: Option<Player>,
    pub players: Vec<Player>,
}

impl CrapsGame {
    pub fn new(game_type: CrapsType, mut players: Vec<Player>) -> Self {
        if !players.is_empty() {
            players[0].is_shooter = true;

            // Set up circular linked list
            for i in 0..players.len() {
                let next_index = (i + 1) % players.len();
                players[i].next_player = Some(Box::new(players[next_index].clone()));
            }
        }

        let current_shooter = players.first().cloned();

        CrapsGame {
            game_type,
            game_state: GameState::ComeOut,
            shooter: current_shooter,
            players,
        }
    }

    pub fn advance_shooter(&mut self) {
        if let Some(current_shooter) = &self.shooter {
            if let Some(next_player) = &current_shooter.next_player {
                self.shooter = Some((**next_player).clone());
            }
        }
    }

    pub fn reset_game(&mut self) {
        self.game_state = GameState::ComeOut;
        self.advance_shooter();
    }

    pub fn come_out_roll(&mut self ){
        let d1 = dice::Die::new(6).unwrap().roll();
        let d2 = dice::Die::new(6).unwrap().roll();
        let total = d1+ d2;
        match self.game_type {
            CrapsType::StandardCraps | CrapsType::EasyCraps => self.standard_craps_come_out(d1, d2, total),
            CrapsType::CraplessCraps => self.crapless_craps_come_out(d1, d2, total),
        }
    }

    pub fn standard_craps_come_out(&mut self, d1: u8, d2: u8, total: u8) {
        match total {
            7 | 11 => self.game_state = GameState::NaturalWin,
            2 | 3 | 12 => self.game_state = GameState::Craps,
            4 | 5 | 6 | 8 | 9 | 10 => self.game_state = GameState::PointSet(total),
            _ => {}
        }
    }

    pub fn crapless_craps_come_out(&mut self, d1: u8, d2: u8, total: u8) {
        match total {
            7 | 11 => self.game_state = GameState::NaturalWin,
            2 | 3 | 4 | 5 | 6 | 8 | 9 | 10 | 12 => self.game_state = GameState::PointSet(total),
            _ => {}
        }
    }


    pub fn point_roll(&mut self, roll: u8) {
        if let GameState::PointSet(point) = self.game_state {
            if roll == point {
                self.game_state = GameState::PointWin;
            } else if roll == 7 {
                self.game_state = GameState::Lose;
            } else if roll == 11 {

            }
        }
    }

}