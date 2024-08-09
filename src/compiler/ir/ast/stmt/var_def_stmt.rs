use std::rc::Rc;

use ahash::AHashMap;

use crate::compiler::{
    ds::{ symbol_table::{ SSAKey, SymbolTableRef }, value::ValueType },
    error_handler::{ CompileError, ErrorHandler, ReportedError, SrcCharsRange },
    ir::{
        ast::expr::IdentifierExpr,
        icfg::{ dag::{ DAGDefineNode, DAGNode, DAG }, icfg_builder::ICFGBuilder },
    },
    parser::token::TokenMetadata,
    traits::{ Dissasemble, ExprTrait },
};

use super::{ ExprStmt, GotoNodeIds, LinearControlFlow, NodeIdsRange, StmtTrait };

#[derive(Debug)]
pub struct VarDefStmt<'ast> {
    ident_expr: IdentifierExpr,
    value_type: Option<ValueType>,
    is_mutable: bool,
    value: Option<ExprStmt<'ast>>,
}

impl<'ast> VarDefStmt<'ast> {
    pub fn new(
        name: Rc<str>,
        value_type: Option<ValueType>,
        is_mutable: bool,
        value: Option<ExprStmt<'ast>>,
        token_metadata: TokenMetadata
    ) -> Self {
        Self {
            ident_expr: IdentifierExpr::new(name, token_metadata),
            value_type,
            is_mutable,
            value,
        }
    }

    pub fn get_resolved_value_type(
        &mut self,
        symbol_table_ref: &SymbolTableRef
    ) -> Result<ValueType, CompileError> {
        let provided_value_type = self.value_type.as_ref();
        let value_type_based_on_value = match &mut self.value {
            Some(v) => Some(v.type_check(&symbol_table_ref)?),
            None => None,
        };

        let value_type = match (provided_value_type.cloned(), value_type_based_on_value) {
            (None, None) => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "The type of '{}' cannot be determined. Please provide a type: '{} ..'",
                                self.get_name(),
                                self.get_name()
                            ),
                            self.get_metadata().into()
                        )
                    )
                );
            }
            (Some(provided_type), None) => provided_type,
            (None, Some(found_type)) => found_type,
            (Some(provided_type), Some(found_type)) => {
                if provided_type.is(&found_type) {
                    provided_type
                } else {
                    return Err(
                        CompileError::new(
                            ReportedError::new(
                                format!(
                                    "Type of '{}' is provided as '{}', but type '{}' was found based on its provided value",
                                    self.get_name(),
                                    provided_type.dissasemble(),
                                    found_type.dissasemble()
                                ),
                                {
                                    let mut chars_range: SrcCharsRange = self.get_metadata().into();
                                    chars_range.merge(
                                        &self.value.as_ref().unwrap().collect_metadata()
                                    );
                                    chars_range
                                }
                            )
                        )
                    );
                }
            }
        };
        Ok(value_type)
    }

    pub fn get_name(&self) -> Rc<str> {
        self.ident_expr.get_lexeme()
    }

    pub fn get_value_type(&self) -> Option<&ValueType> {
        self.value_type.as_ref()
    }

    pub fn get_is_mutable(&self) -> bool {
        self.is_mutable
    }

    pub fn get_value(&self) -> Option<&ExprStmt<'ast>> {
        self.value.as_ref()
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.ident_expr.get_raw_metadata()
    }

    pub fn set_ssa_subscript(&mut self, ssa_subscript: usize) {
        self.ident_expr.set_ssa_subscript(ssa_subscript)
    }

    pub fn get_ssa_key(&self) -> SSAKey {
        self.ident_expr.get_ssa_key()
    }
}

impl<'ast> Dissasemble for VarDefStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mutable_string = match self.is_mutable {
            true => "mut ".to_string(),
            false => "".to_string(),
        };

        let value_string = match &self.value {
            Some(value) => format!(" := {}", value.dissasemble()),
            None => "".to_string(),
        };

        match &self.value_type {
            Some(value_type) => {
                format!(
                    "{}{}{}{}\n",
                    mutable_string,
                    self.get_name(),
                    value_type.dissasemble(),
                    value_string
                )
            }
            None => {
                format!("{}{}{}\n", mutable_string, self.get_ssa_key().dissasemble(), value_string)
            }
        }
    }
}

impl<'ast> StmtTrait for VarDefStmt<'ast> {
    type ReturnTypeCompileIntoICFG = NodeIdsRange;

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) -> Self::ReturnTypeCompileIntoICFG {
        todo!()
    }

    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {
        println!("I run before (var-def)");
        match symbol_table_ref.get_mut().declare_var(self) {
            Ok(ssa_key) => self.set_ssa_subscript(ssa_key.get_subscript()),
            Err(err) => error_handler.report_compile_error(err),
        }
        println!("I run after (var-def)")
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        Some(self)
    }
}

impl<'ast> LinearControlFlow for VarDefStmt<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        let value_node_id = self.value
            .as_ref()
            .map(|expr| expr.compile_into_dag(dag, ident_node_id_map));

        let define_node_id = dag.push_node(
            DAGNode::DefineNode(
                DAGDefineNode::new(
                    self.ident_expr.get_ssa_key(),
                    self.is_mutable,
                    value_node_id.is_some()
                )
            )
        );

        if let Some(value_node_id) = value_node_id {
            dag.add_edge(define_node_id, value_node_id);
        } else {
            panic!("Define statement has to have a value for now!");
        }

        dag.set_entry_node_id(define_node_id);

        define_node_id
    }
}
