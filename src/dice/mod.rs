/// A module for Dice related functions.

#[cfg(feature = "python")]
pub mod python_bindings;

use rand::Rng;

pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl Die {
    pub fn new(sides: u8) -> Option<Self> {
        match sides {
            4 => Some(Die::D4),
            6 => Some(Die::D6),
            8 => Some(Die::D8),
            10 => Some(Die::D10),
            12 => Some(Die::D12),
            20 => Some(Die::D20),
            100 => Some(Die::D100),
            _ => None,
        }
    }

    pub fn sides(&self) -> u8 {
        match self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
            Die::D100 => 100,
        }
    }

    pub fn roll(&self) -> u8 {
        let mut rng = rand::rng();
        rng.random_range(1..=self.sides())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_die_creation() {
        assert!(Die::new(6).is_some());
        assert!(Die::new(20).is_some());
        assert!(Die::new(7).is_none());
    }

    #[test]
    fn test_die_sides() {
        let die = Die::new(10).unwrap();
        assert_eq!(die.sides(), 10);

        let die = Die::new(4).unwrap();
        assert_eq!(die.sides(), 4);
    }

    #[test]
    fn test_die_roll() {
        let die = Die::new(6).unwrap();
        for _ in 0..100 {
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 6);
        }

        let die = Die::new(20).unwrap();
        for _ in 0..100 {
            let roll = die.roll();
            assert!(roll >= 1 && roll <= 20);
        }
    }
}

