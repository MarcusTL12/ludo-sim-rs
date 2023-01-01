pub mod ludo_state;
pub mod strategy;

use ludo_state::LudoState;

fn main() {
    let state = LudoState::new();

    println!("{state:?}");
}
