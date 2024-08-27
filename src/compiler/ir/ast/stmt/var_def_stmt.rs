use std::rc::Rc;

use ahash::AHashMap;

use crate::compiler::{
    ds::{ ssa_ident::SSAIdent, symbol_table::UserSymbolVar, value::ValueType },
    error_handler::{ CompileError, ErrorHandler, ReportedError, SrcCharsRange },
    ir::{
        ast::{ expr::IdentifierExpr, AST_DISSASEMBLE_INDENTATION },
        icfg::{
            dag::{ DAGDefineNode, DAGNode, DAG },
            icfg_builder::{ CFGBuilder, ICFGBuilder },
            ICFG,
        },
    },
    parser::token::TokenMetadata,
    traits::{ AstDissasemble, Dissasemble, ExprTrait },
    ProgramSymbolTablePhase1,
    SymbolVar,
};

use super::{ ExprStmt, GotoNodeIds, LinearControlFlow, StmtTrait };

#[derive(Debug)]
pub struct VarDefStmt<'ast> {
    ident_expr: IdentifierExpr,
    value_type: Option<ValueType>,
    mut_keyword_metadata: Option<TokenMetadata>,
    value: Option<ExprStmt<'ast>>,
}

impl<'ast> VarDefStmt<'ast> {
    pub fn new(
        ident_expr: IdentifierExpr,
        value_type: Option<ValueType>,
        mut_keyword_metadata: Option<TokenMetadata>,
        value: Option<ExprStmt<'ast>>
    ) -> Self {
        Self {
            ident_expr,
            value_type,
            mut_keyword_metadata,
            value,
        }
    }

    // pub fn get_resolved_value_type(
    //     &mut self,
    //     symbol_table_ref: &SymbolTableRef
    // ) -> Result<ValueType, CompileError> {
    //     let provided_value_type = self.value_type.as_ref();
    //     let value_type_based_on_value = match &mut self.value {
    //         Some(v) => Some(v.type_check(&symbol_table_ref)?),
    //         None => None,
    //     };

    //     let value_type = match (provided_value_type.cloned(), value_type_based_on_value) {
    //         (None, None) => {
    //             return Err(
    //                 CompileError::new(
    //                     ReportedError::new(
    //                         format!(
    //                             "The type of '{}' cannot be determined. Please provide a type: '{} ..'",
    //                             self.get_name(),
    //                             self.get_name()
    //                         ),
    //                         self.get_metadata().into()
    //                     )
    //                 )
    //             );
    //         }
    //         (Some(provided_type), None) => provided_type,
    //         (None, Some(found_type)) => found_type,
    //         (Some(provided_type), Some(found_type)) => {
    //             if provided_type.is(&found_type) {
    //                 provided_type
    //             } else {
    //                 return Err(
    //                     CompileError::new(
    //                         ReportedError::new(
    //                             format!(
    //                                 "Type of '{}' is provided as '{}', but type '{}' was found based on its provided value",
    //                                 self.get_name(),
    //                                 provided_type.dissasemble(),
    //                                 found_type.dissasemble()
    //                             ),
    //                             {
    //                                 let mut chars_range: SrcCharsRange = self.get_metadata().into();
    //                                 chars_range.merge(
    //                                     &self.value.as_ref().unwrap().collect_metadata()
    //                                 );
    //                                 chars_range
    //                             }
    //                         )
    //                     )
    //                 );
    //             }
    //         }
    //     };
    //     Ok(value_type)
    // }

    pub fn get_name(&self) -> Rc<str> {
        self.ident_expr.get_lexeme()
    }

    pub fn get_value_type(&self) -> Option<&ValueType> {
        self.value_type.as_ref()
    }

    pub fn get_is_mutable(&self) -> bool {
        self.mut_keyword_metadata.is_some()
    }

    pub fn get_value(&self) -> Option<&ExprStmt<'ast>> {
        self.value.as_ref()
    }

    pub fn get_metadata(&self) -> TokenMetadata {
        self.ident_expr.get_metadata()
    }
}

impl<'ast> StmtTrait for VarDefStmt<'ast> {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        todo!()
    }

    fn is_linear_control_flow(&self) -> bool {
        true
    }
    fn validate_stmt(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    ) {
        let type_checked_val = match
            self.value.as_mut().map(|val| { val.type_check(program_symbol_table) })
        {
            Some(res) =>
                match res {
                    Ok(v) => { Some(v) }
                    Err(err) => {
                        error_handler.report_compile_error(err);
                        return;
                    }
                }
            None => None,
        };

        let value_type = self.value_type
            .as_ref()
            .cloned()
            .map(|value_type| (
                if let Some(type_checked_val) = &type_checked_val {
                    if type_checked_val.is(&value_type) {
                        value_type
                    } else {
                        todo!("Report that types is not the same")
                    }
                } else {
                    value_type
                }
            ))
            .unwrap_or_else(|| type_checked_val.as_ref().cloned().unwrap());

        self.value_type = Some(value_type.clone());

        program_symbol_table.insert_var(
            self.ident_expr.get_ssa_ident().clone(),
            UserSymbolVar::new(
                value_type,
                self.ident_expr.get_metadata(),
                self.mut_keyword_metadata
            )
        )
    }
    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        Some(self)
    }
}

impl<'ast> LinearControlFlow for VarDefStmt<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize {
        let value_node_id = self.value
            .as_ref()
            .map(|expr| expr.compile_into_dag(dag, ident_node_id_map, icfg_builder));

        let define_node_id = dag.push_node(
            DAGNode::DefineNode(
                DAGDefineNode::new(
                    self.ident_expr.get_ssa_ident().clone(),
                    self.get_is_mutable(),
                    value_node_id.is_some(),
                    self.value_type.as_ref().expect("Expected value type in VarDefStmt").clone()
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

impl<'ast> Dissasemble for VarDefStmt<'ast> {
    fn dissasemble(&self) -> String {
        let mutable_string = match self.mut_keyword_metadata.is_some() {
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
                    "{}{} {}{}\n",
                    mutable_string,
                    self.get_name(),
                    value_type.dissasemble(),
                    value_string
                )
            }
            None => {
                format!(
                    "{}{}{}\n",
                    mutable_string,
                    self.ident_expr.get_ssa_ident().dissasemble(),
                    value_string
                )
            }
        }
    }
}

impl<'ast> AstDissasemble for VarDefStmt<'ast> {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String {
        let mutable_string = match self.mut_keyword_metadata.is_some() {
            true => "mut ".to_string(),
            false => "".to_string(),
        };

        let value_string = match &self.value {
            Some(value) => format!(" := {}", value.dissasemble()),
            None => "".to_string(),
        };

        let final_str = match &self.value_type {
            Some(value_type) => {
                format!(
                    "{}{} {}{}\n",
                    mutable_string,
                    self.get_name(),
                    value_type.dissasemble(),
                    value_string
                )
            }
            None => {
                format!(
                    "{}{}{}\n",
                    mutable_string,
                    self.ident_expr.get_ssa_ident().dissasemble(),
                    value_string
                )
            }
        };

        format!(
            "[{}]: {}{}",
            program_symbol_table.get_current_symbol_table_id(),
            " ".repeat(AST_DISSASEMBLE_INDENTATION * scope_depth),
            final_str
        )
    }
}
