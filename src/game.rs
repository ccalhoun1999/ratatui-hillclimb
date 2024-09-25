use nalgebra::SVector;
// use rapier2d_f64::counters::Timer;
use rapier2d_f64::dynamics::{RevoluteJointBuilder, RigidBodyHandle};
use rapier2d_f64::na::point;
use rapier2d_f64::prelude::nalgebra;
use rapier2d_f64::prelude::{
    vector, CCDSolver, ColliderBuilder, ColliderSet, DefaultBroadPhase, ImpulseJointSet,
    IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, PhysicsPipeline,
    QueryPipeline, RigidBodyBuilder, RigidBodySet,
};

pub struct Car {
    pub rear_wheel_radius: f64,
    pub front_wheel_radius: f64,
    pub body_half_width: f64,
    pub body_half_height: f64,
}

impl Car {
    pub fn new(
        rear_wheel_radius: f64,
        front_wheel_radius: f64,
        body_half_width: f64,
        body_half_height: f64,
    ) -> Car {
        Car {
            rear_wheel_radius,
            front_wheel_radius,
            body_half_width,
            body_half_height,
        }
    }
}

pub struct Game {
    car: Car,
    gravity: SVector<f64, 2>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    car_body_handle: RigidBodyHandle,
    front_wheel_handle: RigidBodyHandle,
    rear_wheel_handle: RigidBodyHandle,
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
    pub fn new(car: Car) -> Game {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();

        // create the ground
        // TODO: convert to procedural terrain
        let collider = ColliderBuilder::cuboid(100.0, 1.0)
            // .collision_groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1))
            .build();
        collider_set.insert(collider);

        let car_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .linear_damping(0.5)
            .build();
        let car_body_collider = ColliderBuilder::cuboid(car.body_half_width, car.body_half_height)
            // .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2))
            .build();
        let car_body_handle = rigid_body_set.insert(car_body);
        collider_set.insert_with_parent(car_body_collider, car_body_handle, &mut rigid_body_set);

        let rear_wheel = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0])
            .angular_damping(1.0)
            .build();
        let rear_wheel_collider = ColliderBuilder::ball(car.rear_wheel_radius)
            .restitution(0.7)
            // .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2))
            .build();
        let rear_wheel_handle = rigid_body_set.insert(rear_wheel);
        collider_set.insert_with_parent(
            rear_wheel_collider,
            rear_wheel_handle,
            &mut rigid_body_set,
        );

        let front_wheel = RigidBodyBuilder::dynamic()
            .translation(vector![car.body_half_width, 10.0])
            .angular_damping(1.0)
            .build();
        let front_wheel_collider = ColliderBuilder::ball(car.front_wheel_radius)
            // .restitution(0.7)
            // .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2))
            .build();
        let front_wheel_handle = rigid_body_set.insert(front_wheel);
        collider_set.insert_with_parent(
            front_wheel_collider,
            front_wheel_handle,
            &mut rigid_body_set,
        );

        let rear_wheel_joint = RevoluteJointBuilder::new()
            .local_anchor1(point![-car.body_half_width, -car.body_half_height])
            .local_anchor2(point![0.0, 0.0])
            .contacts_enabled(false)
            // .motor_velocity(1000.0, 100.5)
            .build()
            .data;
        // impulse_joint_set.insert(rear_wheel_handle, car_body_handle, rear_wheel_joint, true);
        impulse_joint_set.insert(car_body_handle, rear_wheel_handle, rear_wheel_joint, true);
        // multibody_joint_set
        //     .insert(car_body_handle, rear_wheel_handle, rear_wheel_joint, true)
        //     .unwrap();

        let front_wheel_joint = RevoluteJointBuilder::new()
            .local_anchor1(point![car.body_half_width, -car.body_half_height])
            .local_anchor2(point![0.0, 0.0])
            .contacts_enabled(false)
            .build()
            .data;
        // impulse_joint_set.insert(front_wheel_handle, car_body_handle, front_wheel_joint, true);
        impulse_joint_set.insert(car_body_handle, front_wheel_handle, front_wheel_joint, true);
        // multibody_joint_set
        //     .insert(car_body_handle, front_wheel_handle, front_wheel_joint, true)
        //     .unwrap();

        // let mut timer = Timer::new();
        // timer.start();

        Game {
            car,
            gravity: vector![0.0, -9.81],
            rigid_body_set,
            collider_set,
            car_body_handle,
            front_wheel_handle,
            rear_wheel_handle,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set,
            multibody_joint_set,
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

        // self.reset_torque();
        // self.timer.resume();

        // let ball_body = &self.rigid_body_set[self.ball_body_handle];
        // println!("Ball altitude: {}", ball_body.translation().y);
    }

    pub fn get_car(&self) -> &Car {
        &self.car
    }

    pub fn get_rear_wheel_torque(&self) -> f64 {
        self.rigid_body_set[self.rear_wheel_handle].user_torque()
    }

    pub fn get_car_body_x(&self) -> f64 {
        self.rigid_body_set[self.car_body_handle].translation().x
    }

    pub fn get_car_body_y(&self) -> f64 {
        self.rigid_body_set[self.car_body_handle].translation().y
    }

    pub fn get_car_body_angle(&self) -> f64 {
        self.rigid_body_set[self.car_body_handle].rotation().angle()
    }

    pub fn get_front_wheel_x(&self) -> f64 {
        self.rigid_body_set[self.front_wheel_handle].translation().x
    }

    pub fn get_front_wheel_y(&self) -> f64 {
        self.rigid_body_set[self.front_wheel_handle].translation().y
    }

    pub fn get_rear_wheel_x(&self) -> f64 {
        self.rigid_body_set[self.rear_wheel_handle].translation().x
    }

    pub fn get_rear_wheel_y(&self) -> f64 {
        self.rigid_body_set[self.rear_wheel_handle].translation().y
    }

    pub fn apply_torque(&mut self, torque: f64) {
        self.rigid_body_set[self.rear_wheel_handle].add_torque(torque, true);
    }

    // pub fn reset_torque(&mut self) {
    // self.rigid_body_set[self.rear_wheel_handle].reset_torques(true);
    // }
}
