use nalgebra::SVector;
// use rapier2d_f64::counters::Timer;
use rapier2d_f64::dynamics::RigidBodyHandle;
use rapier2d_f64::prelude::nalgebra;
use rapier2d_f64::prelude::{
    vector, CCDSolver, ColliderBuilder, ColliderSet, DefaultBroadPhase, ImpulseJointSet,
    IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline,
    QueryPipeline, RigidBodyBuilder, RigidBodySet,
};

pub struct Game {
    gravity: SVector<f64, 2>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    ball_body_handle: RigidBodyHandle,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    // pub timer: Timer,
    // physics_hooks: dyn PhysicsHooks,
    // event_handler: &EventHandler,
}

impl Game {
    pub fn new() -> Game {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* create the ground, TODO: convert to procedural terrain */
        let collider = ColliderBuilder::cuboid(100.0, 1.0).build();
        collider_set.insert(collider);

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        // let mut timer = Timer::new();
        // timer.start();

        Game {
            gravity: vector![0.0, -9.81],
            rigid_body_set,
            collider_set,
            ball_body_handle,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            // timer,
            // physics_hooks: (),
            // event_handler: (),
        }

        /* Create other structures necessary for the simulation. */
    }

    // TODO: the physics loop needs to be detached from the game loop
    // as rendering is done slowly
    pub fn step_physics(&mut self) {
        // self.timer.pause();
        // self.integration_parameters.dt = self.timer.time();
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
        // self.timer.resume();

        // let ball_body = &self.rigid_body_set[self.ball_body_handle];
        // println!("Ball altitude: {}", ball_body.translation().y);
    }

    pub fn get_ball_height(&self) -> f64 {
        self.rigid_body_set[self.ball_body_handle].translation().y
    }
}
