use rand::seq::SliceRandom;

/// Returns a 52-card deck in order
///
/// # Examples
///
/// ```
/// use twentyone::cards;
/// let mut deck = cards::create_deck();
/// ```
pub fn create_deck() -> Vec<[char; 2]> {
    let mut deck = Vec::new();
    let suits = ['S', 'H', 'C', 'D'];
    let values = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    // Generate deck
    for i in 0..4 {
        for j in 0..13 {
            deck.push([suits[i], values[j]]);
        }
    }

    deck
}

/// Returns a shoe with a specified amount of decks in it
///
/// # Arguments
///
/// * `deck_count` - The amount of decks to be placed in the shoe
///
/// # Examples
///
/// ```
/// use twentyone::cards;
/// let mut shoe = cards::create_shoe(6);
/// ```
pub fn create_shoe(deck_count: u8) -> Vec<[char; 2]> {
    let mut shoe = Vec::new();
    for _ in 0..deck_count {
        shoe.append(&mut create_deck());
    }
    shoe
}

/// Shuffles a deck or shoe into a random order
///
/// # Arguments
///
/// * `deck` - The deck or shoe to shuffle
///
/// # Examples
///
/// ```
/// use twentyone::cards;
/// let mut deck = cards::create_deck();
/// cards::shuffle_deck(&mut deck);
/// ```
pub fn shuffle_deck(deck: &mut Vec<[char; 2]>) {
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
}

/// Returns the first card from a deck or shoe, then removes it
///
/// # Arguments
///
/// * `deck` - The deck or shoe to draw from
///
/// # Examples
///
/// ```
/// use twentyone::cards;
/// let mut deck = cards::create_deck();
/// let card = cards::draw_card(&mut deck);
/// ```
pub fn draw_card(deck: &mut Vec<[char; 2]>) -> Result<[char; 2], ()> {
    if deck.len() > 0 {
        let card = deck[0];
        deck.remove(0);
        Ok(card)
    } else {
        Err(())
    }
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use crate::cards;

    #[test]
    fn card_tests() {
        // -- Deck --
        let mut deck = cards::create_deck();
        cards::shuffle_deck(&mut deck);
        let card = deck.get(0).unwrap().clone();
        // Draw card (returns first card of deck and removes it from the vector)
        assert_eq!(cards::draw_card(&mut deck).unwrap(), card);
        // Ensure that the vector length has been reduced from 52 to 51
        assert_eq!(deck.len(), 51);

        // -- Shoe --
        let mut shoe = cards::create_shoe(6);
        cards::shuffle_deck(&mut shoe);
        let card = shoe.get(0).unwrap().clone();
        // Draw card (returns first card of deck and removes it from the vector)
        assert_eq!(cards::draw_card(&mut shoe).unwrap(), card);
        // Ensure that the vector length has been reduced from 312 to 311
        assert_eq!(shoe.len(), 311);
    }
}
