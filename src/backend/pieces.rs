use std::{alloc::System, time::SystemTime};

pub use behaviors::{moves_in_direction, Behavior};

#[allow(dead_code)]
#[derive(Debug, PartialEq,Clone,Copy)]
pub enum PieceColor{
    Black, White,
}
#[derive(Debug,Clone,Copy)]
pub enum PieceType{
    King, Queen, Bishop, Knight, Rook, Pawn, Empty,
}
#[derive(Debug,Clone, Copy)]
pub struct Piece{
    pub piece_type: PieceType,
    pub color: PieceColor
}


#[allow(unused_variables)]
impl Piece{
    pub fn from_char(c:char) -> Option<Piece> {
        //Black is lowercase, White is uppercase
        let color = if c.is_lowercase() {PieceColor::Black} else {PieceColor::White};
        let piece_type = match c{
            'k'|'K' => Some(PieceType::King),
            'q'|'Q' => Some(PieceType::Queen),
            'b'|'B' => Some(PieceType::Bishop),
            'n'|'N' => Some(PieceType::Knight),
            'r'|'R' => Some(PieceType::Rook),
            'p'|'P' => Some(PieceType::Pawn),
            //'1' => Some(PieceType::Empty),
            _ => None
        }?;
        Some(Piece{piece_type, color})
    }
    pub fn to_char(from: &Option<Self>) -> char{
        match from{
            Some(piece) =>  match piece.piece_type{
                PieceType::King => if piece.color==PieceColor::Black {'k'} else {'K'},
                PieceType::Queen => if piece.color==PieceColor::Black {'q'} else {'Q'},
                PieceType::Bishop => if piece.color==PieceColor::Black {'b'} else {'B'},
                PieceType::Knight => if piece.color==PieceColor::Black {'n'} else {'N'},
                PieceType::Rook => if piece.color==PieceColor::Black {'r'} else {'R'},
                PieceType::Pawn => if piece.color==PieceColor::Black {'p'} else {'P'},
                PieceType::Empty => ' ',
            },
            None => ' ',
        }

    }

}

impl PieceColor{
    pub fn opposite(&self) -> Self{
        match self{
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        }
    }
    
}


mod behaviors{
    pub type Behavior = fn(&GameState, Field) -> Vec<Move>;
    pub type BehaviorTrait = dyn Fn(&GameState, Field) -> Vec<Move>;
    pub type BoxedBehavior = Box<BehaviorTrait>;
    
    fn a() { let x = piece_behavior; }
    pub fn piece_behavior(piece: PieceType) -> Behavior{
        match piece {
            PieceType::King => king_moves,
            PieceType::Queen => |game_state, origin| 
                [rook_moves(game_state, origin),bishop_moves(game_state, origin)].concat(),
            PieceType::Bishop => bishop_moves,
            PieceType::Knight => king_moves,
            PieceType::Rook => rook_moves,
            PieceType::Pawn => pawn_moves,
            PieceType::Empty => panic!("deprecated"),
        }
    }

    use crate::{backend::{GameState, pieces::{Piece,PieceColor,PieceType},moves::{Field,Move},ChessError}};
    
    //directions
    const HORIZONTALS:[(i8,i8);4]=   [(1,0),(0,-1),(-1,0),(0,1)];
    const DIAGONALS:[(i8,i8);4]  =   [(1,1),(1,-1),(-1,1),(-1,-1)];
    const ALL_DIRS:[(i8,i8);8]   =   [(1,1),(1,-1),(-1,1),(-1,-1),  (1,0),(0,-1),(-1,0),(0,1)];
    const NO_DIR:[(i8,i8);0]     =   [];
 
    



    //helper fn
    pub fn moves_in_direction(game_state: &GameState, origin: Field, dir: (i8,i8), own_color: PieceColor) -> Vec<Move>{
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



    const add_behavior: fn(Behavior, Behavior) -> BoxedBehavior = |x, y| 
    Box::new( |game_state, origin|
    [x(game_state, origin),y(game_state, origin)].concat());

    trait Semigroup{
        fn add(self, other:Self) -> Self;
    }
    impl<T: Clone> Semigroup for Vec<T>{
        fn add(self, other:Self) -> Self{
            [self, other].concat()
        }
    }

    type Map<In,Out> = Box<dyn Fn(In) -> Out>;
    type AddFn<In,Out:Semigroup> = fn( Map<In, Out>, Map<In, Out>) -> Map<In, Out>;

    fn add_functions<In,Out:Semigroup>(a: Map<In, Out>, b:Map<In, Out>) -> Map<In, Out> {
        Box::new(|x| a(x).add(b(x)))
    } 




    fn compose(a: Behavior, b: Behavior) -> Box<dyn Fn(&GameState, Field) -> Vec<Move>>{
        Box::new(|game_state, origin| [a(game_state, origin),b(game_state, origin)].concat())

    }


    fn king_moves(game_state: &GameState, origin: Field) -> Vec<Move>{
        
    }   
    fn bishop_moves(game_state: &GameState, origin: Field) -> Vec<Move>;
    fn rook_moves(game_state: &GameState, origin: Field) -> Vec<Move>;
    fn knight_moves(game_state: &GameState, origin: Field) -> Vec<Move>;
    fn pawn_moves(game_state: &GameState, origin: Field) -> Vec<Move>;
    //etc ...
}