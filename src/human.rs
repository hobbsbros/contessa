//! Implements functions for the `Player` trait from a human's perspective.

use crate::{
    Card,
    Action,
    Player,
    PlayerMetadata,
};

use inquire::{
    Select,
    MultiSelect,
};

pub struct Human {
    id: usize,
    opponents: usize,
    hand: [Card; 2],
    coins: u8,
}

impl Player for Human {
    /// Gets the metadata of the given player.
    fn get_metadata(&self) -> PlayerMetadata {
        PlayerMetadata::Human
    }

    /// Checks whether or not a player has a given card.
    fn check(&self, card: Card) -> bool {
        self.hand.contains(&card)
    }
    
    /// Replaces one of the cards in this player's hand with the given card.
    /// 
    /// Note: call this function only when you know for sure that the outgoing
    /// card exists in this player's hand or you will mess things up.
    fn replace(&mut self, current: Card, new: Card) {
        println!("You discard {} and draw {}", current, new);

        if self.hand[0] == current {
            self.hand[0] = new;
        } else {
            self.hand[1] = new;
        }
    }

    /// Exchanges cards (used on `Ambassador`).
    fn exchange(&mut self, cards: &[Card]) -> Vec<Card> {
        let mut options = self.hand.to_vec();
        for card in cards {
            options.push(*card);
        }

        let retained = match MultiSelect::new("Please select two cards to retain.", options.clone()).prompt() {
            Ok(cards) => cards,
            Err(_) => todo!(),
        };

        if retained.len() != 2 {
            todo!();
        }

        // Note: it's OK to use `Option::unwrap` because we know that this element
        // exists in the list

        let index1 = options.iter().position(|x| *x == retained[0]).unwrap();
        options.remove(index1);

        let index2 = options.iter().position(|x| *x == retained[1]).unwrap();
        options.remove(index2);

        self.hand[0] = retained[0];
        self.hand[1] = retained[1];

        options
    }

    /// Gets the number of coins this player has.
    fn get_coins(&self) -> u8 {
        self.coins
    }

    /// Computes *a priori* probabilities of each player having certain cards.
    fn compute_hands(&mut self, _: &[Card]) {
        println!("You have {} and {}", self.hand[0], self.hand[1]);
    }

    /// Deals the given cards to the player.
    fn deal(&mut self, hand: [Card; 2]) {
        self.hand = hand;
    }

    /// Gains the number of coins specified.
    fn gain_coins(&mut self, coins: u8) {
        println!("You gain {} coins", coins);
        self.coins += coins;
    }

    /// Loses the number of coins specified.
    fn lose_coins(&mut self, coins: u8) -> u8 {
        let lost = if self.coins >= coins {
            self.coins -= coins;
            coins
        } else {
            let stolen = self.coins;
            self.coins = 0;
            stolen
        };

        println!("You lose {} coins", lost);

        lost
    }

    /// Forces the player to lose one influence.
    fn lose_influence(&mut self) -> Card {
        let mut options = Vec::new();

        for card in self.hand {
            if card != Card::None {
                options.push(card);
            }
        }

        if options.len() == 0 {
            return Card::None;
        }

        let lost = match Select::new("Please select a card to lose influence.", options).prompt() {
            Ok(card) => card,
            Err(_) => todo!(),
        };

        if self.hand[0] == lost {
            self.hand[0] = Card::None;
        } else {
            self.hand[1] = Card::None;
        }

        lost
    }

    /// Asks this player if he challenges a claim.
    /// 
    /// Returns `true` if the player challenges and `false` otherwise.
    fn check_challenge(&self, active_player: usize, card: Card) -> bool {
        if card == Card::None {
            // You can't challenge Income, ForeignAid, or Coup
            return false;
        }

        if self.is_eliminated() {
            // You can't challenge if you're out
            return false;
        }

        let options = vec![
            "Yes",
            "No",
        ];

        let prompt = format!("Do you challenge player {}'s claim to have {}?", active_player, card);

        let ans = match Select::new(&prompt, options).prompt() {
            Ok(a) => a,
            Err(_) => todo!(),
        };

        match ans {
            "Yes" => true,
            "No" => false,
            _ => unreachable!(),
        }
    }

    /// Asks this player if he blocks an action.
    /// 
    /// Returns `true` if the player blocks and `false` otherwise.  Also returns
    /// the card with which the player blocks.
    fn check_block(&self, action: Action) -> (bool, Card) {
        if self.is_eliminated() {
            // You can't block if you're out
            return (false, Card::None);
        }

        match action {
            Action::ForeignAid => (),
            Action::Assassinate (_) => (),
            Action::Steal (_) => (),
            _ => return (false, Card::None),
        }

        let options = vec![
            "Yes",
            "No",
        ];

        let prompt = format!("Do you block {}?", action);

        let ans = match Select::new(&prompt, options).prompt() {
            Ok(a) => a,
            Err(_) => todo!(),
        };

        match ans {
            "Yes" => if action == Action::ForeignAid {
                (true, Card::Duke)
            } else if let Action::Assassinate (target) = action {
                if target == self.id {
                    return (false, Card::None);
                }

                (true, Card::Contessa)
            } else if let Action::Steal (target) = action {
                if target == self.id {
                    return (false, Card::None);
                }

                let options = vec![
                    Card::Captain,
                    Card::Ambassador,
                ];

                let ans = match Select::new("With which card do you block?", options).prompt() {
                    Ok(card) => card,
                    Err(_) => todo!(),
                };

                (true, ans)
            } else {
                // All other actions are "unblockable"
                unreachable!()
            },
            "No" => (false, Card::None),
            _ => unreachable!(),
        }
    }

    /// Checks whether or not a player is still in the game.
    fn is_eliminated(&self) -> bool {
        self.hand == [Card::None, Card::None]
    }

    /// Select an action based on actions available.
    fn select_action(&self, eliminated_players: &[usize]) -> Action {
        let actions = self.get_available_actions(eliminated_players);

        let mut options = Vec::new();

        for action in actions {
            options.push(action);
        }

        let ans = match Select::new("Please select an action.", options).prompt() {
            Ok(a) => a,
            Err(_) => todo!(),
        };

        ans
    }
}

impl Human {
    /// Constructs a new player with the given ID and number of opponents.
    pub fn new(id: usize, opponents: usize) -> Self {
        Self {
            id,
            opponents,
            hand: [Card::None, Card::None],
            coins: 2,
        }
    }

    /// Gets the actions available for this player.
    fn get_available_actions(&self, eliminated_players: &[usize]) -> Vec<Action> {
        // If this player is eliminated, he must Pass.
        if self.is_eliminated() {
            return vec![Action::Pass];
        }

        let mut actions: Vec<Action> = Vec::new();

        // If this player has 10 coins or more, he must Coup.
        if self.coins >= 10 {
            for i in 0..=self.opponents {
                if i != self.id && !eliminated_players.contains(&i) {
                    actions.push(Action::Coup (i));
                }
            }

            return actions;
        }

        // Income and ForeignAid are always available.
        actions.push(Action::Income);
        actions.push(Action::ForeignAid);

        // Coup is available (for any player that is not this one) if this player has
        // at least 7 coins.
        if self.coins >= 7 {
            for i in 0..=self.opponents {
                if i != self.id && !eliminated_players.contains(&i) {
                    actions.push(Action::Coup (i));
                }
            }
        }

        // For each card we have, we can take "safe" actions

        // Dukes can Tax
        actions.push(Action::Tax);

        // Captains can Steal
        for i in 0..=self.opponents {
            if i != self.id && !eliminated_players.contains(&i) {
                actions.push(Action::Steal (i));
            }
        }

        // Ambassadors can Exchange
        actions.push(Action::Exchange);

        // Assassins can Assassinate
        if self.coins >= 3 {
            for i in 0..=self.opponents {
                if i != self.id && !eliminated_players.contains(&i) {
                    actions.push(Action::Assassinate (i));
                }
            }
        }

        // Contessas can't take any actions :(

        actions
    }
}