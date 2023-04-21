pub use backend::pieces;
pub use backend::{moves::*, ChessError};
mod backend;
mod frontend;

pub use backend::{GameState, moves::{Field,Move}};

pub const DEFAULT_GAME_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fen() {
        GameState::from_fen(DEFAULT_GAME_FEN).unwrap();
    }

    #[test]
    fn debug_move() {
        let mut game_state = GameState::from_fen(DEFAULT_GAME_FEN).unwrap();

        let a = Field(0, 0);
        let b = Field(1, 3);
        println!("{:#?}", game_state[b]);
        let move_in_question = Move::from_squares(a, b);

        println!("{}", move_in_question.to_str());
        assert!(game_state.apply_move(move_in_question).is_ok());
        println!("after Move: ");
        game_state.print(None);
        println!("{:#?}", game_state[b]);
    }

    #[test]
    fn debug_movegen() {
        let mut game_state = GameState::from_fen(DEFAULT_GAME_FEN).unwrap();
        game_state.apply_move(Move::from_squares(Field(0, 0), Field(4, 4))).unwrap();
        game_state.print(None);
        let generated_moves = game_state.moves_from(Field(4, 4));
        println!("{:#?}", generated_moves);
    }

    #[test]
    fn all_possible_moves(){
        let game_state = GameState::from_fen(DEFAULT_GAME_FEN).unwrap();

        game_state.print(None);
        let generated_moves = game_state.possible_moves(pieces::PieceColor::White);
        println!("{:#?}", generated_moves);
    }
}
