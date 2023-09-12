# Void Stranger Solver

**This code contains small but real spoilers for Void Stranger.**

Solver for (certain classes of) [Void Stranger](https://store.steampowered.com/app/2121980/Void_Stranger/) puzzles.

If your interest in [Sokoban-type](https://en.wikipedia.org/wiki/Sokoban) puzzles is anything north of "I can tolerate them", you should [go play Void Stranger](https://store.steampowered.com/app/2121980/Void_Stranger/) right now.

This repo is more for sharing fun code than actual useful software for solving puzzles.

## Running

With [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed:
```bash
cargo test --release
```

will run the test suite of all hardcoded levels.
The test suite can be run without `--release`, but it's much much slower.

```bash
cargo run --release
```

will solve a level and print out the sequence of game states involved in the solve.
Currently, the level to solve is hardcoded.
