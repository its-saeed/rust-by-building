# Complex Numbers — The Math Behind Mandelbrot

You do not need a mathematics degree to render the Mandelbrot set. You need to understand one idea: a complex number is a point in a 2D plane, and multiplying complex numbers has a simple formula. That is all.

---

## Real numbers are a line

The numbers you use every day — 0, 1, -3, 3.14, -½ — live on a number line. One dimension.

```
 ←────────────────────────────────→
 -3   -2   -1    0    1    2    3
```

---

## Complex numbers are a plane

A **complex number** adds a second dimension. It is a pair of real numbers written as:

```
a + bi
```

where `a` is the **real part** and `b` is the **imaginary part**. The letter `i` is defined as the square root of -1 — but you do not need to worry about what that means philosophically. In practice, `i` is just a label that keeps the two dimensions from colliding when you multiply.

You can think of `a + bi` as simply a point `(a, b)` on a 2D grid called the **complex plane**:

```
imaginary axis
      ↑
   2i │         • 1 + 2i
      │
   1i │    • 0 + 1i
      │
   0  ├──────────────────→ real axis
      │ 0    1    2    3
  -1i │
      │
  -2i │
```

`3 + 0i` is just the real number 3. `0 + 2i` is a point on the imaginary axis. `1 + 2i` is the point (1, 2). Complex numbers generalise real numbers to two dimensions.

---

## Adding complex numbers

Add real parts, add imaginary parts — same as adding 2D vectors:

```
(a + bi) + (c + di) = (a+c) + (b+d)i

Example:
(1 + 2i) + (3 + 1i) = 4 + 3i
```

---

## Multiplying complex numbers

This is where `i` matters. Use ordinary algebra, but replace `i²` with `-1`:

```
(a + bi)(c + di)
= ac + adi + bci + bdi²
= ac + adi + bci + bd(-1)
= (ac - bd) + (ad + bc)i
```

So the result is a new point: real part `ac - bd`, imaginary part `ad + bc`.

**Example**: `(1 + 2i) × (3 + 1i)`
- real: 1×3 - 2×1 = 1
- imaginary: 1×1 + 2×3 = 7
- result: `1 + 7i`

In Rust this looks like:

```rust
let (a, b) = (1.0_f64, 2.0);  // 1 + 2i
let (c, d) = (3.0_f64, 1.0);  // 3 + 1i

let real = a*c - b*d;          // 1.0
let imag = a*d + b*c;          // 7.0
// result: 1 + 7i
```

---

## Squaring a complex number

`z² = (a + bi)(a + bi)`:
- real: `a² - b²`
- imaginary: `2ab`

In Rust:

```rust
let (a, b) = (x, y);           // z = x + yi
let x_new = a*a - b*b;         // real part of z²
let y_new = 2.0 * a * b;       // imaginary part of z²
```

This is the core of the Mandelbrot computation.

---

## Magnitude (distance from origin)

The **magnitude** of `a + bi` is its distance from the origin `(0, 0)`:

```
|a + bi| = √(a² + b²)
```

This is just the Pythagorean theorem. We use the magnitude to check whether a number has "escaped" — grown far from the origin.

In code we compare `a² + b²` against `4` (same as comparing the magnitude against `2`) to avoid the expensive square root:

```rust
if x*x + y*y > 4.0 {
    // |z| > 2, it has escaped
}
```

---

## The Mandelbrot iteration

For each point `c = cx + cy·i` in the complex plane, we iterate:

```
z₀ = 0
z₁ = z₀² + c
z₂ = z₁² + c
z₃ = z₂² + c
...
```

Two things can happen:
- `|z|` grows past 2 — the point **escapes**. It is *not* in the Mandelbrot set. The iteration count at escape determines its colour.
- `|z|` stays small forever — the point is **in the set**. We colour it black.

In practice we cap the iteration count at some maximum (say 256). If the point has not escaped by then, we call it "in the set."

```
For each pixel (px, py):
  map to complex coordinates (cx, cy)
  x = 0, y = 0
  for i in 0..max_iter:
    if x² + y² > 4:  → escaped at iteration i → colour by i
    xnew = x² - y² + cx
    y    = 2xy + cy
    x    = xnew
  → did not escape → colour black
```

That is the entire algorithm. The rest is mapping screen coordinates to complex coordinates and mapping iteration counts to colours.

---

## Why it produces fractal structure

Points on the boundary of the set escape slowly — they need many iterations. Points far away escape quickly. The richness of the image comes from this boundary being infinitely complex: zoom in anywhere on the edge and you find new spirals, tendrils, and copies of the whole set.

None of that changes the code. The same 10-line loop produces infinite complexity.
