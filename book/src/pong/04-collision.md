# Lesson 4 — Collision Detection

> **Goal**: Make the ball bounce off paddles using `Rect::overlaps`.
>
> **Concepts**: `Rect::overlaps`, immutable borrows alongside a mutable borrow, position correction, hit-position deflection.

---

## `Rect::overlaps`

macroquad's `Rect` has a built-in method for testing whether two rectangles intersect:

```rust
rect_a.overlaps(&rect_b) -> bool
```

That's our entire collision detection — one method call. Compare this to project 6, where we computed distances, normals, and penetration depths manually. For axis-aligned rectangles, the library does it all.

---

## Adding `Ball::check_paddles`

Add a method to `Ball` that tests against both paddles and responds:

```rust
impl Ball {
    fn check_paddles(&mut self, left: &Paddle, right: &Paddle) {
        if self.rect.overlaps(&left.rect) {
            self.rect.x = left.rect.x + left.rect.w; // push ball out
            self.vel.x = self.vel.x.abs();            // force rightward
        }
        if self.rect.overlaps(&right.rect) {
            self.rect.x = right.rect.x - self.rect.w; // push ball out
            self.vel.x = -self.vel.x.abs();            // force leftward
        }
    }
}
```

### Direction

Same technique as wall bouncing: `.abs()` forces the velocity in the correct direction regardless of its current sign.

```
left paddle hit:              right paddle hit:
  ┃ ←ball→  →                 ← ←ball→  ┃
  ┃          →    becomes:    ←           ┃
  ┃                            ←
vel.x = vel.x.abs()          vel.x = -vel.x.abs()
(force positive = rightward)  (force negative = leftward)
```

### Position correction

```rust
self.rect.x = left.rect.x + left.rect.w;
```

This pushes the ball's left edge to the paddle's right edge, snapping it out of the overlap. Without this, the ball can get stuck inside the paddle — `overlaps` returns `true` on the next frame too, flipping velocity back and forth every frame.

---

## Borrow checker note

The call in `main` looks like:

```rust
ball.check_paddles(&left, &right);
```

`ball` is borrowed mutably, `left` and `right` are borrowed immutably. Rust allows this because they are **different variables** — the mutable borrow of `ball` doesn't conflict with shared borrows of `left` and `right`. If `left` or `right` were fields of the same struct as `ball`, it would be a different story.

---

## Adding deflection

A flat reflection (velocity `x` flips, `y` unchanged) is functional but predictable — both players quickly learn to keep the rally going forever. Classic Pong adds a deflection angle based on where on the paddle the ball hits:

```
top of paddle   → deflects upward more
centre          → no change to y
bottom          → deflects downward more
```

```rust
fn deflect(&mut self, paddle: &Paddle) {
    // where on the paddle did the ball hit? 0.0 = top, 1.0 = bottom
    let hit = (self.rect.y + self.rect.h / 2.0 - paddle.rect.y) / paddle.rect.h;
    // remap to -1.0 .. 1.0
    let factor = (hit - 0.5) * 2.0;
    // current speed (magnitude of velocity vector)
    let speed = (self.vel.x * self.vel.x + self.vel.y * self.vel.y).sqrt();
    self.vel.y = factor * speed * 0.75;
}
```

Call `deflect` inside each overlap block, after correcting position and before setting `vel.x`:

```rust
if self.rect.overlaps(&left.rect) {
    self.rect.x = left.rect.x + left.rect.w;
    self.deflect(left);
    self.vel.x = self.vel.x.abs();
}
```

`0.75` caps the maximum vertical deflection at 75% of the ball's speed — enough to be interesting without producing nearly-vertical trajectories that are impossible to reach.

---

## Your task

Open `lessons/7-pong/lesson-04/project/src/main.rs`.

1. Add `fn check_paddles(&mut self, left: &Paddle, right: &Paddle)` to `impl Ball`.  
   - Test `self.rect.overlaps(&left.rect)` and `self.rect.overlaps(&right.rect)`.
   - Correct position, force velocity direction.
2. Add `fn deflect(&mut self, paddle: &Paddle)` and call it inside each overlap block.
3. In the game loop, call `ball.check_paddles(&left, &right)` after `ball.update(dt)`.

```sh
cargo run --bin pong-04
```

The ball should now bounce off paddles. Try hitting it at the top and bottom of the paddle — the angle should change. The ball still bounces off the left and right walls (scoring comes next).
