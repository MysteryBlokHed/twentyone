//! Functions and structures required in almost all situations
pub use crate::cards::{create_deck, create_shoe, shuffle_deck};
pub use crate::game::{
    Dealer, DealerRequest, GameConfig, Player, PlayerAction, PlayerActionError, DEFAULT_CONFIG,
};
