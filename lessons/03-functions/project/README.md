# Project — Calculator library

A small calculator exposed as a library. You implement five functions; the test suite checks them.

## What to build

Open `src/lib.rs`. You'll see five function signatures marked with `todo!()`. Replace each with a real implementation.

| Function         | Signature                          | Behavior                              |
|------------------|------------------------------------|---------------------------------------|
| `add`            | `(a: i32, b: i32) -> i32`          | Sum                                   |
| `sub`            | `(a: i32, b: i32) -> i32`          | Difference                            |
| `mul`            | `(a: i32, b: i32) -> i32`          | Product                               |
| `is_positive`    | `(n: i32) -> bool`                 | `true` if strictly greater than 0     |
| `describe_sign`  | `(n: i32) -> &'static str`         | Returns `"positive"`, `"negative"`, or `"zero"` |

## Run the tests

```sh
rbb test 03
```

All six tests should pass. No warnings either — `cargo test` with warnings-as-errors is what `rbb test` runs under the hood.

## Constraints

- No `if let`, no `match` — those come later. Use `if`/`else` only.
- No libraries. Stdlib only. Don't add anything to `Cargo.toml`.
