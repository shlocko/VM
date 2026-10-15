use std::collections::HashMap;

use crate::{error::VMError, vm::VM};

pub type NativeFunction = Box<dyn Fn(&mut VM) -> Result<(), VMError>>;
pub enum RegistryEntry {
    Function(NativeFunction),
    Namespace(HashMap<String, RegistryEntry>),
}

pub struct NativeRegistry(pub HashMap<String, RegistryEntry>);

impl NativeRegistry {
    pub fn call(&self, path_string: String, vm: &mut VM) -> Result<(), VMError> {
        let path: Vec<&str> = path_string.split("::").collect();
        let entry = self.0.get(path[0]);
        match entry {
            Some(RegistryEntry::Function(f)) => f(vm),
            Some(RegistryEntry::Namespace(n)) => self.call_helper(n, &path[1..], vm),
            None => todo!(),
        }
    }
    pub fn call_helper(
        &self,
        map: &HashMap<String, RegistryEntry>,
        path: &[&str],
        vm: &mut VM,
    ) -> Result<(), VMError> {
        match map.get(path[0]) {
            Some(RegistryEntry::Function(f)) => f(vm),
            Some(RegistryEntry::Namespace(n)) => self.call_helper(n, &path[1..], vm),
            None => todo!(),
        }
    }
}
