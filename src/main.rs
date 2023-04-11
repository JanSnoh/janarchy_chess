#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
//use crate::backend::moves::Field;

use std::{io, process::ExitCode};
use jkfunctools;
use backend::{ChessError, moves::*};
use backend::pieces::PieceColor;
mod frontend;
mod backend;

const DEFAULT_GAME_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut game_state = match backend::GameState::from_fen(DEFAULT_GAME_FEN){
        Ok(state) => state,
        Err(x) => return println!("ERROR: {}",x),
    };
    //println!("{:#?}",game_state);
    game_state.prnt(Some(PieceColor::Black));

    //silly testing of Rust behavior
    //"abcdefg".chars().map(|x| x as usize-'a' as usize).map(|x| println!("{}",x)).for_each(drop);
}


fn start_game_loop() -> io::Result<String>{
    loop{
        game_loop()?;
    }
}

fn game_loop() -> io::Result<String>{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok("yup".to_owned())
}
        
mod tests{
    use super::*;

    #[allow(unused_variables)]
    #[test]
    fn test_from_fen(){
        let mut game_state = match backend::GameState::from_fen(DEFAULT_GAME_FEN){
            Ok(state) => state,
            Err(_) => panic!("Cannot make GameState from FEN"),
        };
    }

    #[test]
    fn debug_move() -> ExitCode{
        let mut game_state = match backend::GameState::from_fen(DEFAULT_GAME_FEN){
            Ok(state) => state,
            Err(_) => return ExitCode::FAILURE,
        };

        type Field = backend::moves::Field;
        let a = backend::moves::Field(0,0);
        let b = backend::moves::Field(1,3);
        println!("{:#?}",game_state[b]);
        let move_in_question = backend::moves::Move::new(a, b);

        println!("{}",move_in_question.to_str());
        assert!(game_state.apply_move(move_in_question).is_ok());
        println!("after Move: ");
        game_state.prnt(None);
        println!("{:#?}",game_state[b]);
        return ExitCode::SUCCESS;
    }

    #[test]
    fn debug_movegen() -> ExitCode{
        let mut game_state = match backend::GameState::from_fen(DEFAULT_GAME_FEN){
            Ok(state) => state,
            Err(_) => return ExitCode::FAILURE,
        };
        game_state.apply_move(Move::new(Field(0,0), Field(4,4)));
        let start = std::time::Instant::now();
        let generated_moves = game_state.moves_from(Field(4, 4));
        println!("generating moves took {:#?} ", start.elapsed());
        println!("{:#?}", generated_moves);

        ExitCode::SUCCESS
    }
}
