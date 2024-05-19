use std::{ collections::HashMap, fmt::format };

use crate::{
    ast::Ast,
    error_handler::ErrorHandler,
    vm::instructions::Instruction,
    compiler::icfg::ICFG,
};

pub mod cfg;
pub mod icfg;
mod vm_symbol_table;
// pub mod ir_graph;
// mod ir_generator;
// mod bytecode_generator;

pub struct Compiler<'a> {
    error_handler: &'a mut ErrorHandler,
}

impl<'a> Compiler<'a> {
    pub fn new(error_handler: &'a mut ErrorHandler) -> Self {
        Self { error_handler }
    }

    #[profiler::function_tracker]
    pub fn compile(&mut self, ast: Ast) -> Option<Vec<Instruction>> {
        let mut icfg = ICFG::from_ast(ast);

        return Some(icfg.optimize_and_generate_bytecode());

        panic!("sdf");

        let mut cfg = ast.generate_cfg();

        cfg.dissassemble();

        let instructions = cfg.optimize_and_generate_bytecode();

        #[cfg(debug_assertions)]
        {
            let mut indentation_level = 0;
            let indentation_size = 4;

            if !self.error_handler.has_error() {
                println!("Optimized instructions:");
                println!("----------------------\n");

                for instruction in &instructions {
                    let print_string = format!(
                        "{}{}",
                        " ".repeat(indentation_level * indentation_size),
                        instruction.dissassemble()
                    );

                    println!("{}", print_string);
                }
                println!("\n----------------------");
            }
        }

        Some(instructions)
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

/*
/*


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
    definitions: HashMap<(String, usize), (Variable, bool)>, // (lexeme, subscript), (variable, is_definition)
}

impl EnvironmentScope {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }
}

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

    pub fn insert(
        &mut self,
        variable_name: String,
        value: Variable,
        subscript: usize,
        is_definition: bool
    ) {
        self.scopes[self.scope_depth].definitions.insert(
            (variable_name, subscript),
            (value, is_definition)
        );
    }

    pub fn get_variable_with_highest_subscript(
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
                    variable = Some(&var.0);
                    break;
                }
            }
        }

        (variable, scope)
    }

    pub fn get_latest_variable_definition(&self, name: &String) -> (Option<usize>, Option<usize>) {
        let mut register = None;
        let mut scope = None;

        println!("scopes: {:#?}", self.scopes);

        for i in (0..=self.scope_depth).rev() {
            for ((lexeme, _), (variable, is_definition)) in self.scopes[i].definitions.iter() {
                if lexeme == name && *is_definition {
                    register = Some(variable.location);
                    scope = Some(i);
                    return (register, scope);
                }
            }
        }

        (register, scope)
    }

    pub fn count_variables_of_name(&self, name: &String) -> (usize, Option<ValueType>, bool) {
        let mut count = 0;
        let mut value_type: Option<ValueType> = None;
        let mut is_mutable = false;
        for i in (0..=self.scope_depth).rev() {
            for ((lexeme, _), variable) in self.scopes[i].definitions.iter() {
                if lexeme == name {
                    if count == 0 {
                        value_type = Some(variable.0.value_type);
                        is_mutable = variable.0.is_mutable;
                    }
                    count += 1;
                }
            }
        }

        (count, value_type, is_mutable)
    }
}

*/
*/
