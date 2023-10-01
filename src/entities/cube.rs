use cgmath::{Quaternion, Vector3};

use crate::engine::{
    LogicalDevice, MaterialLoading, ResourceManager, StandardInstance, StandardMaterial, TInstance,
};

use crate::{
    app::{EntityAction, EntityConfiguration, InputHandler, TEntity, UpdateFrequency},
    engine::{EngineResult, StandardMesh, TMesh},
};

#[derive(Debug, Default)]
pub struct Cube {
    mesh: Option<StandardMesh>,
    instances: Vec<StandardInstance>,
}

impl Cube {
    pub const TAG: &str = "Cube";

    pub fn new(instances: Vec<StandardInstance>) -> Self {
        Self {
            mesh: None,
            instances,
        }
    }
}

impl TEntity for Cube {
    fn entity_configuration(&self) -> EntityConfiguration {
        EntityConfiguration::new(Self::TAG, UpdateFrequency::None, true)
    }

    fn prepare_render(&mut self, logical_device: &LogicalDevice) -> EngineResult<()> {
        let material = StandardMaterial::from_path(logical_device, "cube_uv_color_grid.png")?;
        // let material = StandardMaterial::from_path(logical_device, "cube_uv_map.png")?;

        let mesh = ResourceManager::gltf_instanced_mesh_from_path(
            logical_device,
            "cube.glb",
            self.instances.clone(),
            MaterialLoading::Replace(material),
        )?;

        self.mesh = Some(mesh);

        Ok(())
    }

    fn meshes(&self) -> Vec<&dyn TMesh> {
        vec![self.mesh.as_ref().unwrap()]
    }
}
