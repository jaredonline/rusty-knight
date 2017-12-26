use board::{
    Board,
    Piece,
    Color,
};
use position::{
    Position,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Move {
    start: Position,
    end: Position,
    piece: Piece,
}

impl Board {
    pub fn filtered_moves_for<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let position = position.into();
        let piece = self.piece_at(position);

        let mut moves = self.moves_for(position);

        moves = self.filter_in_check(position, moves, piece.color());
        moves = self.filter_occupied_space(position, moves, piece.color());

        moves
    }

    pub fn moves_for(&self, position: Position) -> Vec<Position> {
        let piece = self.piece_at(position);
        match piece {
            Piece::BlackPawn | Piece::WhitePawn => self.pawn_moves(position, piece),
            Piece::BlackKing | Piece::WhiteKing => self.king_moves(position, piece),
            Piece::BlackQueen | Piece::WhiteQueen => self.queen_moves(position),
            Piece::BlackRook | Piece::WhiteRook => self.rook_moves(position),
            Piece::BlackBishop | Piece::WhiteBishop => self.bishop_moves(position),
            Piece::BlackKnight | Piece::WhiteKnight => self.knight_moves(position),
            
            Piece::Empty => Vec::new(),

            // _ => unreachable!(),
        }
    }

    pub fn position_in_bounds(column: i8, row: i8) -> Result<Position, &'static str> {
        if column > 0 && column <= 8 && row > 0 && row <= 8 {
            return Ok(Position::new(column.into(), row as u8));
        } 

        Err("Invalid position")
    }

    pub fn offset_in_bounds<P: Into<Position>>(position: P, column_offset: i8, row_offset: i8) -> Result<Position, &'static str> {
        let position = position.into();
        let row = (position.row as i8) + row_offset;
        let c : i8 = position.column.into();
        let column = c + column_offset;

        Board::position_in_bounds(column, row)
    }

    fn pawn_moves<P: Into<Position>>(&self, position: P, piece: Piece) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        match piece {
            Piece::WhitePawn => {
                if position.row < 8 {
                    moves.push(Position::new(position.column, position.row + 1));
                }

                if position.row == 2 {
                    moves.push(Position::new(position.column, 4));
                }
            },

            Piece::BlackPawn => {
                if position.row > 1 {
                    moves.push(Position::new(position.column, position.row - 1));
                }

                if position.row == 7.into() {
                    moves.push(Position::new(position.column, 5));
                }
            },

            _ => unreachable!(),
        }

        moves
    }

    fn king_moves<P: Into<Position>>(&self, position: P, piece: Piece) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        for i in -1 .. 2 {
            for j in -1 .. 2 {
                if i == 0 && j == 0 {
                    continue;
                }

                match Board::offset_in_bounds(position, i, j) {
                    Ok(position) => moves.push(position),
                    _ => {}
                }
            }
        }

        moves
    }

    fn filter_in_check(&self, start: Position, positions: Vec<Position>, color: Color) -> Vec<Position> {
        let mut moves = Vec::new();

        for position in positions {
            let board = self.hypothetical_move(start, position);
            if !board.in_check(color) {
                moves.push(position);
            }
        }

        moves
    }

    pub fn filter_occupied_space(&self, position: Position, positions: Vec<Position>, color: Color) -> Vec<Position> {
        let mut moves = Vec::new();

        for position in positions {
            let piece = self.piece_at(position);
            match piece {
                Piece::Empty => {
                    moves.push(position);
                },
                _ => {
                    if piece.color() != color {
                        moves.push(position);
                    }
                },
            }
        }

        moves
    }

    fn under_attack<P: Into<Position>>(&self, position: P, color: Color) -> bool {
        let position = position.into();

        for (i, piece) in self.enumerate_pieces() {
            let i : Position = i.into();
            match piece {
                &Piece::Empty => { continue; },
                _ => {}
            }

            if piece.color() != color {
                for m in self.moves_for(i) {
                    if m == position {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn queen_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        for offset in Board::diagonal_offsets() {
            match Board::offset_in_bounds(position, offset.0, offset.1) {
                Ok(position) => moves.push(position),
                _ => {}
            }
        }

        for offset in Board::column_offsets() {
            match Board::offset_in_bounds(position, offset.0, offset.1) {
                Ok(position) => moves.push(position),
                _ => {}
            }
        }

        moves
    }

    fn rook_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        for offset in Board::column_offsets() {
            match Board::offset_in_bounds(position, offset.0, offset.1) {
                Ok(position) => moves.push(position),
                _ => {}
            }
        }

        moves
    }

    fn bishop_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        for offset in Board::diagonal_offsets() {
            match Board::offset_in_bounds(position, offset.0, offset.1) {
                Ok(position) => moves.push(position),
                _ => {}
            }
        }

        moves
    }

    fn knight_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        for offset in [(2, 1), (1, 2), (-2, 1), (-1, 2), (2, -1), (1, -2), (-2, -1), (-1, -2)].iter() {
            match Board::offset_in_bounds(position, offset.0, offset.1) {
                Ok(position) => moves.push(position),
                _ => {}
            }
        }

        moves
    }

    fn diagonal_offsets() -> Vec<(i8, i8)> {
        let mut offsets = Vec::new();

        for i in 1 .. 9 {
            // diaganols
            offsets.push((i, i));
            offsets.push((i * -1, i));
            offsets.push((i, i * -1));
            offsets.push((i * -1, i * -1));
        }

        offsets
    }

    fn column_offsets() -> Vec<(i8, i8)> {
        let mut offsets = Vec::new();

        for i in 1 .. 9 {
            // rows + columns
            offsets.push((i, 0));
            offsets.push((i * -1, 0));
            offsets.push((0, i));
            offsets.push((0, i * -1));
        }

        offsets
    }
}

#[cfg(test)]
mod tests {
    use board::Board;
    use board::Piece;
    use position::Position;

    macro_rules! assert_movement {
        ($piece:expr, $position:expr, $($expected:expr),*) =>  {
            let mut board = Board::empty();
            board.add_piece($piece, $position);
            let mut moves : Vec<Position> = Vec::new();
            $(
                moves.push($expected.into());
            )*
            moves.sort();

            let mut position_moves = board.filtered_moves_for($position);
            position_moves.sort();
            assert_eq!(position_moves, moves);
        }
    }

    macro_rules! assert_board_movement {
        ($board:expr, $position:expr, $($expected:expr),*) => {
            let mut moves : Vec<Position> = Vec::new();
            $(
                moves.push($expected.into());
            )*
            moves.sort();

            let mut position_moves = $board.filtered_moves_for($position);
            position_moves.sort();
            assert_eq!(position_moves, moves);
        }
    }

    macro_rules! assert_board_no_movement {
        ($board:expr, $position:expr) => {
            let moves : Vec<Position> = Vec::new();

            let mut position_moves = $board.filtered_moves_for($position);
            position_moves.sort();
            assert_eq!(position_moves, moves);
        }
    }
    
    macro_rules! assert_no_movement {
        ($piece:expr, $position:expr) =>  {
            let mut board = Board::empty();
            board.add_piece($piece, $position);
            let moves = Vec::new();
            assert_eq!(board.filtered_moves_for($position), moves);
        }
    }

    #[test]
    fn white_pawn_movement() {
        assert_movement!(Piece::WhitePawn, "a2", "a3", "a4");
        assert_movement!(Piece::WhitePawn, "a3", "a4");
        assert_movement!(Piece::WhitePawn, "d2", "d3", "d4");
        assert_no_movement!(Piece::WhitePawn, "a8");
    }

    #[test]
    fn black_pawn_movement() {
        assert_movement!(Piece::BlackPawn, "a7", "a6", "a5");
        assert_movement!(Piece::BlackPawn, "a6", "a5");
        assert_movement!(Piece::BlackPawn, "d7", "d6", "d5");
        assert_no_movement!(Piece::BlackPawn, "a1");
    }

    #[test]
    fn white_en_passant() {
        assert!(false);
    }

    #[test]
    fn black_en_passant() {
        assert!(false);
    }

    #[test]
    fn king_movement() {
        assert_movement!(Piece::WhiteKing, "b4", "a3", "a4", "a5", "b3", "b5", "c3", "c4", "c5");
        assert_movement!(Piece::BlackKing, "a1", "a2", "b1", "b2");
    }

    #[test]
    fn queen_movement() {
        assert_movement!(Piece::BlackQueen, "d4", "a1", "a4", "a7", "b2", "b4", "b6", "c3", "c4", "c5", "d1", "d2", "d3", "d5", "d6", "d7", "d8", "e3", "e4", "e5", "f2", "f4", "f6", "g1", "g4", "g7", "h4", "h8");
    }

    #[test]
    fn castle() {
        assert!(false);
    }

    #[test]
    fn rook_movement() {
        assert_movement!(Piece::BlackRook, "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1", "c1", "d1", "e1", "f1", "g1", "h1");
    }

    #[test]
    fn knight_movement() {
        assert_movement!(Piece::BlackKnight, "a1", "b3", "c2");
        assert_movement!(Piece::WhiteKnight, "d4", "e6", "f5", "f3", "e2", "c2", "b3", "b5", "c6");
    }

    #[test]
    fn bishop_movement() {
        assert_movement!(Piece::BlackBishop, "d4", "a1", "a7", "b2", "b6", "c3", "c5", "e3", "e5", "f2", "f6", "g1", "g7", "h8");
    }

    #[test]
    fn black_pawn_exchange() {
        assert!(false);
    }

    #[test]
    fn white_pawn_exchange() {
        assert!(false);
    }

    #[test]
    fn blank_square() {
        let board = Board::empty();
        let moves = [
        ].to_vec();
        assert_eq!(board.filtered_moves_for("a2"), moves);
    }

    #[test]
    fn cant_move_into_check() {
        let mut board = Board::empty();
        board.add_piece(Piece::BlackRook, "b8");
        board.add_piece(Piece::WhiteKing, "a1");
        assert_board_movement!(board, "a1", "a2");

        let mut board = Board::empty();
        board.add_piece(Piece::BlackKing, "d4");
        board.add_piece(Piece::WhiteQueen, "c1");
        board.add_piece(Piece::WhiteBishop, "a2");
        assert_board_movement!(board, "d4", "d3", "e5", "e4");
    }

    #[test]
    fn cant_move_pinned_piece() {
        let mut board = Board::empty();
        board.add_piece(Piece::BlackKing, "d4");
        board.add_piece(Piece::BlackPawn, "e4");
        board.add_piece(Piece::WhiteRook, "f4");

        assert_board_no_movement!(board, "e4");
    }

    #[test]
    fn checkmate() {
        assert!(false);
    }

    #[test]
    fn must_move_out_of_check() {
        let mut board = Board::empty();
        board.add_piece(Piece::BlackKing, "a8");
        board.add_piece(Piece::WhiteRook, "a1");
        board.add_piece(Piece::BlackPawn, "b7");

        assert_board_no_movement!(board, "b7");
        assert_board_movement!(board, "a8", "b8");
    }

    #[test]
    fn take_a_piece() {
        assert!(false);
    }
}
