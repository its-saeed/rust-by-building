# Lesson 2 — Characters & Arithmetic

> **Goal**: Shift a single character by the key. The first real piece of the cipher.
>
> **Concepts**: `char`, ASCII, binary representation, type casting with `as`, `u8`, integer arithmetic.

---

## The `char` type

In Rust, a single character is written with **single quotes**:

```rust
let letter = 'H';
let exclamation = '!';
let digit = '3';
```

This is different from a string (`&str`), which uses double quotes and holds many characters:

```rust
let word = "Hello";   // &str — multiple characters
let letter = 'H';     // char — exactly one character
```

They are not interchangeable. `'H'` and `"H"` look similar but are different types. Mixing them up is a compile error.

---

## Characters are numbers

Every character your computer stores is actually a number. The mapping from numbers to characters is called **ASCII** (American Standard Code for Information Interchange).

Here are the ones we care about for the Caesar cipher:

| Character | ASCII value |
|-----------|-------------|
| `'A'`     | 65          |
| `'B'`     | 66          |
| `'C'`     | 67          |
| ...       | ...         |
| `'Z'`     | 90          |
| `'a'`     | 97          |
| `'b'`     | 98          |
| `'c'`     | 99          |
| ...       | ...         |
| `'z'`     | 122         |

Uppercase runs from 65 to 90. Lowercase runs from 97 to 122. Both ranges span exactly 26 values — one per letter.

Notice: `'a'` is `'A' + 32`. That gap of 32 is how uppercase and lowercase relate in ASCII. We'll use this in lesson 5.

---

## What numbers look like in binary

Computers don't store the number 65 as a decimal — they store **bits**: zeros and ones. Eight bits make one **byte**.

65 in binary:

```
65 = 64 + 1
   = 2⁶  +  2⁰
   = 0 1 0 0 0 0 0 1
```

Each position is a power of 2, counted right to left from 0:

```
Position:  7    6    5    4    3    2    1    0
Power:    128   64   32   16    8    4    2    1
Bit:        0    1    0    0    0    0    0    1

Result: 0×128 + 1×64 + 0×32 + 0×16 + 0×8 + 0×4 + 0×2 + 1×1 = 65
```

So `'A'` is stored as `01000001`.

Now `'a'` (97):

```
97 = 64 + 32 + 1
   = 0 1 1 0 0 0 0 1
```

Compare the two:

```
'A' = 0 1 0 0 0 0 0 1   (65)
'a' = 0 1 1 0 0 0 0 1   (97)
          ^
          bit 5 is flipped
```

The only difference is **bit 5** (the 32s position). Flipping one bit converts between uppercase and lowercase. ASCII was designed this way on purpose.

For the Caesar cipher, the key insight is: **shifting a letter means adding to its ASCII number**. `'H'` is 72. Shift by 3: `72 + 3 = 75`. And 75 is `'K'`. That's the whole cipher.

---

## The `u8` type

To work with ASCII values we need a number type that fits in one byte. That's `u8`:

- `u` — **unsigned** (no negative numbers, starts at 0)
- `8` — **8 bits**, values from 0 to 255

ASCII characters all have values between 0 and 127, which fits in a `u8`.

Compare with `i32` from lesson 1:

| Type | Signed? | Bits | Range |
|------|---------|------|-------|
| `i32` | yes | 32 | −2,147,483,648 to 2,147,483,647 |
| `u8`  | no  |  8 | 0 to 255 |

Use `u8` when working with single ASCII characters. Use `i32` for general integers.

---

## Type casting with `as`

Rust never automatically converts between types. A `char` is not a `u8`, and you cannot add them directly. You cast explicitly with `as`:

```rust
let letter = 'H';
let code = letter as u8;   // char → u8: gives 72
```

And back:

```rust
let code: u8 = 75;
let letter = code as char;   // u8 → char: gives 'K'
```

`as` converts the value to the type you specify. It always requires you to be explicit — Rust never silently changes types for you.

### Chaining casts

You can cast in a chain:

```rust
let letter = 'H';
let shifted = (letter as u8 + 3) as char;
```

Step by step:
1. `letter as u8` → `72`
2. `72 + 3` → `75`
3. `75 as char` → `'K'`

The parentheses around `letter as u8 + 3` matter — they make sure the addition happens before the final cast to `char`.

---

## What about `'Z' + 3`?

If we naively shift `'Z'` (90) by 3:

```rust
let shifted = ('Z' as u8 + 3) as char;
println!("{}", shifted);   // prints ']' — wrong!
```

90 + 3 = 93, and ASCII 93 is `']'`, not `'C'`. The cipher is supposed to **wrap around**: after `'Z'` comes `'A'` again.

We'll fix this in lesson 5 using the modulo operator (`%`). For now, our cipher only works on letters that don't overflow the alphabet. That's fine — we're building incrementally.

---

## Putting it together

After lesson 2, the project prints a single shifted character:

```rust
fn main() {
    let message = "Hello, World!";
    let key: u8 = 3;
    let first_char = 'H';

    let shifted = (first_char as u8 + key) as char;

    println!("=== Caesar Cipher ===");
    println!("Message : {}", message);
    println!("Key     : {}", key);
    println!("'{}' + {} = '{}'", first_char, key, shifted);
}
```

Output:

```
=== Caesar Cipher ===
Message : Hello, World!
Key     : 3
'H' + 3 = 'K'
```

Only one character shifted. The full message comes in lesson 4.

---

## Exercises

Run:

```sh
rbb watch caesar-02
```

Four exercises. Each targets exactly one concept from this lesson. When all pass, open `project/src/main.rs` and add the single-character shift.
