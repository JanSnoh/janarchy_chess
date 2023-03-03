use std::array;
use crate::{DEFAULT_GAME_FEN, backend::{GameState, pieces::{Piece,PieceColor,PieceType},moves::{Field,Move},ChessError}};


impl GameState{
    pub fn moves_from_square(&self, origin: Field) -> Result<Vec<Move>,ChessError>{
        let moves = Vec::new();
        let movee = self[origin].ok_or(ChessError::EmptyMoveOrigin)?;

        pub(crate) fn add_moves_in_direction(game_state: &GameState, origin: Field, dir: (u8,u8), own_color: PieceColor) -> Vec<Move>{
            let mut direction_moves = Vec::new();
            //let next_sq: Field = origin.add_vec(dir);
            let mut step = dir;
            while let Ok(next_sq) = origin.add_vec(step){
                step=(step.0*2,step.1*2);
                direction_moves.push(Move::new(origin, next_sq));
                match game_state[next_sq] {
                    Some(Piece{color, ..}) if color == own_color => {break;},
                    Some(Piece{..}) => {
                        direction_moves.push(Move::new(origin, next_sq)); 
                        break;},
                    None => direction_moves.push(Move::new(origin, next_sq)),
                }
            }
            direction_moves
        }


        match movee.piece_type{
            PieceType::King => todo!(),
            PieceType::Queen => todo!(),
            PieceType::Bishop => todo!(),
            PieceType::Knight => todo!(),
            PieceType::Rook => todo!(),
            PieceType::Pawn => todo!(),
            PieceType::Empty => panic!(),
        }
    
        todo!("test{:?}",origin);
        Ok(moves)
    }

    pub fn legal_moves(&mut self) -> [Vec<Move>; 64] {
        array::from_fn(|i| self.moves_from_square(Field(i%8, i/8)).unwrap_or(Vec::new()))
    }
}
