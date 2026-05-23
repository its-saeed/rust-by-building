# Lesson 6 — Polish

> **Goal**: Give each ball a random color and display the body count on screen.
>
> **Concepts**: adding a field to an existing struct, `Color`, `draw_text`, `format!`, random numbers.

---

## Adding color to `Body`

Right now every ball draws as `WHITE`. To give each body its own color, add a `color` field to the struct:

```rust
struct Body {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,   // new
}
```

`Color` is macroquad's color type — it's already in scope via `use macroquad::prelude::*`.

Update the constructor and draw method:

```rust
impl Body {
    fn new(position: Vec2, velocity: Vec2, radius: f32, color: Color) -> Self {
        Body { position, velocity, radius, color }
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}
```

---

## Random colors

macroquad includes a random number generator. Generate a bright, saturated color by keeping the components above 0.4:

```rust
fn random_color() -> Color {
    Color::new(
        macroquad::rand::gen_range(0.4, 1.0),
        macroquad::rand::gen_range(0.4, 1.0),
        macroquad::rand::gen_range(0.4, 1.0),
        1.0,
    )
}
```

`Color::new(r, g, b, a)` takes four `f32` values in `0.0..=1.0`. The `a` is alpha (opacity) — `1.0` is fully opaque.

`macroquad::rand::gen_range(min, max)` returns a random value in `[min, max)`.

Use it when spawning:

```rust
world.add_body(Body::new(
    Vec2::new(mx, my),
    Vec2::new(0.0, 0.0),
    20.0,
    random_color(),
));
```

---

## Displaying the body count

`draw_text` renders a string to the screen:

```rust
draw_text(&format!("Bodies: {}", world.bodies.len()), 10.0, 24.0, 24.0, WHITE);
```

`format!` works like `println!` but returns a `String` instead of printing. The `&` before it passes the string as `&str` — which `draw_text` expects.

The arguments are: text, x, y, font size, color. Put this at the end of the draw phase, after `world.draw_all()`.

---

## Your task

Open `lessons/5-many-bodies/lesson-06/project/src/main.rs`.

1. Add `color: Color` to `Body` and update `new` and `draw`.
2. Add a `random_color()` function using `macroquad::rand::gen_range`.
3. Update all `Body::new(...)` calls to pass a color — use `random_color()` for spawned balls, `WHITE` or any fixed color for the initial bodies.
4. After `world.draw_all()`, add a `draw_text` call showing the body count.

Run it and click to fill the screen with colorful bouncing balls.

**That's project 5 complete.** You have a `World` that owns many bodies, steps them all efficiently with `iter_mut`, applies gravity, enforces boundaries, and lets you spawn new bodies interactively. The ownership model that drives all of it — one mutable reference at a time — is the same model the collision system in project 6 will rely on.
