//Uses
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone)]
pub struct LoadedResource {
    pub data: LoadedResourceData,
}

#[derive(Clone, Debug)]
pub enum LoadedResourceData {
    Text(String),
}

#[derive(Clone, Copy, Debug)]
pub enum ResourceLoadType {
    PlainText,
}

#[derive(Error, Debug)]
pub enum ResourceError {
    #[error("The resource ID \"{}\" is invalid!", 0)]
    InvalidResourceId(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, ResourceError>;

//Manages the loading and caching of all resources
pub struct ResourceSystem {
    loaded_resources: HashMap<String, Arc<LoadedResource>>,
    root_path: PathBuf,
}

impl ResourceSystem {
    pub fn new(path: PathBuf) -> ResourceSystem {
        ResourceSystem {
            loaded_resources: HashMap::new(),
            root_path: path,
        }
    }

    pub fn get_loaded_resource(
        &mut self,
        resource_id: &str,
        load_type: ResourceLoadType,
    ) -> Result<Arc<LoadedResource>> {
        match self.loaded_resources.get(resource_id) {
            Some(resource) => Ok(resource.clone()),
            None => self.load_resource(resource_id, load_type),
        }
    }

    fn load_resource(
        &mut self,
        resource_id: &str,
        load_type: ResourceLoadType,
    ) -> Result<Arc<LoadedResource>> {
        let resource_id_path = PathBuf::from(OsString::from(resource_id));
        if resource_id_path.has_root() || !resource_id_path.is_relative() {
            return Err(ResourceError::InvalidResourceId(resource_id.to_owned()));
        }

        let resource_path = {
            let root = self.root_path.to_owned();
            root.join(resource_id_path)
        };

        match load_type {
            ResourceLoadType::PlainText => {
                let data_string = std::fs::read_to_string(resource_path)?;
                Ok(Arc::new(LoadedResource {
                    data: LoadedResourceData::Text(data_string),
                }))
            }
        }
    }
}
