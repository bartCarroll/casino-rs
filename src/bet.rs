use std::collections::HashMap;

pub struct Bet {
    pub chips: HashMap<Chip, u32> // token -> count
}

impl Default for Bet {
    fn default() -> Self {
        Self::new()
    }
}

impl Bet {

    pub fn new() -> Self {
        Self {
            chips: HashMap::new()
        }
    }

    pub fn place_chip(&mut self, chip: Chip, count: u32) {
        *self.chips.entry(chip).or_insert(0) += count;
    }


    pub fn place_multiple(&mut self, chips: HashMap<Chip, u32>) {
        chips.iter().for_each(|(chip, count)| {
            self.place_chip(chip.clone(), *count);
        });
    }

    pub fn remove_chip(&mut self, chip: &Chip, count: u32) -> Result<(), &'static str> {
        match self.chips.get_mut(chip) {
            Some(n) if *n >= count => {
                *n -= count;
                if *n == 0 {
                    self.chips.remove(chip);
                }
                Ok(())
            }
            _ => Err("insufficient chips in bet"),
        }
    }

    pub fn total_cents(&self) -> u64 {
        self.chips.iter().map(|(tok, &count)| tok.value_cents.saturating_mul(count as u64)).sum()
    }


}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chip {
    pub name: String,
    pub display: String,
    /// value in cents to avoid floating point rounding issues
    pub value_cents: u64,
}
