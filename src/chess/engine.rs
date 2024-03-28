use super::board::Board;
use super::cmove::CMove;
use std::i32;

fn evaluate() -> i32 {
    1
}

fn alpha_beta(mut alpha: i32, beta: i32, depth: i32, moves: &Vec<CMove>, board: &mut Board) -> i32 {
    if depth == 0 {
        return evaluate();
    }

    for m in moves {
        board.make_move_mut(m);
        let eval = -alpha_beta(-beta, -alpha, depth - 1, moves, board);
        // unmake_move(); TODO: implement
        if eval >= beta {
            return beta; // fail hard
        }
        if eval > alpha {
            alpha = eval;
        }
    }
    alpha
}
