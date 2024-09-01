use ahash::AHashMap;

use crate::{
    compiler::{
        ds::{
            ssa_ident::SSAIdent,
            symbol_table::ProgramSymbolTablePhase1,
            value::{ Value, ValueType },
        },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::icfg::{
            dag::{ DAGAssignNode, DAGConstNode, DAGDeclareNode, DAGIndexNode, DAGNode, DAG },
            icfg_builder::ICFGBuilder,
        },
        traits::ExprTrait,
        Dissasemble,
    },
    macros::merge_chars_range,
};

use super::Expr;

#[derive(Debug)]
pub struct ArrayExpr<'ast> {
    items: Vec<Expr<'ast>>,
    item_types: Option<ValueType>,
}

impl<'ast> ArrayExpr<'ast> {
    pub fn new(items: Vec<Expr<'ast>>) -> Self {
        Self {
            items,
            item_types: None,
        }
    }

    pub fn get_items(&self) -> &Vec<Expr<'ast>> {
        &self.items
    }

    pub fn get_item_types(&self) -> Option<&ValueType> {
        self.item_types.as_ref()
    }
}

impl<'ast> Dissasemble for ArrayExpr<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("[");
        for i in 0..self.items.len() {
            string_builder += &self.items[i].dissasemble();
            if i != self.items.len() - 1 {
                string_builder += ", ";
            }
        }
        string_builder += "]";
        string_builder
    }
}

impl<'ast> ExprTrait for ArrayExpr<'ast> {
    fn get_result_type(&self) -> ValueType {
        ValueType::Array((
            Box::new(self.item_types.as_ref().expect("Expected to be set in typecheck").clone()),
            self.items.len(),
        ))
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        merge_chars_range!(
            self.items
                .iter()
                .map(|item| item.collect_metadata())
                .collect()
        )
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder,
        declaring_ssa_ident: Option<&SSAIdent>
    ) -> usize {
        let mut prev_node = dag.get_entry_node_id();

        if let Some(ssa_ident) = declaring_ssa_ident {
            for (i, item_expr) in self.items.iter().enumerate() {
                let index_node_id = dag.push_node(
                    DAGNode::IndexNode(
                        DAGIndexNode::new(
                            ssa_ident.clone(),
                            self.item_types.as_ref().expect("Set typechecking").clone(),
                            self.items.len()
                        )
                    )
                );
                let index_expr_node_id = dag.push_node(
                    DAGNode::ConstNode(DAGConstNode::new(Value::Int(i as i64)))
                );

                dag.add_edge(index_node_id, index_expr_node_id);

                let assign_node_id = dag.push_node(
                    DAGNode::AssignNode(
                        DAGAssignNode::new(
                            ssa_ident.clone(),
                            self.item_types.as_ref().expect("Set typechecking").clone(),
                            true
                        )
                    )
                );
                let value_node_id = item_expr.compile_into_dag(
                    dag,
                    ident_node_id_map,
                    icfg_builder,
                    declaring_ssa_ident
                );

                dag.add_edge(assign_node_id, index_node_id);
                dag.add_edge(assign_node_id, value_node_id);
                dag.add_edge(prev_node, assign_node_id);

                prev_node = assign_node_id;
            }
        }

        prev_node
    }

    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<(ValueType, Option<SSAIdent>), CompileError> {
        let mut array_items_type: Option<ValueType> = None;

        let mut reported_errors = vec![];

        self.items.iter_mut().for_each(|item| {
            let result = item.type_check(program_symbol_table);
            match result {
                Ok((v, _)) => {
                    if let Some(items_type) = &array_items_type {
                        if !items_type.is(&v) {
                            reported_errors.push(
                                ReportedError::new(
                                    format!(
                                        "Expected array items to be of type '{}' but got '{}'",
                                        items_type.dissasemble(),
                                        v.dissasemble()
                                    ),
                                    item.collect_metadata()
                                )
                            )
                        }
                    } else {
                        array_items_type = Some(v);
                    }
                }
                Err(err) => { reported_errors.extend(err.take_errs()) }
            }
        });

        if reported_errors.len() != 0 {
            Err(CompileError::new_multiple(reported_errors))
        } else {
            self.item_types = array_items_type.clone();
            Ok((
                ValueType::Array((
                    Box::new(array_items_type.expect("Expected a type for the array")),
                    self.items.len(),
                )),
                None,
            ))
        }
    }
}
