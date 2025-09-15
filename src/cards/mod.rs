//! Module for card-related structures and functions

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

// Core trait for card ranks; games can define their own enums implementing this.
pub trait Rank: Copy + Clone + core::fmt::Debug + PartialEq + Eq + core::hash::Hash + 'static {
    fn all() -> &'static [Self]
    where
        Self: Sized;
    fn display(&self) -> &'static str;
    fn is_face(&self) -> bool;
}

// Optional: extra behavior needed for Blackjack
pub trait BlackjackRank: Rank {
    fn blackjack_value(&self) -> u8;
    fn is_ace(&self) -> bool;
}

// Optional: extra behavior needed for Baccarat
pub trait BaccaratRank: Rank {
    fn baccarat_value(&self) -> u8;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card<R: Rank> {
    pub suit: Suit,
    pub rank: R,
}

impl<R: Rank> Card<R> {
    pub fn new(suit: Suit, rank: R) -> Self {
        Card { suit, rank }
    }

    pub fn display(&self) -> String {
        format!("{}{}", self.rank.display(), self.suit.symbol())
    }

    pub fn is_face_card(&self) -> bool {
        self.rank.is_face()
    }
}

// Blackjack-specific helpers available when the rank supports Blackjack rules
impl<R: BlackjackRank> Card<R> {
    pub fn value(&self) -> u8 {
        self.rank.blackjack_value()
    }
    pub fn is_ace(&self) -> bool {
        self.rank.is_ace()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CardCollection<R: Rank> {
    pub(crate) cards: Vec<Card<R>>,
}

impl<R: Rank> Default for CardCollection<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Rank> CardCollection<R> {
    pub fn new() -> Self {
        CardCollection { cards: Vec::new() }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card<R>> {
        self.cards.pop()
    }

    pub fn push(&mut self, card: Card<R>) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

pub struct Deck<R: Rank> {
    pub cards: CardCollection<R>,
}

impl<R: Rank> Default for Deck<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Rank> Deck<R> {
    pub fn new() -> Self {
        let mut cards = CardCollection::new();
        for &suit in Suit::all() {
            for &rank in R::all() {
                cards.push(Card::new(suit, rank));
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle();
    }

    pub fn deal(&mut self) -> Option<Card<R>> {
        self.cards.deal()
    }
}

pub struct Shoe<R: Rank> {
    pub shoe: CardCollection<R>,
}

impl<R: Rank> Shoe<R> {
    pub fn new(num_decks: usize) -> Self {
        let mut shoe = CardCollection::new();
        for _ in 0..num_decks {
            for &suit in Suit::all() {
                for &rank in R::all() {
                    shoe.push(Card::new(suit, rank));
                }
            }
        }
        Shoe { shoe }
    }

    pub fn shuffle(&mut self) {
        self.shoe.shuffle();
    }

    pub fn deal(&mut self) -> Option<Card<R>> {
        self.shoe.deal()
    }

    pub fn len(&self) -> usize {
        self.shoe.len()
    }
}

// A generic Hand type if needed by callers; games often define their own.
pub struct Hand<R: Rank> {
    pub cards: Vec<Card<R>>,
}

// A default rank implementation that mirrors a standard 13-rank deck.
// This is useful for tests and callers that don't define a custom rank.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StandardRank {
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

impl Rank for StandardRank {
    fn all() -> &'static [Self] {
        &[
            StandardRank::Two,
            StandardRank::Three,
            StandardRank::Four,
            StandardRank::Five,
            StandardRank::Six,
            StandardRank::Seven,
            StandardRank::Eight,
            StandardRank::Nine,
            StandardRank::Ten,
            StandardRank::Jack,
            StandardRank::Queen,
            StandardRank::King,
            StandardRank::Ace,
        ]
    }

    fn display(&self) -> &'static str {
        match self {
            StandardRank::Two => "2",
            StandardRank::Three => "3",
            StandardRank::Four => "4",
            StandardRank::Five => "5",
            StandardRank::Six => "6",
            StandardRank::Seven => "7",
            StandardRank::Eight => "8",
            StandardRank::Nine => "9",
            StandardRank::Ten => "10",
            StandardRank::Jack => "J",
            StandardRank::Queen => "Q",
            StandardRank::King => "K",
            StandardRank::Ace => "A",
        }
    }

    fn is_face(&self) -> bool {
        matches!(self, StandardRank::Jack | StandardRank::Queen | StandardRank::King)
    }
}

impl BlackjackRank for StandardRank {
    fn blackjack_value(&self) -> u8 {
        match self {
            StandardRank::Two => 2,
            StandardRank::Three => 3,
            StandardRank::Four => 4,
            StandardRank::Five => 5,
            StandardRank::Six => 6,
            StandardRank::Seven => 7,
            StandardRank::Eight => 8,
            StandardRank::Nine => 9,
            StandardRank::Ten | StandardRank::Jack | StandardRank::Queen | StandardRank::King => 10,
            StandardRank::Ace => 11, // Count adjustment handled by blackjack logic
        }
    }

    fn is_ace(&self) -> bool {
        matches!(self, StandardRank::Ace)
    }
}

impl BaccaratRank for StandardRank {
    fn baccarat_value(&self) -> u8 {
        match self {
            StandardRank::Two => 2,
            StandardRank::Three => 3,
            StandardRank::Four => 4,
            StandardRank::Five => 5,
            StandardRank::Six => 6,
            StandardRank::Seven => 7,
            StandardRank::Eight => 8,
            StandardRank::Nine => 9,
            StandardRank::Ten | StandardRank::Jack | StandardRank::Queen | StandardRank::King => 0,
            StandardRank::Ace => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_value() {
        let card = Card::<StandardRank>::new(Suit::Hearts, StandardRank::Ace);
        // Uses blackjack values by default when calling value()
        assert_eq!(card.value(), 11);

        let card = Card::<StandardRank>::new(Suit::Spades, StandardRank::Ten);
        assert_eq!(card.value(), 10);

        let card = Card::<StandardRank>::new(Suit::Diamonds, StandardRank::Three);
        assert_eq!(card.value(), 3);
    }

    #[test]
    fn test_deck_creation() {
        let deck = Deck::<StandardRank>::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_shuffle() {
        let mut deck1 = Deck::<StandardRank>::new();
        let mut deck2 = Deck::<StandardRank>::new();
        deck1.shuffle();
        deck2.shuffle();
        // There's a small chance they could be the same after shuffling, but it's very unlikely
        assert_ne!(deck1.cards, deck2.cards);
    }

    #[test]
    fn test_deal_card() {
        let mut deck = Deck::<StandardRank>::new();
        let initial_len = deck.cards.len();
        let card = deck.deal();
        assert!(card.is_some());
        assert_eq!(deck.cards.len(), initial_len - 1);
    }

    #[test]
    fn test_shoe_creation() {
        let shoe = Shoe::<StandardRank>::new(6);
        assert_eq!(shoe.shoe.len(), 52 * 6);
    }

    #[test]
    fn test_shoe_shuffle() {
        let mut shoe1 = Shoe::<StandardRank>::new(6);
        let mut shoe2 = Shoe::<StandardRank>::new(6);
        shoe1.shuffle();
        shoe2.shuffle();
        assert_ne!(shoe1.shoe, shoe2.shoe);
    }

    #[test]
    fn test_deal_from_shoe() {
        let mut shoe = Shoe::<StandardRank>::new(6);
        let initial_len = shoe.shoe.len();
        let card = shoe.deal();
        assert!(card.is_some());
        assert_eq!(shoe.shoe.len(), initial_len - 1);
    }

}