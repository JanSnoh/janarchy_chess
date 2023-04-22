
//use crate::backend::moves::Field;

use std::{io};
use janarchy_chess::{GameState, pieces};

const DEFAULT_GAME_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut game_state = match GameState::from_fen(DEFAULT_GAME_FEN) {
        Ok(state) => state,
        Err(x) => return println!("ERROR: {}", x),
    };
    //println!("{:#?}",game_state);
    game_state.print(Some(pieces::PieceColor::Black));

    //silly testing of Rust behavior
    //"abcdefg".chars().map(|x| x as usize-'a' as usize).map(|x| println!("{}",x)).for_each(drop);
}


fn game_loop() -> io::Result<String> {
    loop{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

    }
    Ok("yup".to_owned())
}

