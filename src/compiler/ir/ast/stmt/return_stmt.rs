use crate::compiler::{
    ds::value::ValueType,
    error_handler::{ CompileError, ErrorHandler, ReportedError },
    ir::{
        ast::AST_DISSASEMBLE_INDENTATION,
        icfg::{
            cfg::{ CFGNode, CFGNodeId, CFGNodeType, CFGReturnNode, CFG },
            icfg_builder::{ CFGBuilder, ICFGBuilder },
            ICFG,
        },
    },
    parser::token::TokenMetadata,
    traits::{ AstDissasemble, Dissasemble },
    ProgramSymbolTablePhase1,
};

use super::{ ExprStmt, GotoNodeIds, LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct ReturnStmt<'ast> {
    return_expr: Option<ExprStmt<'ast>>,
    metadata: TokenMetadata,
}

impl<'ast> ReturnStmt<'ast> {
    pub fn new(return_expr: Option<ExprStmt<'ast>>, metadata: TokenMetadata) -> Self {
        Self { return_expr, metadata }
    }
}

impl<'ast> StmtTrait for ReturnStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        let return_node_id = cfg_builder.push_cfg_node(
            CFGNode::new(
                CFGNodeType::ReturnNode(
                    CFGReturnNode::new(
                        self.return_expr.as_ref().map(|expr| expr.compile_to_dag(icfg_builder)),
                        cfg_builder.get_ret_type().clone()
                    )
                )
            )
        );
        goto_node_ids.push_return_node_id(return_node_id);
    }

    fn is_linear_control_flow(&self) -> bool {
        false
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        None
    }

    fn validate_stmt(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        if let Some(ret_type) = program_symbol_table.get_fn_ret_type() {
            let found_ret_type = if let Some(ret_expr) = &mut self.return_expr {
                match ret_expr.type_check(program_symbol_table) {
                    Ok(v) => Some(v),
                    Err(err) => {
                        error_handler.report_compile_error(err);
                        return;
                    }
                }
            } else {
                None
            };

            match (ret_type, found_ret_type) {
                (ValueType::Void, None) => {}
                (v1, None) => {
                    error_handler.report_compile_error(
                        CompileError::new(
                            ReportedError::new(
                                format!(
                                    "Expected return type: '{}' but got '()'",
                                    v1.dissasemble()
                                ),
                                self.metadata.into()
                            )
                        )
                    )
                }
                (v1, Some(v2)) => {
                    if !v1.is(&v2) {
                        error_handler.report_compile_error(
                            CompileError::new(
                                ReportedError::new(
                                    format!(
                                        "Expected return type: '{}' but got '{}'",
                                        v1.dissasemble(),
                                        v2.dissasemble()
                                    ),
                                    self.metadata.into()
                                )
                            )
                        )
                    }
                }
            }
        } else {
            error_handler.report_compile_error(
                CompileError::new(
                    ReportedError::new(
                        "Return statements cannot be used outside of functions".to_string(),
                        self.metadata.into()
                    )
                )
            );
        }
    }
}

impl<'ast> Dissasemble for ReturnStmt<'ast> {
    fn dissasemble(&self) -> String {
        match &self.return_expr {
            Some(return_expr) => format!("return {}", return_expr.dissasemble()),
            None => "return".to_string(),
        }
    }
}

impl<'ast> AstDissasemble for ReturnStmt<'ast> {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        let ret_string = match &self.return_expr {
            Some(return_expr) =>
                format!(
                    "return {}",
                    return_expr.ast_dissasemble(program_symbol_table, scope_depth)
                ),
            None => "return".to_string(),
        };
        format!(
            "[{}]: {}{}\n",
            program_symbol_table.get_current_symbol_table_id(),
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth),
            ret_string
        )
    }
}
