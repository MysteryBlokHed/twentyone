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
    /// The dealer's up card
    UpCard([char; 2]),
    /// The dealer's hit card
    HitCard([char; 2]),
    /// The dealer's hand after they have played
    DealerHand(Vec<[char; 2]>),
    /// An error with a PlayerAction
    ///
    /// # Arguments
    ///
    /// * `PlayerActionError` - More info on why the error occurred
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

/// Configure different aspects of the game
///
/// # Fields
///
/// * `stand_soft_17` - Whether the dealer should stand on soft 17 or hit
/// * `blackjack_payout` - The multiplier for when a player gets a blackjack
/// * `double_after_split` - Whether to allow doubling down after splitting
pub struct GameConfig {
    pub stand_soft_17: bool,
    pub blackjack_payout: f32,
    pub splitting: bool,
    pub doubling_down: bool,
    pub double_after_split: bool,
}

/// A default configuration for game settings.
///
/// Stands on soft 17, pays out blackjacks 3 to 2, and allows doubling after splitting.
pub const DEFAULT_CONFIG: GameConfig = GameConfig {
    stand_soft_17: true,
    blackjack_payout: 1.5,
    splitting: true,
    doubling_down: true,
    double_after_split: true,
};

/// Describes a blackjack dealer
pub struct Dealer<'a> {
    hand: Vec<[char; 2]>,
    shoe: Vec<[char; 2]>,
    players: Vec<Player>,
    config: GameConfig,
    callback: &'a dyn Fn(DealerRequest, Option<&Player>, &Dealer) -> PlayerAction,
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
    /// `callback` is passed a `DealerRequest` and an `Option<&Player>`.
    /// The option will always have a player if it applies to the event (eg. betting),
    /// but will not have a player for dealer updates (eg. up card, dealer hits).
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
    /// | `DealerRequest::UpCard([char; 2])`          | `PlayerAction::None`                                                                                 |
    /// | `DealerRequest::HitCard([char; 2])`         | `PlayerAction::None`                                                                                 |
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
    /// Example code is available in the [Quick Start](../index.html#quick-start) from the main page.
    pub fn new<'a>(
        shoe: Vec<[char; 2]>,
        game_config: GameConfig,
        callback: &'a dyn Fn(DealerRequest, Option<&Player>, &Dealer) -> PlayerAction,
    ) -> Dealer {
        Dealer {
            hand: Vec::new(),
            shoe: shoe,
            players: Vec::new(),
            config: game_config,
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
        for _ in 0..2 {
            cards::hit_card(&mut self.shoe, &mut self.hand);
            for player in self.players.iter_mut() {
                cards::hit_card(&mut self.shoe, &mut player.hands_mut()[0]);
            }
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
    pub fn play_round(&mut self, clear_table: bool) {
        if clear_table {
            self.clear_table();
        }

        let mut player_bets: Vec<i32> = Vec::new();

        // Get bets
        for i in 0..self.players.len() {
            loop {
                let bet = (self.callback)(DealerRequest::Bet, Some(&self.players[i]), &self);
                if let PlayerAction::Bet(amount) = bet {
                    // Check if player can afford bet
                    if self.players[i].money() >= &amount {
                        player_bets.push(amount);
                        *self.players[i].money_mut() -= amount;
                        break;
                    } else {
                        let error = PlayerActionError::NotEnoughMoney(0, bet);
                        (self.callback)(DealerRequest::Error(error), Some(&self.players[i]), &self);
                    }
                } else {
                    let error = PlayerActionError::UnexpectedAction(0, bet);
                    (self.callback)(DealerRequest::Error(error), Some(&self.players[i]), &self);
                }
            }
        }

        // Deal hands
        self.deal_hands();

        // Send dealer up card
        (self.callback)(DealerRequest::UpCard(self.hand[1]), None, &self);

        // Get player actions
        for i in 0..self.players.len() {
            let mut can_double: Vec<bool>;
            let mut can_split: bool;
            if self.config.doubling_down {
                // Check if player has enough money to double down
                can_double = vec![self.players[i].money() >= &player_bets[i]];
            } else {
                can_double = vec![false];
            }
            if self.config.splitting {
                // Check if player cards are valid for a split and if player has enough money
                can_split = crate::game::can_split(&self.players[i].hands()[0]) && can_double[0];
            } else {
                can_split = false;
            }

            // Keep track of stood hands
            let mut stood = vec![false];

            // Keep track of original bet
            // Used when doubling after splitting
            let original_bet = player_bets[i];

            // Active hand
            let mut hand_count = 1;
            let mut j = 0;
            // Get actions from each hand, one at a time
            // Using a loop and incrementing j manually because for loops would not recheck
            // length of player.hands() after a split
            loop {
                if j >= hand_count {
                    break;
                }
                while !stood[j] {
                    let action =
                        (self.callback)(DealerRequest::Play(j), Some(&self.players[i]), &self);
                    match action {
                        PlayerAction::Hit => {
                            self.hit_card(i, j);
                            can_double[j] = false;
                        }
                        PlayerAction::Stand => stood[j] = true,
                        PlayerAction::DoubleDown => {
                            if can_double[j] {
                                *self.players[i].money_mut() -= original_bet;
                                player_bets[i] += original_bet;
                                stood[j] = true;
                                self.hit_card(i, j);
                                can_double[j] = false;
                            } else {
                                (self.callback)(
                                    DealerRequest::Error(PlayerActionError::UnexpectedAction(
                                        j, action,
                                    )),
                                    Some(&self.players[i]),
                                    &self,
                                );
                            }
                        }
                        PlayerAction::Split => {
                            if can_split {
                                *self.players[i].money_mut() -= original_bet;
                                player_bets[i] += original_bet;
                                self.players[i].hands_mut().push(Vec::new());
                                stood.push(false);
                                if self.config.double_after_split && self.config.doubling_down {
                                    can_double.push(true);
                                } else {
                                    can_double[0] = false;
                                    can_double.push(false);
                                }
                                // "Draw" card from first hand and place it into second
                                let card = cards::draw_card(
                                    self.players[i].hands_mut().get_mut(0).unwrap(),
                                );
                                self.players[i].hands_mut()[1].push(card.unwrap());
                                // Hit another card to each hand
                                self.hit_card(i, 0);
                                self.hit_card(i, 1);
                                hand_count = 2;
                                can_split = false;
                            } else {
                                (self.callback)(
                                    DealerRequest::Error(PlayerActionError::UnexpectedAction(
                                        j, action,
                                    )),
                                    Some(&self.players[i]),
                                    &self,
                                );
                            }
                        }
                        _ => {
                            let error = PlayerActionError::UnexpectedAction(j, action);
                            (self.callback)(
                                DealerRequest::Error(error),
                                Some(&self.players[i]),
                                &self,
                            );
                        }
                    }

                    // Check if the hand is busted
                    if get_hand_value(&self.players[i].hands()[j], true) > 21 {
                        stood[j] = true;
                    }
                }
                j += 1;
            }
        }

        // Dealer play
        let mut busted = false;
        loop {
            let hand_value = get_hand_value(&self.hand, true);
            if hand_value > 21 {
                busted = true;
                break;
            } else if hand_value >= 17 {
                if self.config.stand_soft_17 {
                    break;
                // Check if hand is exactly 17 contains an ace
                } else if hand_value == 17 && self.hand.iter().any(|&i| i[1] == 'A') {
                    // Check if ace is acting as an 11 or a 1
                    if hand_value == get_hand_value(&self.hand, false) {
                        let card = cards::draw_card(&mut self.shoe).unwrap();
                        self.hand.push(card);
                        (self.callback)(DealerRequest::HitCard(card), None, &self);
                    } else {
                        break;
                    }
                }
                break;
            } else {
                let card = cards::draw_card(&mut self.shoe).unwrap();
                self.hand.push(card);
                (self.callback)(DealerRequest::HitCard(card), None, &self);
            }
        }

        // Pay out winners
        let dealer_hand_value = get_hand_value(&self.hand, true);
        for i in 0..self.players.len() {
            for j in 0..self.players[i].hands().len() {
                let hand_value = get_hand_value(&self.players[i].hands()[j], true);
                // Check if player busted
                if hand_value > 21 {
                    continue;
                }

                // Pay out normal amount if player did not bust, did not have blackjack,
                // and beat dealer/dealer busted
                if hand_value < 21 && (busted || hand_value > dealer_hand_value) {
                    self.players[i].money +=
                        player_bets[i] * 2 / self.players[i].hands().len() as i32;
                } else if hand_value == 21 && (busted || hand_value > dealer_hand_value) {
                    // Check if player had blackjack
                    if self.players[i].hands()[j].len() == 2 {
                        // Make sure dealer didn't have blackjack
                        if dealer_hand_value == 21 && self.hand.len() == 2 {
                            // Push, refund player
                            self.players[i].money += player_bets[i];
                        } else {
                            // Pay out 3 to 2
                            self.players[i].money += player_bets[i]
                                + (player_bets[i] as f32 * self.config.blackjack_payout) as i32;
                        }
                    } else {
                        self.players[i].money +=
                            player_bets[i] * 2 / self.players[i].hands().len() as i32;
                    }
                // Push, refund player
                } else if hand_value == dealer_hand_value {
                    self.players[i].money += player_bets[i];
                }
            }
        }

        (self.callback)(DealerRequest::DealerHand(self.hand.clone()), None, &self);
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
