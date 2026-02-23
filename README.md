# RustNoodle

A terminal version of the Kanoodle game written in Rust: place all pieces (polyominoes) on a 5x11 grid. Two pieces are pre‑placed at random, and you complete the rest of the grid to solve the puzzle.

The game guarantees that you never get an unsolvable position. If random generation takes more than 10 seconds, it falls back to a default puzzle with a known solution.

## Run

There are 3 options that control the display mode:

```bash
cargo run                  # classic mode
cargo run -- --bg          # background color + contrasting letter
cargo run -- --modern      # background color only + legend
```

## Display modes

- **Classic (default)**: colored letters.  
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/8745e683-a8be-44b3-b6ff-f2ba7d005e66" />
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/5b33f0c6-d35f-404b-b6c8-1de1ed63b959" />

- **`--bg`**: colored background + contrasting letter.  
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/232057f8-94ff-4464-b0dc-355c1e9ec33d" />
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/7bc71ddb-9757-4c27-b561-2544d68093f8" />

- **`--modern`**: colored background without letters; a legend `A=□` … `L=□` is shown above the grid.  
<img width="513" height="145" alt="image" src="https://github.com/user-attachments/assets/23d0b7a9-3ff2-43b8-8072-b887a3720d2d" />
<img width="273" height="109" alt="image" src="https://github.com/user-attachments/assets/b07e6f51-ddfa-4b54-a056-68dc0c7a550f" />

## In‑game commands

- Place a piece  
  - `PIECE ORIENTATION POSITION` (e.g. `C 0 A2`)  
  - `PIECE ORIENTATION X Y`      (e.g. `C 0 0 2`)
- Show all pieces (orientation 0): `show all`  
<img width="524" height="66" alt="image" src="https://github.com/user-attachments/assets/9567fbbc-ced1-48e2-9471-34fe2f775360" />

- Show all orientations of a piece: `show PIECE` (e.g. `show C`)  
<img width="524" height="66" alt="image" src="https://github.com/user-attachments/assets/cb3e25fd-45ea-494b-8994-15e26711ebf8" />
- Remove a placed piece: `del PIECE` (e.g. `del C`)
- Show the initial puzzle solution and quit: `solution`
- Try to solve the current state (10s max): `resolve`
- Reset: `reset`
- Quit: `quit`

Have fun!
