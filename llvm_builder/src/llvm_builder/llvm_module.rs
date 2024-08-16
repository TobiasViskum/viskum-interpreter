use crate::BuildLLVM;

use super::{ llvm_function::Function, llvm_type::{ Type, TypeI32 } };

pub struct Module {
    // constants
    // globals
    functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            functions: vec![],
        }
    }

    pub fn push_func(&mut self, func: Function) {
        self.functions.push(func);
    }
}

impl BuildLLVM for Module {
    fn build(&self) -> String {
        let mut string_builder = "".to_string();

        for function in self.functions.iter() {
            string_builder += function.build().as_str();
            string_builder += "\n";
        }

        string_builder
    }
}
