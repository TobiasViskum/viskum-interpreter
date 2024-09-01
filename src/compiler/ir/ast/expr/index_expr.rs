use ahash::AHashMap;

use crate::{
    compiler::{
        ds::{
            ssa_ident::{ self, SSAIdent },
            symbol_table::ProgramSymbolTablePhase1,
            value::ValueType,
        },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::icfg::{
            dag::{ DAGDefineNode, DAGIdentNode, DAGIndexNode, DAGNode, DAG },
            icfg_builder::ICFGBuilder,
        },
        traits::ExprTrait,
        Dissasemble,
    },
    macros::merge_chars_range,
};

use super::{ ArrayExpr, Expr, IdentifierExpr };

#[derive(Debug)]
pub struct IndexExpr<'ast> {
    array_ssa_ident: Option<SSAIdent>,
    field_expr: &'ast mut Expr<'ast>,
    index_expr: &'ast mut Expr<'ast>,
    array_items_type: Option<ValueType>,
    array_items_count: usize,
    result_type: Option<ValueType>,
}

impl<'ast> IndexExpr<'ast> {
    pub fn new(field_expr: &'ast mut Expr<'ast>, index_expr: &'ast mut Expr<'ast>) -> Self {
        Self {
            array_ssa_ident: None,
            field_expr,
            index_expr,
            array_items_count: 0,
            array_items_type: None,
            result_type: None,
        }
    }

    pub fn get_field_expr(&self) -> &Expr<'ast> {
        self.field_expr
    }

    pub fn get_mut_field_expr(&mut self) -> &mut Expr<'ast> {
        self.field_expr
    }

    pub fn get_index_expr(&self) -> &Expr<'ast> {
        self.index_expr
    }

    pub fn get_mut_index_expr(&mut self) -> &mut Expr<'ast> {
        self.index_expr
    }
}

impl<'ast> ExprTrait for IndexExpr<'ast> {
    fn get_result_type(&self) -> ValueType {
        self.result_type.as_ref().expect("TC").clone()
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        merge_chars_range!(self.field_expr.collect_metadata(), self.index_expr.collect_metadata())
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder,
        declaring_ssa_ident: Option<&SSAIdent>
    ) -> usize {
        if let Some(ssa_ident) = &self.array_ssa_ident {
            let index_expr_node_id = self.index_expr.compile_into_dag(
                dag,
                ident_node_id_map,
                icfg_builder,
                declaring_ssa_ident
            );

            let index_node_id = dag.push_node(
                DAGNode::IndexNode(
                    DAGIndexNode::new(
                        ssa_ident.clone(),
                        self.array_items_type
                            .as_ref()
                            .expect("Expected array_items_type to be set during typechecking")
                            .clone(),
                        self.array_items_count
                    )
                )
            );

            // dag.add_edge(index_node_id, field_expr_node_id);
            dag.add_edge(index_node_id, index_expr_node_id);

            index_node_id
        } else {
            panic!("Right now only indexing like this is supported: {{ident}}[{{expr}}]")
        }

        // match &self.field_expr {
        //     FieldIndexExprType::ArrayExpr(expr) => {
        //         let random_subscript = {
        //             let mut subscript =
        //                 (dag.get_edges().len() + dag.get_entry_node_id()) * dag.get_nodes().len();
        //             subscript += ident_node_id_map.len() * 2;

        //             subscript
        //         };
        //         let unnamed_ssa_ident = SSAIdent::new("(#unnamed)".into(), random_subscript);

        //         let array_item_type = expr
        //             .get_item_types()
        //             .expect("Expected item types to be set during typechecking")
        //             .clone();

        //         let define_node = DAGNode::DefineNode(
        //             DAGDefineNode::new(
        //                 unnamed_ssa_ident.clone(),
        //                 ValueType::Array((
        //                     Box::new(array_item_type.clone()),
        //                     expr.get_items().len(),
        //                 ))
        //             )
        //         );

        //         let define_node_id = dag.push_node(define_node);
        //         let array_node_id = expr.compile_into_dag(dag, ident_node_id_map, icfg_builder);

        //         dag.add_edge(define_node_id, array_node_id);

        //         // let ident_node_id = dag.push_node(
        //         //     DAGNode::IdentNode(
        //         //         DAGIdentNode::new(
        //         //             unnamed_ssa_ident.clone(),
        //         //             ValueType::Array((Box::new(array_item_type), expr.get_items().len()))
        //         //         )
        //         //     )
        //         // );
        //         let index_expr_node_id = self.index_expr.compile_into_dag(
        //             dag,
        //             ident_node_id_map,
        //             icfg_builder
        //         );

        //         let index_node_id = dag.push_node(
        //             DAGNode::IndexNode(
        //                 DAGIndexNode::new(
        //                     unnamed_ssa_ident.clone(),
        //                     self.array_items_type
        //                         .as_ref()
        //                         .expect("Expected array_items_type to be set during typechecking")
        //                         .clone(),
        //                     self.array_items_count
        //                 )
        //             )
        //         );

        //         dag.add_edge(index_node_id, index_expr_node_id);

        //         index_node_id
        //     }
        //     FieldIndexExprType::IdentExpr(expr) => {
        //         // let field_expr_node_id = expr.compile_into_dag(
        //         //     dag,
        //         //     ident_node_id_map,
        //         //     icfg_builder
        //         // );
        //         let index_expr_node_id = self.index_expr.compile_into_dag(
        //             dag,
        //             ident_node_id_map,
        //             icfg_builder
        //         );

        //         let index_node_id = dag.push_node(
        //             DAGNode::IndexNode(
        //                 DAGIndexNode::new(
        //                     expr.get_ssa_ident().clone(),
        //                     self.array_items_type
        //                         .as_ref()
        //                         .expect("Expected array_items_type to be set during typechecking")
        //                         .clone(),
        //                     self.array_items_count
        //                 )
        //             )
        //         );

        //         // dag.add_edge(index_node_id, field_expr_node_id);
        //         dag.add_edge(index_node_id, index_expr_node_id);

        //         index_node_id
        //     }
        // }
    }

    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<(ValueType, Option<SSAIdent>), CompileError> {
        let (field_type, ssa_ident) = self.field_expr.type_check(program_symbol_table)?;
        let (index_type, _) = self.index_expr.type_check(program_symbol_table)?;

        if let Some(ssa_ident) = ssa_ident {
            self.array_ssa_ident = Some(ssa_ident.clone());
        }

        match &index_type {
            ValueType::Int => {}
            | ValueType::Array(_)
            | ValueType::Bool
            | ValueType::String
            | ValueType::Void
            | ValueType::Ptr(_) => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Expected index expression to be of type '{}' but got '{}'",
                                ValueType::Int.dissasemble(),
                                index_type.dissasemble()
                            ),
                            self.index_expr.collect_metadata()
                        )
                    )
                );
            }
        }

        match field_type {
            ValueType::Array((items_type, items_count)) => {
                self.array_items_count = items_count;
                self.array_items_type = Some(*items_type.clone());
                self.result_type = Some(ValueType::Ptr(Box::new(*items_type.clone())));
                Ok((ValueType::Ptr(items_type), self.array_ssa_ident.clone()))
            }
            | ValueType::Bool
            | ValueType::Int
            | ValueType::String
            | ValueType::Void
            | ValueType::Ptr(_) => {
                return Err(
                    CompileError::new(
                        ReportedError::new(
                            format!(
                                "Index expressions is only possible on arrays or vectors, but got type '{}'",
                                field_type.dissasemble()
                            ),
                            self.field_expr.collect_metadata()
                        )
                    )
                );
            }
        }
    }
}

impl<'ast> Dissasemble for IndexExpr<'ast> {
    fn dissasemble(&self) -> String {
        format!("{}[{}]", self.field_expr.dissasemble(), self.index_expr.dissasemble())
    }
}
