//! Implements functions for the `Player` trait from a computer's perspective.

use std::collections::HashMap;

use crate::{
    Card,
    Action,
    Player,
    PerceivedHand,
    ActionUtilities,
    PlayerMetadata,
};

use rand::{
    random,
};

/// Holds the information and performs the actions of a player.
#[derive(Clone, Debug)]
pub struct Agent {
    /// Stores this player's ID.
    id: usize,

    /// Holds the player's hand.
    hand: [Card; 2],

    /// Provides the number of coins the player has.
    coins: u8,

    /// Establishes the cutoff probability for calling out a potential liar.
    liar_cutoff: f64,

    /// Establishes the perceived cutoff probability for lying.
    lying_cutoff: f64,

    /// Stores the utilities for each action.
    utilities: ActionUtilities,

    /// Stores the number of opponents this player is playing.
    opponents: usize,

    /// Holds the perceived hands of the other players.
    perceived_hands: Vec<PerceivedHand>,
}

impl Player for Agent {
    /// Returns the metadata of this player.
    fn get_metadata(&self) -> PlayerMetadata {
        PlayerMetadata::Computer {
            lying_cutoff: self.lying_cutoff,
            liar_cutoff: self.liar_cutoff,
            utilities: self.utilities,
        }
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
        if self.hand[0] == current {
            self.hand[0] = new;
        } else {
            self.hand[1] = new;
        }
    }

    /// Exchange cards (used by `Ambassador`).
    /// 
    /// Right now, this discards the two cards drawn.  This will
    /// be changed to be more strategic in the future.
    fn exchange(&mut self, cards: &[Card]) -> Vec<Card> {
        cards.to_vec()
    }

    /// Gets the number of coins this player has.
    fn get_coins(&self) -> u8 {
        self.coins
    }

    /// Computes *a priori* probabilities of each player having certain cards.
    fn compute_hands(&mut self, killed: &[Card]) {
        let mut hands = Vec::new();

        // Count cards

        // This count is private (based on private information)
        let mut available: f64 = 15.0;
        let mut counts = HashMap::<Card, f64>::from([
            (Card::Duke, 3.0),
            (Card::Captain, 3.0),
            (Card::Ambassador, 3.0),
            (Card::Assassin, 3.0),
            (Card::Contessa, 3.0),
        ]);

        // This count is public (based on public information)
        let mut public_available: f64 = 15.0;
        let mut public_counts = HashMap::<Card, f64>::from([
            (Card::Duke, 3.0),
            (Card::Captain, 3.0),
            (Card::Ambassador, 3.0),
            (Card::Assassin, 3.0),
            (Card::Contessa, 3.0),
        ]);

        // Remove any "dead" cards
        for card in killed {
            let current: f64 = match counts.get(&card) {
                Some(v) => *v,
                None => todo!(),
            };
            counts.insert(*card, current - 1.0);
            public_counts.insert(*card, current - 1.0);

            available -= 1.0;
            public_available -= 1.0;
        }

        // Remove your current two cards
        if let Some(count) = counts.get(&self.hand[0]) {
            counts.insert(self.hand[0], count - 1.0);
        }

        if let Some(count) = counts.get(&self.hand[1]) {
            counts.insert(self.hand[1], count - 1.0);
        }

        available -= 2.0;

        // Find the *a priori* probabilities for other people's hands
        // Note: it's OK to use `Option::unwrap` here because we know we inserted each of these
        // cards into the hashmap `counts` above
        let a_priori = HashMap::from([
            (Card::Duke, 1.0 - ((available - counts.get(&Card::Duke).unwrap())/available)*(available - 1.0 - counts.get(&Card::Duke).unwrap())/(available - 1.0)),
            (Card::Captain, 1.0 - ((available - counts.get(&Card::Captain).unwrap())/available)*(available - 1.0 - counts.get(&Card::Captain).unwrap())/(available - 1.0)),
            (Card::Ambassador, 1.0 - ((available - counts.get(&Card::Ambassador).unwrap())/available)*(available - 1.0 - counts.get(&Card::Ambassador).unwrap())/(available - 1.0)),
            (Card::Assassin, 1.0 - ((available - counts.get(&Card::Assassin).unwrap())/available)*(available - 1.0 - counts.get(&Card::Assassin).unwrap())/(available - 1.0)),
            (Card::Contessa, 1.0 - ((available - counts.get(&Card::Contessa).unwrap())/available)*(available - 1.0 - counts.get(&Card::Contessa).unwrap())/(available - 1.0)),
        ]);

        // Find the *a priori* probabilities for my hand
        // Note: it's OK to use `Option::unwrap` here because we know we inserted each of these
        // cards into the hashmap `counts` above
        let my_hand = HashMap::from([
            (Card::Duke, 1.0 - ((public_available - public_counts.get(&Card::Duke).unwrap())/public_available)*(public_available - 1.0 - public_counts.get(&Card::Duke).unwrap())/(public_available - 1.0)),
            (Card::Captain, 1.0 - ((public_available - public_counts.get(&Card::Captain).unwrap())/public_available)*(public_available - 1.0 - public_counts.get(&Card::Captain).unwrap())/(public_available - 1.0)),
            (Card::Ambassador, 1.0 - ((public_available - public_counts.get(&Card::Ambassador).unwrap())/public_available)*(public_available - 1.0 - public_counts.get(&Card::Ambassador).unwrap())/(public_available - 1.0)),
            (Card::Assassin, 1.0 - ((public_available - public_counts.get(&Card::Assassin).unwrap())/public_available)*(public_available - 1.0 - public_counts.get(&Card::Assassin).unwrap())/(public_available - 1.0)),
            (Card::Contessa, 1.0 - ((public_available - public_counts.get(&Card::Contessa).unwrap())/public_available)*(public_available - 1.0 - public_counts.get(&Card::Contessa).unwrap())/(public_available - 1.0)),
        ]);
        
        // Fill the list of perceived hands
        for i in 0..=self.opponents {
            let hand = if i == self.id {
                PerceivedHand::from(my_hand.clone())
            } else {
                PerceivedHand::from(a_priori.clone())
            };

            hands.push(hand);
        }

        self.perceived_hands = hands;
    }

    /// Deals the given cards to the player.
    fn deal(&mut self, hand: [Card; 2]) {
        self.hand = hand;
    }

    /// Gains the number of coins specified.
    fn gain_coins(&mut self, coins: u8) {
        self.coins += coins;
    }

    /// Loses the number of coins specified.
    fn lose_coins(&mut self, coins: u8) -> u8 {
        if self.coins >= coins {
            self.coins -= coins;
            coins
        } else {
            let stolen = self.coins;
            self.coins = 0;
            stolen
        }
    }

    /// Forces the player to lose one influence.
    /// 
    /// Right now, this is a random selection.  It will be trained later, probably using
    /// reinforcement learning or regret minimization.
    fn lose_influence(&mut self) -> Card {
        let lost = if self.hand[0] == Card::None && self.hand[1] != Card::None {
            1
        } else if self.hand[1] == Card::None && self.hand[0] != Card::None {
            0
        } else if random() {
            1
        } else {
            0
        };

        let card = self.hand[lost];
        self.hand[lost] = Card::None;

        card
    }

    /// Asks this player if he challenges a claim.
    /// 
    /// Returns `true` if the player challenges and `false` otherwise.
    /// 
    /// Right now, this is based on a trained "liar" threshold.  This may
    /// change in the future.
    fn check_challenge(&self, active_player: usize, card: Card) -> bool {
        if card == Card::None {
            // You can't challenge Income, ForeignAid, or Coup
            return false;
        }

        if self.is_eliminated() {
            // You can't challenge if you're out
            return false;
        }

        // Note: it's OK to use `Option::unwrap` here because we know we're providing
        // one of the five game cards (we just checked `Card::None`) and we know
        // for sure that each of these cards are in each of our perceived hands
        self.perceived_hands[active_player].get(&card).unwrap() < &self.liar_cutoff
    }

    /// Asks this player if he blocks an action.
    /// 
    /// Returns `true` if the player blocks and `false` otherwise.  Also returns
    /// the card with which the player blocks.
    /// 
    /// Right now, this is based on a trained "lying" threshold.  This may
    /// change in the future.
    /// 
    /// Right now, this player is "perfectly selfish": it will only block actions
    /// against it.  This may change in the future.
    fn check_block(&self, action: Action) -> (bool, Card) {
        if self.is_eliminated() {
            // You can't block if you're out
            return (false, Card::None);
        }

        // Note: it's OK to use `Option::unwrap` here because we know we're providing one
        // of the five game cards (we are sure we haven't passed `Card::None`) and we know
        // for sure that each of these cards are in each of our perceived hands
        match action {
            Action::ForeignAid => (self.perceived_hands[self.id].get(&Card::Duke).unwrap() > &self.lying_cutoff, Card::Duke),
            Action::Assassinate (i) => (i == self.id && self.perceived_hands[self.id].get(&Card::Contessa).unwrap() > &self.lying_cutoff, Card::Duke),
            Action::Steal (i) => {
                // For now, only block actions against yourself
                if i != self.id {
                    return (false, Card::None);
                }

                let captain = self.perceived_hands[self.id].get(&Card::Captain).unwrap();
                let ambassador = self.perceived_hands[self.id].get(&Card::Ambassador).unwrap();

                if captain > ambassador {
                    return (captain > &self.lying_cutoff, Card::Captain);
                } else {
                    return (ambassador > &self.lying_cutoff, Card::Ambassador);
                }
            },
            _ => (false, Card::None),
        }
    }

    /// Checks whether or not a player is still in the game.
    fn is_eliminated(&self) -> bool {
        self.hand == [Card::None, Card::None]
    }

    /// Select an action based on actions available.
    fn select_action(&self, eliminated_players: &[usize]) -> Action {
        let actions = self.get_available_actions(eliminated_players);

        // Compute the utility of each action
        let mut utilities = actions.iter()
            .map(|a| (*a, self.compute_utility(*a)))
            .collect::<Vec<(Action, f64)>>();
        
        // Note: it's OK to use `Result::unwrap` here because we know
        // we are passing a valid `f64` from our utility table
        utilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        match utilities.iter().nth(0) {
            Some(action) => action.0,
            None => Action::Pass,
        }
    }
}

/// Implements necessary behaviors of a player.
impl Agent {
    /// Generates a new (random) player.
    pub fn new(id: usize, opponents: usize) -> Self {
        Self {
            id,
            hand: [Card::None, Card::None],
            coins: 2,
            liar_cutoff: random(),
            lying_cutoff: random(),
            utilities: ActionUtilities::random(),
            opponents,
            perceived_hands: Vec::new(),
        }
    }

    /// Creates a new player with specified patameters.
    pub fn from_metadata(id: usize, opponents: usize, metadata: PlayerMetadata) -> Self {
        if let PlayerMetadata::Computer {
            liar_cutoff,
            lying_cutoff,
            utilities,
        } = metadata {
            Self {
                id,
                hand: [Card::None, Card::None],
                coins: 2,
                liar_cutoff,
                lying_cutoff,
                utilities,
                opponents,
                perceived_hands: Vec::new(),
            }
        } else {
            unreachable!();
        }
    }

    /// Sets the ID of this player.
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    /// Consumes this player and returns a new one with the specified ID.
    pub fn with_id(self, id: usize) -> Self {
        Self {
            id,
            hand: self.hand,
            coins: self.coins,
            liar_cutoff: self.liar_cutoff,
            lying_cutoff: self.lying_cutoff,
            utilities: self.utilities,
            opponents: self.opponents,
            perceived_hands: self.perceived_hands,
        }
    }

    /// "Mutates" this player by slightly modifying the cutoff probabilities.
    pub fn mutate(&self) -> Self {
        Self {
            id: self.id,
            hand: [Card::None, Card::None],
            coins: 2,
            liar_cutoff: self.liar_cutoff + 0.01 * (2.0*random::<f64>() - 1.0),
            lying_cutoff: self.lying_cutoff + 0.01 * (2.0*random::<f64>() - 1.0),
            utilities: self.utilities.mutate(),
            opponents: self.opponents,
            perceived_hands: Vec::new(),
        }
    }

    /// Prepares this player for the next game.
    pub fn clear(&mut self) {
        self.hand = [Card::None, Card::None];
        self.coins = 2;
        self.perceived_hands = Vec::new();
    }

    /// Selects the list of actions available to the player.
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
        if self.hand.contains(&Card::Duke) {
            actions.push(Action::Tax);
        }

        // Captains can Steal
        if self.hand.contains(&Card::Captain) {
            for i in 0..=self.opponents {
                if i != self.id && !eliminated_players.contains(&i) {
                    actions.push(Action::Steal (i));
                }
            }
        }

        // Ambassadors can Exchange
        if self.hand.contains(&Card::Ambassador) {
            actions.push(Action::Exchange);
        }

        // Assassins can Assassinate
        if self.hand.contains(&Card::Assassin) && self.coins >= 3 {
            for i in 0..=self.opponents {
                if i != self.id && !eliminated_players.contains(&i) {
                    actions.push(Action::Assassinate (i));
                }
            }
        }

        // Contessas can't take any actions :(

        // Now we get to the lying: "dangerous" actions
        // Which actions can I take that are deceptive but will not injure me seriously?

        // Note: it's OK to use `Option::unwrap` here because we know we put each of these cards into the PerceivedHand hashmap
        // earlier in the program

        // Dukes can Tax
        if !self.hand.contains(&Card::Duke) && self.perceived_hands[self.id].get(&Card::Duke).unwrap() > &self.lying_cutoff {
            actions.push(Action::Tax);
        }

        // Captains can Steal
        if !self.hand.contains(&Card::Captain) && self.perceived_hands[self.id].get(&Card::Captain).unwrap() > &self.lying_cutoff {
            for i in 0..=self.opponents {
                if i != self.id && !eliminated_players.contains(&i) {
                    actions.push(Action::Steal (i));
                }
            }
        }

        // Ambassadors can Exchange
        if !self.hand.contains(&Card::Ambassador) && self.perceived_hands[self.id].get(&Card::Ambassador).unwrap() > &self.lying_cutoff {
            actions.push(Action::Exchange);
        }

        // Assassins can Assassinate
        if !self.hand.contains(&Card::Assassin) && self.perceived_hands[self.id].get(&Card::Assassin).unwrap() > &self.lying_cutoff && self.coins >= 3 {
            for i in 0..=self.opponents {
                if i != self.id && !eliminated_players.contains(&i) {
                    actions.push(Action::Assassinate (i));
                }
            }
        }

        // Contessas can't take any actions :(

        actions
    }

    /// Computes the utility of a given action according to a utility table.
    fn compute_utility(&self, action: Action) -> f64 {
        let mut utility = match action {
            Action::Income => self.utilities.income,
            Action::ForeignAid => self.utilities.foreignaid,
            Action::Coup (_) => self.utilities.coup,
            Action::Tax => self.utilities.tax,
            Action::Assassinate (_) => self.utilities.assassinate,
            Action::Exchange => self.utilities.exchange,
            Action::Steal (_) => self.utilities.steal,
            Action::Pass => 0.0,
        };

        if action == Action::ForeignAid {
            for (i, hand) in self.perceived_hands.iter().enumerate() {
                // Somebody probably has a duke
                // Note: it's OK to use `Option::unwrap` here because
                // we know we put `Card::Duke` in there earlier
                if i != self.id && hand.get(&Card::Duke).unwrap() > &self.liar_cutoff {
                    utility = 0.0;
                }
            }
        } else if let Action::Steal (target) = action {
            // Note: it's OK to use `Option::unwrap` here because
            // we know we put `Card::Captain` and `Card::Ambassador` in there earlier
            if self.perceived_hands[target].get(&Card::Captain).unwrap() > &self.liar_cutoff
            || self.perceived_hands[target].get(&Card::Ambassador).unwrap() > &self.liar_cutoff {
                utility = 0.0;
            }
        }

        utility
    }
}