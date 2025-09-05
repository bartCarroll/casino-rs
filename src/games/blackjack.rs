
pub struct Player {
    name: String,
    balance: f64,
}

pub struct Dealer {
    // TODO: Add dealer-specific fields if needed
}

pub struct BlackjackGame {
    dealer: Dealer,
    players: Vec<Player>,

}