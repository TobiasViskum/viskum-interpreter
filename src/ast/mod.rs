use crate::{
    operations::{ BinaryOp, UnaryOp },
    parser::{ ast_generator::AstEnvironment, token::TokenMetadata },
    value::ValueType,
};

use self::{ expr::Expr, stmt::{ FunctionArgument, FunctionStmt, ScopeStmt, Stmt } };

mod generate_cfg;
pub mod expr;
pub mod stmt;
mod symbol_table;
mod type_check;

#[derive(Debug)]
pub struct Ast {
     main_scope: ScopeStmt,

}

impl Ast {
   
    pub fn new(main_scope: ScopeStmt) -> Self {


        Self {
            main_scope,
        
        }
    }
}
