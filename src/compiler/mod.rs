use std::collections::HashMap;

use crate::{
    ast::Ast,
    compiler::bytecode_generator::BytecodeGenerator,
    error_handler::ErrorHandler,
    value::ValueType,
    vm::instructions::VMInstruction,
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
    variables: HashMap<(String, usize), Variable>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn insert(&mut self, variable_name: String, value: Variable, subscript: usize) {
        self.variables.insert((variable_name, subscript), value);
    }

    pub fn _get(&self, variable_name: String, subscript: usize) -> Option<&Variable> {
        self.variables.get(&(variable_name, subscript))
    }

    fn get_variable_with_highest_subscript(&self, name: &String) -> Option<&Variable> {
        let mut highest_subscript = 0;
        let mut variable = None;
        for ((lexeme, subscript), var) in self.variables.iter() {
            if lexeme == name && *subscript > highest_subscript {
                highest_subscript = *subscript;
                variable = Some(var);
            }
        }

        variable
    }

    fn count_variables_of_name(&self, name: &String) -> (usize, Option<ValueType>, bool) {
        let mut count = 0;
        let mut value_type: Option<ValueType> = None;
        let mut is_mutable = false;
        for ((lexeme, _), variable) in self.variables.iter() {
            if lexeme == name {
                if count == 0 {
                    value_type = Some(variable.value_type.clone());
                    is_mutable = variable.is_mutable;
                }
                count += 1;
            }
        }

        (count, value_type, is_mutable)
    }
}

pub struct Compiler<'a> {
    error_handler: &'a mut ErrorHandler,
    environment: Environment,
}

impl<'a> Compiler<'a> {
    pub fn new(error_handler: &'a mut ErrorHandler) -> Self {
        Self {
            error_handler,
            environment: Environment::new(),
        }
    }

    pub fn compile(&mut self, ast: Ast) -> Option<Vec<VMInstruction>> {
        let mut ir_generator = IRGenerator::new(self.error_handler, &mut self.environment);
        let ir_graph = ir_generator.generate_ir_from_ast(ast);

        let mut bytecode_generator = BytecodeGenerator::new(&ir_graph);

        if self.error_handler.has_error() {
            return None;
        }

        bytecode_generator.generate_bytecode();

        // #[cfg(debug_assertions)]
        // {
        //     if !self.error_handler.has_error() {
        //         println!("Unoptimized instructions:");
        //         bytecode_generator.dissassemble();
        //     }
        // }

        let optimized_registers = bytecode_generator.get_optimized_registers();

        #[cfg(debug_assertions)]
        {
            if !self.error_handler.has_error() {
                println!("Optimized instructions:");
                for instruction in &optimized_registers {
                    println!("{}", instruction.dissassemble());
                }
            }
        }

        Some(optimized_registers)
    }
}
