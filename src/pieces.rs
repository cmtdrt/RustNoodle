use crate::model::{Orientation, Piece, PieceId};

/// Génère les rotations et symétries d’une forme de base.
fn generate_orientations(base: &[(i32, i32)]) -> Vec<Orientation> {
    let mut all = Vec::new();

    let transforms: &[fn(i32, i32) -> (i32, i32)] = &[
        // rotations 0, 90, 180, 270 et leurs miroirs
        |x, y| (x, y),
        |x, y| (-x, y),
        |x, y| (y, x),
        |x, y| (-y, x),
        |x, y| (x, -y),
        |x, y| (y, -x),
        |x, y| (-x, -y),
        |x, y| (-y, -x),
    ];

    for t in transforms {
        let mut coords: Vec<(i32, i32)> = base.iter().map(|(x, y)| t(*x, *y)).collect();
        let min_x = coords.iter().map(|(x, _)| *x).min().unwrap();
        let min_y = coords.iter().map(|(_, y)| *y).min().unwrap();
        for c in &mut coords {
            c.0 -= min_x;
            c.1 -= min_y;
        }
        coords.sort();
        if !all.iter().any(|o: &Orientation| o.cells == coords) {
            all.push(Orientation { cells: coords });
        }
    }
    all
}

// Génère les différentes pièces du jeu
pub fn kanoodle_pieces() -> Vec<Piece> {
    let defs: Vec<(char, Vec<(i32, i32)>)> = vec![
        ('A', vec![(0, 0), (1, 0), (2, 0), (0, 1)]),
        ('B', vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
        ('C', vec![(0, 0), (1, 0), (0, 1), (0, 2), (1, 2)]),
        ('D', vec![(2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]),
        ('E', vec![(0, 0), (1, 0), (1, 1), (2, 1), (2, 2)]),
        ('F', vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 1)]),
        ('G', vec![(0, 0), (1, 0), (0, 1)]),
        ('H', vec![(0, 0), (1, 0), (2, 0), (3, 0), (2, 1)]),
        ('I', vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        ('J', vec![(0, 0), (1, 0), (2, 0), (3, 0), (3, 1)]),
        ('K', vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        ('L', vec![(0, 0), (1, 0), (2, 0), (0, 1), (0, 2)]),
    ];

    defs.into_iter()
        .map(|(c, base)| Piece {
            id: PieceId(c),
            orientations: generate_orientations(&base),
        }).collect()
}

