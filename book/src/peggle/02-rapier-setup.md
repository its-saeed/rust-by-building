# Lesson 2 — Rapier Setup

> **Goal**: Add Rapier to the project, create a `PhysicsWorld` wrapper, and give every peg and wall a static collider. The ball still does not exist — but the collision geometry is ready.
>
> **Concepts**: `RigidBodySet`, `ColliderSet`, `PhysicsPipeline`, static colliders, newtype wrapper for handles.

---

## Adding the dependency

In `Cargo.toml`:

```toml
[dependencies]
macroquad = { workspace = true }
rapier2d  = { workspace = true }
```

`rapier2d` is declared in the workspace manifest with the `crossbeam-channel` feature, which enables the event handler you will use in Lesson 6.

---

## `physics.rs` — wrapping Rapier

Rapier requires many cooperating structs. Rather than passing them all through every function, wrap them in one struct:

```rust
use rapier2d::prelude::*;

pub struct PhysicsWorld {
    pub bodies:    RigidBodySet,
    pub colliders: ColliderSet,
    pipeline:      PhysicsPipeline,
    gravity:       Vector<f32>,
    params:        IntegrationParameters,
    islands:       IslandManager,
    broad_phase:   DefaultBroadPhase,
    narrow_phase:  NarrowPhase,
    impulse_joints:   ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver:    CCDSolver,
}
```

`bodies` and `colliders` are `pub` because other modules need to read body positions from them. The rest are Rapier internals — keep them private.

### `new`

```rust
impl PhysicsWorld {
    pub fn new() -> Self {
        PhysicsWorld {
            bodies:           RigidBodySet::new(),
            colliders:        ColliderSet::new(),
            pipeline:         PhysicsPipeline::new(),
            gravity:          vector![0.0, 500.0],
            params:           IntegrationParameters::default(),
            islands:          IslandManager::new(),
            broad_phase:      DefaultBroadPhase::new(),
            narrow_phase:     NarrowPhase::new(),
            impulse_joints:   ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver:       CCDSolver::new(),
        }
    }
```

Gravity is `vector![0.0, 500.0]` — 500 px/s² downward. macroquad's y-axis points down, and so does Rapier's default when positive gravity is in the +y direction. No coordinate flip needed.

### `step`

```rust
    pub fn step(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.params,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );
    }
```

`&()` for the last two arguments means "no physics hooks, no event handler." You will replace the second `&()` with a real event handler in Lesson 6.

---

## Adding peg colliders

Each peg needs a static collider — a circle at the peg's position with no rigid body parent:

```rust
    pub fn add_peg_collider(&mut self, pos: Vec2) -> ColliderHandle {
        let collider = ColliderBuilder::ball(crate::PEG_RADIUS)
            .translation(vector![pos.x, pos.y])
            .build();
        self.colliders.insert(collider)
    }
```

`ColliderBuilder::ball(radius)` creates a circle shape. `.translation` sets the position. `self.colliders.insert(collider)` — no body argument — makes it static. The method returns the `ColliderHandle` so you can store it alongside the `Peg` to remove it later.

Store the handle in `Peg`:

```rust
// peg.rs
pub struct Peg {
    pub pos:    Vec2,
    pub kind:   PegKind,
    pub hit:    bool,
    pub handle: ColliderHandle,
}
```

And update `Peg::new` to accept a handle:

```rust
pub fn new(pos: Vec2, kind: PegKind, handle: ColliderHandle) -> Self {
    Peg { pos, kind, hit: false, handle }
}
```

In `main.rs`, build the pegs by first creating the collider, then the `Peg`:

```rust
let mut physics = PhysicsWorld::new();

let mut pegs: Vec<Peg> = level::level_one()
    .into_iter()
    .map(|(pos, kind)| {
        let handle = physics.add_peg_collider(pos);
        Peg::new(pos, kind, handle)
    })
    .collect();
```

For this to work, `level_one` should return `Vec<(Vec2, PegKind)>` rather than `Vec<Peg>`, since `Peg` now needs a handle that only exists after adding the collider. Update `level.rs` accordingly.

---

## Adding wall colliders

Three static cuboids — left wall, right wall, ceiling:

```rust
    pub fn add_walls(&mut self) {
        let hw = crate::WINDOW_W / 2.0;
        let hh = crate::WINDOW_H / 2.0;
        // cuboid half-extents: thickness=10, length=full side
        for (tx, ty, hx, hy) in [
            (hw,  -5.0,  hw,  10.0),  // ceiling
            (-5.0, hh,   10.0, hh),   // left wall
            (crate::WINDOW_W + 5.0, hh, 10.0, hh), // right wall
        ] {
            let c = ColliderBuilder::cuboid(hx, hy)
                .translation(vector![tx, ty])
                .build();
            self.colliders.insert(c);
        }
    }
```

`ColliderBuilder::cuboid(hx, hy)` takes **half-extents** — half the width and half the height. A 10-pixel-thick ceiling spanning 800 pixels has `hx = 400`, `hy = 5`.

There is no bottom wall — the ball should fall out of the screen when it misses the bucket.

---

## The newtype pattern for handles

In `ball.rs`, introduce a newtype wrapper around the Rapier handle:

```rust
use rapier2d::prelude::RigidBodyHandle;

pub struct BallHandle(pub RigidBodyHandle);
```

A newtype is a struct with a single field. Its only purpose is to give the inner type a distinct name in your code — `BallHandle` rather than `RigidBodyHandle`. This prevents accidentally passing a peg's `ColliderHandle` where a ball's handle is expected, and gives you a place to add methods later.

The ball struct itself is empty for now:

```rust
pub struct Ball {
    pub handle: BallHandle,
}
```

You will complete it in Lesson 5.

---

## Your task

Open `lessons/8-peggle/lesson-02/project/src/`.

1. In `Cargo.toml`: add `rapier2d = { workspace = true }`.
2. Create `physics.rs` with `PhysicsWorld::new()`, `step()`, `add_peg_collider()`, `add_walls()`.
3. Add `ColliderHandle` field to `Peg`. Update `Peg::new` to accept it.
4. Update `level.rs` to return `Vec<(Vec2, PegKind)>` instead of `Vec<Peg>`.
5. Create `ball.rs` with `BallHandle(RigidBodyHandle)` and an empty `Ball` struct.
6. In `main.rs`: add `mod physics; mod ball;`, create `PhysicsWorld`, call `add_walls()`, build pegs with handles, call `physics.step()` each frame.

```sh
cargo run --bin peggle-02
```

The scene looks identical to Lesson 1 — nothing moves. But the collision geometry is now in place and being stepped every frame.
