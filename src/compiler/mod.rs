use std::collections::HashMap;

use crate::{
    ast::Ast,
    compiler::bytecode_generator::BytecodeGenerator,
    error_handler::ErrorHandler,
    value::{ Value, ValueType },
    vm::instructions::Instruction,
};

use self::ir_generator::IRGenerator;

pub mod ir_graph;
mod ir_generator;

mod bytecode_generator;

#[derive(Debug)]
pub struct Variable {
    location: usize,
    value_type: ValueType,
    is_mutable: bool,
}

impl Variable {
    pub fn new(location: usize, value_type: ValueType, is_mutable: bool) -> Self {
        Self {
            location,
            value_type,
            is_mutable,
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, (Variable, usize)>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn insert(&mut self, variable_name: String, value: Variable) {
        let new_subscript = self.count_variables_of_name(&variable_name) + 1;
        self.variables.insert(variable_name, (value, new_subscript));
    }

    pub fn get(&self, variable_name: &String) -> Option<&(Variable, usize)> {
        self.variables.get(variable_name)
    }

    pub fn count_variables_of_name(&self, name: &String) -> usize {
        self.variables
            .keys()
            .filter(|variable| variable == &name)
            .count()
    }
}

pub struct Compiler<'a> {
    error_handler: &'a ErrorHandler,
    environment: Environment,
}

impl<'a> Compiler<'a> {
    pub fn new(error_handler: &'a ErrorHandler) -> Self {
        Self {
            error_handler,
            environment: Environment::new(),
        }
    }

    pub fn compile(&mut self, ast: Ast) -> Vec<Instruction> {
        let mut ir_generator = IRGenerator::new(self.error_handler, &mut self.environment);
        let ir_graph = ir_generator.generate_ir_from_ast(ast);

        let mut bytecode_generator = BytecodeGenerator::new(ir_graph);

        bytecode_generator.generate_bytecode();

        bytecode_generator.optimize_registers();

        #[cfg(debug_assertions)]
        {
            if !self.error_handler.has_error() {
                bytecode_generator.dissassemble();
            }
        }

        bytecode_generator.get_instructions()
    }
}
