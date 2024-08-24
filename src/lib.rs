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
