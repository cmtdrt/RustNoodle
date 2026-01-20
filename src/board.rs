use std::fmt;

use crate::color::colorize_piece_char;
use crate::config::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::model::{Orientation, PieceId};

#[derive(Clone)]
pub struct Board {
    pub cells: [[Option<PieceId>; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    // Vérifie si une coordonnée est à l'intérieur du plateau
    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < BOARD_WIDTH && (y as usize) < BOARD_HEIGHT
    }

    // Vérifie si une pièce peut être placée à une position donnée
    pub fn can_place(&self, ori: &Orientation, x0: i32, y0: i32) -> bool {
        for (dx, dy) in &ori.cells {
            let x = x0 + dx;
            let y = y0 + dy;
            if !self.is_inside(x, y) {
                return false;
            }
            if self.cells[y as usize][x as usize].is_some() {
                return false;
            }
        }
        true
    }

    // Place une pièce sur le plateau
    pub fn place(&mut self, piece_id: PieceId, ori: &Orientation, x0: i32, y0: i32) {
        for (dx, dy) in &ori.cells {
            let x = (x0 + dx) as usize;
            let y = (y0 + dy) as usize;
            self.cells[y][x] = Some(piece_id);
        }
    }

    // Retire une pièce du plateau (plus performant que remove_piece qui balaye tout le plateau)
    pub fn remove(&mut self, piece_id: PieceId, ori: &Orientation, x0: i32, y0: i32) {
        for (dx, dy) in &ori.cells {
            let x = (x0 + dx) as usize;
            let y = (y0 + dy) as usize;
            if self.cells[y][x] == Some(piece_id) {
                self.cells[y][x] = None;
            }
        }
    }

    // Vérifie si le plateau est rempli
    pub fn is_full(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|c| c.is_some()))
    }

    // Retire une pièce du plateau
    pub fn remove_piece(&mut self, id: PieceId) {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.cells[y][x] == Some(id) {
                    self.cells[y][x] = None;
                }
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "   {}",
            (0..BOARD_WIDTH)
                .map(|i| (b'A' + i as u8) as char)
                .collect::<String>()
        )?;
        for (y, row) in self.cells.iter().enumerate() {
            write!(f, "{:2} ", y)?;
            for cell in row {
                match cell {
                    Some(id) => write!(f, "{}", colorize_piece_char(*id))?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

