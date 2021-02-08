//! A blackjack engine for Rust.
//!
//! # Quick Start
//!
//! Here is an example to get the library working.
//! It automatically makes the player hit when their hand value is <17,
//! and stand when it's >= 17.
//!
//! ```
//! use twentyone::cards::*;
//! use twentyone::game::*;
//!
//! // Callback that will be used to return player's actions such as betting or hitting
//! fn callback(request: DealerRequest, player: Option<&Player>, dealer: &Dealer) -> PlayerAction {
//!     match request {
//!         // Dealer asking player to play, along with a hand index
//!         DealerRequest::Play(i) => {
//!             // Get the value of the player's hand
//!             let value = get_hand_value(&player.unwrap().hands()[i], true);
//!             println!("Player's hand value is {}", value);
//!             // Hit if the hand value is <17, stand if it isn't
//!             if value < 17 {
//!                 println!("Player is hitting");
//!                 PlayerAction::Hit
//!             } else {
//!                 println!("Player is standing");
//!                 PlayerAction::Stand
//!             }
//!         }
//!         // Dealer requesting a bet
//!         DealerRequest::Bet => {
//!             // Bet $10
//!             println!("Player is betting $10");
//!             PlayerAction::Bet(10)
//!         }
//!         // Dealer showing their hand when the game is over
//!         DealerRequest::DealerHand(hand) => {
//!             // Get the value of the player's and the dealer's hand
//!             let dealer_hand_value = get_hand_value(&hand, true);
//!             let player_hand_value = get_hand_value(&dealer.players()[0].hands()[0], true);
//!             // Print both
//!             println!("Player hand value is {}", player_hand_value);
//!             println!("Dealer hand value is {}", dealer_hand_value);
//!             // Print whether the player won, lost, or had a push
//!             if (player_hand_value > dealer_hand_value && player_hand_value <= 21)
//!                 || (player_hand_value <= 21 && dealer_hand_value > 21)
//!             {
//!                 println!("Player Won!");
//!             } else if player_hand_value == dealer_hand_value && player_hand_value <= 21 {
//!                 println!("Push.");
//!             } else {
//!                 println!("Player Lost.");
//!             }
//!             PlayerAction::None
//!         }
//!         // Dealer returning an error with the last action
//!         // Panic for error-checking reasons
//!         DealerRequest::Error(_) => panic!("An error occurred"),
//!         // Other events that don't require specific handling
//!         _ => PlayerAction::None,
//!     }
//! }
//!
//! fn main() {
//!     // Create a six-deck shoe and shuffle it
//!     let mut shoe = create_shoe(6);
//!     shuffle_deck(&mut shoe);
//!
//!     // Create a dealer
//!     let mut dealer = Dealer::new(shoe, &callback);
//!     // Create a player with $1000
//!     let player = Player::new(1000);
//!     // Add the player to the dealer
//!     dealer.players_mut().push(player);
//!
//!     // Auto-play five rounds
//!     for _ in 0..5 {
//!         dealer.play_round(true, true);
//!     }
//! }
//!
//! ```
#![crate_name = "twentyone"]

pub mod cards;
pub mod game;
