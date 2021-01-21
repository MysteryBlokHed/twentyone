use crate::cards;

/// Actions a player can perform
pub enum PlayerAction {
    Hit,
    Stand,
    DoubleDown,
    Split,
    /// Bet an amount of money
    Bet(i32),
    None,
}

/// Requests for the player from the dealer
pub enum DealerRequest {
    /// Request a bet from the player
    Bet,
    /// Request a player to play a hand
    ///
    /// # Arguments
    ///
    /// * `usize` - The index of the hand to play
    Play(usize),
    /// The dealer's hand after they have played
    DealerHand(Vec<[char; 2]>),
    Error(PlayerActionError),
}

/// Reason for a dealer being unable to perform an action
pub enum PlayerActionError {
    /// Not enough money for the requested action
    ///
    /// # Arguments
    ///
    /// * `usize` - The index of the affected hand, if applicable
    /// * `PlayerAction` - The attempted action
    NotEnoughMoney(usize, PlayerAction),
    /// An unexpected action was returned
    ///
    /// # Arguments
    ///
    /// * `usize` - The index of the affected hand, if applicable
    /// * `PlayerAction` - The unexpected action
    UnexpectedAction(usize, PlayerAction),
}

/// Describes a blackjack dealer
pub struct Dealer<'a> {
    hand: Vec<[char; 2]>,
    shoe: Vec<[char; 2]>,
    players: Vec<Player>,
    callback: &'a dyn Fn(DealerRequest, &Player) -> PlayerAction,
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
    /// * `shoe` - The shoe (or deck) to draw from
    /// * `callback` - A function to handle player turns
    ///
    /// `callback` is passed a `DealerRequest` and a reference to the active hand.
    ///
    /// # Callback
    ///
    /// The callback function will always return a `PlayerAction`,
    /// but it should return different things based on the `DealerRequest`:
    ///
    /// | `DealerRequest`                             | `PlayerAction`                                                                                       |
    /// |---------------------------------------------|------------------------------------------------------------------------------------------------------|
    /// | `DealerRequest::Bet`                        | `PlayerAction::Bet(i32)`                                                                             |
    /// | `DealerRequest::Play`                       | One of `PlayerAction::Hit`, `PlayerAction::Stand`, `PlayerAction::DoubleDown`, `PlayerAction::Split` |
    /// | `DealerRequest::Error(PlayerActionError)`   | `PlayerAction::None` and handle the returned error                                                   |
    /// | `DealerRequest::DealerHand(Vec<[char; 2]>)` | `PlayerAction::None`                                                                                 |
    ///
    /// If an unexpected return value is given, the callback will be called
    ///  again with a request of `DealerAction::Error(PlayerActionError::UnexpectedAction)`
    /// along with the index of the affected hand (if applicable)
    /// and the request that was invalid.
    ///
    /// After an error is provided, the dealer will request the same action that
    /// caused the error. If nothing changes, the dealer will infinitely loop.
    ///
    /// # Examples
    ///
    /// ```
    /// use twentyone::cards;
    /// use twentyone::game;
    /// use twentyone::game::{Dealer, Player, PlayerAction, DealerRequest};
    ///
    /// fn callback(request: DealerRequest, player: &Player) -> PlayerAction {
    ///     if let DealerRequest::Bet = request {
    ///         PlayerAction::Bet(10)
    ///     } else if let DealerRequest::Play(i) = request {
    ///         let value = game::get_hand_value(&player.hands()[i], true);
    ///         if value < 17 {
    ///          PlayerAction::Hit
    ///         } else {
    ///             PlayerAction::Stand
    ///         }
    ///     } else {
    ///         PlayerAction::Stand
    ///     }
    /// }
    ///
    /// let shoe = cards::create_shoe(6);
    /// let dealer = Dealer::new(shoe, &callback);
    /// ```
    pub fn new<'a>(
        shoe: Vec<[char; 2]>,
        callback: &'a dyn Fn(DealerRequest, &Player) -> PlayerAction,
    ) -> Dealer {
        Dealer {
            hand: Vec::new(),
            shoe: shoe,
            players: Vec::new(),
            callback: callback,
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
            player.hands_mut().push(Vec::new());
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

    /// Play a round of blackjack
    ///
    /// Calls `callback` to get player bets/actions.
    ///
    /// # Arguments
    ///
    /// * `clear_table` - Clear the table at the beginning of the round
    /// * `stand_17` - `true` if the dealer should stand on soft 17,
    /// `false` if the dealer should hit
    pub fn play_round(&mut self, clear_table: bool, stand_17: bool) {
        if clear_table {
            self.clear_table();
        }

        let mut player_bets: Vec<i32> = Vec::new();

        // Get bets
        for i in 0..self.players.len() {
            loop {
                let bet = (self.callback)(DealerRequest::Bet, &self.players[i]);
                if let PlayerAction::Bet(amount) = bet {
                    // Check if player can afford bet
                    if self.players[i].money() >= &amount {
                        player_bets.push(amount);
                        *self.players[i].money_mut() -= amount;
                        break;
                    } else {
                        let error = PlayerActionError::NotEnoughMoney(0, bet);
                        (self.callback)(DealerRequest::Error(error), &self.players[i]);
                    }
                } else {
                    let error = PlayerActionError::UnexpectedAction(0, bet);
                    (self.callback)(DealerRequest::Error(error), &self.players[i]);
                }
            }
        }

        // Deal hands
        self.deal_hands();

        // Get player actions
        for i in 0..self.players.len() {
            // Check if player has enough money to double down
            let mut can_double = self.players[i].money() >= &player_bets[i];
            // Check if player cards are valid for a split and if player has enough money
            let mut can_split = can_split(&self.players[i].hands()[0]) && can_double;

            // Keep track of stood hands
            let mut stood = vec![false];

            // Request actions from player
            loop {
                for j in 0..self.players[i].hands().len() {
                    if !stood[j] {
                        let action = (self.callback)(DealerRequest::Play(j), &self.players[i]);
                        match action {
                            PlayerAction::Hit => self.hit_card(i, j),
                            PlayerAction::Stand => stood[j] = true,
                            PlayerAction::DoubleDown => {
                                if can_double {
                                    *self.players[i].money_mut() -= player_bets[i];
                                    player_bets[i] *= 2;
                                }
                            }
                            PlayerAction::Split => {
                                if can_split {
                                    *self.players[i].money_mut() -= player_bets[i];
                                    player_bets[i] *= 2;
                                    self.players[i].hands_mut().push(Vec::new());
                                    stood.push(false);
                                    // "Draw" card from first hand and place it into second
                                    let card = cards::draw_card(
                                        self.players[i].hands_mut().get_mut(0).unwrap(),
                                    );
                                    self.players[i].hands_mut()[1].push(card.unwrap());
                                    // Hit another card to each hand
                                    self.hit_card(i, 0);
                                    self.hit_card(i, 1);
                                }
                            }
                            _ => {
                                let error = PlayerActionError::UnexpectedAction(j, action);
                                (self.callback)(DealerRequest::Error(error), &self.players[i]);
                            }
                        }
                    }
                }
                can_double = false;
                can_split = false;

                // Check if any hands have beeen stood
                for i in 0..self.players[i].hands().len() {
                    if get_hand_value(&self.players[i].hands()[i], true) > 21 {
                        stood[i] = true;
                    }
                }

                // Break if every hand is stood
                if stood[0] && stood.iter().min() == stood.iter().max() {
                    break;
                }
            }
        }

        // Dealer play
        loop {
            let hand_value = get_hand_value(&self.hand, true);
            if hand_value >= 17 {
                if stand_17 {
                    break;
                // Check if hand is exactly 17 contains an ace
                } else if hand_value == 17 && self.hand.iter().any(|&i| i[1] == 'A') {
                    // Check if ace is acting as an 11 or a 1
                    if hand_value == get_hand_value(&self.hand, false) {
                        cards::hit_card(&mut self.shoe, &mut self.hand);
                    } else {
                        break;
                    }
                }
                break;
            } else {
                cards::hit_card(&mut self.shoe, &mut self.hand);
            }
        }

        // Pay out winners
        for i in 0..self.players.len() {
            for j in 0..self.players[i].hands().len() {
                if get_hand_value(&self.players[i].hands()[j], true) < 21 {
                    // Give back double the bet over the amount of hands
                    // (stops from overpaying split hands if both won)
                    self.players[i].money +=
                        player_bets[i] * 2 / self.players[i].hands().len() as i32;
                }
            }
        }

        (self.callback)(
            DealerRequest::DealerHand(self.hand.clone()),
            &Player::new(0),
        );
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
            hands: vec![Vec::new()],
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
    if auto_aces {
        // Check if an ace being 11 would bust the hand
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
    use crate::game::{Dealer, DealerRequest, Player, PlayerAction};
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
        fn callback(request: DealerRequest, player: &Player) -> PlayerAction {
            match request {
                DealerRequest::Play(i) => {
                    println!("Dealer requested play");
                    let value = game::get_hand_value(&player.hands()[i], true);
                    if value < 17 {
                        println!("Hand is <17, hitting");
                        PlayerAction::Hit
                    } else {
                        println!("Hand is >=17, standing");
                        PlayerAction::Stand
                    }
                }
                DealerRequest::Bet => {
                    println!("Dealer requested bet");
                    PlayerAction::Bet(10)
                }
                DealerRequest::DealerHand(hand) => {
                    println!(
                        "Dealer hand value was {}",
                        game::get_hand_value(&hand, true)
                    );
                    PlayerAction::None
                }
                DealerRequest::Error(_) => {
                    println!("Dealer returned an error");
                    PlayerAction::None
                }
            }
        }

        let mut shoe = cards::create_shoe(6);
        cards::shuffle_deck(&mut shoe);
        let mut dealer = Dealer::new(shoe, &callback);
        // Mutable reference to players vector
        let players = dealer.players_mut();

        let player = Player::new(1000);
        players.push(player);

        // Try playing 5 rounds
        for _ in 0..5 {
            println!("--- New Round ---");
            dealer.play_round(true, true);
        }
    }
}
