#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
//use crate::backend::moves::Field;

use std::{io, process::ExitCode};
use jkfunctools;
use backend::ChessError;
use backend::pieces::PieceColor;
mod frontend;
mod backend;

const DEFAULT_GAME_FEN: &str = "rnbqkbnr/pppppppp/8/1R6/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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
