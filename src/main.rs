//! Main executable for the Contessa Coup Engine.

use contessa::{
    Player,
    Engine,
    ActionUtilities,
};

fn main() {
    train(25);    
}

/// Train 1000 Contessa Engines against one another for a specified number of generations.
fn train(generations: usize) {
    // FIRST GENERATION //
    let mut winners = Vec::new();

    for i in 0..1000 {
        let players = vec![
            Player::new(0, 3),
            Player::new(1, 3),
            Player::new(2, 3),
            Player::new(3, 3),
        ];
        let mut engine = Engine::new(players);
        let mut player = engine.play(false);

        // Reset this player
        player.set_id(0);
        player.clear();

        winners.push(player);

        println!("Generation 0, Game {} complete", i);
    }

    // FOLLOWING GENERATIONS //

    for gen in 0..(generations - 1) {
        // Mutate players and set up new games
        let mut new_winners = Vec::new();

        for (i, player) in winners.iter().enumerate() {
            let players = vec![
                player.clone(),
                player.mutate().with_id(1),
                player.mutate().with_id(2),
                player.mutate().with_id(3),
            ];
            let mut engine = Engine::new(players);
            let mut player = engine.play(false);

            // Reset this player
            player.set_id(0);
            player.clear();

            new_winners.push(player);

            println!("Generation {}, Game {} complete", gen, i);
        }

        winners = new_winners;
    }

    dbg!(&winners);
}