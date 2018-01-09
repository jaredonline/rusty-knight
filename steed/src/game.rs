#[cfg(test)]
mod tests {
    use board::{Board, Piece};

    #[test]
    fn three_fold_repitition_draw() {
        let mut board = Board::empty();
        board.add_piece(Piece::BlackKing, "a8");
        board.add_piece(Piece::WhiteKing, "a1");

        board.move_piece("a1", "a2");
        board.move_piece("a8", "a7");
        board.move_piece("a2", "a1");
        board.move_piece("a7", "a8");

        board.move_piece("a1", "a2");
        board.move_piece("a8", "a7");
        board.move_piece("a2", "a1");
        board.move_piece("a7", "a8");

        board.move_piece("a1", "a2");
        board.move_piece("a8", "a7");
        board.move_piece("a2", "a1");
        board.move_piece("a7", "a8");

        assert!(board.threefold_draw());

        let mut board = Board::empty();
        board.add_piece(Piece::BlackKing, "a8");
        board.add_piece(Piece::WhiteKing, "a1");

        assert!(!board.threefold_draw());
    }
}
