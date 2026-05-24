# Physics Primer

This page covers the physics concepts our engine is built on — position, velocity, acceleration, and vectors. No prior knowledge needed.

---

## Position

**Position** is where an object is in space. In our 2D engine, it's a point described by two numbers:

- `x` — horizontal distance from the left edge, in pixels
- `y` — vertical distance from the top edge, in pixels

The top-left corner of the window is `(0, 0)`. `x` grows to the right. `y` grows **downward** — this is the screen convention, opposite to the math classroom.

```
(0,0) ──────────────────► x
  │
  │     ● (400, 300)
  │
  ▼
  y
```

A ball at position `(400, 300)` sits 400 pixels from the left and 300 pixels from the top — the centre of an 800×600 window.

---

## Velocity

**Velocity** describes how position changes over time.

Think of a car on a motorway. Its position is where it is on the road. Its velocity is how fast it's moving, and in which direction — not just "100 km/h" but "100 km/h heading north."

In our engine, velocity is also two numbers:

- `velocity.x` — how many pixels per second the object moves horizontally
- `velocity.y` — how many pixels per second it moves vertically

A ball with velocity `(200, 0)` moves 200 pixels to the right per second and stays at the same height. A ball with velocity `(0, 150)` falls straight down at 150 pixels per second.

**The key relationship:**

```
position_next = position_now + velocity × time_elapsed
```

If you move at 200 pixels/second for 0.016 seconds, you travel `200 × 0.016 = 3.2 pixels`. That's one time step in our simulation.

---

## Acceleration

**Acceleration** describes how velocity changes over time.

Back to the car: the speedometer reading is velocity. Pressing the accelerator pedal changes that reading — that's acceleration. Hard braking is negative acceleration.

Gravity is a constant downward acceleration. It doesn't move objects directly — it steadily increases their downward velocity. That's why a falling ball speeds up: each step gravity adds a bit more downward velocity, so the next step it falls a bit further.

```
velocity_next = velocity_now + acceleration × time_elapsed
```

In our engine, gravity is `500` pixels per second per second downward. After one second of freefall from rest, a ball has `velocity.y = 500`. After two seconds, `velocity.y = 1000`.

---

## The full picture: a → v → p

The three values form a chain. Each one is the rate of change of the next:

```
acceleration  ──changes──►  velocity  ──changes──►  position
```

Each time step, we apply both steps in order:

```
velocity += acceleration * dt    ← forces change velocity
position += velocity    * dt     ← velocity changes position
```

This two-step process is called **Euler integration** — the simplest way to simulate continuous physics in discrete steps. It's not perfect (small errors accumulate over time), but it's good enough for a game and easy to understand.

---

## Scalars vs vectors

A **scalar** is just a number — temperature, mass, time. It has a magnitude but no direction.

A **vector** has both a **magnitude** (size) and a **direction**.

Speed is a scalar: "100 km/h." Velocity is a vector: "100 km/h heading north." The direction is part of the value.

In 2D, we represent vectors as a pair `(x, y)`. The two components together encode both direction and magnitude:

| Vector | Meaning |
|--------|---------|
| `(200, 0)` | Moving right at 200 px/s |
| `(-200, 0)` | Moving left at 200 px/s |
| `(0, 150)` | Falling down at 150 px/s |
| `(100, 100)` | Moving diagonally (right and down equally) |

The actual speed — the magnitude of the vector `(x, y)` — is `√(x² + y²)`. For `(100, 100)` that's `√20000 ≈ 141` pixels/second.

---

## Vector arithmetic

Vectors add component by component:

```
(3, 2) + (1, 4) = (4, 6)
```

Physically: if you're moving at `(3, 2)` and a force pushes you by `(1, 4)`, the result is `(4, 6)`.

Multiplying a vector by a scalar scales its magnitude:

```
(3, 2) × 5 = (15, 10)
```

Physically: five seconds at velocity `(3, 2)` means you've moved `(15, 10)` pixels total.

These are the only two operations our engine needs for basic motion. The `Vec2` type you build in lesson 2 implements both.

---

## What the engine does

Here's the full simulation loop in physics terms:

1. **Apply forces** — gravity adds downward acceleration to each body's velocity
2. **Integrate** — advance each body's position by its velocity × dt
3. **Detect collisions** — check if any bodies overlap
4. **Resolve collisions** — adjust velocities so bodies bounce correctly
5. **Enforce boundaries** — keep bodies inside the screen
6. **Draw** — render each body at its current position

Projects 4 and 5 cover steps 1, 2, 5, and 6. Projects 6 and 7 add steps 3 and 4.

The game loop mechanics — what `dt` is, what a frame is, how FPS works — are explained in lesson 1, alongside the code that implements them.
