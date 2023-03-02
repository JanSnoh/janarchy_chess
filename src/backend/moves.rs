#![allow(dead_code)]


use std::fmt::{self, Debug};
use std::fmt::Write as FmtWrite;

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',];


#[derive(Copy, Clone, Default, PartialEq)]
pub struct Field(pub usize, pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Castling{KingBlack,QueenBlack,KingWhite,QueenWhite}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Move{pub origin: Field, pub target: Field, hostile:bool, castle:Option<Castling>, knight_boost:bool}

#[test]
fn test_move_compare(){
    let move_a = Move::new(Field(2, 2), Field(4, 4));
    let move_b = Move::new(Field(2, 3), Field(4, 4));
    
    assert!(move_a==move_b);
    assert_eq!(move_a,move_b);
}


impl Move{
    pub fn new(origin: Field, target: Field) -> Self { Self { origin, target, ..Default::default() } }
    pub fn from_str(input:String) -> Option<Move>{
        todo!("test{:?}",input)
    }
    pub fn to_str(&self) -> String{
        format!("From {:#?} To {:#?}",  self.origin, self.target)
    }
    
    pub fn origin_and_destination(&self) -> (Field,Field){
        (self.origin, self.target)
    }
}

impl fmt::Debug for Field{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Square {}{}", ASCII_UPPER[(self.0) as usize], 8-self.1)
    }
}

impl Field{
    pub fn add_vec(&self, other:(u8,u8)) -> Self{
        Field(usize::try_from(self.0 as u8 +other.0), (self.1 as u8 +other.1) as usize)
    }
}