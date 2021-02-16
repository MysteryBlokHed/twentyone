use std::io;
use twentyone::cards::*;
use twentyone::game::*;

/// Turn a card into a 2-character string
fn card_to_printable(card: &[char; 2]) -> String {
    format!(
        "{}{}",
        // Suit
        match card[0] {
            'S' => '♠',
            'H' => '♥',
            'C' => '♣',
            'D' => '♦',
            _ => ' ',
        },
        // Value
        card[1]
    )
}

fn callback(request: DealerRequest, p: Option<&Player>, dealer: &Dealer) -> PlayerAction {
    match request {
        DealerRequest::Bet => {
            println!("Current Balance: {}", p.unwrap().money());
            println!("Bet: ");
            // Read line for bet
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("");
            // Strip whitespace
            input.retain(|c| !c.is_whitespace());
            PlayerAction::Bet(input.parse::<i32>().unwrap())
        }
        DealerRequest::Play(i) => {
            println!("Your hand:");
            // Print hand
            for card in p.unwrap().hands()[i].iter() {
                print!("|{}|", card_to_printable(card));
            }
            // Print hand value
            println!(
                " Total value: {}",
                get_hand_value(&p.unwrap().hands()[i], true)
            );
            // Request action from user
            println!("Enter one of [H]it, [S]tand, [D]ouble Down, S[p]lit");
            // Read line for action
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("");
            // Strip whitespace
            input.retain(|c| !c.is_whitespace());
            // Perform action
            match &input.to_ascii_lowercase()[..] {
                "h" | "hit" => PlayerAction::Hit,
                "s" | "stand" => PlayerAction::Stand,
                "d" | "double" | "double down" => PlayerAction::DoubleDown,
                "p" | "split" => PlayerAction::Split,
                _ => PlayerAction::None,
            }
        }
        DealerRequest::UpCard(card) => {
            println!("Dealer up card: {}", card_to_printable(&card));
            PlayerAction::None
        }
        DealerRequest::HitCard(card) => {
            println!("Dealer hit: {}", card_to_printable(&card));
            PlayerAction::None
        }
        DealerRequest::DealerHand(h) => {
            let dealer_value = get_hand_value(&h, true);
            // Print dealer hand
            println!("Dealer hand:");
            for card in h.iter() {
                print!("|{}|", card_to_printable(card));
            }
            // Print hand value
            println!(" Total value: {}\n", get_hand_value(&h, true));
            // Print results of player hands
            for i in 0..dealer.players()[0].hands().len() {
                let player_value = get_hand_value(&dealer.players()[0].hands()[i], true);
                // Print hand
                println!("Player Hand {}:", i + 1);
                for card in dealer.players()[0].hands()[i].iter() {
                    print!("|{}|", card_to_printable(card));
                }
                // Print hand value
                println!(
                    " Total value: {}",
                    get_hand_value(&dealer.players()[0].hands()[i], true)
                );
                if player_value > dealer_value && player_value <= 21
                    || (player_value <= 21 && dealer_value > 21)
                {
                    println!("Result Hand {}: Win!", i + 1);
                } else if player_value < dealer_value || player_value > 21 {
                    println!("Result Hand {}: Loss.", i + 1);
                } else {
                    println!("Result Hand {}: Push.", i + 1);
                }
                println!();
            }
            PlayerAction::None
        }
        DealerRequest::Error(e) => {
            match e {
                PlayerActionError::NotEnoughMoney(_, _) => println!("Not enough money."),
                PlayerActionError::UnexpectedAction(_, _) => println!("Cannot perform action."),
            }
            PlayerAction::None
        }
    }
}

fn main() {
    let mut shoe = create_shoe(6);
    shuffle_deck(&mut shoe);

    let player = Player::new(1000);

    let mut dealer: Dealer;

    dealer = Dealer::new(shoe, DEFAULT_CONFIG, &callback);
    dealer.players_mut().push(player);

    loop {
        dealer.play_round(true);
    }
}
