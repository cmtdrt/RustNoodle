# RustNoodle

Une version terminal du jeu Kanoodle écrite en Rust : placer toutes les pièces (polyominos) sur une grille 5x11. Deux pièces sont pré‑placées au hasard, et tu complètes le reste de la grille pour résoudre le puzzle.

Le jeu s'assure de ne pas avoir de partie impossible à résoudre. Si jamais la génération aléatoire dépasse 10s, on bascule sur un puzzle par défaut.

## Lancer

Il existe 3 options qui affectent le mode d’affichage :
```bash
cargo run
cargo run -- --bg
cargo run -- --modern
```

## Modes d’affichage

- **Classique (défaut)** : lettres colorées.
- **`--bg`** : fond coloré + lettre contrastée.
- **`--modern`** : fond coloré sans lettre ; une légende `A=□` … `L=□` est affichée au-dessus de la grille.

## Commandes en jeu

- Placer une pièce  
  - `PIECE ORIENTATION POSITION` (ex: `C 0 A2`)  
  - `PIECE ORIENTATION X Y`      (ex: `C 0 0 2`)
- Voir toutes les pièces (orientation 0) : `show all`
- Voir les orientations d’une pièce : `show PIECE` (ex: `show C`)
- Retirer une pièce placée : `del PIECE` (ex: `del C`)
- Afficher la solution du puzzle initial : `solution`
- Résoudre l’état courant (10s max) : `resolve`
- Réinitialiser : `reset`
- Quitter : `quit`

Have fun !