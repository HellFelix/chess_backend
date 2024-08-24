#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use env_logger;
use std::time::SystemTime;

mod utils;
use utils::fen::*;

mod board;
pub use board::Board;

mod tests;

fn main() {
    env_logger::init();
    unsafe {
        init_targets();
    }
    let board = Board::from(START_POSITION);
    println!("{board}");
    let start = SystemTime::now();
    board.generate_legal_moves();
    let stop = start.elapsed();
    println!("Generating moves took {} us", stop.unwrap().as_micros());
    // unsafe {
    //     let mut v = vec![a2, b2, c2, d2, e2, f2, g2, h2];
    //     let b = squares_to_bitboard(v.as_mut_ptr(), v.len() as i32);
    //     println!("{}", BitBoard(b));
    // }

    // let mut bitboard = BitBoard::from_ranks([
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 1, 1, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 0],
    // ]);
    // let bitboard_map = piece_map_bitboards {
    //     pawns: 0,
    //     king: 2,
    //     queens: 0,
    //     bishops: 0,
    //     knights: 0,
    //     rooks: 0,
    // };

    // unsafe {
    //     init_targets();
    // }
    // println!("{bitboard}");
    // println!("---------------------------");
    // let start = SystemTime::now();
    // unsafe {
    //     init_targets();
    //     let start = SystemTime::now();
    //     let attacks = pawnTargets(d7, BLACK, bitboard.0);
    //     println!("Generated targets in {}us", start.elapsed().unwrap().as_micros());
    //     println!("{}", BitBoard(attacks));
    // }
    // println!("{}", start.elapsed().unwrap().as_micros());

    // let mut n = 0;
    // for number in 1..=8 {
    //     for letter in vec!["a", "b", "c", "d", "e", "f", "g", "h"] {
    //         println!("const {letter}{number}: i32 = {n};");
    //         n += 1;
    //     }
    // }
}
