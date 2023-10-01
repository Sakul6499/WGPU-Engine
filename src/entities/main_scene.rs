use std::collections::HashMap;
use std::path::Path;

use cgmath::num_traits::Pow;
use cgmath::{Quaternion, Vector3};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Billow, Perlin};
use rand::Rng;

use crate::engine::{StandardInstance, TInstance};

use crate::app::{EntityAction, EntityConfiguration, InputHandler, TEntity, UpdateFrequency};
use crate::entities::Cube;

#[derive(Debug, Default)]
pub struct MainScene {}

impl MainScene {
    pub const TAG: &str = "MainScene";
}

pub struct Voxel {
    position: Vector3<i32>,
}

impl Voxel {
    pub fn new(position: Vector3<i32>) -> Self {
        Self { position }
    }

    pub fn to_instance(&self) -> StandardInstance {
        StandardInstance::new(
            Vector3::new(
                self.position.x as f32,
                self.position.y as f32,
                self.position.z as f32,
            ),
            Quaternion::new(0.0, 0.0, 0.0, 0.0),
        )
    }

    pub fn position(&self) -> Vector3<i32> {
        self.position
    }
}

pub struct WorldGenerator {
    seed: u32,
    size: u32,
    map: HashMap<Vector3<i32>, Voxel>,
}

impl WorldGenerator {
    pub fn from_random_seed(size: u32) -> Self {
        let seed = rand::thread_rng().gen_range(u32::MIN..=u32::MAX);
        Self::from_seed(seed, size)
    }

    pub fn from_seed(seed: u32, size: u32) -> Self {
        let billow = Billow::<Billow<Perlin>>::new(seed);
        let noise_map = PlaneMapBuilder::<_, 2>::new(billow)
            .set_size(size as usize, size as usize)
            .set_x_bounds(-1.0, 1.0)
            .set_y_bounds(-1.0, 1.0)
            .build();

        #[cfg(debug_assertions)]
        let mut pixels: Vec<u8> = Vec::new();

        let mut output: HashMap<Vector3<i32>, Voxel> = HashMap::new();
        let radius = ((size as f32) / 2.0).pow(2);

        let center = size / 2;

        for x in 0..size {
            for z in 0..size {
                let distance = distance(x as f32, 0.0, z as f32, center as f32, 0.0, center as f32);

                if distance <= radius {
                    let noise_value = noise_map.get_value(x as usize, z as usize);

                    if noise_value <= 0.0 {
                        #[cfg(debug_assertions)]
                        pixels.push((noise_value * 255.0) as u8);
                        continue;
                    } else {
                        #[cfg(debug_assertions)]
                        pixels.push(255u8);
                    }

                    let depth = (noise_value * 2.0) as i32;

                    for y in -depth..0 {
                        // Note: Convert coordinates to be centered
                        let position: Vector3<i32> = Vector3::new(
                            (x as i32) - (center as i32),
                            y,
                            (z as i32) - (center as i32),
                        );

                        let voxel = Voxel::new(position);

                        output.insert(position, voxel);
                    }
                } else {
                    #[cfg(debug_assertions)]
                    pixels.push(255u8);
                }
            }
        }

        #[cfg(debug_assertions)]
        image::save_buffer(
            Path::new(&"noise_map.png".to_string()),
            &pixels,
            size,
            size,
            image::ColorType::L8,
        )
        .expect("failed to write debug noise_map");

        Self {
            seed,
            size,
            map: output,
        }
    }

    pub fn at(&self, position: Vector3<i32>) -> Option<&Voxel> {
        self.map.get(&position)
    }

    pub fn to_instances(&self) -> Vec<StandardInstance> {
        let mut initial_counter = 0;

        let mut instances: Vec<StandardInstance> = Vec::new();

        for position in self.map.keys() {
            let origin = self.map.get(position).unwrap(); // Must exist
            initial_counter += 1;

            let x_pos = self.map.get(&(position + Vector3::new(1, 0, 0)));
            let x_neg = self.map.get(&(position + Vector3::new(-1, 0, 0)));
            let y_pos = self.map.get(&(position + Vector3::new(0, 1, 0)));
            let y_neg = self.map.get(&(position + Vector3::new(0, -1, 0)));
            let z_pos = self.map.get(&(position + Vector3::new(0, 0, 1)));
            let z_neg = self.map.get(&(position + Vector3::new(0, 0, -1)));

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

            // Counter == 6 means the Voxel is fully encased from each side.
            // Counter cannot be lower than 0 and not higher than 5.
            // Each increase in count is equal to one face of the Voxel
            // being covered.
            if counter <= 5 {
                let instance = origin.to_instance();
                instances.push(instance);
            }
        }

        println!("Initial: {}", initial_counter);
        println!("Now: {}", instances.len());
        instances
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }

    pub fn size(&self) -> u32 {
        self.size
    }
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
        let world_generator = WorldGenerator::from_random_seed(128);
        log::debug!("Seed: {}", world_generator.seed());

        let instances = world_generator.to_instances();

        let cube = Box::new(Cube::new(instances));

        vec![
            EntityAction::Remove(vec![Self::TAG.to_string()]),
            EntityAction::Spawn(vec![cube]),
        ]
    }
}
