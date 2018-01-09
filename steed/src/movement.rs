use board::{Board, Color, Piece};
use position::Position;

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

    pub fn offset_in_bounds<P: Into<Position>>(
        position: P,
        column_offset: i8,
        row_offset: i8,
    ) -> Result<Position, &'static str> {
        let position = position.into();
        let row = (position.row as i8) + row_offset;
        let c: i8 = position.column.into();
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
            }

            Piece::BlackPawn => {
                if position.row > 1 {
                    moves.push(Position::new(position.column, position.row - 1));
                }

                if position.row == 7 {
                    moves.push(Position::new(position.column, 5));
                }
            }

            _ => unreachable!(),
        }

        moves
    }

    fn king_moves<P: Into<Position>>(&self, position: P, _: Piece) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }

                if let Ok(position) = Board::offset_in_bounds(position, i, j) {
                    moves.push(position);
                }
            }
        }

        moves
    }

    fn filter_in_check(
        &self,
        start: Position,
        positions: Vec<Position>,
        color: Color,
    ) -> Vec<Position> {
        let mut moves = Vec::new();

        for position in positions {
            let board = self.hypothetical_move(start, position);
            if !board.in_check(color) {
                moves.push(position);
            }
        }

        moves
    }

    pub fn filter_occupied_space(
        &self,
        _: Position,
        positions: Vec<Position>,
        color: Color,
    ) -> Vec<Position> {
        let mut moves = Vec::new();

        for position in positions {
            let piece = self.piece_at(position);
            match piece {
                Piece::Empty => {
                    moves.push(position);
                }
                _ => {
                    if piece.color() != color {
                        moves.push(position);
                    }
                }
            }
        }

        moves
    }

    fn queen_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        for position in self.diagonal_moves(position) {
            moves.push(position);
        }

        for position in self.columnar_moves(position) {
            moves.push(position);
        }

        moves
    }

    fn rook_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        self.columnar_moves(position)
    }

    fn bishop_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        self.diagonal_moves(position)
    }

    fn knight_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();

        for offset in &[
            (2, 1),
            (1, 2),
            (-2, 1),
            (-1, 2),
            (2, -1),
            (1, -2),
            (-2, -1),
            (-1, -2),
        ] {
            if let Ok(position) = Board::offset_in_bounds(position, offset.0, offset.1) {
                moves.push(position);
            }
        }

        moves
    }

    fn projection_moves<P: Into<Position>>(
        &self,
        position: P,
        projection: Vec<(i8, i8)>,
    ) -> Vec<Position> {
        let position = position.into();
        let mut moves = Vec::new();
        let piece_color = self.piece_at(position).color();

        for offset in projection {
            match Board::offset_in_bounds(position, offset.0, offset.1) {
                Ok(position) => {
                    let piece = self.piece_at(position);
                    let color = piece.color();

                    match piece {
                        Piece::Empty => moves.push(position),
                        _ => {
                            if color != piece_color {
                                moves.push(position);
                            }

                            break;
                        }
                    }
                }
                _ => break,
            }
        }

        moves
    }

    fn diagonal_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let mut moves = Vec::new();
        let position = position.into();

        for projection in Board::diagonal_offsets() {
            for position in self.projection_moves(position, projection) {
                moves.push(position);
            }
        }

        moves
    }

    fn columnar_moves<P: Into<Position>>(&self, position: P) -> Vec<Position> {
        let mut moves = Vec::new();
        let position = position.into();

        for projection in Board::column_offsets() {
            for position in self.projection_moves(position, projection) {
                moves.push(position);
            }
        }

        moves
    }

    fn diagonal_offsets() -> Vec<Vec<(i8, i8)>> {
        let mut projections = Vec::new();

        let mut first = Vec::new();
        let mut second = Vec::new();
        let mut third = Vec::new();
        let mut fourth = Vec::new();
        for i in 1..9 {
            // diaganols
            first.push((i, i));
            second.push((-i, i));
            third.push((i, -i));
            fourth.push((-i, -i));
        }
        projections.push(first);
        projections.push(second);
        projections.push(third);
        projections.push(fourth);

        projections
    }

    fn column_offsets() -> Vec<Vec<(i8, i8)>> {
        let mut projections = Vec::new();

        let mut first = Vec::new();
        let mut second = Vec::new();
        let mut third = Vec::new();
        let mut fourth = Vec::new();

        for i in 1..9 {
            // rows + columns
            first.push((i, 0));
            second.push((-i, 0));
            third.push((0, i));
            fourth.push((0, -i));
        }

        projections.push(first);
        projections.push(second);
        projections.push(third);
        projections.push(fourth);

        projections
    }
}

#[cfg(test)]
mod tests {
    use board::{Board, Color, Piece};
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
        assert_movement!(
            Piece::WhiteKing,
            "b4",
            "a3",
            "a4",
            "a5",
            "b3",
            "b5",
            "c3",
            "c4",
            "c5"
        );
        assert_movement!(Piece::BlackKing, "a1", "a2", "b1", "b2");
    }

    #[test]
    fn queen_movement() {
        assert_movement!(
            Piece::BlackQueen,
            "d4",
            "a1",
            "a4",
            "a7",
            "b2",
            "b4",
            "b6",
            "c3",
            "c4",
            "c5",
            "d1",
            "d2",
            "d3",
            "d5",
            "d6",
            "d7",
            "d8",
            "e3",
            "e4",
            "e5",
            "f2",
            "f4",
            "f6",
            "g1",
            "g4",
            "g7",
            "h4",
            "h8"
        );
    }

    #[test]
    fn castle() {
        assert!(false);
    }

    #[test]
    fn rook_movement() {
        assert_movement!(
            Piece::BlackRook,
            "a1",
            "a2",
            "a3",
            "a4",
            "a5",
            "a6",
            "a7",
            "a8",
            "b1",
            "c1",
            "d1",
            "e1",
            "f1",
            "g1",
            "h1"
        );
    }

    #[test]
    fn knight_movement() {
        assert_movement!(Piece::BlackKnight, "a1", "b3", "c2");
        assert_movement!(
            Piece::WhiteKnight,
            "d4",
            "e6",
            "f5",
            "f3",
            "e2",
            "c2",
            "b3",
            "b5",
            "c6"
        );
    }

    #[test]
    fn bishop_movement() {
        assert_movement!(
            Piece::BlackBishop,
            "d4",
            "a1",
            "a7",
            "b2",
            "b6",
            "c3",
            "c5",
            "e3",
            "e5",
            "f2",
            "f6",
            "g1",
            "g7",
            "h8"
        );
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
        let moves = [].to_vec();
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
        let mut board = Board::empty();
        board.add_piece(Piece::WhiteKing, "a1");
        board.add_piece(Piece::BlackKing, "b2");
        board.add_piece(Piece::BlackRook, "b8");

        assert!(board.in_check(Color::White));
        assert_board_no_movement!(board, "a1");
        assert!(board.checkmate(Color::White));

        let mut board = Board::empty();
        board.add_piece(Piece::WhiteKing, "a1");
        board.add_piece(Piece::BlackKing, "b2");

        assert!(board.in_check(Color::White));
        assert_board_movement!(board, "a1", "b2");
        assert!(!board.checkmate(Color::White));
    }

    #[test]
    fn stalemate() {
        let mut board = Board::empty();
        board.add_piece(Piece::WhiteKing, "a1");
        board.add_piece(Piece::BlackRook, "b8");
        board.add_piece(Piece::BlackRook, "h2");

        assert!(!board.in_check(Color::White));
        assert_board_no_movement!(board, "a1");
        assert!(board.stalemate());

        let mut board = Board::empty();
        board.add_piece(Piece::WhiteKing, "a1");
        board.add_piece(Piece::BlackKing, "b2");

        assert!(board.in_check(Color::White));
        assert_board_movement!(board, "a1", "b2");
        assert!(!board.stalemate());
    }

    #[test]
    fn pieces_block_movement() {
        let mut board = Board::empty();
        board.add_piece(Piece::BlackRook, "a8");
        board.add_piece(Piece::BlackPawn, "a7");
        board.add_piece(Piece::BlackKnight, "b8");

        assert_board_no_movement!(board, "a8");

        let mut board = Board::empty();
        board.add_piece(Piece::BlackRook, "a8");
        board.add_piece(Piece::WhitePawn, "a7");
        board.add_piece(Piece::BlackBishop, "b8");

        assert_board_movement!(board, "a8", "a7");

        let mut board = Board::empty();
        board.add_piece(Piece::BlackQueen, "d8");
        board.add_piece(Piece::BlackBishop, "c8");
        board.add_piece(Piece::BlackBishop, "e8");
        board.add_piece(Piece::WhiteBishop, "d7");

        assert_board_movement!(board, "d8", "d7", "c7", "b6", "a5", "e7", "f6", "g5", "h4");

        let mut board = Board::empty();
        board.add_piece(Piece::BlackQueen, "d8");
        board.add_piece(Piece::BlackBishop, "c8");
        board.add_piece(Piece::BlackBishop, "e8");
        board.add_piece(Piece::WhiteBishop, "e7");
        board.add_piece(Piece::BlackPawn, "c7");

        assert_board_movement!(board, "d8", "d7", "d6", "d5", "d4", "d3", "d2", "d1", "e7");

        let mut board = Board::empty();
        board.add_piece(Piece::BlackBishop, "c8");
        board.add_piece(Piece::BlackPawn, "b7");
        board.add_piece(Piece::WhiteBishop, "d7");

        assert_board_movement!(board, "c8", "d7");
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
