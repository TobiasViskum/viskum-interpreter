use crate::{
    ast::Ast,
    compiler::bytecode_generator::BytecodeGenerator,
    error_handler::ErrorHandler,
    vm::instructions::Instruction,
};

use self::ir_generator::IRGenerator;

pub mod ir_graph;
mod ir_generator;
mod bytecode_generator;

pub struct Compiler<'a> {
    error_handler: &'a ErrorHandler,
}

impl<'a> Compiler<'a> {
    pub fn new(error_handler: &'a ErrorHandler) -> Self {
        Self {
            error_handler,
        }
    }

    pub fn compile(&mut self, ast: Ast) -> Vec<Instruction> {
        let mut ir_generator = IRGenerator::new(self.error_handler);
        let ir_graph = ir_generator.generate_ir_from_ast(ast);

        let mut bytecode_generator = BytecodeGenerator::new(ir_graph);

        bytecode_generator.generate_bytecode();

        bytecode_generator.optimize_registers();

        bytecode_generator.dissassemble();

        bytecode_generator.get_instructions()
    }
}
