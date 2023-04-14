#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
//use crate::backend::moves::Field;

use backend::pieces::PieceColor;
use backend::{moves::*, ChessError};
use jkfunctools;
use std::{io, process::ExitCode};
mod backend;
mod frontend;

pub use backend::{GameState, moves::{Field,Move}};

const DEFAULT_GAME_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut game_state = match backend::GameState::from_fen(DEFAULT_GAME_FEN) {
        Ok(state) => state,
        Err(x) => return println!("ERROR: {}", x),
    };
    //println!("{:#?}",game_state);
    game_state.prnt(Some(PieceColor::Black));

    //silly testing of Rust behavior
    //"abcdefg".chars().map(|x| x as usize-'a' as usize).map(|x| println!("{}",x)).for_each(drop);
}

fn start_game_loop() -> io::Result<String> {
    loop {
        game_loop()?;
    }
}

fn game_loop() -> io::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok("yup".to_owned())
}

mod tests {
    use super::*;

    #[allow(unused_variables)]
    #[test]
    fn test_from_fen() {
        let mut game_state = backend::GameState::from_fen(DEFAULT_GAME_FEN).unwrap();
    }

    #[test]
    fn debug_move() {
        let mut game_state = backend::GameState::from_fen(DEFAULT_GAME_FEN).unwrap();

        let a = Field(0, 0);
        let b = Field(1, 3);
        println!("{:#?}", game_state[b]);
        let move_in_question = Move::from_squares(a, b);

        println!("{}", move_in_question.to_str());
        assert!(game_state.apply_move(move_in_question).is_ok());
        println!("after Move: ");
        game_state.prnt(None);
        println!("{:#?}", game_state[b]);
    }

    #[test]
    fn debug_movegen() {
        let mut game_state = backend::GameState::from_fen(DEFAULT_GAME_FEN).unwrap();
        game_state.apply_move(Move::from_squares(Field(0, 0), Field(4, 4))).unwrap();
        game_state.prnt(None);
        let start = std::time::Instant::now();
        let generated_moves = game_state.moves_from(Field(4, 4));
        println!("generating moves took {:#?} ", start.elapsed());
        println!("{:#?}", generated_moves);
    }

    #[test]
    fn all_possible_moves(){
        let mut game_state = backend::GameState::from_fen(DEFAULT_GAME_FEN).unwrap();

        game_state.prnt(None);
        let generated_moves = game_state.possible_moves(PieceColor::White);
        println!("{:#?}", generated_moves);
    }
}
