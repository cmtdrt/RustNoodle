use crate::config::BOARD_WIDTH;

// Convertit une lettre de colonne en indice d'abscisse
pub fn col_letter_to_x(c: char) -> Option<i32> {
    let uc = c.to_ascii_uppercase();
    let max = (b'A' + (BOARD_WIDTH as u8) - 1) as char;
    if ('A'..=max).contains(&uc) {
        Some((uc as u8 - b'A') as i32)
    } else {
        None
    }
}

// Parse une position de type "A2" -> (x, y)
pub fn parse_pos_a2(token: &str) -> Option<(i32, i32)> {
    let t = token.trim();
    if t.len() < 2 {
        return None;
    }
    let mut chars = t.chars();
    let col = chars.next()?;
    let x = col_letter_to_x(col)?;
    let row_str: String = chars.collect();
    let y: i32 = row_str.parse().ok()?;
    Some((x, y))
}

