use crate::{
    ast::expr::ExprBuilder,
    error_handler::CompileError,
    parser::{ precedence::Precedence, Parser },
};

#[derive(Debug)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Parser, &mut ExprBuilder) -> Result<(), CompileError>>,
    pub infix: Option<fn(&mut Parser, &mut ExprBuilder) -> Result<(), CompileError>>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn get_prefix(
        &self
    ) -> Option<fn(&mut Parser, &mut ExprBuilder) -> Result<(), CompileError>> {
        self.prefix
    }

    pub fn get_infix(
        &self
    ) -> Option<fn(&mut Parser, &mut ExprBuilder) -> Result<(), CompileError>> {
        self.infix
    }

    pub fn get_precedence(&self) -> &Precedence {
        &self.precedence
    }
}
