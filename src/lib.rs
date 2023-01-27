//! Main library for the Contessa Coup Engine.

mod engine;

use std::collections::HashMap;

use rand::random;

pub use engine::{
    Engine,
};

/// Enumerates the cards available in the game.
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Card {
    Duke,
    Captain,
    Ambassador,
    Assassin,
    Contessa,
    None,
}

/// Enumerates the actions available in the game.
#[derive(Debug)]
pub enum Action {
    Income,

    ForeignAid,

    /// When a player chooses to `Coup`, he must choose a target.
    Coup (usize),

    Tax,

    /// When a player chooses to `Assassinate`, he must choose a target.
    Assassinate (usize),

    Exchange,

    /// When a player chooses to `Steal`, he must choose a target.
    Steal (usize),

    /// Note that a player can only `Pass` if he has been eliminated (has zero influence).
    Pass,
}

/// Holds a perceived hand.
#[derive(Debug)]
pub struct PerceivedHand {
    hand: HashMap<Card, f64>,
}

/// Implements commonly used functions performed on perceived hands.
impl PerceivedHand {
    /// Constructs a new perceived hand.
    pub fn new() -> Self {
        let hand = HashMap::from([
            (Card::Duke, 0.0),
            (Card::Captain, 0.0),
            (Card::Ambassador, 0.0),
            (Card::Assassin, 0.0),
            (Card::Contessa, 0.0),
        ]);

        Self {
            hand,
        }
    }

    /// Constructs a new perceived hand from a given hashmap.
    pub fn from(hand: HashMap<Card, f64>) -> Self {
        Self {
            hand,
        }
    }

    /// Gets a probability from the hand.
    pub fn get(&self, card: &Card) -> Option<&f64> {
        self.hand.get(card)
    }

    /// Assigns a given hand (if it is known with certainty).
    pub fn assign(&mut self, cards: [Card; 2]) {
        if cards[0] != cards[1] {
            self.hand.insert(cards[0], 1.0);
            self.hand.insert(cards[1], 1.0);
        } else {
            self.hand.insert(cards[0], 2.0);
        }
    }
}

/// Holds the information and performs the actions of a player.
#[derive(Debug)]
pub struct Player {
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

    /// Stores the number of opponents this player is playing.
    opponents: usize,

    /// Holds the perceived hands of the other players.
    perceived_hands: Vec<PerceivedHand>,
}

/// Implements necessary behaviors of a player.
impl Player {
    /// Generates a new (random) player.
    pub fn new(id: usize, opponents: usize) -> Self {
        Self {
            id,
            hand: [Card::None, Card::None],
            coins: 2,
            liar_cutoff: random(),
            lying_cutoff: random(),
            opponents,
            perceived_hands: Vec::new(),
        }
    }

    /// Creates a new player with specified cutoffs.
    pub fn with_specified_cutoffs(id: usize, opponents: usize, liar_cutoff: f64, lying_cutoff: f64) -> Self {
        Self {
            id,
            hand: [Card::None, Card::None],
            coins: 2,
            liar_cutoff,
            lying_cutoff,
            opponents,
            perceived_hands: Vec::new(),
        }
    }

    /// "Mutates" the current player by slightly modifying the cutoff probabilities.
    pub fn mutate(&self, id: usize) -> Self {
        Self {
            id,
            hand: [Card::None, Card::None],
            coins: 2,
            liar_cutoff: self.liar_cutoff + 0.01 * (2.0*random::<f64>() - 1.0),
            lying_cutoff: self.lying_cutoff + 0.01 * (2.0*random::<f64>() - 1.0),
            opponents: self.opponents,
            perceived_hands: Vec::new(),
        }
    }

    /// Computes *a priori* probabilities of each player having certain cards.
    pub fn compute_hands(&mut self, killed: &[Card]) {
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
        let current = match counts.get(&self.hand[0]) {
            Some(v) => v,
            None => todo!(),
        };
        counts.insert(self.hand[0], current - 1.0);

        let current = match counts.get(&self.hand[1]) {
            Some(v) => v,
            None => todo!(),
        };
        counts.insert(self.hand[1], current - 1.0);

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
    pub fn deal(&mut self, hand: [Card; 2]) {
        self.hand = hand;
    }

    /// Gains the number of coins specified.
    pub fn gain_coins(&mut self, coins: u8) {
        self.coins += coins;
    }

    /// Loses the number of coins specified.
    pub fn lose_coins(&mut self, coins: u8) {
        self.coins -= coins;
    }

    /// Forces the player to lose one influence.
    /// 
    /// Right now, this is a random selection.  It will be trained later, probably using
    /// reinforcement learning or regret minimization.
    pub fn lose_influence(&mut self) -> Card {
        let lost = if self.hand[0] == Card::None {
            1
        } else if self.hand[1] == Card::None {
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
    /// Right now, this is based on a trained "liar" threshold.  This may
    /// change in the future.
    pub fn check_challenge(&self, active_player: usize, card: Card) -> bool {
        // Note: it's OK to use `Option::unwrap` here because we know we're providing
        // one of the five game cards (`Card::None` will never be passed) and we know
        // for sure that each of these cards are in each of our perceived hands
        self.perceived_hands[active_player].get(&card).unwrap() < &self.liar_cutoff
    }

    /// Selects the list of actions available to the player.
    pub fn get_available_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        // If this player has 10 coins or more, he must Coup.
        if self.coins >= 10 {
            for i in 0..=self.opponents {
                if i != self.id {
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
                if i != self.id {
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
                if i != self.id {
                    actions.push(Action::Steal (i));
                }
            }
        }

        // Ambassadors can Exchange
        if self.hand.contains(&Card::Ambassador) {
            actions.push(Action::Exchange);
        }

        // Assassins can Assassinate
        if self.hand.contains(&Card::Assassin) {
            for i in 0..=self.opponents {
                if i != self.id {
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
                if i != self.id {
                    actions.push(Action::Steal (i));
                }
            }
        }

        // Ambassadors can Exchange
        if !self.hand.contains(&Card::Ambassador) && self.perceived_hands[self.id].get(&Card::Ambassador).unwrap() > &self.lying_cutoff {
            actions.push(Action::Exchange);
        }

        // Assassins can Assassinate
        if !self.hand.contains(&Card::Assassin) && self.perceived_hands[self.id].get(&Card::Assassin).unwrap() > &self.lying_cutoff {
            for i in 0..=self.opponents {
                if i != self.id {
                    actions.push(Action::Assassinate (i));
                }
            }
        }

        // Contessas can't take any actions :(

        actions
    }
}