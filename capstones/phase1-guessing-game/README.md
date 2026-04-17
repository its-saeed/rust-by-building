# Capstone — Guessing game

Phase 1 capstone. Completed after lesson 04 (Control flow).

You build a number-guessing game: the program picks a random number between 1 and 100, and the player has up to 7 tries to guess it. Each guess gets a hint: "too high", "too low", or "correct".

## What you'll practice

- Functions with parameters and return types (lesson 03)
- `if`/`else if`/`else` (lesson 04)
- Loops with early exit (lesson 04)
- Reading a line from stdin and parsing it
- Handling bad input gracefully

## Tests

`tests/game.rs` drives the game by piping input to the binary. You shouldn't need to modify it — just implement `src/main.rs` so the tests pass.

```sh
rbb test capstone phase1-guessing-game
```

(Full brief to be written.)
