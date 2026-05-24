# Collision Physics

This page covers the math behind velocity-based collision response — the dot product, relative velocity, and impulse. Lesson 4 uses all three.

---

## The dot product

The **dot product** of two vectors is:

```
a · b = a.x * b.x + a.y * b.y
```

That's it — multiply the components, add them up. The result is a single number (a scalar), not a vector.

### What does it mean?

The dot product measures **how much two vectors point in the same direction**.

If `n` is a unit vector (length = 1.0), then `a · n` gives the length of `a`'s projection onto `n` — how far `a` reaches in the direction of `n`:

```
          a
          ▲
         /|
        / |
       /  |
──────●───┼────► n
      │◄─►│
      a · n
```

| Value of `a · n` | Meaning |
|-----------------|---------|
| positive | `a` has a component in the same direction as `n` |
| zero | `a` is perpendicular to `n` |
| negative | `a` has a component opposite to `n` |

This turns out to be exactly what we need to determine whether two balls are approaching each other.

---

## The collision normal

When two circles overlap, the **collision normal** `n` is the unit vector from the center of ball A to the center of ball B:

```
  ●────────►────────●
  A        n        B
```

It points in the direction of the collision — perpendicular to the contact surface.

---

## Relative velocity

Both balls are moving. What matters isn't the absolute velocity of each — it's **how fast they're moving toward each other**.

The relative velocity of A with respect to B is:

```
vrel = va - vb
```

If A is moving right at 200 px/s and B is stationary, `vrel = (200, 0)`.  
If A is moving right at 200 and B is moving right at 150, `vrel = (50, 0)` — A approaches B at only 50 px/s.

### Speed along the normal

We only care about motion along the collision axis (the normal). Movement perpendicular to the normal doesn't affect the collision at all.

```
vn = dot(vrel, n)
```

- `vn > 0` — A is approaching B along the normal → respond
- `vn ≤ 0` — A is moving away from B → already separating, skip

This check avoids "phantom collisions": two balls that are still overlapping from a previous frame but are already moving apart. Applying an impulse in that case would make them stick instead of separate.

---

## Impulse

An **impulse** is an instantaneous change in velocity — a sudden push applied at the moment of contact.

We apply equal and opposite impulses to both balls along the collision normal:

```
va  +=  (j / ma) * n      ← A pushed away from B
vb  -=  (j / mb) * n      ← B pushed away from A
```

Where `j` is the impulse magnitude, `ma` and `mb` are the masses, and `n` is the normal (pointing from A toward B).

### Finding the impulse magnitude

We want the collision to be **elastic** — kinetic energy is conserved, balls bounce without losing speed. For an elastic collision the relative velocity along the normal reverses:

```
vn_after = -e * vn_before
```

`e` is the **coefficient of restitution**: `1.0` = perfectly elastic (full bounce), `0.0` = perfectly inelastic (balls stick).

Working out the algebra gives:

```
j = -(1 + e) * vn / (1/ma + 1/mb)
```

With `e = 1` and equal masses `m`:

```
j = -2 * vn / (2/m) = -m * vn
```

Since `vn > 0` when approaching, `j` is negative — meaning the push is in the `-n` direction for A and `+n` for B, which is correct: both balls bounce away from the contact point.

---

## The full response, step by step

```
1. Compute vrel = va - vb
2. Compute vn = dot(vrel, n)
3. If vn ≤ 0, skip (already separating)
4. Compute j = -(1 + e) * vn / (1/ma + 1/mb)
5. va += (j / ma) * n
6. vb -= (j / mb) * n
```

This is what lesson 4 implements in Rust.
