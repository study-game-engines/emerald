use crate::world::*;
use crate::physics::*;
use crate::physics::components::*;
use crate::{Instant, Vector2};
use crate::EmeraldError;

use rapier2d::geometry::{ColliderHandle, ColliderBuilder, Collider, ContactEvent, ProximityEvent};
use rapier2d::dynamics::{RigidBodyHandle, RigidBodyBuilder, RigidBody, RigidBodyMut};

pub struct PhysicsHandler<'a> {
    physics_engine: &'a mut  PhysicsEngine,
    world: &'a mut hecs::World,
}
impl<'a> PhysicsHandler<'a> {
    pub fn new(physics_engine: &'a mut PhysicsEngine, world: &'a mut hecs::World) -> Self {
        PhysicsHandler {
            world,
            physics_engine,
        }
    }

    pub fn try_recv_contact(&mut self) -> Result<ContactEvent, EmeraldError> { self.physics_engine.try_recv_contact() }
    pub fn try_recv_proximity(&mut self) -> Result<ProximityEvent, EmeraldError> { self.physics_engine.try_recv_proximity() }

    pub fn create_body(&mut self, desc: &RigidBodyBuilder) -> RigidBodyHandle {
        self.physics_engine.create_body(desc)
    }

    pub fn create_collider(&mut self, body_handle: RigidBodyHandle, desc: &ColliderBuilder) -> ColliderHandle {
        self.physics_engine.create_collider(body_handle, &desc)
    }

    pub fn rigid_body_mut(&mut self, body_handle: RigidBodyHandle) -> Option<RigidBodyMut> {
        self.physics_engine.bodies.get_mut(body_handle)
    }

    pub fn step(&mut self) {
        self.step_n(1);
    }

    pub fn step_n(&mut self, n: u32) {
        let start = Instant::now();

        self.physics_engine.sync_physics_world_to_game_world(&mut self.world);
        
        for _ in 0..n {
            self.physics_engine.step();
        }

        self.physics_engine.sync_game_world_to_physics_world(&mut self.world);

        let end = Instant::now();
    }
}