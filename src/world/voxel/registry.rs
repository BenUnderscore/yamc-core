//Uses
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Attribute: 'static + Any + Send + Sync {}
impl<A: 'static + Any + Send + Sync> Attribute for A {}

/// Stores one type of attribute for all registered voxel types
pub struct AttributeRegistry<A: Attribute> {
    map: Vec<Option<A>>,
    label: String,
}

impl<A: Attribute> AttributeRegistry<A> {
    pub fn new(attribute_label: &str) -> AttributeRegistry<A> {
        AttributeRegistry {
            map: Vec::new(),
            label: attribute_label.to_owned(),
        }
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn register(&mut self, id: u16, attribute_obj: A) -> Result<(), super::Error> {
        if id as usize >= self.map.len() {
            self.map.resize_with(id as usize, Option::default);
        }

        match self.map[id as usize] {
            Some(_) => Err(super::Error::AttributeAlreadyRegistered(id)),
            None => Ok(self.map[id as usize] = Some(attribute_obj)),
        }
    }

    pub fn find(&self, id: u16) -> Result<&A, super::Error> {
        if id as usize >= self.map.len() {
            return Err(super::Error::AttributeMissing(id));
        }
        match &self.map[id as usize] {
            Some(attr) => Ok(attr),
            None => Err(super::Error::AttributeMissing(id)),
        }
    }
}

/// Provides facilities to store all attributes in one centralized location
pub struct AttributeRegistries {
    regs: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl AttributeRegistries {
    pub fn new() -> AttributeRegistries {
        AttributeRegistries {
            regs: HashMap::new(),
        }
    }

    pub fn add_registry<A: Attribute>(
        &mut self,
        reg: AttributeRegistry<A>,
    ) -> Result<(), super::Error> {
        let type_id = TypeId::of::<A>();
        if self.regs.contains_key(&type_id) {
            return Err(super::Error::RegistryAlreadyAdded(
                std::any::type_name::<A>(),
            ));
        }

        self.regs.insert(type_id, Arc::new(reg));

        Ok(())
    }

    pub fn get_registry<A: Attribute>(&self) -> Option<Arc<AttributeRegistry<A>>> {
        let type_id = TypeId::of::<A>();
        Some(
            self.regs
                .get(&type_id)?
                .clone()
                .downcast::<AttributeRegistry<A>>()
                .unwrap(),
        )
    }
}

/// Allows for a reverse-lookup of strings to voxel IDs, useful for scripting convenience
/// and serialization consistency.
pub struct NameRegistry {
    map: HashMap<String, u16>,
}

impl NameRegistry {
    pub fn new() -> NameRegistry {
        NameRegistry {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, id: u16) -> Result<(), super::Error> {
        match self.map.insert(name.to_owned(), id) {
            Some(id) => Err(super::Error::NameAlreadyRegistered(id)),
            None => Ok(()),
        }
    }

    pub fn find(&self, name: &str) -> Option<u16> {
        self.map.get(name).map(|id| *id)
    }
}
