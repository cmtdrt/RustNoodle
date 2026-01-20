mod board;
mod color;
mod config;
mod input;
mod model;
mod pieces;
mod puzzle;
mod render;
mod solver;
mod spinner;

use std::io::{self, Write};
use std::time::Duration;

use board::Board;
use color::{render_mode, set_render_mode, RenderMode};
use config::{BOARD_HEIGHT, BOARD_WIDTH, RESOLVE_TIMEOUT_SECS};
use input::parse_pos_a2;
use model::{Piece, PieceId};
use pieces::kanoodle_pieces;
use puzzle::{sample_puzzle, Puzzle};
use render::{print_all_pieces_row, print_legend, print_piece_shapes};
use solver::solve_to_board;
use spinner::{run_with_spinner, run_with_spinner_timeout};

fn main() {
    // Args: `cargo run -- --bg` ou `cargo run -- --modern`
    let args: Vec<String> = std::env::args().collect();
    let use_modern = args.iter().any(|a| a == "--modern");
    let use_bg = args.iter().any(|a| a == "--bg");
    let mode = if use_modern {
        RenderMode::Modern
    } else if use_bg {
        RenderMode::Background
    } else {
        RenderMode::Foreground
    };
    set_render_mode(mode);

    let puzzle = run_with_spinner("Génération d'un puzzle résolvable", || sample_puzzle());
    let mut board = puzzle.starting_board.clone();
    let mut available = puzzle.available_pieces.clone();
    let solution = puzzle.solution.clone();

    print_welcome();

    loop {
        println!();
        println!("Grille actuelle :");
        
        if render_mode() == RenderMode::Modern {
            print_legend();
            println!();
        }
        println!("{board}");

        if board.is_full() || available.is_empty() {
            println!("Bravo ! Tu as rempli la grille (ou utilisé toutes les pièces disponibles).");
            break;
        }

        print_remaining_pieces(&available);

        print!("Entre une commande : ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erreur de lecture. Réessaie.");
            continue;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if handle_command(input, &mut board, &mut available, &puzzle, &solution) {
            break;
        }
    }
}

fn print_welcome() {
    println!("=== Kanoodle – version texte interactive ===");
    println!("Grille: {}x{}", BOARD_WIDTH, BOARD_HEIGHT);
    println!("Commandes disponibles:");
    println!();
    println!("Placer une pièce:");
    println!("  - PIECE ORIENTATION POSITION (ex: C 0 A2)");
    println!("  - PIECE ORIENTATION X Y      (ex: C 0 0 2)");
    println!("Voir toutes les pièces:");
    println!("  - show all");
    println!("Voir les orientations d'une pièce:");
    println!("  - show PIECE (ex: show C)");
    println!("Retirer une pièce:");
    println!("  - del PIECE (ex: del C)");
    println!("Solution du puzzle initial (instantané) :");
    println!("  - solution");
    println!("Résoudre ce que tu as commencé (10s max) :");
    println!("  - resolve");
    println!("Autres:");
    println!("  - reset, quit");
}

fn print_remaining_pieces(available: &[Piece]) {
    println!("Pièces restantes - Orientations possibles");
    let mut sorted = available.to_vec();
    sorted.sort_by_key(|p| p.id.0);
    for p in &sorted {
        println!("{} - {}", p.id.0, p.orientations.len());
    }
}

/// Retourne true si la partie doit se terminer
fn handle_command(
    input: &str,
    board: &mut Board,
    available: &mut Vec<Piece>,
    puzzle: &Puzzle,
    solution: &Board,
) -> bool {
    match input.to_lowercase().as_str() {
        "quit" => {
            println!("Au revoir !");
            return true;
        }
        "reset" => {
            *board = puzzle.starting_board.clone();
            *available = puzzle.available_pieces.clone();
            println!("Puzzle réinitialisé.");
            return false;
        }
        "solution" => {
            println!("Solution (pour le puzzle initial) :");
            println!("{solution}");
            return true;
        }
        "resolve" => {
            return handle_resolve(board, available);
        }
        cmd if cmd == "show all" => {
            print_all_pieces_row();
            return false;
        }
        cmd if cmd.starts_with("show ") => {
            handle_show(cmd, available);
            return false;
        }
        cmd if cmd.starts_with("del ") => {
            handle_del(cmd, board, available);
            return false;
        }
        _ => {
            handle_place(input, board, available);
            return false;
        }
    }
}

fn handle_resolve(board: &Board, available: &[Piece]) -> bool {
    let board_snapshot = board.clone();
    let available_snapshot = available.to_vec();
    let solved = run_with_spinner_timeout(
        "Résolution (10s max)",
        Duration::from_secs(RESOLVE_TIMEOUT_SECS),
        move || solve_to_board(&board_snapshot, &available_snapshot),
    );

    match solved {
        Some(Some(sol)) => {
            println!();
            println!("Solution trouvée :");
            println!("{sol}");
            true
        }
        Some(None) => {
            println!("Aucune solution trouvée depuis l'état actuel.");
            false
        }
        None => {
            println!("Aucune solution trouvée dans le temps imparti (10s). La partie continue.");
            false
        }
    }
}

fn handle_show(cmd: &str, available: &[Piece]) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() != 2 {
        println!("Utilisation : show PIECE (ex: show C)");
        return;
    }
    let ch = parts[1].chars().next().unwrap_or(' ').to_ascii_uppercase();
    if let Some(p) = available.iter().find(|p| p.id.0 == ch) {
        print_piece_shapes(p);
    } else {
        println!("La pièce {} n'est pas disponible (déjà utilisée ou inexistante).", ch);
    }
}

fn handle_del(cmd: &str, board: &mut Board, available: &mut Vec<Piece>) {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.len() != 2 {
        println!("Utilisation : del PIECE (ex: del C)");
        return;
    }
    let ch = parts[1].chars().next().unwrap_or(' ').to_ascii_uppercase();
    let id = PieceId(ch);

    if ch == 'A' || ch == 'B' {
        println!("Tu ne peux pas retirer la pièce {} : elle fait partie de la position de départ.", ch);
        return;
    }

    let all_pieces = kanoodle_pieces();
    if let Some(p) = all_pieces.into_iter().find(|p| p.id == id) {
        if available.iter().any(|ap| ap.id == id) {
            println!("La pièce {} est déjà disponible, rien à retirer sur la grille.", ch);
        } else {
            board.remove_piece(id);
            available.push(p);
            println!("Pièce {} retirée de la grille et remise dans les pièces disponibles.", ch);
        }
    } else {
        println!("La pièce {} n'existe pas dans ce puzzle.", ch);
    }
}

fn handle_place(input: &str, board: &mut Board, available: &mut Vec<Piece>) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 && parts.len() != 4 {
        println!("Format invalide.");
        println!("  - PIECE ORIENTATION POSITION (ex: C 0 A2)");
        println!("  - PIECE ORIENTATION X Y (ex: C 0 0 2)");
        return;
    }

    let piece_char = match parts[0].chars().next() {
        Some(c) => c.to_ascii_uppercase(),
        None => {
            println!("Caractère de pièce invalide.");
            return;
        }
    };

    let ori_idx: usize = match parts[1].parse() {
        Ok(v) => v,
        Err(_) => {
            println!("Orientation invalide (doit être un entier).");
            return;
        }
    };

    let (x, y) = if parts.len() == 3 {
        match parse_pos_a2(parts[2]) {
            Some(pos) => pos,
            None => {
                println!("Position invalide. Utilise par ex: A2, B0, K4 (lettre A..K + ligne 0..4).");
                return;
            }
        }
    } else {
        let x: i32 = match parts[2].parse() {
            Ok(v) => v,
            Err(_) => {
                println!("X invalide (doit être un entier).");
                return;
            }
        };
        let y: i32 = match parts[3].parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Y invalide (doit être un entier).");
                return;
            }
        };
        (x, y)
    };

    let pos = available.iter().position(|p| p.id.0 == piece_char);
    let Some(idx) = pos else {
        println!("La pièce {} n'est pas disponible (déjà utilisée ou inexistante).", piece_char);
        return;
    };

    let piece = available[idx].clone();
    if ori_idx >= piece.orientations.len() {
        println!(
            "Orientation invalide pour la pièce {} (0..{}).",
            piece.id.0,
            piece.orientations.len() - 1
        );
        return;
    }

    let ori = &piece.orientations[ori_idx];
    if !board.can_place(ori, x, y) {
        println!("Impossible de placer la pièce {} à cette position.", piece.id.0);
        return;
    }

    board.place(piece.id, ori, x, y);
    available.remove(idx);

    println!(
        "Pièce {} placée à partir de ({}, {}) avec orientation {}.",
        piece.id.0, x, y, ori_idx
    );
}
