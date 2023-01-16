use chess::board::*;

fn main() {
    let b = Board::new();
    let to_squares: Vec<Square> = b.generate_moves(Color::White).iter().map(|m| m.get_to()).collect();
    dbg!(to_squares);
}