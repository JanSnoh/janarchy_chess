
use crate::{backend::{GameState, pieces::{Piece,PieceColor,PieceType},moves::{Field,Move},ChessError}};

pub type Behavior = fn(&GameState, Field) -> Vec<Move>;
pub type BehaviorTrait = dyn Fn(&GameState, Field) -> Vec<Move>;
pub type BoxedBehavior = Box<BehaviorTrait>;

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


//directions
const HORIZONTALS:[(i8,i8);4]=   [(1,0),(0,-1),(-1,0),(0,1)];
const DIAGONALS:[(i8,i8);4]  =   [(1,1),(1,-1),(-1,1),(-1,-1)];
const ALL_DIRS:[(i8,i8);8]   =   [(1,1),(1,-1),(-1,1),(-1,-1),  (1,0),(0,-1),(-1,0),(0,1)];
const L_SHAPES:[(i8,i8);8]   =   [(2,1),(2,-1),(-2,1),(-2,-1),  (1,2),(2,-1),(-1,2),(2,1)];
const NO_DIR:[(i8,i8);0]     =   [];
 




//helper fn
fn moves_in_direction(game_state: &GameState, origin: Field, dir: (i8,i8), own_color: PieceColor) -> Vec<Move>{
    let mut direction_moves = Vec::new();
    //let next_sq: Field = origin.add_vec(dir);
    let mut step = dir;
    while let Ok(next_sq) = origin.add_vec(step){
        step=(step.0*2,step.1*2);        //This might be stupid why did I write this? Also too tired to test rn
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



fn compose(f: BoxedBehavior,  g: BoxedBehavior) -> BoxedBehavior{
    Box::new(move |game_state, origin| [f(game_state, origin),g(game_state, origin)].concat())
}


fn king_moves(_game_state: &GameState, origin: Field) -> Vec<Move>{
    ALL_DIRS.iter()
    .filter_map(|dir| origin.add_vec(*dir).ok())
    .map(|x| Move::new(origin,x) )
    .collect()
}
fn bishop_moves(game_state: &GameState, origin: Field) -> Vec<Move>{
    DIAGONALS.iter().map(|dir| moves_in_direction(game_state, 
        origin,
        *dir,
        game_state[origin].unwrap().color)).flatten().collect()
}
fn rook_moves(game_state: &GameState, origin: Field) -> Vec<Move>{
    HORIZONTALS.iter().map(|dir| moves_in_direction(game_state, 
        origin,
        *dir,
        game_state[origin].unwrap().color)).flatten().collect()
}
#[allow(unused_variables)]
fn knight_moves(game_state: &GameState, origin: Field) -> Vec<Move>   {
    L_SHAPES.iter().filter_map(|dir| origin.add_vec(*dir).ok()).map(|target| Move::new(origin, target)).collect()
}
fn pawn_moves(game_state: &GameState, origin: Field) -> Vec<Move>{
    match game_state[origin].unwrap().color {
        PieceColor::Black => if let Ok(x) = origin.add_vec((0,1)) 
            {vec![Move::new(origin, x)]} else {vec![]},
        PieceColor::White => if let Ok(x) = origin.add_vec((0,-1)) 
            {vec![Move::new(origin, x)]} else {vec![]},
    }
}
//etc ...
