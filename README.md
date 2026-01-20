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
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/8745e683-a8be-44b3-b6ff-f2ba7d005e66" />
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/5b33f0c6-d35f-404b-b6c8-1de1ed63b959" />


- **`--bg`** : fond coloré + lettre contrastée.
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/232057f8-94ff-4464-b0dc-355c1e9ec33d" />
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/7bc71ddb-9757-4c27-b561-2544d68093f8" />


- **`--modern`** : fond coloré sans lettre ; une légende `A=□` … `L=□` est affichée au-dessus de la grille.
<img width="513" height="145" alt="image" src="https://github.com/user-attachments/assets/23d0b7a9-3ff2-43b8-8072-b887a3720d2d" />
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/b07e6f51-ddfa-4b54-a056-68dc0c7a550f" />



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
