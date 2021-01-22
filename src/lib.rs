//! A blackjack engine for Rust.
//!
//! # Quick Start
//!
//! A quick example to get the library working:
//!
//! ```
//! use twentyone::cards::*;
//! use twentyone::game::*;
//!
//! // Callback that will be used to return player's actions such as betting or hitting
//! fn callback(request: DealerRequest, player: &Player) -> PlayerAction {
//!     match request {
//!         // Dealer asking player to play, along with a hand index
//!         DealerRequest::Play(i) => {
//!             // Get the value of the player's hand
//!             let value = get_hand_value(&player.hands()[i], true);
//!             // Hit if the hand value is <17, stand if it isn't
//!             if value < 17 {
//!                 PlayerAction::Hit
//!             } else {
//!                 PlayerAction::Stand
//!             }
//!         }
//!         // Dealer requesting a bet
//!         DealerRequest::Bet => {
//!             // Bet $10
//!             PlayerAction::Bet(10)
//!         }
//!         // Dealer showing their hand when the game is over
//!         DealerRequest::DealerHand(hand) => {
//!             // Print the value of the dealer's hand
//!             println!("Dealer hand was {}", get_hand_value(&hand, true));
//!             PlayerAction::None
//!         }
//!         // Dealer returning an error with the last action
//!         DealerRequest::Error(_) => {
//!             PlayerAction::None
//!         }
//!     }
//! }
//! ```
#![crate_name = "twentyone"]

pub mod cards;
pub mod game;
