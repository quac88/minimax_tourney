// moves.rs

use super::{Position, MoveList, ArrayVec};

impl Position {
    // return every legal state reached by one move of w1
    pub fn w1_step(&self) -> MoveList {
        let next_move = self.moves_played + 1;
        // is w1 already home
        if self.w1 == 9 {
            return MoveList::new();            
        }

        // next is just an empty vector that will hold all the positions after a legal w1 move
        let mut next: ArrayVec<Position, 6>  = MoveList::new();
        let (w1, w2, b1, b2) = (self.w1, self.w2, self.b1, self.b2);

        // helper to test if a square is free. A square in this program refers to any position in the game board array
        let free = |square: u8| {
            (square == 9 || square != w2)
                && square != b1
                && square != b2
                && (1..=9).contains(&square)
        };

        // if w1 is at the last square (8), move it to the home square (9)
        if w1 == 8 {
            next.push(Position { w1: 9, w2, b1, b2, moves_played: next_move }); // Vec doesn't implement += for single items, so we use push to add exactly one item in O(1)
        // if the square after w1 is free, move w1 to the next square
        } else if free(w1 + 1) {
            next.push(Position {
                w1: w1 + 1,
                w2,
                b1,
                b2,
                moves_played: next_move,
            });
        } else if free(w1 + 2) {
            let dest: u8 = w1 + 2;
            // jump over b1
            if w1 + 1 == b1 {
                if dest == 9 {
                    // keep b1 in place
                    // move b1 back to 8
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 8 && free(8) {
                    // move b1 back to 8
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1: 8,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 7 && free(7) {
                    // move b1 back to 7
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1: 7,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 6 && free(6) && dest != 8 {
                    // move b1 back to 6
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1: 6,
                        b2,
                        moves_played: next_move,
                    });
                } else {
                    // b1 can't move back - keep it in place
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1,
                        b2,
                        moves_played: next_move,
                    });
                }
            }
            // jump over b2
            let dest: u8 = w1 + 2;
            if w1 + 1 == b2 {
                if dest == 9 {
                    // keep b1 in place
                    // move b1 back to 8
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 8 && free(8) {
                    // move b2 back to 8
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1,
                        b2: 8,
                        moves_played: next_move,
                    });
                } else if dest != 7 && free(7) {
                    // move b2 back to 7
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1,
                        b2: 7,
                        moves_played: next_move,
                    });
                } else if dest != 6 && free(6) && dest != 8 {
                    // move b2 back to 6
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1,
                        b2: 6,
                        moves_played: next_move,
                    });
                } else {
                    // b2 can't move back - keep it in place
                    next.push(Position {
                        w1: dest,
                        w2,
                        b1,
                        b2,
                        moves_played: next_move,
                    });
                }
            }
            //
            if w1 + 1 != b1 && w1 + 1 != b2 {
                // if the square after w1 is free, move w1 to the next square
                next.push(Position {
                    w1: w1 + 2,
                    w2,
                    b1,
                    b2,
                    moves_played: next_move,
                });
            }
        } else if free(w1 + 3) {
            next.push(Position {
                w1: w1 + 3,
                w2,
                b1,
                b2,
                moves_played: next_move,
            });
        } else if free(w1 + 4) {
            next.push(Position {
                w1: w1 + 4,
                w2,
                b1,
                b2,
                moves_played: next_move,
            });
        } 
        next
    }

    pub fn w2_step(&self) -> MoveList {
        let next_move = self.moves_played + 1;
        if self.w2 == 9 {
            return MoveList::new()
        }

        let mut next: ArrayVec<Position, 6> = MoveList::new();
        let (w1, w2, b1, b2) = (self.w1, self.w2, self.b1, self.b2);

        let free = |square: u8| {
            (square == 9 || square != w1)
                && square != b1
                && square != b2
                && (1..=9).contains(&square)
        };
        if w2 == 8 {
            next.push(Position { w1, w2: 9, b1, b2, moves_played: next_move });
        } else if free(w2 + 1) {
            next.push(Position {
                w1,
                w2: w2 + 1,
                b1,
                b2,
                moves_played: next_move,
            });
        } else if free(w2 + 2) {
            let dest: u8 = w2 + 2;
            // jump over b1
            if w2 + 1 == b1 {
                if dest == 9 {
                    // keep b1 in place
                    // move b1 back to 8
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 8 && free(8) {
                    // move b1 back to 8
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1: 8,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 7 && free(7) {
                    // move b1 back to 7
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1: 7,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 6 && free(6) && dest != 8 {
                    // move b1 back to 6
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1: 6,
                        b2,
                        moves_played: next_move,
                    });
                } else {
                    // b1 can't move back - keep it in place
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1,
                        b2,
                        moves_played: next_move,
                    });
                }
            }
            let dest: u8 = w2 + 2;
            // jump over b2
            if w2 + 1 == b2 {
                if dest == 9 {
                    // keep b2 in place
                    // move b2 back to 8
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1,
                        b2,
                        moves_played: next_move,
                    });
                } else
                if dest != 8 && free(8) {
                    // move b2 back to 8
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1,
                        b2: 8,
                        moves_played: next_move,
                    });
                } else if dest != 7 && free(7) {
                    // move b2 back to 7
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1,
                        b2: 7,
                        moves_played: next_move,
                    });
                } else if dest != 6 && free(6) && dest != 8 {
                    // move b2 back to 6
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1,
                        b2: 6,
                        moves_played: next_move,
                    });
                } else {
                    next.push(Position {
                        w1,
                        w2: dest,
                        b1,
                        b2,
                        moves_played: next_move,
                    });
                }
            }

            if w2 + 1 != b1 && w2 + 1 != b2 {
                // if the square after w1 is free, move w1 to the next square
                next.push(Position {
                    w1,
                    w2: w2 + 2,
                    b1,
                    b2,
                    moves_played: next_move,
                });
            }
        } else if free(w2 + 3) {
            next.push(Position {
                w1,
                w2: w2 + 3,
                b1,
                b2,
                moves_played: next_move,
            });
        } else if free(w2 + 4) {
            next.push(Position {
                w1,
                w2: w2 + 4,
                b1,
                b2,
                moves_played: next_move,
            });
        } 
        next
    }

    pub fn b1_step(&self) -> MoveList {
        let next_move = self.moves_played + 1;
        if self.b1 == 0 {
            return MoveList::new();
        }

        let mut next: ArrayVec<Position, 6>  = MoveList::new();
        let (w1, w2, b1, b2) = (self.w1, self.w2, self.b1, self.b2);

        let free = |square: u8| {
            (square == 0 || square != b2)
                && square != w1
                && square != w2
                && (0..=8).contains(&square)
        };

        if b1 == 1 {
            next.push(Position { w1, w2, b1: 0, b2, moves_played: next_move });
        } else if free(b1 - 1) {
            next.push(Position {
                w1,
                w2,
                b1: b1 - 1,
                b2,
                moves_played: next_move,
            });
        } else if free(b1 - 2) {
            // jump over w1
            let dest: u8 = b1 - 2;
            if b1 - 1 == w1 {
                if dest == 0 {
                    // keep w1 in place
                    // move w1 back to 0
                    next.push(Position {
                        w1,
                        w2,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 1 && free(1) {
                    // move w1 back to 1
                    next.push(Position {
                        w1: 1,
                        w2,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 2 && free(2) {
                    // move w1 back to 2
                    next.push(Position {
                        w1: 2,
                        w2,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 3 && free(3) && dest != 1 {
                    // move b1 back to 3
                    next.push(Position {
                        w1: 3,
                        w2,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                } else {
                    // w1 can't move back - keep it in place
                    next.push(Position {
                        w1,
                        w2,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                }
            }
            let dest: u8 = b1 - 2;
            // jump over w2
            if b1 - 1 == w2 {
                if dest == 0 {
                    // keep w2 in place
                    // move w2 back to 0
                    next.push(Position {
                        w1,
                        w2,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                }
                if dest != 1 && free(1) {
                    // move w2 back to 1
                    next.push(Position {
                        w1,
                        w2: 1,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 2 && free(2) {
                    // move w2 back to 2
                    next.push(Position {
                        w1,
                        w2: 2,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                } else if dest != 3 && free(3) && dest != 1 {
                    // move w2 back to 3
                    next.push(Position {
                        w1,
                        w2: 3,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                } else {
                    // w2 can't move back - keep it in place
                    next.push(Position {
                        w1,
                        w2,
                        b1: dest,
                        b2,
                        moves_played: next_move,
                    });
                }
            }
            if b1 - 1 != w1 && b1 - 1 != w2 {
                // if the square after b1 is free, move b1 to the next square
                next.push(Position {
                    w1,
                    w2,
                    b1: b1 - 2,
                    b2,
                    moves_played: next_move,
                });
            }
        } else if free(b1 - 3) {
            next.push(Position {
                w1,
                w2,
                b1: b1 - 3,
                b2,
                moves_played: next_move,
            });
        } else if free(b1 - 4) {
            next.push(Position {
                w1,
                w2,
                b1: b1 - 4,
                b2,
                moves_played: next_move,
            });
        } 
        next
    }

    pub fn b2_step(&self) -> MoveList {
        let next_move = self.moves_played + 1;
        if self.b2 == 0 {
            return MoveList::new()
        }

        let mut next: ArrayVec<Position, 6>  = MoveList::new();
        let (w1, w2, b1, b2) = (self.w1, self.w2, self.b1, self.b2);

        let free = |square: u8| {
            (square == 0 || square != b1)
                && square != w1
                && square != w2
                && (0..=8).contains(&square)
        };

        if b2 == 1 {
            next.push(Position { w1, w2, b1, b2: 0, moves_played: next_move });
        } else if free(b2 - 1) {
            next.push(Position {
                w1,
                w2,
                b1,
                b2: b2 - 1,
                moves_played: next_move,
            });
        } else if free(b2 - 2) {
            let dest: u8 = b2 - 2;
            // jump over w1
            if b2 - 1 == w1 {
                if dest == 0 {
                    // keep w1 in place
                    // move w1 back to 0
                    next.push(Position {
                        w1,
                        w2,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                } else if dest != 1 && free(1) {
                    // move w1 back to 1
                    next.push(Position {
                        w1: 1,
                        w2,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                } else if dest != 2 && free(2) {
                    // move w1 back to 2
                    next.push(Position {
                        w1: 2,
                        w2,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                } else if dest != 3 && free(3) && dest != 1 {
                    // move b1 back to 3
                    next.push(Position {
                        w1: 3,
                        w2,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                } else {
                    // w1 can't move back - keep it in place
                    next.push(Position {
                        w1,
                        w2,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                }
            }
            let dest: u8 = b2 - 2;
            // jump over w2
            if b2 - 1 == w2 {
                if dest == 0 {
                    // keep w2 in place
                    // move w2 back to 0
                    next.push(Position {
                        w1,
                        w2,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                } else if dest != 1 && free(1) {
                    // move w2 back to 1
                    next.push(Position {
                        w1,
                        w2: 1,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                } else if dest != 2 && free(2) {
                    // move w2 back to 2
                    next.push(Position {
                        w1,
                        w2: 2,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                } else if dest != 3 && free(3) && dest != 1 {
                    // move w2 back to 3
                    next.push(Position {
                        w1,
                        w2: 3,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                } else {
                    // w2 can't move back - keep it in place
                    next.push(Position {
                        w1,
                        w2,
                        b1,
                        b2: dest,
                        moves_played: next_move,
                    });
                }
            }

            if b2 - 1 != w1 && b2 - 1 != w2 {
                // if the square after b1 is free, move b1 to the next square
                next.push(Position {
                    w1,
                    w2,
                    b1,
                    b2: b2 - 2,
                    moves_played: next_move,
                });
            }
        } else if free(b2 - 3) {
            next.push(Position {
                w1,
                w2,
                b1,
                b2: b2 - 3,
                moves_played: next_move,
            });
        } else if free(b2 - 4) {
            next.push(Position {
                w1,
                w2,
                b1,
                b2: b2 - 4,
                moves_played: next_move,
            });
        }
        next
    }
}