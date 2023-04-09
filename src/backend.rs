//use std::{fmt::Debug};

pub mod pieces;
pub mod moves;
pub mod move_logic; 

use core::fmt;
use std::{ops::{Index, IndexMut}, process::Output, collections::HashSet, array};

use crate::{DEFAULT_GAME_FEN, backend::pieces::PieceColor};
use self::moves::{Move,Field, Castling};



#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct GameState{
    pub fields: [Option<pieces::Piece>;8*8],
    turn_color: pieces::PieceColor,
    castling_options: HashSet<Castling>,//CASTLING
    en_passant_target: Option<Field>,
    fifty_move_rule_counter: u16, //FIFTY MOVE COUNTER
    move_counter: u16,
    legal_moves: Option<[Vec<Move>; 8*8]>
}

#[derive(Debug, Clone)]
pub enum ChessError{
    MoveError(Move),
    LoadError(String),
    OutOfBounds,
    EmptyMoveOrigin
}

impl Default for GameState{
    fn default() -> Self {
        Self { fields: board_from_fen_str(&DEFAULT_GAME_FEN.split(' ').collect::<Vec<&str>>()).unwrap(),
            turn_color: pieces::PieceColor::White, 
            castling_options: HashSet::from([Castling::KingBlack,Castling::KingWhite,Castling::QueenWhite,Castling::QueenBlack]), 
            en_passant_target: None, 
            fifty_move_rule_counter: 0, 
            move_counter: 1,
            legal_moves: None
        }
    }
}


impl GameState{
    ///A move is doable if there is a piece to move, and it doesn't land on an ally square except special moves
    fn move_is_doable(&self, m: &Move) -> bool{
        let (origin,destination) = m.origin_and_destination();
        //exceptions are moves in which a piece can move onto an ally piece
        let exception = false;

        if self[origin].is_none() {return false}
        
        //Both options for checking if destination has ally piece are unstable.
        //if (self[destination].is_some_and(|x| x.color==self[origin].unwrap().color)) {return false}
        //if let Some(destination_piece) = self[destination] && (destination_piece.color==self[origin].unwrap().color) {return false}
        if matches!(self[destination], Some(destination_piece) if destination_piece.color == self[origin].unwrap().color && !exception) {return false}

        return true;
    }
    
    pub fn apply_move(&mut self, move_in_question: Move) -> Result<(), ChessError> {
        let (origin,destination) = move_in_question.origin_and_destination();
        let piece = self[origin].ok_or(ChessError::EmptyMoveOrigin)?;

        if !self.move_is_doable(&move_in_question) {return Err(ChessError::MoveError(move_in_question))}

        self[destination] = self[origin].take();
        self.move_end();
        Ok(())
    }
    
    fn move_end(&mut self){
        self.turn_color = self.turn_color.opposite();
    }
    
    pub fn from_fen(s: &str) -> Result<Self,ChessError>{
        //!Loads a GameState from a Forsyth–Edwards Notation string.
        //! 
        //!example string: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1

        let raw_data = s.split(' ').collect::<Vec<&str>>();
        let field_array = board_from_fen_str(&raw_data)?;
        let whose_turn = if raw_data[1]=="w" {pieces::PieceColor::White} else {pieces::PieceColor::Black};
        let move_num:u16 = raw_data[5].parse().unwrap();

        Ok(GameState{ fields: field_array, turn_color: whose_turn, move_counter: move_num, ..Default::default()})
    }
    
}

fn board_from_fen_str(raw_data: &Vec<&str>) -> Result<[Option<pieces::Piece>; 64], ChessError> {
    let mut field_data = Vec::new();
    let mut i = 0;
    for x in raw_data[0].chars(){
        if x =='/' {continue;}
        if x.is_numeric(){
            for _ in 0..x.to_digit(10).unwrap(){
                field_data.push(pieces::Piece::from_char('1'));
                i+=1;
            }
            continue;
        } 
        field_data.push(pieces::Piece::from_char(x));
        i+=1;
    }
    let field_array:[Option<pieces::Piece>;8*8] = field_data.try_into()
                                                .map_err(|_|ChessError::LoadError(format!("Couldn't load FEN, {i} Squares instead of 64")))?;
    Ok(field_array)
} 


impl Index<Field> for GameState{
    type Output = Option<pieces::Piece>;
    fn index(&self, location: Field) -> &Self::Output{
        let index: usize = location.0+location.1*8;
        &self.fields[index]
    }
}
impl IndexMut<Field> for GameState{
    fn index_mut(&mut self, location: Field) -> &mut Self::Output{
        let index: usize = location.0+location.1*8;
        &mut self.fields[index]
    }
}





impl fmt::Display for ChessError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self{
            ChessError::MoveError(mov) => write!(f, "Move '{}' is invalid!",mov.to_str()),
            ChessError::LoadError(msg) => write!(f, "{}",msg),
            ChessError::OutOfBounds => write!(f, "Field is out of Bounds!"),
            ChessError::EmptyMoveOrigin => write!(f, "Can't move out of an empty Square")
        }
    }
}
