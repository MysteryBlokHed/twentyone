#[cfg(test)]
mod tests {
    use twentyone::game::{Dealer, DealerRequest, Player, PlayerAction};
    use twentyone::{cards, game};

    #[test]
    fn deck_tests() {
        let mut deck = cards::create_deck();
        cards::shuffle_deck(&mut deck);
        let card = deck.get(0).unwrap().clone();
        // Draw card (returns first card of deck and removes it from the vector)
        assert_eq!(cards::draw_card(&mut deck).unwrap(), card);
        // Ensure that the vector length has been reduced from 52 to 51
        assert_eq!(deck.len(), 51);
    }

    #[test]
    fn shoe_tests() {
        let mut shoe = cards::create_shoe(6);
        cards::shuffle_deck(&mut shoe);
        let card = shoe.get(0).unwrap().clone();
        // Draw card (returns first card of deck and removes it from the vector)
        assert_eq!(cards::draw_card(&mut shoe).unwrap(), card);
        // Ensure that the vector length has been reduced from 312 to 311
        assert_eq!(shoe.len(), 311);
    }

    #[test]
    fn hand_tests() {
        let mut deck = cards::create_deck();
        cards::shuffle_deck(&mut deck);
        let mut hand: Vec<[char; 2]> = Vec::new();
        let card = deck.get(0).unwrap().clone();
        // Hit card from deck to hand
        cards::hit_card(&mut deck, &mut hand);
        assert_eq!(card, hand[0]);
    }

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
        fn callback(request: DealerRequest, player: Option<&Player>, _: &Dealer) -> PlayerAction {
            match request {
                DealerRequest::Play(i) => {
                    println!("Dealer requested play");
                    let value = game::get_hand_value(&player.unwrap().hands()[i], true);
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
                DealerRequest::UpCard(card) => {
                    println!("Dealer up card: {}{}", card[0], card[1]);
                    PlayerAction::None
                }
                DealerRequest::HitCard(card) => {
                    println!("Dealer hit card: {}{}", card[0], card[1]);
                    PlayerAction::None
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
                DealerRequest::LowCards => {
                    println!("Dealer low on cards, automatically creating new shoe");
                    PlayerAction::None
                }
            }
        }

        let mut shoe = cards::create_shoe(6);
        cards::shuffle_deck(&mut shoe);
        let mut dealer = Dealer::new(shoe, game::DEFAULT_CONFIG, &callback);
        // Mutable reference to players vector
        let players = dealer.players_mut();

        let player = Player::new(1000);
        players.push(player);

        // Try playing 5 rounds
        for _ in 0..5 {
            println!("--- New Round ---");
            dealer.play_round(true);
        }
    }
}
