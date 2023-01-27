//! Main executable for the Contessa Coup Engine.

use contessa::{
    Player,
    Engine,
};

fn main() {
    let mut players = vec![
        Player::new(0, 3),
        Player::new(1, 3),
        Player::new(2, 3),
        Player::new(3, 3),
    ];

    let mut engine = Engine::new(&mut players);

    dbg!(&engine);

    engine.turn();

    dbg!(&engine);
}