use std::collections::HashMap;
use crate::bet::Chip;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    /// map token -> count
    wallet: HashMap<Chip, u32>,
}

impl Player {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            wallet: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, token: Chip, count: u32) {
        *self.wallet.entry(token).or_insert(0) += count;
    }

    pub fn deposit_multiple(&mut self, tokens: HashMap<Chip, u32>) {
        tokens.iter().for_each(|(token, count)| {
            self.deposit(token.clone(), *count);
        });
    }

    pub fn withdraw(&mut self, token: &Chip, count: u32) -> Result<(), &'static str> {
        match self.wallet.get_mut(token) {
            Some(n) if *n >= count => {
                *n -= count;
                if *n == 0 {
                    self.wallet.remove(token);
                }
                Ok(())
            }
            _ => Err("insufficient tokens"),
        }
    }

    pub fn token_count(&self, token: &Chip) -> u32 {
        *self.wallet.get(token).unwrap_or(&0)
    }

    /// total balance in cents
    pub fn total_cents(&self) -> u64 {
        self.wallet
            .iter()
            .map(|(tok, &count)| tok.value_cents.saturating_mul(count as u64))
            .sum()
    }

    /// convenience: total balance as floating dollars (for display)
    pub fn total_balance(&self) -> f64 {
        self.total_cents() as f64 / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn sample_tokens() -> (Chip, Chip, Chip, Chip) {
        let quarter = Chip { name: "quarter".to_string(), display: "25¢".to_string(), value_cents: 25 };
        let ones_chip = Chip { name: "one dollar".to_string(), display: "$1".to_string(), value_cents: 100 };
        let fives_chip = Chip { name: "five dollar".to_string(), display: "$5".to_string(), value_cents: 500 };
        let tens_chip = Chip { name: "ten dollar".to_string(), display: "$10".to_string(), value_cents: 1000 };
        (quarter, ones_chip, fives_chip, tens_chip)
    }

    #[test]
    fn test_player_wallet() {
        let mut player = Player::new("Alice");
        let (quarter, _ones_chip, _fives_chip, _tens_chip) = sample_tokens();
        assert_eq!(player.total_cents(), 0);
        assert_eq!(player.total_balance(), 0.0);
        player.deposit(quarter.clone(), 4);
        assert_eq!(player.token_count(&quarter), 4);
        assert_eq!(player.total_cents(), 100);
    }

    #[test]
    fn test_deposit_multiple() {
        let mut player = Player::new("Bob");
        let (quarter, ones_chip, fives_chip, _tens_chip) = sample_tokens();
        let mut map = HashMap::new();
        map.insert(quarter.clone(), 4);
        map.insert(ones_chip.clone(), 2);
        map.insert(fives_chip.clone(), 1);
        player.deposit_multiple(map);
        assert_eq!(player.total_cents(), 4 * 25 + 2 * 100 + 1 * 500);
    }

    #[test]
    fn test_withdraw() {
        let mut player = Player::new("Charlie");
        let (quarter, ones_chip, _fives_chip, _tens_chip) = sample_tokens();
        player.deposit(quarter.clone(), 4);
        player.deposit(ones_chip.clone(), 2);
        assert_eq!(player.total_cents(), 4 * 25 + 2 * 100);
        assert!(player.withdraw(&quarter, 2).is_ok());
        assert_eq!(player.token_count(&quarter), 2);
        assert_eq!(player.total_cents(), 2 * 25 + 2 * 100);
        assert!(player.withdraw(&ones_chip, 3).is_err());
        assert_eq!(player.token_count(&ones_chip), 2);
    }
}