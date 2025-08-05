//lib.rs
use clap::Parser;
use std::cell::Cell;
use std::fmt;
use arrayvec::ArrayVec;
mod moves;
use core::cmp::Reverse;

const MOVE_LIMIT: u8 = 25;

thread_local! {
    // Thread-local storage for the current search depth
    static EVAL_COUNT: Cell<usize> = Cell::new(0); // Cell::new(0) - start counter at 0
}

type MoveList = ArrayVec<Position, 6>; // max 6 moves in a game

/// Call at the start of every search.
pub fn reset_eval_counter() {
    EVAL_COUNT.with(|c: &Cell<usize>| c.set(0));
}

// .with method yeilds a reference to the contained value which csannot outlive the current thread or escape the given closure.

/// Call after the search to read the number of leaves visited.
pub fn eval_counter() -> usize {
    EVAL_COUNT.with(|c: &Cell<usize>| c.get()) 
}

// Command line interface for the game
#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, default_value_t = 1)]
    pub w1: u8,
    #[arg(long, default_value_t = 2)]
    pub w2: u8,
    #[arg(long, default_value_t = 7)]
    pub b1: u8,
    #[arg(long, default_value_t = 8)]
    pub b2: u8,
}

// struct for positions of the game board
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Position {
    pub w1: u8,
    pub w2: u8,
    pub b1: u8,
    pub b2: u8,
    pub moves_played: u8,
}

// implement Display trait for Position to pretty print the board
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.w1, self.w2, self.b1, self.b2)
    }
}

// implement Position methods
impl Position {
    pub fn new(w1: u8, w2: u8, b1: u8, b2: u8) -> Position {
        Position { w1, w2, b1, b2, moves_played: 0 }
    }

    pub fn white_win(&self) -> bool {
        self.w1 == 9 && self.w2 == 9
    }

    pub fn black_win(&self) -> bool {
        self.b1 == 0 && self.b2 == 0
    }

    #[inline(always)]
    fn static_eval(&self) -> i32 {
        if self.white_win() { 100 }
        else if self.black_win() { -100 }
        else { (self.w1 as i32 + self.w2 as i32 + self.b1 as i32 + self.b2 as i32) - 18 }
    }

    pub fn estimate_position(&self) -> i32 {
        EVAL_COUNT.with(|c: &Cell<usize>| c.set(c.get() + 1)); // increment the evaluation counter
        if self.moves_played >= 50 {
            return -100;
        }
        if self.white_win() {
            return 100;
        } else if self.black_win() {
            return -100;
        }
        return (self.w1 as i32 + self.w2 as i32 + self.b1 as i32 + self.b2 as i32) - 18;
    }

    pub fn estimate_position_improved(&self, white_to_move: bool) -> i32 {
        // count this call
        EVAL_COUNT.with(|c: &Cell<usize>| c.set(c.get() + 1));
        if self.moves_played >= MOVE_LIMIT {
            return -100;
        }

        // hard wins
        if self.white_win() { return  100; }
        if self.black_win() { return -100; }

        // ---------- step 2: 1 white + 1 black pawn on the main board? ----------
        let white_squares: [u8; 2] = [self.w1, self.w2];
        let black_squares: [u8; 2] = [self.b1, self.b2];

        let white_on: Vec<u8> = white_squares
            .iter().copied().filter(|&sq| (1..=8).contains(&sq)).collect();
        let black_on: Vec<u8> = black_squares
            .iter().copied().filter(|&sq| (1..=8).contains(&sq)).collect();

        if white_on.len() == 1 && black_on.len() == 1 {
            let sum: i32 = white_on[0] as i32 + black_on[0] as i32;
            let even: bool = sum % 2 == 0;

            let white_wins: bool = if even {
                // even sum ⇒ White wins if Black is to move
                !white_to_move
            } else {
                // odd sum  ⇒ White wins if White is to move
                white_to_move
            };
            return if white_wins { 100 } else { -100 };
        }
        // ---------- step 3: fallback ----------
        (self.w1 as i32 + self.w2 as i32 + self.b1 as i32 + self.b2 as i32) - 18
    }

    fn white_children(p: &Position) -> MoveList {
        let mut v: ArrayVec<Position, 6> = MoveList::new();
        if p.w1 != 9 {
            v.extend(p.w1_step());
        }
        if p.w2 != 9 {
            v.extend(p.w2_step());
        }
        v
    }

    fn black_children(p: &Position) -> MoveList {
        let mut v: ArrayVec<Position, 6> = MoveList::new();
        if p.b1 != 0 {
            v.extend(p.b1_step());
        }
        if p.b2 != 0 {
            v.extend(p.b2_step());
        }
        v
    }

    pub fn ordered_children(&self, is_max: bool, pv_move: Option<Position>) -> MoveList {
        let mut kids: MoveList = if is_max {
            Position::white_children(self)
        } else {
            Position::black_children(self)
        };

        if let Some(pv) = pv_move {
            if let Some(idx) = kids.iter().position(|&p| p == pv) {
                let m = kids.swap_remove(idx);
                // move the pv move to the front
                kids.insert(0, m);
            }
        }

        if kids.len() <= 2 {
            return kids;       
        }

        if is_max {
            kids.sort_by_cached_key(|p| Reverse(p.static_eval()));
        } else {
            kids.sort_by_cached_key(|p| p.static_eval());
        }
        kids
    }

    // max_min version for white
    pub fn max_min(pos: &Position, depth: u8) -> i32 {
        if depth == 0 || pos.white_win() || pos.black_win() {
            return pos.estimate_position();
        }

        let mut v: i32 = i32::MIN;
        for child in Position::white_children(pos) {
            v = v.max(Position::min_max(&child, depth - 1));
        }
        v
    }

    pub fn max_min_improved(pos: &Position, depth: u8) -> i32 {
        if depth == 0 || pos.white_win() || pos.black_win() {
            return pos.estimate_position_improved(true);
        }

        let mut v: i32 = i32::MIN;
        for child in Position::white_children(pos) {
            v = v.max(Position::min_max_improved(&child, depth - 1));
        }
        v
    }

    // alpha-beta version - white
    pub fn max_min_ab(pos: &Position, depth: u8, mut alpha: i32, beta: i32, ply: usize, pv: Option<Position>) -> i32 {
        if depth == 0 || pos.white_win() || pos.black_win() {
            return pos.estimate_position();
        }

        let mut v: i32 = i32::MIN;
        for (i, child) in
            pos.ordered_children(true, pv).into_iter().enumerate() {
        // only the *first* child of this node gets the pv chain
        let child_pv = if i == 0 { pv } else { None };
        v = v.max(Position::min_max_ab(&child, depth-1,
                                    alpha, beta, ply+1, child_pv)); // v is the max between the current v and the min of the children
            alpha = alpha.max(v);
            if alpha >= beta {
                break;
            }
        }
        v
    }

    pub fn max_min_ab_improved(pos: &Position, depth: u8, mut alpha: i32, beta: i32, ply: usize, pv: Option<Position>) -> i32 {
        if depth == 0 || pos.white_win() || pos.black_win() {
            return pos.estimate_position_improved(true);
        }

        let mut v: i32 = i32::MIN;
        for (i, child) in 
                pos.ordered_children(true, pv).into_iter().enumerate() {
            let child_pv = if i == 0 { pv } else { None };
            v = v.max(Position::min_max_ab_improved(&child, depth - 1, alpha, beta, ply + 1, child_pv));
            alpha = alpha.max(v);
            if alpha >= beta {
                break; // beta cut-off
            }
        }
        v
    }

    // min_max version for black
    pub fn min_max(pos: &Position, depth: u8) -> i32 {
        if depth == 0 || pos.white_win() || pos.black_win() {
            return pos.estimate_position();
        }

        let mut v: i32 = i32::MAX;
        for child in Position::black_children(pos) {
            v = v.min(Position::max_min(&child, depth - 1));
        }
        v
    }

    pub fn min_max_improved(pos: &Position, depth: u8) -> i32 {
        if depth == 0 || pos.white_win() || pos.black_win() {
            return pos.estimate_position_improved(false);
        }

        let mut v: i32 = i32::MAX;
        for child in Position::black_children(pos) {
            v = v.min(Position::max_min_improved(&child, depth - 1));
        }
        v
    }

    //alpha-beta min_max version - black
    pub fn min_max_ab(pos: &Position, depth: u8, alpha: i32, mut beta: i32, ply: usize, pv: Option<Position>) -> i32 {
        if depth == 0 || pos.white_win() || pos.black_win() {
            return pos.estimate_position();
        }

        let mut v: i32 = i32::MAX;
        for (i, child) in
                pos.ordered_children(false, pv).into_iter().enumerate() {
            // only the *first* child of this node gets the pv chain
            let child_pv: Option<Position> = if i == 0 { pv } else { None };
            v = v.min(Position::max_min_ab(&child, depth - 1, alpha, beta, ply + 1, child_pv)); // v is the min between the current v and the max of the children
            beta = beta.min(v);
            if beta <= alpha {
                break; // alpha cut-off
            }
        }
        v
    }

    pub fn min_max_ab_improved(pos: &Position, depth: u8, alpha: i32, mut beta: i32, ply: usize, pv: Option<Position>) -> i32 {
        if depth == 0 || pos.white_win() || pos.black_win() {
            return pos.estimate_position_improved(false);
        }

        let mut v: i32 = i32::MAX;
        for (i, child) in
                pos.ordered_children(false, pv).into_iter().enumerate() {
            // only the *first* child of this node gets the pv chain
            let child_pv: Option<Position> = if i == 0 { pv } else { None };
            v = v.min(Position::max_min_ab_improved(&child, depth - 1, alpha, beta, ply + 1, child_pv));
            beta = beta.min(v);
            if beta <= alpha {
                break; // alpha cut-off
            }
        }
        v
    }

    // best white move using plain min_max
    pub fn best_white_move(pos: &Position, depth: u8) -> Option<(Position, i32)> {
        Position::white_children(pos)
            .into_iter()
            .map(|child: Position| {
                let score: i32 = Position::min_max(&child, depth.saturating_sub(1)); // we use saturating_sub to elimate possibility of underflow on u8 // there is a case where depth is zero. saturating_sub() removes the need to add if depth > 0 { depth - 1 } else { 0 }.
                (child, score)
            })
            .max_by_key(|&(_, score)| score)
    }

    pub fn best_white_move_improved(pos: &Position, depth: u8) -> Option<(Position, i32)> {
        Position::white_children(pos)
            .into_iter()
            .map(|child: Position| {
                let score: i32 =
                    Position::min_max_improved(&child, depth.saturating_sub(1)); // we use saturating_sub to elimate possibility of underflow on u8 // there is a case where depth is zero. saturating_sub() removes the need to add if depth > 0 { depth - 1 } else { 0 }.
                (child, score)
            })
            .max_by_key(|&(_, score)| score)
    }

    // best white move using min_max + alpha-beta
    pub fn best_white_move_ab(pos: &Position, max_depth: u8)
        -> Option<(Position, i32)> {

        let mut best: Option<(Position, i32)> = None;

        for d in 1..=max_depth {
            best = Position::white_children(pos)
                .into_iter()
                .map(|child| {
                    let score = Position::min_max_ab(
                        &child,
                        d.saturating_sub(1),
                        i32::MIN,
                        i32::MAX,
                        1,                          // ply
                        best.map(|(m, _)| m)        // pv from previous iter
                    );
                    (child, score)
                })
                .max_by_key(|&(_, s)| s);
        }
        best
    }

    pub fn best_white_move_ab_improved(pos: &Position, max_depth: u8)
        -> Option<(Position, i32)> {

        let mut best: Option<(Position, i32)> = None;

        for d in 1..=max_depth {
            best = Position::white_children(pos)
                .into_iter()
                .map(|child| {
                    let score = Position::min_max_ab_improved(
                        &child,
                        d.saturating_sub(1),
                        i32::MIN,
                        i32::MAX,
                        1,                          // ply
                        best.map(|(m, _)| m)        // pv from previous iter
                    );
                    (child, score)
                })
                .max_by_key(|&(_, s)| s);
        }
        best
    }

    pub fn best_black_move(pos: &Position, depth: u8) -> Option<(Position, i32)> {
        Position::black_children(pos)
            .into_iter()
            .map(|child: Position| {
                let score: i32 = Position::max_min(&child, depth.saturating_sub(1));
                (child, score)
            })
            .min_by_key(|&(_, score)| score)
    }

    pub fn best_black_move_improved(pos: &Position, depth: u8) -> Option<(Position, i32)> {
        Position::black_children(pos)
            .into_iter()
            .map(|child: Position| {
                let score: i32 = Position::max_min_improved(&child, depth.saturating_sub(1));
                (child, score)
            })
            .min_by_key(|&(_, score)| score)
    }

    // best white move using min_max + alpha-beta
    pub fn best_black_move_ab(
        pos: &Position,
        max_depth: u8,
    ) -> Option<(Position, i32)> {
        let mut best: Option<(Position, i32)> = None;

        for d in 1..=max_depth {
            best = Position::black_children(pos)
                .into_iter()
                .map(|child| {
                    // Black just moved, so next node is a MAX (White) node:
                    let score = Position::max_min_ab(
                        &child,
                        d.saturating_sub(1),
                        i32::MIN,
                        i32::MAX,
                        1,                         // ply
                        best.map(|(m, _)| m),      // PV move from previous iter
                    );
                    (child, score)
                })
                .min_by_key(|&(_, s)| s);          // Black wants the lowest score
        }
        best
    }

    pub fn best_black_move_ab_improved(
        pos: &Position,
        max_depth: u8,
    ) -> Option<(Position, i32)> {
        let mut best: Option<(Position, i32)> = None;

        for d in 1..=max_depth {
            best = Position::black_children(pos)
                .into_iter()
                .map(|child| {
                    // Black just moved, so next node is a MAX (White) node:
                    let score = Position::max_min_ab_improved(
                        &child,
                        d.saturating_sub(1),
                        i32::MIN,
                        i32::MAX,
                        1,                         // ply
                        best.map(|(m, _)| m),      // PV move from previous iter
                    );
                    (child, score)
                })
                .min_by_key(|&(_, s)| s);          // Black wants the lowest score
        }
        best
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;
    fn legal_position(p: &Position) -> bool {
        let w_ok = |sq: u8| matches!(sq, 1..=8 | 9);
        let b_ok = |sq: u8| matches!(sq, 0 | 1..=8);

        if !w_ok(p.w1) || !w_ok(p.w2) || !b_ok(p.b1) || !b_ok(p.b2) {
            return false;
        }

        let mut occ: HashSet<u8> = HashSet::new();
        let white_home_clash =
            (p.b1 == 9 || p.b2 == 9) && (p.w1 == 9 || p.w2 == 9);
        let black_home_clash =
            p.w1 == 0 || p.w2 == 0; // white can never be on 0 legally

        if white_home_clash || black_home_clash {
            return false;
        }

        // check duplicates outside home squares
        for &sq in [p.w1, p.w2, p.b1, p.b2].iter() {
            if sq == 0 || sq == 9 {
                continue; 
            }
            if !occ.insert(sq) {
                // duplication found
                return false;
            }
        }
        true
    }

     // Helper function to assert that all positions in a MoveList are legal
    fn assert_all_legal(parent: &Position, moves: MoveList) {
        for child in moves {
            assert!(
                legal_position(&child),
                "Illegal child\n  from parent: {:?}\n  to child  : {:?} \n",
                parent,
                child,
            );
        }
    }

    #[test]
    fn generators_produce_only_legal_positions() {
        for w1 in 1..=9 {
            for w2 in 1..=9 {
                for b1 in 0..=8 {
                    for b2 in 0..=8 {
                        let p = Position { w1, w2, b1, b2, moves_played: 0 };
                        if !legal_position(&p) {
                            continue; // skip impossible starting positions
                        }

                        // w generators
                        assert_all_legal(&p, p.w1_step());
                        assert_all_legal(&p, p.w2_step());

                        // b generators
                        assert_all_legal(&p, p.b1_step());
                        assert_all_legal(&p, p.b2_step());
                    }
                }
            }
        }
    }
    #[test]
    fn improved_eval_parity_rule() {
        // White on square 1, Black on square 4 → sum = 5 (odd)
        let p = Position::new(1, 9, 4, 0);   // exactly one pawn each on board

        // White to move on an odd sum ⇒ White should win
        assert_eq!(p.estimate_position_improved(true), 100);

        // Black to move on an odd sum ⇒ Black should win
        assert_eq!(p.estimate_position_improved(false), -100);
    }
}