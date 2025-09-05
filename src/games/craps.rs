pub enum CrapsType {
    StandardCraps,
    CraplessCraps,
    EasyCraps,
}

pub enum GameState {
    ComeOut,
    Point(u8),
    Win,
    Lose,
}

pub struct CrapsGame {
    pub game_type: CrapsType,
    pub game_state: GameState,
}

impl CrapsGame {
    pub fn new(game_type: CrapsType) -> Self {
        CrapsGame { game_type, game_state: GameState::ComeOut }
    }

}