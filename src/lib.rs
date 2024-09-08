#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod utils;
use utils::fen::*;

mod board;
pub use board::{Board, ChessMove, FinishedState, GameState};
pub use utils::{
    fen::{
        CASTLE_KINGSIDE_POSITION, CASTLE_QUEENSIDE_POSITION, CHECK_POSITION, CMK_POSITION,
        KILLER_POSITION, PROMOTION_POSITION, START_POSITION, TRICKY_POSITION,
    },
    Colour,
};

mod tests;

pub fn init() {
    unsafe {
        init_targets();
    }
}
