use crate::compiler::{ error_handler::CompileError, ir::ast::{ expr::ExprBuilder, AstArena } };

use super::{ precedence::Precedence, Parser };

type ParseRuleMethod = for<'b> fn(
    &mut Parser,
    &mut ExprBuilder<'b>,
    &'b AstArena<'b>
) -> Result<(), CompileError>;

#[derive(Debug)]
pub struct ParseRule {
    pub prefix: Option<ParseRuleMethod>,
    pub infix: Option<ParseRuleMethod>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn get_prefix(&self) -> Option<ParseRuleMethod> {
        self.prefix
    }

    pub fn get_infix(&self) -> Option<ParseRuleMethod> {
        self.infix
    }

    pub fn get_precedence(&self) -> &Precedence {
        &self.precedence
    }
}
