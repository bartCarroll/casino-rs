pub struct Symbol{
    name: String,
    payout_multiplier: u32,
}

pub struct Reel{
    symbols: Vec<Symbol>,
}
pub struct Slot{
    reels: Vec<Reel>,
    paylines: Vec<Vec<usize>>, // Each payline is a vector of reel indices
}

