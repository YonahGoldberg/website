use chess::Board;
fn main() {
    for board in Board::PAWN_ATTACKS[1] {
        Board::print_bb(board);
    }
}