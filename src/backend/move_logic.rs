use std::{array, slice::Iter, alloc::System};
use crate::{DEFAULT_GAME_FEN, backend::{GameState, pieces::{Piece,PieceColor,PieceType,},moves::{Field,Move},ChessError}};



impl GameState{
    pub fn moves_from_square(&self, origin: Field) -> Result<Vec<Move>,ChessError>{
        let mut moves = Vec::new();
        let movee = self[origin].ok_or(ChessError::EmptyMoveOrigin)?;

        type Dir = (i8,i8);
        type DirIter<'a> = Iter<'a,Dir>;
        

        fn move_if_doable(game_state: &GameState, origin: Field, dir:&Dir) -> Option<Move>{
            let target = origin.add_vec(*dir).ok()?;
            let res = Move::new(origin, target);
            if game_state.move_is_doable(&res) {
                Some(res)
            } else {
                None
            }
        } 

        let mut add_dirs = move |dirs: DirIter, moves: &mut Vec<Move>| dirs.
            for_each(|dir| moves.append(
            &mut moves_in_direction(self, origin, *dir, movee.color)));


        //remove when done with copying to pieces/behaviors
        match movee.piece_type{
            //directional moves
            PieceType::Bishop => add_dirs(DIAGONALS.iter(), &mut moves),
            PieceType::Rook => add_dirs(HORIZONTALS.iter(), &mut moves),
            PieceType::Queen => add_dirs(ALL_DIRS.iter(),&mut moves),
            PieceType::Empty => panic!("deprecated"),
            //"special" moves
            PieceType::King => moves.append(&mut ALL_DIRS.iter().map(|dir|move_if_doable(self, origin, dir)).
                filter_map(|x| x).collect()),
            PieceType::Knight => todo!(),
            PieceType::Pawn => todo!(),
            
        }
        


        Ok(moves)
    }

    pub fn legal_moves(&mut self) -> [Vec<Move>; 64] {
        
        array::from_fn(|i| self.moves_from_square(Field(i%8, i/8)).unwrap_or(Vec::new()))
    }
}

