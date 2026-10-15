use std::{collections::HashMap, rc::Rc, sync::LazyLock};

use crate::{error::VMError, value::Value, vm::VM};

pub type NativeFunction = fn(&mut VM) -> Result<(), VMError>;
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
pub static REGISTRY: LazyLock<NativeRegistry> = LazyLock::new(|| {
    let mut registry: NativeRegistry = NativeRegistry(HashMap::new());

    let mut vm_map: HashMap<String, RegistryEntry> = HashMap::new();
    let mut vm_string_map: HashMap<String, RegistryEntry> = HashMap::new();
    vm_string_map.insert(
        "concat".into(),
        RegistryEntry::Function(|vm: &mut VM| -> Result<(), VMError> {
            let s1 = vm.stack.pop()?;
            let s2 = vm.stack.pop()?;
            match (&s1, &s2) {
                (Value::String(str1), Value::String(str2)) => {
                    let s_concat = (**str1).clone() + (**str2).as_str();
                    vm.stack.push(Value::String(Rc::new(s_concat)))?;
                    Ok(())
                }
                _ => {
                    return Err(VMError::InvalidOperandType(
                        Value::String(Rc::new("".to_string())),
                        s1,
                    ))
                }
            }
        }),
    );
    vm_map.insert("string".into(), RegistryEntry::Namespace(vm_string_map));
    registry
        .0
        .insert("vm".into(), RegistryEntry::Namespace(vm_map));

    registry
});
