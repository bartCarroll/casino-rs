#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub name: String,
    pub display: String,
    /// value in cents to avoid floating point rounding issues
    pub value_cents: u64,
}
