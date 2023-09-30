use std::collections::HashMap;

use cgmath::num_traits::Pow;
use cgmath::{Quaternion, Vector3};
use rand::Rng;

use crate::engine::{StandardInstance, TInstance};

use crate::app::{EntityAction, EntityConfiguration, InputHandler, TEntity, UpdateFrequency};
use crate::entities::Cheese;

#[derive(Debug, Default)]
pub struct MainScene {}

impl MainScene {
    pub const TAG: &str = "MainScene";
}

pub struct Voxel {
    position: (i32, i32, i32),
}

impl Voxel {
    pub fn to_instance(&self) -> StandardInstance {
        StandardInstance::new(
            Vector3::new(
                self.position.0 as f32,
                self.position.1 as f32,
                self.position.2 as f32,
            ),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        )
    }
}

pub struct Shape {
    min_x: i32,
    min_y: i32,
    min_z: i32,
    max_x: i32,
    max_y: i32,
    max_z: i32,
    map: HashMap<(i32, i32, i32), Voxel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ShapeAction {
    Additive,
    Subtractive,
    NoChange,
}

impl Shape {
    pub fn new(min_x: i32, min_y: i32, min_z: i32, max_x: i32, max_y: i32, max_z: i32) -> Self {
        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            map: HashMap::new(),
        }
    }

    pub fn process<T>(&mut self, action: ShapeAction, shapeable: T) -> &mut Self
    where
        T: Shapable,
    {
        for x in self.min_x..self.max_x {
            for y in self.min_y..self.max_y {
                for z in self.min_z..self.max_z {
                    match shapeable.work(action, x, y, z) {
                        ShapeAction::Additive => {
                            self.map.insert(
                                (x, y, z),
                                Voxel {
                                    position: (x, y, z),
                                },
                            );
                        }
                        ShapeAction::Subtractive => {
                            self.map.remove(&(x, y, z));
                        }
                        ShapeAction::NoChange => (),
                    }
                }
            }
        }

        self
    }

    pub fn make_instances(&self) -> Vec<StandardInstance> {
        let mut initial_counter = 0;

        let mut instances: Vec<StandardInstance> = Vec::new();
        for x in self.min_x..self.max_x {
            for y in self.min_y..self.max_y {
                for z in self.min_z..self.max_z {
                    let origin = self.map.get(&(x, y, z));
                    if origin.is_none() {
                        continue;
                    }
                    initial_counter += 1;

                    let x_pos = self.map.get(&(x + 1, y, z));
                    let x_neg = self.map.get(&(x - 1, y, z));
                    let y_pos = self.map.get(&(x, y + 1, z));
                    let y_neg = self.map.get(&(x, y - 1, z));
                    let z_pos = self.map.get(&(x, y, z + 1));
                    let z_neg = self.map.get(&(x, y, z - 1));

                    let mut counter = 0;
                    if x_pos.is_some() {
                        counter += 1;
                    }
                    if x_neg.is_some() {
                        counter += 1;
                    }
                    if y_pos.is_some() {
                        counter += 1;
                    }
                    if y_neg.is_some() {
                        counter += 1;
                    }
                    if z_pos.is_some() {
                        counter += 1;
                    }
                    if z_neg.is_some() {
                        counter += 1;
                    }

                    if counter <= 5 {
                        let instance = origin.unwrap().to_instance();
                        instances.push(instance);
                    }
                }
            }
        }

        println!("Initial: {}", initial_counter);
        println!("Now: {}", instances.len());
        instances
    }
}

pub trait Shapable {
    fn work(&self, action: ShapeAction, x: i32, y: i32, z: i32) -> ShapeAction;
}

pub struct Sphere {
    position: (i32, i32, i32),
    radius: f32,
}

impl Sphere {
    pub fn new(position: (i32, i32, i32), radius: f32) -> Self {
        Self { position, radius }
    }
}

impl Shapable for Sphere {
    fn work(&self, action: ShapeAction, x: i32, y: i32, z: i32) -> ShapeAction {
        let distance = distance(
            self.position.0 as f32,
            self.position.1 as f32,
            self.position.2 as f32,
            x as f32,
            y as f32,
            z as f32,
        );

        let radius_squared = self.radius.pow(2.0);

        if distance <= radius_squared {
            return action;
        }

        ShapeAction::NoChange
    }
}

pub struct UniformCube {
    position: (i32, i32, i32),
    diameter: i32,
}

impl UniformCube {
    pub fn new(position: (i32, i32, i32), diameter: i32) -> Self {
        Self { position, diameter }
    }
}

impl Shapable for UniformCube {
    fn work(&self, action: ShapeAction, x: i32, y: i32, z: i32) -> ShapeAction {
        if x >= self.position.0 - (self.diameter - 1)
            && x <= self.position.0 + self.diameter - 1
            && y >= self.position.1 - (self.diameter - 1)
            && y <= self.position.1 + self.diameter - 1
            && z >= self.position.2 - (self.diameter - 1)
            && z <= self.position.2 + self.diameter - 1
        {
            return action;
        }

        ShapeAction::NoChange
    }
}

pub struct VariableCube {
    position: (i32, i32, i32),
    width: i32,
    height: i32,
    depth: i32,
}

impl VariableCube {
    pub fn at_point(position: (i32, i32, i32), width: i32, height: i32, depth: i32) -> Self {
        Self {
            position,
            width,
            height,
            depth,
        }
    }

    pub fn at_center(position: (i32, i32, i32), width: i32, height: i32, depth: i32) -> Self {
        Self {
            position: (
                position.0 - width / 2,
                position.1 - height / 2,
                position.2 - depth / 2,
            ),
            width,
            height,
            depth,
        }
    }
}

impl Shapable for VariableCube {
    fn work(&self, action: ShapeAction, x: i32, y: i32, z: i32) -> ShapeAction {
        if x >= self.position.0
            && x <= self.position.0 + (self.width - 1)
            && y >= self.position.1
            && y <= self.position.1 + (self.height - 1)
            && z >= self.position.2
            && z <= self.position.2 + (self.depth - 1)
        {
            return action;
        }

        ShapeAction::NoChange
    }
}

fn distance_from_zero<I: Into<f32>>(x: I, y: I, z: I) -> f32 {
    distance(x.into(), y.into(), z.into(), 0.0, 0.0, 0.0)
}

fn distance<I: Into<f32>>(x1: I, y1: I, z1: I, x2: I, y2: I, z2: I) -> f32 {
    (x2.into() - x1.into()).pow(2.0)
        + (y2.into() - y1.into()).pow(2.0)
        + (z2.into() - z1.into()).pow(2.0)
}

impl TEntity for MainScene {
    fn entity_configuration(&self) -> EntityConfiguration {
        EntityConfiguration::new(Self::TAG, UpdateFrequency::Fast, false)
    }

    fn update(&mut self, _delta_time: f64, _input_handler: &InputHandler) -> Vec<EntityAction> {
        // let instances: Vec<StandardInstance> = (-5..5)
        //     .into_iter()
        //     .flat_map(|x| {
        //         (-5..5).into_iter().flat_map(move |y| {
        //             (-5..5).into_iter().map(move |z| {
        //                 let distance = distance_from_zero(x, y, z);

        //                 struct Sphere {
        //                     position: (f64, f64, f64),
        //                     radius: f64,
        //                 }

        //                 if distance <= radius {
        //                     Some(StandardInstance::new(
        //                         Vector3::new((x as f32) * 2.0, (y as f32) * 2.0, (z as f32) * 2.0),
        //                         Quaternion::new(0.0, 0.0, 0.0, 0.0),
        //                     ))
        //                 } else {
        //                     None
        //                 }
        //             })
        //         })
        //     })
        //     .filter(|x| x.is_some())
        //     .map(|x| x.unwrap())
        //     .collect();

        let mut rng = rand::thread_rng();

        let min_x = -50;
        let min_y = -50;
        let min_z = -50;
        let max_x = 50;
        let max_y = 50;
        let max_z = 50;

        let mut shape = Shape::new(min_x, min_y, min_z, max_x, max_y, max_z);

        shape.process(ShapeAction::Additive, Sphere::new((0, 0, 0), 15.0));

        for _ in 0..15 {
            let radius = rng.gen_range(5.0..10.0);

            shape.process(
                ShapeAction::Additive,
                Sphere::new(
                    (
                        rng.gen_range((min_x + radius as i32)..=(max_x - radius as i32)),
                        rng.gen_range((min_y + radius as i32)..=(max_y - radius as i32)),
                        rng.gen_range((min_z + radius as i32)..=(max_z - radius as i32)),
                    ),
                    radius,
                ),
            );
        }

        // let y_range = (INITIAL_RADIUS - INITIAL_RADIUS / 4.0) as i32..=max_y;
        // log::debug!(">>> {:?}", y_range);

        shape.process(
            ShapeAction::Subtractive,
            VariableCube::at_point(
                (min_x, 0, min_z),
                min_x.abs() + max_x,
                min_y.abs() + max_y,
                min_z.abs() + max_z,
            ),
        );

        let instances = shape.make_instances();

        let cheese = Box::new(Cheese::new(instances));

        vec![
            EntityAction::Remove(vec![Self::TAG.to_string()]),
            EntityAction::Spawn(vec![cheese]),
        ]
    }
}
