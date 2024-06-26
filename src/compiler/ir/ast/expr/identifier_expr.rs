use std::rc::Rc;

use crate::compiler::{
    ds::{ symbol_table::{ SSAKey, SymbolTableActions, SymbolTableRef }, value::ValueType },
    error_handler::{ CompileError, ReportedError, SrcCharsRange },
    ir::icfg::dag::DAG,
    parser::token::TokenMetadata,
    Dissasemble,
};

use super::ExprTrait;

#[derive(Debug)]
pub struct IdentifierExpr {
    lexeme: Rc<str>,
    ssa_subscript: usize,
    metadata: TokenMetadata,
}

impl IdentifierExpr {
    pub fn new(lexeme: Rc<str>, metadata: TokenMetadata) -> Self {
        Self { lexeme, ssa_subscript: 0, metadata }
    }

    pub fn get_lexeme(&self) -> Rc<str> {
        Rc::clone(&self.lexeme)
    }

    pub fn get_raw_metadata(&self) -> TokenMetadata {
        self.metadata
    }

    pub fn is_var_mutable(&self, symbol_table_ref: &SymbolTableRef) -> Result<bool, CompileError> {
        match symbol_table_ref.get().lookup_as_var(&self.lexeme) {
            Ok(symbol_var) => Ok(symbol_var.get_is_mutable()),
            Err(msg) => Err(CompileError::new(ReportedError::new(msg, self.collect_metadata()))),
        }
    }

    pub fn get_ssa_key(&self) -> SSAKey {
        SSAKey::new(Rc::clone(&self.lexeme), self.ssa_subscript)
    }

    pub fn set_ssa_subscript(&mut self, ssa_subscript: usize) {
        self.ssa_subscript = ssa_subscript;
    }
}

impl Dissasemble for IdentifierExpr {
    fn dissasemble(&self) -> String {
        format!("{}", self.lexeme)
    }
}

impl ExprTrait for IdentifierExpr {
    // fn evaluate(&mut self, ast_symbol_table: &AstSymbolTable) -> super::ExprEvaluateResult {
    //     todo!("Evaluate identifer if it hasn't changed and if its a variable")
    // }

    fn compile_to_dag_node(&self, _: &mut DAG) -> usize {
        todo!()
    }

    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError> {
        match symbol_table_ref.get().lookup(&self.lexeme) {
            Some((ssa_key, symbol)) => {
                match symbol.try_value_type_as_var() {
                    Ok(vtype) => {
                        self.set_ssa_subscript(ssa_key.get_subscript());
                        Ok(vtype)
                    }
                    Err(msg) =>
                        Err(CompileError::new(ReportedError::new(msg, self.collect_metadata()))),
                }
            }
            None => {
                Err(
                    CompileError::new(
                        ReportedError::new(
                            format!("Undefined variable '{}'", self.lexeme),
                            self.collect_metadata()
                        )
                    )
                )
            }
        }
    }

    // fn type_check_and_constant_fold(&mut self, ast_symbol_table: &AstSymbolTable) -> ExprResult {
    //     match ast_symbol_table.get(&self.lexeme, &self.metadata) {
    //         Ok(symbol) => {
    //             match symbol {
    //                 Some(symbol) =>
    //                     match symbol {
    //                         AstSymbol::Variable { value_type, .. } =>
    //                             Ok(ExprResultOk::new(value_type.clone(), false)),
    //                         AstSymbol::Function { return_type, .. } =>
    //                             Ok(ExprResultOk::new(return_type.clone(), false)),
    //                     }
    //                 None => {
    //                     Err(
    //                         ExprResultErr::new(
    //                             vec![
    //                                 CompileError::new(
    //                                     ReportedError::new(
    //                                         format!("Undefined variable '{}'", self.lexeme),
    //                                         self.metadata.into()
    //                                     )
    //                                 )
    //                             ]
    //                         )
    //                     )
    //                 }
    //             }
    //         }
    //         Err(err) => ExprResult::Err(ExprResultErr::new(vec![err])),
    //     }
    // }

    fn collect_metadata(&self) -> SrcCharsRange {
        self.metadata.into()
    }
}
