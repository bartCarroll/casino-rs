#[cfg(feature = "python")]
pub mod python_bindings;

use rand::{Rng};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RouletteColor {
    Red,
    Black,
    Green,
}

impl RouletteColor {
    pub fn all() -> &'static [RouletteColor] {
        &[RouletteColor::Red, RouletteColor::Black, RouletteColor::Green]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RouletteColor::Red => "Red",
            RouletteColor::Black => "Black",
            RouletteColor::Green => "Green",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RouletteNumber {
    pub number: u16,
    pub color: RouletteColor,
}

impl RouletteNumber {
    pub fn new(number: u16) -> Option<Self> {
        match number {
            0 => Some(RouletteNumber {
                number,
                color: RouletteColor::Green,
            }),
            37 => Some(RouletteNumber {
                number,
                color: RouletteColor::Green,
            }),
            38 => Some(RouletteNumber {
                number,
                color: RouletteColor::Green,
            }),
            1..=36 => {
                let color = if [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36]
                    .contains(&number)
                {
                    RouletteColor::Red
                } else {
                    RouletteColor::Black
                };
                Some(RouletteNumber { number, color })
            }
            _ => None,
        }
    }
}

pub enum WheelType {
    American, // 0, 00
    European, // 0
    TripleZero, // 0, 00, 000
}

impl WheelType {
    pub fn all() -> &'static [WheelType] {
        &[WheelType::American, WheelType::European, WheelType::TripleZero]
    }

    pub fn numbers(&self) -> Vec<RouletteNumber> {
        match self {
            WheelType::American => (0..38)
                .filter_map(|n| RouletteNumber::new(n))
                .collect(),
            WheelType::European => (0..=36)
                .filter_map(|n| RouletteNumber::new(n))
                .collect(),
            WheelType::TripleZero => (0..40)
                .filter_map(|n| RouletteNumber::new(n))
                .collect(),
        }
    }
}

pub struct RouletteWheel {
    pub wheel: WheelType,
}

impl RouletteWheel {
    pub fn new(wheel: WheelType) -> Self {
        RouletteWheel { wheel }
    }

    pub fn spin(&self) -> RouletteNumber {
        let numbers = self.wheel.numbers();
        let mut rng = rand::rng();
        let index = rng.random_range(0..numbers.len());
        numbers[index]
    }
}

#[cfg(test)]
mod tests {

    pub use super::*;

    #[test]
    fn test_roulette_number_creation() {
        assert_eq!(
            RouletteNumber::new(0),
            Some(RouletteNumber {
                number: 0,
                color: RouletteColor::Green
            })
        );
        assert_eq!(
            RouletteNumber::new(37),
            Some(RouletteNumber {
                number: 37,
                color: RouletteColor::Green
            })
        );
        assert_eq!(
            RouletteNumber::new(38),
            Some(RouletteNumber {
                number: 38,
                color: RouletteColor::Green
            })
        );
        assert_eq!(
            RouletteNumber::new(1),
            Some(RouletteNumber {
                number: 1,
                color: RouletteColor::Red
            })
        );
        assert_eq!(
            RouletteNumber::new(2),
            Some(RouletteNumber {
                number: 2,
                color: RouletteColor::Black
            })
        );
        assert_eq!(RouletteNumber::new(39), None);
    }

    #[test]
    fn test_wheel_numbers() {
        let american_wheel = WheelType::American;
        assert_eq!(american_wheel.numbers().len(), 38); // 0-36 + 00 (37)

        let european_wheel = WheelType::European;
        assert_eq!(european_wheel.numbers().len(), 37); // 0-36

        let triple_zero_wheel = WheelType::TripleZero;
        assert_eq!(triple_zero_wheel.numbers().len(), 39); // 0-36 + 00 (37) + 000 (38)
    }

    #[test]
    fn test_wheel_spin_american() {
        let wheel = RouletteWheel::new(WheelType::American);
        for _ in 0..100 {
            let result = wheel.spin();
            assert!(result.number <= 38);
        }
    }

    #[test]
    fn test_wheel_spin_european() {
        let wheel = RouletteWheel::new(WheelType::European);
        for _ in 0..100 {
            let result = wheel.spin();
            assert!(result.number <= 36);
        }
    }

    #[test]
    fn test_wheel_spin_triple_zero() {
        let wheel = RouletteWheel::new(WheelType::TripleZero);
        for _ in 0..100 {
            let result = wheel.spin();
            assert!(result.number <= 38);
        }
    }
}
