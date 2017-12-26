use std::default::Default;
use std::fmt;
use std::fmt::{
    Formatter,
    Debug
};
use std::iter::{
    Enumerate,
};

use std::slice::{
    Iter,
};

use position::Position;
use movement::Move;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Piece {
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteKing,
    WhiteQueen,

    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackKing,
    BlackQueen,

    Empty,
}

impl Piece {
    pub fn color(self) -> Color {
        match self {
            Piece::WhitePawn |
            Piece::WhiteRook |
            Piece::WhiteKnight |
            Piece::WhiteBishop |
            Piece::WhiteKing |
            Piece::WhiteQueen => Color::White,

            _ => Color::Black
        }
    }
}

#[derive(Clone)]
struct BoardLayout {
    layout: [Piece; 64],
}

impl BoardLayout {
    fn add_piece(&mut self, piece: Piece, position: Position) {
        let index : usize = position.into();
        self.layout[index] = piece;
    }
}

impl Debug for BoardLayout {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let _ = write!(f, "[");
        for piece in self.layout.iter() {
            match piece {
                &Piece::Empty => {
                    let _ = write!(f, " - ");
                },
                _ => {
                    let _ = write!(f, "{:?} ", piece);
                }
            }
        }
        write!(f, "]")
    }
}

impl PartialEq for BoardLayout {
    fn eq(&self, other: &BoardLayout) -> bool {
        for (i, p) in self.layout.iter().enumerate() {
            if p != &other.layout[i] {
                return false;
            }
        }

        return true;
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    layout: BoardLayout,
    history: Vec<Move>,
    white_can_castle: bool,
    black_can_castle: bool,
    to_move: Color
}

impl Board {
    pub fn empty() -> Board {
        Board {
            to_move: Color::White,
            history: Vec::new(),
            white_can_castle: true,
            black_can_castle: true,
            layout: BoardLayout {
                layout: [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ]
            }
        }
    }

    pub fn enumerate_pieces(&self) -> Enumerate<Iter<Piece>> {
        self.layout.layout.iter().enumerate()
    }

    pub fn piece_at<P: Into<Position>>(&self, position: P) -> Piece {
        let position = position.into();
        let index : usize = position.into();
        self.layout.layout[index].clone()
    }

    pub fn add_piece<P: Into<Position>>(&mut self, piece: Piece, position: P) {
        let position : Position = position.into();
        let index : usize = position.into();

        self.layout.add_piece(piece, position);
    }

    pub fn hypothetical_move<P: Into<Position>>(&self, start: P, end: P) -> Board {
        let mut board = self.clone();
        let start = start.into();
        let end = end.into();
        let end_index : usize = end.into();
        let start_index : usize = start.into();

        let piece = self.piece_at(start);
        board.layout.layout[start_index] = Piece::Empty;
        board.layout.layout[end_index] = piece;

        board
    }

    pub fn in_check(&self, color: Color) -> bool {
        match self.find_king(color) {
            Some(position) => {
                for (i, piece) in self.enumerate_pieces() {
                    if piece.color() != color {
                        let i : Position = i.into();
                        for m in self.moves_for(i) {
                            if m == position {
                                return true;
                            }
                        }
                    }

                    continue;
                }
            },
            _ => {},
        }

        return false;
    }

    fn find_king(&self, color: Color) -> Option<Position> {
        for (i, piece) in self.enumerate_pieces() {
            if piece.color() == color {
                match piece {
                    &Piece::BlackKing | &Piece::WhiteKing => {
                        let position : Position = i.into();
                        return Some(position);
                    },
                    _ => continue,
                }
            }
        }

        None
    }
}

impl Default for Board {
    fn default() -> Board {
        let board : [Piece; 64] = [
            Piece::BlackRook,
            Piece::BlackKnight,
            Piece::BlackBishop,
            Piece::BlackQueen,
            Piece::BlackKing,
            Piece::BlackBishop,
            Piece::BlackKnight,
            Piece::BlackRook,

            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,
            Piece::BlackPawn,

            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,

            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,

            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,

            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,
            Piece::Empty,

            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,
            Piece::WhitePawn,

            Piece::WhiteRook,
            Piece::WhiteKnight,
            Piece::WhiteBishop,
            Piece::WhiteQueen,
            Piece::WhiteKing,
            Piece::WhiteBishop,
            Piece::WhiteKnight,
            Piece::WhiteRook,
        ];

        Board {
            layout: BoardLayout {
                layout: board
            },
            to_move: Color::White,
            history: Vec::new(),
            white_can_castle: true,
            black_can_castle: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Piece::*;
    use std::mem::size_of;

    #[test]
    fn size_constraints() {
        // TODO: Would love to get this down
        assert_eq!(size_of::<Board>(), 96);
    }

    #[test]
    fn board_construction() {
        let board = Board {
            to_move: Color::White,
            history: Vec::new(),
            white_can_castle: true,
            black_can_castle: true,
            layout: BoardLayout {
                layout: [
                    BlackRook,
                    BlackKnight,
                    BlackBishop,
                    BlackQueen,
                    BlackKing,
                    BlackBishop,
                    BlackKnight,
                    BlackRook,

                    BlackPawn,
                    BlackPawn,
                    BlackPawn,
                    BlackPawn,
                    BlackPawn,
                    BlackPawn,
                    BlackPawn,
                    BlackPawn,

                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,

                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,

                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,

                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,

                    WhitePawn,
                    WhitePawn,
                    WhitePawn,
                    WhitePawn,
                    WhitePawn,
                    WhitePawn,
                    WhitePawn,
                    WhitePawn,

                    WhiteRook,
                    WhiteKnight,
                    WhiteBishop,
                    WhiteQueen,
                    WhiteKing,
                    WhiteBishop,
                    WhiteKnight,
                    WhiteRook,
                ],
            },
        };

        assert_eq!(board, Board::default());
    }

    #[test]
    fn hypothetical_move() {
        let mut board = Board::empty();
        board.add_piece(Piece::WhitePawn, "a2");
        
        let new_board = board.hypothetical_move("a2", "a3");
        let mut board = Board::empty();
        board.add_piece(Piece::WhitePawn, "a3");
    
        assert_eq!(board, new_board);
    }
    
    #[test]
    fn in_check() {
        let mut board = Board::empty();
        assert!(!board.in_check(Color::White));

        board.add_piece(Piece::WhiteKing, "a1");
        board.add_piece(Piece::BlackRook, "a8");

        assert!(board.in_check(Color::White));
        assert!(!board.in_check(Color::Black));

        let mut board = Board::empty();
        board.add_piece(Piece::BlackKing, "a8");
        board.add_piece(Piece::WhiteKnight, "c7");

        assert!(board.in_check(Color::Black));
        assert!(!board.in_check(Color::White));
    }
}
