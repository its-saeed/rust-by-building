# Rapier Primer

> **Goal**: Understand what Rapier does, how its pieces fit together, and how it will sit alongside macroquad in Peggle.
>
> **Concepts**: rigid bodies, colliders, the physics pipeline, collision events, the relationship between physics handles and game state.

---

## What Rapier is

Rapier is a physics simulation library. You give it a description of the world — shapes, masses, velocities, gravity — and each frame it advances time by a small step, resolving collisions and updating positions. You read those positions back and draw them with macroquad.

Rapier handles the hard parts: detecting when shapes overlap, computing how they should push each other apart, and updating velocities according to conservation of momentum. You handled all of this manually in Projects 4–6. Here, Rapier does it for you.

---

## The pieces

Rapier separates concerns into distinct data structures. You create all of them once and pass them around:

```rust
use rapier2d::prelude::*;

let mut rigid_body_set  = RigidBodySet::new();
let mut collider_set    = ColliderSet::new();
let mut pipeline        = PhysicsPipeline::new();
let     gravity         = vector![0.0, 9.81];
let     integration_parameters = IntegrationParameters::default();
let mut island_manager  = IslandManager::new();
let mut broad_phase     = DefaultBroadPhase::new();
let mut narrow_phase    = NarrowPhase::new();
let mut impulse_joints  = ImpulseJointSet::new();
let mut multibody_joints = MultibodyJointSet::new();
let mut ccd_solver      = CCDSolver::new();
```

That is a lot of types. In practice you wrap them all in a single `PhysicsWorld` struct (which we will write in Lesson 2) so that `main.rs` only sees one thing.

### `RigidBodySet` — what moves

A `RigidBody` is an object with mass, position, and velocity that Rapier simulates. You build one with a builder and insert it into the set:

```rust
let body = RigidBodyBuilder::dynamic()
    .translation(vector![400.0, 100.0])
    .build();
let handle = rigid_body_set.insert(body);
```

`insert` returns a **handle** — a small value (like an index) that you keep to look up or modify the body later. You do not hold a direct reference to the body; you go through the set.

### `ColliderSet` — what has shape

A `Collider` defines the shape used for collision detection. A collider is attached to a rigid body:

```rust
let collider = ColliderBuilder::ball(6.0)   // radius 6 pixels
    .restitution(0.8)                        // bounciness 0–1
    .build();
collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);
```

You can also insert a collider without a parent body — it becomes a **static collider** (it does not move):

```rust
let peg_collider = ColliderBuilder::ball(8.0)
    .translation(vector![200.0, 300.0])
    .build();
collider_set.insert(peg_collider);
```

Static colliders are how Peggle's pegs work: they have shape and position but no rigid body, so they never move.

### Stepping the simulation

Each frame, call `pipeline.step` with all the sets:

```rust
pipeline.step(
    &gravity,
    &integration_parameters,
    &mut island_manager,
    &mut broad_phase,
    &mut narrow_phase,
    &mut rigid_body_set,
    &mut collider_set,
    &mut impulse_joints,
    &mut multibody_joints,
    &mut ccd_solver,
    &(),   // physics hooks
    &(),   // event handler
);
```

After this call, every dynamic body's position has been updated. You read a body's position back out like this:

```rust
let body = &rigid_body_set[handle];
let pos  = body.translation();  // &Vector2<f32>
```

---

## Collision events

To know when the ball hits a peg, you use an **event collector** instead of the empty `&()` in the step call:

```rust
let (collision_send, collision_recv) = crossbeam_channel::unbounded();
let event_handler = ChannelEventCollector::new(
    collision_send,
    crossbeam_channel::unbounded().0,  // we ignore contact force events
);

pipeline.step(
    ...
    &event_handler,
);

while let Ok(event) = collision_recv.try_recv() {
    // event.collider1 and event.collider2 are ColliderHandles
}
```

`ColliderHandle`s identify which colliders touched. You keep a map from `ColliderHandle` → peg index so you can look up which peg was hit.

---

## Handles vs. game state

This is the most important thing to understand about Rapier's API: **Rapier owns the physics data; you own the game data**.

You never store a `RigidBody` directly in your `Ball` struct. You store the `RigidBodyHandle` that Rapier gave you, and when you need position or velocity you look it up:

```rust
pub struct Ball {
    pub handle: RigidBodyHandle,
    // no position stored here — ask Rapier
}

impl Ball {
    pub fn position(&self, bodies: &RigidBodySet) -> Vec2 {
        let t = bodies[self.handle].translation();
        Vec2::new(t.x, t.y)
    }
}
```

When a peg is hit and should be removed, you remove both the Rapier collider and your own `Peg` struct from its `Vec`. Neither owns the other; they are linked by the handle.

---

## Coordinates

Rapier uses SI units by default (metres, kg, seconds). For a pixel-scale game, set gravity to something that feels right in pixels-per-second-squared, not real-world `9.81 m/s²`. We will use `vector![0.0, 500.0]` — 500 px/s² downward — which gives the ball a satisfying arc.

macroquad's y-axis points **down** (y=0 is the top of the screen). Rapier's default is y-up, but we will configure it y-down to match macroquad by setting positive gravity in the y direction and using macroquad positions directly.

---

## What Rapier handles, what you still write

| Rapier does | You still write |
|-------------|-----------------|
| Circle–circle collision detection | Deciding what to do when a collision fires (remove peg, play sound) |
| Velocity response and bounce | Aiming, launch direction, trajectory preview |
| Gravity integration | All drawing (Rapier has no renderer) |
| Sleeping idle bodies | Game state, score, win/lose |
| CCD (no tunneling at high speed) | Camera, UI, asset loading |
