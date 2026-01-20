use crate::color::{colorize_piece_char, legend_swatch};
use crate::model::PieceId;
use crate::model::Piece;
use crate::pieces::kanoodle_pieces;

// Affiche la légende des pièces
pub fn print_legend() {
    let mut pieces = kanoodle_pieces();
    pieces.sort_by_key(|p| p.id.0);

    let mut line = String::new();
    for (i, p) in pieces.iter().enumerate() {
        if i > 0 {
            line.push_str("  ");
        }
        line.push(p.id.0);
        line.push('=');
        line.push_str(&legend_swatch(PieceId(p.id.0)));
    }
    println!("{line}");
}

// Affiche les formes possibles d'une pièce
pub fn print_piece_shapes(piece: &Piece) {
    println!("Formes possibles pour la pièce {} :", piece.id.0);
    if piece.orientations.is_empty() {
        return;
    }

    let mut grids: Vec<Vec<Vec<char>>> = Vec::new();
    let mut heights: Vec<usize> = Vec::new();
    let mut widths: Vec<usize> = Vec::new();

    for ori in &piece.orientations {
        let max_x = ori.cells.iter().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = ori.cells.iter().map(|(_, y)| *y).max().unwrap_or(0);
        let w = (max_x + 1) as usize;
        let h = (max_y + 1) as usize;

        let mut grid = vec![vec!['.'; w]; h];
        for (x, y) in &ori.cells {
            if *x >= 0 && *y >= 0 {
                grid[*y as usize][*x as usize] = piece.id.0;
            }
        }

        heights.push(h);
        widths.push(w);
        grids.push(grid);
    }

    if grids.is_empty() {
        return;
    }

    let max_h = *heights.iter().max().unwrap();
    for row in 0..max_h {
        let mut line = String::new();
        for (idx, grid) in grids.iter().enumerate() {
            if row < grid.len() {
                for ch in &grid[row] {
                    if *ch == piece.id.0 {
                        line.push_str(&colorize_piece_char(piece.id));
                    } else {
                        line.push(*ch);
                    }
                }
            } else {
                for _ in 0..widths[idx] {
                    line.push(' ');
                }
            }
            if idx < grids.len() - 1 {
                line.push(' ');
                line.push('|');
                line.push(' ');
            }
        }
        println!("{line}");
    }
}

// Affiche toutes les pièces en une seule ligne
pub fn print_all_pieces_row() {
    let pieces = kanoodle_pieces();
    if pieces.is_empty() {
        return;
    }

    let mut grids: Vec<Vec<Vec<char>>> = Vec::new();
    let mut heights: Vec<usize> = Vec::new();
    let mut widths: Vec<usize> = Vec::new();

    for p in &pieces {
        if p.orientations.is_empty() {
            continue;
        }
        let ori = &p.orientations[0];
        let max_x = ori.cells.iter().map(|(x, _)| *x).max().unwrap_or(0);
        let max_y = ori.cells.iter().map(|(_, y)| *y).max().unwrap_or(0);
        let w = (max_x + 1) as usize;
        let h = (max_y + 1) as usize;

        let mut grid = vec![vec!['.'; w]; h];
        for (x, y) in &ori.cells {
            if *x >= 0 && *y >= 0 {
                grid[*y as usize][*x as usize] = p.id.0;
            }
        }

        heights.push(h);
        widths.push(w);
        grids.push(grid);
    }

    if grids.is_empty() {
        return;
    }

    let max_h = *heights.iter().max().unwrap();
    let spacing = 3;

    for row in 0..max_h {
        let mut line = String::new();
        for (idx, grid) in grids.iter().enumerate() {
            if row < grid.len() {
                for ch in &grid[row] {
                    if *ch == pieces[idx].id.0 {
                        line.push_str(&colorize_piece_char(pieces[idx].id));
                    } else {
                        line.push(*ch);
                    }
                }
            } else {
                for _ in 0..widths[idx] {
                    line.push(' ');
                }
            }
            for _ in 0..spacing {
                line.push(' ');
            }
        }
        println!("{line}");
    }
}

