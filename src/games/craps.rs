use crate::dice;
use crate::player::Player;
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
    pub shooter: u8,
    pub players: Vec<Player>,
}

impl CrapsGame {
    pub fn new(game_type: CrapsType, players: Vec<Player>) -> Self {
        CrapsGame {
            game_type,
            game_state: GameState::ComeOut,
            shooter: 0,
            players,
        }
    }

    pub fn advance_shooter(&mut self) {
        self.shooter = (self.shooter + 1) % self.players.len() as u8;
    }

    pub fn reset_game(&mut self) {
        self.game_state = GameState::ComeOut;
        self.advance_shooter();
    }

    pub fn is_hardway(&self, d1: u8, d2: u8, total: u8) -> bool {
        (d1 == d2) && (d1 == total || total == 6 || total == 8 || total == 10)
    }

    pub fn come_out_roll(&mut self ){
        let d1 = dice::Die::new(6).unwrap().roll();
        let d2 = dice::Die::new(6).unwrap().roll();

        // hard way check
        let total = d1+ d2;
        if self.is_hardway(d1, d2, total) {
            // handle hard way logic here
        }
        match self.game_type {
            CrapsType::StandardCraps | CrapsType::EasyCraps => self.standard_craps_come_out(total),
            CrapsType::CraplessCraps => self.crapless_craps_come_out(total),
        }
    }

    pub fn standard_craps_come_out(&mut self, total: u8) {
        match total {
            7 | 11 => self.game_state = GameState::NaturalWin,
            2 | 3 | 12 => self.game_state = GameState::Craps,
            4 | 5 | 6 | 8 | 9 | 10 => self.game_state = GameState::PointSet(total),
            _ => {}
        }
    }

    pub fn crapless_craps_come_out(&mut self, total: u8) {
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