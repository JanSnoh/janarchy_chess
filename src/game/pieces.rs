pub(crate) use behaviors::{piece_behavior};

mod behaviors;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    Black,
    White,
}
#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
    Empty,
}
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

#[allow(unused_variables)]
impl Piece {
    pub fn from_char(c: char) -> Option<Piece> {
        //Black is lowercase, White is uppercase
        let color = if c.is_lowercase() {
            PieceColor::Black
        } else {
            PieceColor::White
        };
        let piece_type = match c {
            'k' | 'K' => Some(PieceType::King),
            'q' | 'Q' => Some(PieceType::Queen),
            'b' | 'B' => Some(PieceType::Bishop),
            'n' | 'N' => Some(PieceType::Knight),
            'r' | 'R' => Some(PieceType::Rook),
            'p' | 'P' => Some(PieceType::Pawn),
            //'1' => Some(PieceType::Empty),
            _ => None,
        }?;
        Some(Piece { piece_type, color })
    }
    pub fn to_char(from: &Option<Self>) -> char {
        match from {
            Some(piece) => match piece.piece_type {
                PieceType::King => {
                    if piece.color == PieceColor::Black {
                        'k'
                    } else {
                        'K'
                    }
                }
                PieceType::Queen => {
                    if piece.color == PieceColor::Black {
                        'q'
                    } else {
                        'Q'
                    }
                }
                PieceType::Bishop => {
                    if piece.color == PieceColor::Black {
                        'b'
                    } else {
                        'B'
                    }
                }
                PieceType::Knight => {
                    if piece.color == PieceColor::Black {
                        'n'
                    } else {
                        'N'
                    }
                }
                PieceType::Rook => {
                    if piece.color == PieceColor::Black {
                        'r'
                    } else {
                        'R'
                    }
                }
                PieceType::Pawn => {
                    if piece.color == PieceColor::Black {
                        'p'
                    } else {
                        'P'
                    }
                }
                PieceType::Empty => ' ',
            },
            None => ' ',
        }
    }
}

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::Black => PieceColor::White,
            PieceColor::White => PieceColor::Black,
        }
    }
    pub fn to_int(&self) -> i8 {
        match self {
            PieceColor::Black => 1,
            PieceColor::White => -1,
        }
    }
}
