use std::time::{Duration, Instant};

use crate::board::Board;
use crate::config::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::model::Piece;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SolveStatus {
    Solved,
    NoSolution,
    Timeout,
}

pub fn solve_to_board(starting_board: &Board, available_pieces: &[Piece]) -> Option<Board> {
    let mut board = starting_board.clone();
    if solve_recursive(&mut board, available_pieces, 0) {
        Some(board)
    } else {
        None
    }
}

pub fn solve_to_board_deadline(
    starting_board: &Board,
    available_pieces: &[Piece],
    timeout: Duration,
) -> (Option<Board>, SolveStatus) {
    let mut board = starting_board.clone();
    let deadline = Instant::now() + timeout;
    let status = solve_recursive_deadline(&mut board, available_pieces, 0, deadline);
    match status {
        SolveStatus::Solved => (Some(board), SolveStatus::Solved),
        SolveStatus::NoSolution => (None, SolveStatus::NoSolution),
        SolveStatus::Timeout => (None, SolveStatus::Timeout),
    }
}

fn solve_recursive(board: &mut Board, pieces: &[Piece], idx: usize) -> bool {
    if idx == pieces.len() {
        return board.is_full();
    }

    let piece = &pieces[idx];
    for y in 0..BOARD_HEIGHT as i32 {
        for x in 0..BOARD_WIDTH as i32 {
            for ori in &piece.orientations {
                if board.can_place(ori, x, y) {
                    board.place(piece.id, ori, x, y);
                    if solve_recursive(board, pieces, idx + 1) {
                        return true;
                    }
                    board.remove(piece.id, ori, x, y);
                }
            }
        }
    }
    false
}

fn solve_recursive_deadline(
    board: &mut Board,
    pieces: &[Piece],
    idx: usize,
    deadline: Instant,
) -> SolveStatus {
    if Instant::now() >= deadline {
        return SolveStatus::Timeout;
    }
    if idx == pieces.len() {
        return if board.is_full() {
            SolveStatus::Solved
        } else {
            SolveStatus::NoSolution
        };
    }

    let piece = &pieces[idx];
    for y in 0..BOARD_HEIGHT as i32 {
        for x in 0..BOARD_WIDTH as i32 {
            for ori in &piece.orientations {
                if Instant::now() >= deadline {
                    return SolveStatus::Timeout;
                }
                if board.can_place(ori, x, y) {
                    board.place(piece.id, ori, x, y);
                    match solve_recursive_deadline(board, pieces, idx + 1, deadline) {
                        SolveStatus::Solved => return SolveStatus::Solved,
                        SolveStatus::Timeout => {
                            board.remove(piece.id, ori, x, y);
                            return SolveStatus::Timeout;
                        }
                        SolveStatus::NoSolution => {
                            board.remove(piece.id, ori, x, y);
                        }
                    }
                }
            }
        }
    }
    SolveStatus::NoSolution
}

