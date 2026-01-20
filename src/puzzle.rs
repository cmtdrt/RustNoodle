use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;
use std::time::Instant;

use crate::board::Board;
use crate::config::{BOARD_HEIGHT, BOARD_WIDTH, GENERATION_TIMEOUT_SECS, SOLVER_DEADLINE_SECS};
use crate::model::{Piece, PieceId};
use crate::pieces::kanoodle_pieces;
use crate::solver::{solve_to_board_deadline, SolveStatus};

#[derive(Clone)]
pub struct Puzzle {
    pub starting_board: Board,
    pub available_pieces: Vec<Piece>,
    pub solution: Board,
}

// Génère un puzzle par défaut
// (dans le cas où la génération aléatoire ne trouve pas de puzzle résolvable dans le temps imparti)
pub fn default_puzzle() -> Puzzle {
    let mut board = Board::new();
    let pieces = kanoodle_pieces();
    let mut remaining = Vec::new();

    for p in &pieces {
        if p.id.0 == 'A' {
            let ori = &p.orientations[0];
            if board.can_place(ori, 0, 0) {
                board.place(p.id, ori, 0, 0);
            }
        } else if p.id.0 == 'B' {
            let ori = &p.orientations[0];
            let x = 4;
            let y = 2;
            if board.can_place(ori, x, y) {
                board.place(p.id, ori, x, y);
            }
        } else {
            remaining.push(p.clone());
        }
    }

    // Solution du puzzle par défaut
    // (On la stocke pour éviter de la recalculer à chaque fois)
    let solution_str = [
        "AAADDDHEEJJ",
        "AKDDHHHHEEJ",
        "KKKLBBFFGEJ",
        "CKCLBBFFGGJ",
        "CCCLLLFIIII",
    ];

    let mut solution = Board::new();
    for (y, row_str) in solution_str.iter().enumerate() {
        for (x, ch) in row_str.chars().enumerate() {
            if ch != '.' {
                solution.cells[y][x] = Some(PieceId(ch));
            }
        }
    }

    Puzzle {
        starting_board: board,
        available_pieces: remaining,
        solution,
    }
}

// Génère un puzzle aléatoire
pub fn sample_puzzle() -> Puzzle {
    let mut rng = rand::thread_rng();
    let all_pieces = kanoodle_pieces();
    let start_time = Instant::now();

    loop {
        if start_time.elapsed().as_secs() >= GENERATION_TIMEOUT_SECS {
            println!("Temps de génération dépassé (10s), on bascule sur un puzzle par défaut.");
            return default_puzzle();
        }

        let mut board = Board::new();
        let mut pieces_copy = all_pieces.clone();
        pieces_copy.shuffle(&mut rng);
        if pieces_copy.len() < 2 {
            println!("Aucun puzzle résolvable trouvé rapidement, on bascule sur la configuration par défaut (A+B).");
            return default_puzzle();
        }

        let piece1 = pieces_copy.remove(0);
        let piece2 = pieces_copy.remove(0);

        if !place_random(&mut board, &piece1, &mut rng) {
            continue;
        }
        if !place_random(&mut board, &piece2, &mut rng) {
            continue;
        }

        let (solution_opt, status) = solve_to_board_deadline(
            &board,
            &pieces_copy,
            Duration::from_secs(SOLVER_DEADLINE_SECS),
        );

        if let Some(solution) = solution_opt {
            return Puzzle {
                starting_board: board,
                available_pieces: pieces_copy,
                solution,
            };
        }
        if status == SolveStatus::Timeout {
            continue;
        }
    }
}

// Place une pièce aléatoirement sur le plateau
fn place_random(board: &mut Board, piece: &Piece, rng: &mut impl Rng) -> bool {
    const MAX_LOCAL_ATTEMPTS: usize = 120;

    for _ in 0..MAX_LOCAL_ATTEMPTS {
        let x = rng.gen_range(0..BOARD_WIDTH as i32);
        let y = rng.gen_range(0..BOARD_HEIGHT as i32);
        let ori = &piece.orientations[rng.gen_range(0..piece.orientations.len())];
        if board.can_place(ori, x, y) {
            board.place(piece.id, ori, x, y);
            return true;
        }
    }
    false
}

