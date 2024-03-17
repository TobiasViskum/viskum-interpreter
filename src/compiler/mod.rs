use std::{ collections::HashMap, fmt::format };

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
struct EnvironmentScope {
    definitions: HashMap<(String, usize), Variable>,
}

impl EnvironmentScope {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    scopes: Vec<EnvironmentScope>,
    scope_depth: usize,
}

impl Environment {
    pub fn new() -> Self {
        let mut scopes = Vec::new();
        scopes.push(EnvironmentScope::new());

        Self {
            scopes,
            scope_depth: 0,
        }
    }

    pub fn start_scope(&mut self) {
        self.scopes.push(EnvironmentScope::new());
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
        self.scope_depth -= 1;
    }

    pub fn insert(&mut self, variable_name: String, value: Variable, subscript: usize) {
        self.scopes[self.scope_depth].definitions.insert((variable_name, subscript), value);
        // println!("INSERT, scopes: {:#?}", self.scopes);
    }

    pub fn get(&self, variable_name: String, subscript: usize) -> Option<&Variable> {
        // println!("GET, scopes: {:#?}", self.scopes);
        self.scopes[self.scope_depth].definitions.get(&(variable_name, subscript))
    }

    fn get_variable_with_highest_subscript(
        &self,
        name: &String
    ) -> (Option<&Variable>, Option<usize>) {
        let mut highest_subscript = 0;
        let mut variable = None;
        let mut scope = None;

        for i in (0..=self.scope_depth).rev() {
            for ((lexeme, subscript), var) in self.scopes[i].definitions.iter() {
                if lexeme == name && *subscript > highest_subscript {
                    scope = Some(i);
                    highest_subscript = *subscript;
                    variable = Some(var);
                }
            }
        }

        (variable, scope)
    }

    fn count_variables_of_name(&self, name: &String) -> (usize, Option<ValueType>, bool) {
        let mut count = 0;
        let mut value_type: Option<ValueType> = None;
        let mut is_mutable = false;
        for i in (0..=self.scope_depth).rev() {
            for ((lexeme, _), variable) in self.scopes[i].definitions.iter() {
                if lexeme == name {
                    if count == 0 {
                        value_type = Some(variable.value_type);
                        is_mutable = variable.is_mutable;
                    }
                    count += 1;
                }
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
        let mut ir_generator = IRGenerator::new(&mut self.environment);
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
            let mut indentation_level = 0;
            let indentation_size = 4;

            if !self.error_handler.has_error() {
                println!("Optimized instructions:");
                println!("----------------------\n");
                for instruction in &optimized_registers {
                    match instruction {
                        VMInstruction::EndScope => {
                            indentation_level -= 1;
                        }
                        _ => {}
                    }

                    let print_string = format!(
                        "{}{}",
                        " ".repeat(indentation_level * indentation_size),
                        instruction.dissassemble()
                    );

                    match instruction {
                        VMInstruction::StartScope => {
                            println!("{}\n", print_string);
                        }
                        VMInstruction::EndScope => {
                            println!("\n{}", print_string);
                        }
                        _ => println!("{}", print_string),
                    }

                    match instruction {
                        VMInstruction::StartScope => {
                            indentation_level += 1;
                        }
                        _ => {}
                    }
                }
                println!("\n----------------------")
            }
        }

        Some(optimized_registers)
    }
}

/*
DEFINE R0 1
STARTSCOPE

    DEFINE R0 2
    ADD R1 R0 1
    STARTSCOPE

        DEFINE R0 3
        ADD R1 R0 1

    ENDSCOPE
    LOAD R2 true

ENDSCOPE
LOAD R1 false
HALT
*/
