/// Module for card-related structures and functions

#[cfg(feature = "python")]
pub mod python_bindings;

use rand::rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub fn all() -> &'static [Suit] {
        &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades]
    }

    pub fn symbol(&self) -> char {
        match self {
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
            Suit::Spades => '♠',
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Suit::Hearts | Suit::Diamonds => "Red",
            Suit::Clubs | Suit::Spades => "Black",
        }
    }

}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
impl Rank {
    pub fn all() -> &'static [Rank] {
        &[
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card{
    pub fn value(&self) -> u8 {
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11, // Ace can also be 1, but we'll handle that in game logic
        }
    }
}
impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    pub fn display(&self) -> String {
        let rank_str = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        let suit_str = match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        format!("{}{}", rank_str, suit_str)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CardCollection {
    cards: Vec<Card>,
}

impl CardCollection {
    pub fn new() -> Self {
        CardCollection { cards: Vec::new() }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

pub struct Deck {
    pub cards: CardCollection,
}
impl Deck {
    pub fn new() -> Self {
        let mut cards = CardCollection::new();
        for &suit in Suit::all() {
            for &rank in Rank::all() {
                cards.push(Card::new(suit, rank));
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle();
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.deal()
    }
}

pub struct Shoe {
    pub shoe: CardCollection,
}
impl Shoe {
    pub fn new(num_decks: usize) -> Self {
        let mut shoe = CardCollection::new();
        for _ in 0..num_decks {
            for &suit in Suit::all() {
                for &rank in Rank::all() {
                    shoe.push(Card::new(suit, rank));
                }
            }
        }
        Shoe { shoe }
    }

    pub fn shuffle(&mut self) {
        self.shoe.shuffle();
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.shoe.deal()
    }

    pub fn len(&self) -> usize {
        self.shoe.len()
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_value() {
        let card = Card::new(Suit::Hearts, Rank::Ace);
        assert_eq!(card.value(), 11);

        let card = Card::new(Suit::Spades, Rank::Ten);
        assert_eq!(card.value(), 10);

        let card = Card::new(Suit::Diamonds, Rank::Three);
        assert_eq!(card.value(), 3);
    }

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_shuffle() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();
        deck1.shuffle();
        deck2.shuffle();
        // There's a small chance they could be the same after shuffling, but it's very unlikely
        assert_ne!(deck1.cards, deck2.cards);
    }

    #[test]
    fn test_deal_card() {
        let mut deck = Deck::new();
        let initial_len = deck.cards.len();
        let card = deck.deal();
        assert!(card.is_some());
        assert_eq!(deck.cards.len(), initial_len - 1);
    }

    #[test]
    fn test_shoe_creation() {
        let shoe = Shoe::new(6);
        assert_eq!(shoe.shoe.len(), 52 * 6);
    }

    #[test]
    fn test_shoe_shuffle() {
        let mut shoe1 = Shoe::new(6);
        let mut shoe2 = Shoe::new(6);
        shoe1.shuffle();
        shoe2.shuffle();
        assert_ne!(shoe1.shoe, shoe2.shoe);
    }

    #[test]
    fn test_deal_from_shoe() {
        let mut shoe = Shoe::new(6);
        let initial_len = shoe.shoe.len();
        let card = shoe.deal();
        assert!(card.is_some());
        assert_eq!(shoe.shoe.len(), initial_len - 1);
    }
}