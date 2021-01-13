use crate::cards;

/// Actions a player can perform
pub enum PlayerAction {
    Hit,
    Stand,
    DoubleDown,
    Split,
}

/// Describes a blackjack dealer
pub struct Dealer<'a> {
    hand: Vec<[char; 2]>,
    shoe: Vec<[char; 2]>,
    players: Vec<Player>,
    on_action: &'a dyn Fn(&Player, &Vec<[char; 2]>) -> PlayerAction,
}

/// Describes a blackjack player
pub struct Player {
    money: i32,
    hands: Vec<Vec<[char; 2]>>,
}

impl Dealer<'_> {
    /// Returns a new Dealer
    ///
    /// # Arguments
    ///
    /// * `shoe` - The shoe (or decck) to draw from
    ///
    /// # Examples
    ///
    /// ```
    /// use twentyone::cards;
    /// use twentyone::game::Dealer;
    /// let shoe = cards::create_shoe();
    /// let dealer = Dealer::new(shoe);
    /// ```
    pub fn new<'a>(
        shoe: Vec<[char; 2]>,
        on_action: &'a dyn Fn(&Player, &Vec<[char; 2]>) -> PlayerAction,
    ) -> Dealer<'a> {
        Dealer {
            hand: Vec::new(),
            shoe: shoe,
            players: Vec::new(),
            on_action: on_action,
        }
    }

    /// Returns a reference to the dealer's hand
    pub fn hand(&self) -> &Vec<[char; 2]> {
        &self.hand
    }

    /// Returns a reference to the dealer's shoe
    pub fn shoe(&self) -> &Vec<[char; 2]> {
        &self.shoe
    }

    /// Returns a reference to the dealer's players
    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }
    /// Returns a mutable reference to the dealer's hand
    pub fn hand_mut(&mut self) -> &mut Vec<[char; 2]> {
        &mut self.hand
    }

    /// Returns a mutable reference to the dealer's shoe
    pub fn shoe_mut(&mut self) -> &mut Vec<[char; 2]> {
        &mut self.shoe
    }

    /// Returns a mutable reference to the dealer's players
    pub fn players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    /// Clear the dealer's and all players' hands
    pub fn clear_table(&mut self) {
        self.hand.clear();
        for player in self.players.iter_mut() {
            player.hands_mut().clear();
        }
    }

    /// Deal a hand to all players
    pub fn deal_hands(&mut self) {
        // Dealer's hand
        cards::hit_card(&mut self.shoe, &mut self.hand);
        cards::hit_card(&mut self.shoe, &mut self.hand);

        // Players' hands
        for player in self.players.iter_mut() {
            cards::hit_card(&mut self.shoe, &mut player.hands_mut()[0]);
            cards::hit_card(&mut self.shoe, &mut player.hands_mut()[0]);
        }
    }

    /// Hit a card to a player
    ///
    /// # Arguments
    ///
    /// * `player` - The index of the player to hit
    /// * `hand` - The index of the player's hand (used for split hands)
    pub fn hit_card(&mut self, player: usize, hand: usize) {
        cards::hit_card(&mut self.shoe, &mut self.players[player].hands[hand]);
    }
}

impl Player {
    /// Returns a new Player
    ///
    /// # Arguments
    ///
    /// * `money` - The amount of money to give the player
    /// * `hands` - A Vector of hands (`Vec<[char; 2]>`)
    ///
    /// # Examples
    ///
    /// ```
    /// use twentyone::game::Player;
    /// let player = Player::new(100);
    /// ```
    pub fn new(money: i32) -> Player {
        Player {
            money: money,
            hands: Vec::new(),
        }
    }

    /// Returns a reference to the player's money
    pub fn money(&self) -> &i32 {
        &self.money
    }

    /// Returns a reference to the player's hands
    pub fn hands(&self) -> &Vec<Vec<[char; 2]>> {
        &self.hands
    }

    /// Returns a mutable reference to the player's money
    pub fn money_mut(&mut self) -> &mut i32 {
        &mut self.money
    }

    /// Returns a mutable reference to the player's hands
    pub fn hands_mut(&mut self) -> &mut Vec<Vec<[char; 2]>> {
        &mut self.hands
    }
}

/// Returns the value of a hand
///
/// # Arguments
///
/// * `hand` - The hand to get the value of
/// * `auto_aces` - Turn aces into 1's if 11's will go over 21
///
/// # Examples
///
/// ```
/// use twentyone::{cards, game};
/// let mut deck = cards::create_deck();
/// cards::shuffle_deck(&mut deck);
/// let mut hand = Vec::new();
/// cards::hit_card(&mut deck, &mut hand);
/// cards::hit_card(&mut deck, &mut hand);
/// println!("{}", game::get_hand_value(&hand, true));
/// ```
pub fn get_hand_value(hand: &Vec<[char; 2]>, auto_aces: bool) -> u8 {
    let mut value = 0;
    let mut aces = 0;
    for i in hand.iter() {
        value += match i[1] {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' | 'J' | 'Q' | 'K' => 10,
            'A' => {
                aces += 1;
                0
            }
            _ => 0,
        }
    }
    // Add aces
    // Check if an ace being 11 would bust the hand
    if auto_aces {
        for _ in 0..aces {
            if value + 11 > 21 {
                value += 1;
            } else {
                value += 11;
            }
        }
    } else {
        value += 11 * aces;
    }
    value
}

/// Returns whether a hand is able to split
///
/// # Arguments
///
/// * `hand` - The hand to be split
///
/// # Examples
///
/// ```
/// use twentyone::{cards, game};
/// let mut deck = cards::create_deck();
/// cards::shuffle_deck(&mut deck);
/// let mut hand = Vec::new();
/// cards::hit_card(&mut deck, &mut hand);
/// cards::hit_card(&mut deck, &mut hand);
/// println!("{}", game::can_split(&hand));
/// ```
pub fn can_split(hand: &Vec<[char; 2]>) -> bool {
    if hand.len() != 2 {
        return false;
    }

    hand[0][1] == hand[1][1]
}

// --- Tests ---
#[cfg(test)]
mod tests {
    use crate::game::{Dealer, Player, PlayerAction};
    use crate::{cards, game};

    #[test]
    fn game_tests() {
        let mut deck = cards::create_deck();
        // Test hand value calculation
        let deck_slice = &deck[..13].iter().cloned().collect();
        assert_eq!(game::get_hand_value(&deck_slice, false), 95);

        cards::shuffle_deck(&mut deck);
        let mut hand = Vec::new();
        cards::hit_card(&mut deck, &mut hand);
        cards::hit_card(&mut deck, &mut hand);
        // Test hand splitting checks
        assert_eq!(game::can_split(&hand), hand[0][1] == hand[1][1]);
    }

    #[test]
    fn player_dealer_tests() {
        fn on_action(_: &Player, hand: &Vec<[char; 2]>) -> PlayerAction {
            let value = game::get_hand_value(hand, true);
            if value < 17 {
                PlayerAction::Hit
            } else {
                PlayerAction::Stand
            }
        }

        let mut shoe = cards::create_shoe(6);
        cards::shuffle_deck(&mut shoe);
        let mut dealer = Dealer::new(shoe, &on_action);
        // Mutable reference to players vector
        let players = dealer.players_mut();

        let player = Player::new(1000);
        players.push(player);

        // Deal hands
        dealer.deal_hands();
    }
}
