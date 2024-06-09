use std::rc::Rc;

use crate::{
    ast::ast_symbol_table::{ AstSymbol, AstSymbolTable },
    compiler::cfg::dag::DAG,
    error_handler::{ CompileError, SrcCharsRange },
    parser::token::TokenMetadata,
    value::ValueType,
    Dissasemble,
};

use super::{ ExprResult, ExprResultErr, ExprResultOk, ExprTrait };

#[derive(Debug)]
pub struct IdentifierExpr {
    lexeme: Rc<str>,
    metadata: TokenMetadata,
}

impl IdentifierExpr {
    pub fn new(lexeme: Rc<str>, metadata: TokenMetadata) -> Self {
        Self { lexeme, metadata }
    }

    pub fn get_lexeme(&self) -> Rc<str> {
        Rc::clone(&self.lexeme)
    }
}

impl Dissasemble for IdentifierExpr {
    fn dissasemble(&self) -> String {
        format!("{}", self.lexeme)
    }
}

impl ExprTrait for IdentifierExpr {
    fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> super::ExprEvaluateResult {
        todo!("Evaluate identifer if it hasn't changed and if its a variable")
    }

    fn compile_to_dag_node(&self, dag: &mut DAG) -> usize {
        todo!()
    }

    fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
        match ast_symbol_table.get(&self.lexeme) {
            Some(symbol) =>
                match symbol {
                    AstSymbol::Variable(var) =>
                        Ok(ExprResultOk::new(var.value_type.clone(), false)),
                    AstSymbol::Function(func) =>
                        Ok(ExprResultOk::new(func.return_type.clone(), false)),
                }
            None => {
                Err(
                    ExprResultErr::new(
                        vec![
                            CompileError::new(
                                format!("Undefined variable '{}'", self.lexeme),
                                self.metadata.into()
                            )
                        ]
                    )
                )
            }
        }
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        self.metadata.into()
    }
}
