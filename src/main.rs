//! Main executable for the Contessa Coup Engine.

use contessa::Engine;

fn main() {
    let engine = Engine::new(4);

    dbg!(&engine.get_deck());
    dbg!(&engine.get_hands());
}