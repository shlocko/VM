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
            None => return Err(VMError::InvalidNativeFunctionName(path[0].into())),
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
            None => return Err(VMError::InvalidNativeFunctionName(path[0].into())),
        }
    }
}
pub static REGISTRY: LazyLock<NativeRegistry> = LazyLock::new(|| {
    let mut registry: NativeRegistry = NativeRegistry(HashMap::new());

    let mut vm_map: HashMap<String, RegistryEntry> = HashMap::new();

    //
    // String Namespace
    //
    let mut vm_string_map: HashMap<String, RegistryEntry> = HashMap::new();
    // String Concat
    vm_string_map.insert(
        "concat".into(),
        RegistryEntry::Function(|vm: &mut VM| -> Result<(), VMError> {
            let s1 = vm.stack.pop()?;
            let s2 = vm.stack.pop()?;
            match (&s1, &s2) {
                (Value::String(str1), Value::String(str2)) => {
                    let s_concat = (**str2).clone() + (**str1).as_str();
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
    vm_string_map.insert(
        "char_at".into(),
        RegistryEntry::Function(|vm: &mut VM| -> Result<(), VMError> {
            let idx = vm.stack.pop()?;
            let s1 = vm.stack.pop()?;
            match (idx, &s1) {
                (Value::Int(idx), Value::String(str1)) => {
                    let c = (*str1).clone().chars().nth(idx as usize);
                    match c {
                        Some(c) => {
                            vm.stack.push(Value::Char(c))?;
                            Ok(())
                        }
                        None => {
                            return Err(VMError::IndexOutsideRangeOfArray(
                                idx as usize,
                                (*str1).clone().chars().collect::<Vec<char>>().len(),
                            ))
                        }
                    }
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

    //
    // IO Namespace
    //
    let mut vm_io_map: HashMap<String, RegistryEntry> = HashMap::new();
    // IO print
    vm_io_map.insert(
        "print".into(),
        RegistryEntry::Function(|vm: &mut VM| -> Result<(), VMError> {
            let val = vm.stack.pop()?;
            match val {
                Value::NULL => print!("NULL"),
                Value::Int(n) => print!("{}", n),
                Value::Float(n) => print!("{}", n),
                Value::String(s) => print!("{}", s),
                Value::Char(c) => print!("{}", c),
                Value::Bool(b) => print!("{}", b),
                Value::Ident(i) => print!("{}", i),
                Value::Function(_, _) => print!("Function"),
                Value::HeapValue(v) => print!("{:?}", (*v).borrow()),
                Value::Array(_) => print!("Array"),
                Value::Map(_) => print!("Map"),
            }
            Ok(())
        }),
    );
    // IO println
    vm_io_map.insert(
        "println".into(),
        RegistryEntry::Function(|vm: &mut VM| -> Result<(), VMError> {
            let val = vm.stack.pop()?;
            match val {
                Value::NULL => println!("NULL"),
                Value::Int(n) => println!("{}", n),
                Value::Float(n) => println!("{}", n),
                Value::String(s) => println!("{}", s),
                Value::Char(c) => println!("{}", c),
                Value::Bool(b) => println!("{}", b),
                Value::Ident(i) => println!("{}", i),
                Value::Function(_, _) => println!("Function"),
                Value::HeapValue(v) => println!("{:?}", (*v).borrow()),
                Value::Array(_) => println!("Array"),
                Value::Map(_) => println!("Map"),
            }
            Ok(())
        }),
    );
    vm_map.insert("io".into(), RegistryEntry::Namespace(vm_io_map));

    registry
        .0
        .insert("vm".into(), RegistryEntry::Namespace(vm_map));

    registry
});
