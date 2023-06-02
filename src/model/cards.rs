
mod cards {
    pub enum CardValue {
        One,
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

    pub enum CardColor {
        Spades,
        Hearts,
        Diamonds,
        Clubs,
    }

    pub struct Card {
        value: CardValue,
        color: CardColor,
    }
}