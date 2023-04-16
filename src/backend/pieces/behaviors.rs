use crate::backend::{
    moves::{Field, Move},
    pieces::{Piece, PieceColor, PieceType},
    ChessError, GameState,
};

pub type Behavior = fn(&GameState, Field) -> Vec<Move>;
pub type BehaviorTrait = dyn Fn(&GameState, Field) -> Vec<Move>;
pub type BoxedBehavior = Box<BehaviorTrait>;

pub fn piece_behavior(piece: PieceType) -> Behavior {
    match piece {
        PieceType::King => king_moves,
        PieceType::Queen => |game_state, origin| {
            [
                rook_moves(game_state, origin),
                bishop_moves(game_state, origin),
            ]
            .concat()
        },
        PieceType::Bishop => bishop_moves,
        PieceType::Knight => knight_moves,
        PieceType::Rook => rook_moves,
        PieceType::Pawn => pawn_moves,
        PieceType::Empty => panic!("deprecated"),
    }
}

//directions
const HORIZONTALS: [(i8, i8); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
const DIAGONALS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const ALL_DIRS: [(i8, i8); 8] = [
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (0, 1),
];
const L_SHAPES: [(i8, i8); 8] = [
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (1, 2),
    (1, -2),
    (-1, 2),
    (-1, -2),
    ];
const NO_DIR: [(i8, i8); 0] = [];

//helper fn
fn moves_in_direction(
    game_state: &GameState,
    origin: Field,
    dir: (i8, i8),
    own_color: PieceColor,
) -> Vec<Move> {
    let mut direction_moves = Vec::new();
    //let next_sq: Field = origin.add_vec(dir);
    let mut step = dir;
    while let Ok(next_sq) = origin.add_vec(step) {
        step = (step.0 + dir.0, step.1 + dir.1); //This might be stupid why did I write this? Also too tired to test rn
        match game_state[next_sq] {
            Some(Piece { color, .. }) if color == own_color => {
                break;
            }
            Some(Piece { .. }) => {
                direction_moves.push(Move::new(origin,next_sq,true,None,false));
                break;
            }
            None => direction_moves.push(Move::from_squares(origin, next_sq)),
        }
    }
    direction_moves
}

fn compose(f: BoxedBehavior, g: BoxedBehavior) -> BoxedBehavior {
    Box::new(move |game_state, origin| [f(game_state, origin), g(game_state, origin)].concat())
}

fn king_moves(game_state: &GameState, origin: Field) -> Vec<Move> {
    ALL_DIRS
        .iter()
        .filter_map(|dir| origin.add_vec(*dir).ok())
        .map(|x| Move::from_squares(origin, x))
        .filter(|mov| game_state.move_is_doable(mov))
        .collect()
}
fn bishop_moves(game_state: &GameState, origin: Field) -> Vec<Move> {
    DIAGONALS
        .iter()
        .map(|dir| moves_in_direction(game_state, origin, *dir, game_state[origin].unwrap().color))
        .flatten()
        .collect()
}
fn rook_moves(game_state: &GameState, origin: Field) -> Vec<Move> {
    HORIZONTALS
        .iter()
        .map(|dir| moves_in_direction(game_state, origin
            , *dir, game_state[origin].unwrap().color))
        .flatten()
        .filter(|mov| game_state.move_is_doable(mov))
        .collect()
}
#[allow(unused_variables)]
fn knight_moves(game_state: &GameState, origin: Field) -> Vec<Move> {
    L_SHAPES
        .iter()
        .filter_map(|dir| origin.add_vec(*dir).ok())
        .map(|target| Move::from_squares(origin, target))
        .filter(|mov| game_state.move_is_doable(mov))
        .collect()
}
fn pawn_moves(game_state: &GameState, origin: Field) -> Vec<Move> {
    let color = game_state[origin].unwrap().color;
    let mut res = Vec::new();
    let at_start = origin.1 == ((8+2*color.to_int()) as usize)%8;
    //straight moves
    if let Ok(one_further) = origin.add_vec((0,color.to_int())) {
        res.push(Move::from_squares(origin, one_further));
        //double straight move
        match origin.add_vec((0, color.to_int()*2)){ 
            Ok(two_further) if at_start => {
                res.push(Move::from_squares(origin, two_further));
            },
            _ => {}
        }
    }


    //diagonal takes
    res.extend(
    [(-1, color.to_int()), (1, color.to_int())].into_iter()
        .filter_map(|x|origin.add_vec(x).ok())
        .filter(|target|game_state[*target]
        .map_or(false, |p|p.color==color))
        .map(|target| Move::new(origin, target, true, None, false)));
    
    res
}
//etc ...
