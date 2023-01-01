#[derive(Debug, Clone, Copy)]
pub enum LudoPiece {
    Prison,
    Out(u16),
    HomeStretch(u16),
    Home,
}

use LudoPiece::*;

#[derive(Debug, Clone)]
pub struct LudoState {
    pub pieces: [[LudoPiece; 4]; 4],
    pub turn: u8,
}

impl LudoState {
    pub fn new() -> Self {
        Self {
            pieces: [[Prison; 4]; 4],
            turn: 0,
        }
    }
}
