use macroquad::prelude::Vec2;
// TODO: add: use rapier2d::prelude::*;

// TODO: implement PhysicsWorld struct wrapping all Rapier pieces:
//   pub struct PhysicsWorld {
//       pub bodies:    RigidBodySet,
//       pub colliders: ColliderSet,
//       pipeline:      PhysicsPipeline,
//       gravity:       Vector<f32>,
//       params:        IntegrationParameters,
//       islands:       IslandManager,
//       broad_phase:   DefaultBroadPhase,
//       narrow_phase:  NarrowPhase,
//       impulse_joints:   ImpulseJointSet,
//       multibody_joints: MultibodyJointSet,
//       ccd_solver:    CCDSolver,
//   }

// TODO: implement PhysicsWorld:
//   pub fn new() -> Self  — gravity = vector![0.0, 500.0]
//
//   pub fn step(&mut self)  — call self.pipeline.step(...) with &() for hooks and events
//
//   pub fn add_peg_collider(&mut self, pos: Vec2) -> ColliderHandle
//     — ColliderBuilder::ball(PEG_RADIUS).translation(vector![pos.x, pos.y]).build()
//     — self.colliders.insert(collider)
//
//   pub fn add_walls(&mut self)
//     — ceiling:     cuboid at (WINDOW_W/2, -5),     half-extents (WINDOW_W/2, 10)
//     — left wall:   cuboid at (-5, WINDOW_H/2),     half-extents (10, WINDOW_H/2)
//     — right wall:  cuboid at (WINDOW_W+5, WINDOW_H/2), half-extents (10, WINDOW_H/2)
