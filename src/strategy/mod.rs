use crate::ludo_state::LudoState;

pub trait Strategy {
    fn do_move(&self, state: &mut LudoState);
}
