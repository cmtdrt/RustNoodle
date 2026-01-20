use crate::model::PieceId;
use std::sync::atomic::{AtomicU8, Ordering};

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_WHITE_TEXT: &str = "\x1b[38;2;255;255;255m"; // Texte blanc pour contraste
const ANSI_BLACK_TEXT: &str = "\x1b[38;2;0;0;0m"; // Texte noir pour fonds clairs

/// 0 = texte coloré (défaut)
/// 1 = fond coloré + lettre contrastée (--bg)
/// 2 = fond coloré sans lettre (--modern)
static RENDER_MODE: AtomicU8 = AtomicU8::new(0);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RenderMode {
    Foreground = 0,
    Background = 1,
    Modern = 2,
}

pub fn set_render_mode(mode: RenderMode) {
    RENDER_MODE.store(mode as u8, Ordering::Relaxed);
}

pub fn render_mode() -> RenderMode {
    match RENDER_MODE.load(Ordering::Relaxed) {
        1 => RenderMode::Background,
        2 => RenderMode::Modern,
        _ => RenderMode::Foreground,
    }
}

// Palette de couleurs - foreground
fn ansi_fg_color_for_piece(id: PieceId) -> &'static str {
    match id.0 {
        'A' => "\x1b[38;2;255;165;0m",     // orange
        'B' => "\x1b[38;2;144;238;144m",   // vert clair
        'C' => "\x1b[38;2;255;255;0m",     // jaune
        'D' => "\x1b[38;2;0;100;0m",       // vert foncé
        'E' => "\x1b[38;2;255;20;147m",    // rose foncé
        'F' => "\x1b[38;2;255;0;0m",       // rouge
        'G' => "\x1b[38;2;192;192;192m",   // gris clair
        'H' => "\x1b[38;2;255;160;122m",   // rose saumon clair
        'I' => "\x1b[38;2;97;38;144m",     // violet foncé
        'J' => "\x1b[38;2;2;54;153m",      // bleu très foncé
        'K' => "\x1b[38;2;128;128;128m",   // gris
        'L' => "\x1b[38;2;173;216;230m",   // bleu clair
        _ => "\x1b[38;2;255;255;255m",     // Par défaut : blanc
    }
}

// Palette de couleurs - background
fn ansi_bg_color_for_piece(id: PieceId) -> &'static str {
    match id.0 {
        'A' => "\x1b[48;2;255;165;0m",     // orange
        'B' => "\x1b[48;2;144;238;144m",   // vert clair
        'C' => "\x1b[48;2;255;255;0m",     // jaune
        'D' => "\x1b[48;2;0;100;0m",       // vert foncé
        'E' => "\x1b[48;2;255;20;147m",    // rose foncé
        'F' => "\x1b[48;2;255;0;0m",       // rouge
        'G' => "\x1b[48;2;192;192;192m",   // gris clair
        'H' => "\x1b[48;2;255;160;122m",   // rose saumon clair
        'I' => "\x1b[48;2;97;38;144m",     // violet foncé
        'J' => "\x1b[48;2;2;54;153m",      // bleu très foncé
        'K' => "\x1b[48;2;128;128;128m",   // gris
        'L' => "\x1b[48;2;173;216;230m",   // bleu clair
        _ => "\x1b[48;2;255;255;255m",     // Par défaut : blanc
    }
}

// Vérifie si le fond est clair
fn is_light_background(id: PieceId) -> bool {
    match id.0 {
        'B' | 'C' | 'G' | 'H' | 'L' => true,
        _ => false,
    }
}

// Colorise un caractère de pièce
pub fn colorize_piece_char(id: PieceId) -> String {
    match render_mode() {
        RenderMode::Foreground => format!("{}{}{}", ansi_fg_color_for_piece(id), id.0, ANSI_RESET),
        RenderMode::Background => {
            let bg_color = ansi_bg_color_for_piece(id);
            let text_color = if is_light_background(id) {
                ANSI_BLACK_TEXT
            } else {
                ANSI_WHITE_TEXT
            };
            format!("{}{}{}{}", bg_color, text_color, id.0, ANSI_RESET)
        }
        RenderMode::Modern => {
            // Une cellule "vide" en fond coloré. On garde largeur 1 pour ne pas casser l'alignement.
            let bg_color = ansi_bg_color_for_piece(id);
            format!("{}{}{}", bg_color, " ", ANSI_RESET)
        }
    }
}

// Petit "swatch" de couleur pour la légende (fond coloré, sans lettre), largeur 2.
pub fn legend_swatch(id: PieceId) -> String {
    let bg_color = ansi_bg_color_for_piece(id);
    format!("{}  {}", bg_color, ANSI_RESET)
}

