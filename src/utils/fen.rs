use super::{Colour, Pieces};
use crate::{board::Board, castling_rights, createBase, piece_map_bitboards, utils::squares};
use core::panic;
use std::convert::From;

pub const EMPTY_BOARD: &str = "8/8/8/8/8/8/8/8 w - - ";
pub const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
pub const CASTLE_KINGSIDE_POSITION: &str =
    "rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1 ";
pub const CASTLE_QUEENSIDE_POSITION: &str = "r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/R3KBNR w KQkq - 0 1 ";

pub const PROMOTION_POSITION: &str = "4k3/1P6/2P5/8/8/8/8/4K3 w - - 0 1";

pub const CHECK_POSITION: &str = "4k3/1P6/2P5/8/8/8/5p2/4K3 w - - 0 1";

pub const TRICKY_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
pub const KILLER_POSITION: &str =
    "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
pub const CMK_POSITION: &str =
    "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ";

/// Create a chess board instance from fen
impl From<&str> for Board {
    fn from(value: &str) -> Self {
        let s = value.split_at(value.find(" ").unwrap());
        let ranks = s.0.split("/");
        let mut args = s.1.split(" ");

        let mut white = Pieces::default();
        let mut black = Pieces::default();
        let mut i: i32 = 63;
        for rank in ranks {
            for c in rank.chars().rev() {
                if let Ok(n) = c.to_string().parse::<i32>() {
                    i -= n;
                } else {
                    match c {
                        'p' => black.pawns.push(i),
                        'P' => white.pawns.push(i),
                        'k' => black.king.push(i),
                        'K' => white.king.push(i),
                        'q' => black.queens.push(i),
                        'Q' => white.queens.push(i),
                        'b' => black.bishops.push(i),
                        'B' => white.bishops.push(i),
                        'n' => black.knights.push(i),
                        'N' => white.knights.push(i),
                        'r' => black.rooks.push(i),
                        'R' => white.rooks.push(i),
                        _ => panic!("Invalid symbol '{c}'"),
                    };
                    i -= 1;
                }
            }
        }

        args.next();
        let side_to_move = match args.next().unwrap() {
            "w" => Colour::White,
            "b" => Colour::Black,
            _ => panic!("Invalid side to move"),
        };

        let castling_rights = castling_rights::from(args.next().unwrap());
        let killer_square = squares::from_str(args.next().unwrap());
        let halfmove = args.next().unwrap().parse::<i32>().unwrap();
        let fullmove = args.next().unwrap().parse::<i32>().unwrap();

        unsafe {
            let base = createBase(
                piece_map_bitboards::from(&mut white),
                piece_map_bitboards::from(&mut black),
            );
            Self::new(
                base,
                killer_square,
                castling_rights,
                side_to_move,
                halfmove,
                fullmove,
            )
        }
    }
}

impl From<&str> for castling_rights {
    fn from(value: &str) -> Self {
        let white_king = value.contains("K");
        let white_queen = value.contains("Q");
        let black_king = value.contains("k");
        let black_queen = value.contains("q");

        Self {
            white_king,
            white_queen,
            black_king,
            black_queen,
        }
    }
}
