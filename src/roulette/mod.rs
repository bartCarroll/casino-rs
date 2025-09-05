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
    pub fn new(number: u16, double_zero: bool, triple_zero: bool) -> Option<Self> {
        match number {
            0 => Some(RouletteNumber {
                number,
                color: RouletteColor::Green,
            }),
            37 if double_zero => Some(RouletteNumber {
                number,
                color: RouletteColor::Green,
            }),
            38 if triple_zero => Some(RouletteNumber {
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
    French,  // 0 with special rules
    TripleZero, // 0, 00, 000 (not commonly used)
}

impl WheelType {
    pub fn all() -> &'static [WheelType] {
        &[WheelType::American, WheelType::European, WheelType::French, WheelType::TripleZero]
    }

    pub fn numbers(&self) -> Vec<RouletteNumber> {
        match self {
            WheelType::American => (0..=38)
                .filter_map(|n| RouletteNumber::new(n, true, false))
                .collect(),
            WheelType::European => (0..=36)
                .filter_map(|n| RouletteNumber::new(n, false, false))
                .collect(),
            WheelType::French => (0..=36)
                .filter_map(|n| RouletteNumber::new(n, false, false))
                .collect(),
            WheelType::TripleZero => (0..=39)
                .filter_map(|n| RouletteNumber::new(n, false, true))
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
