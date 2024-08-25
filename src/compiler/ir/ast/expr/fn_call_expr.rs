use std::rc::Rc;

use ahash::AHashMap;

use crate::{
    compiler::{
        ds::{ ssa_ident::SSAIdent, value::ValueType },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::{
            ast::stmt::ExprStmt,
            icfg::{ dag::{ DAGFnCallNode, DAGNode, DAG }, icfg_builder::ICFGBuilder },
        },
        parser::token::TokenMetadata,
        print_todo,
        traits::{ Dissasemble, ExprTrait, LinearControlFlow },
        ProgramSymbolTablePhase1,
    },
    macros::merge_chars_range,
};

#[derive(Debug)]
pub struct FnCallExpr<'ast> {
    ssa_ident: SSAIdent,
    metadata: TokenMetadata,
    args: Vec<ExprStmt<'ast>>,
    result_type: Option<ValueType>,
}

impl<'ast> FnCallExpr<'ast> {
    pub fn new(ssa_ident: SSAIdent, metadata: TokenMetadata, args: Vec<ExprStmt<'ast>>) -> Self {
        Self {
            ssa_ident,
            metadata,
            args,
            result_type: None,
        }
    }
}

impl<'ast> Dissasemble for FnCallExpr<'ast> {
    fn dissasemble(&self) -> String {
        let mut string_builder = format!("{}(", self.ssa_ident.dissasemble());
        for i in 0..self.args.len() {
            string_builder += &self.args[i].dissasemble();
            if i != 0 {
                string_builder += ", ";
            }
        }
        string_builder += ")";
        string_builder
    }
}

impl<'ast> ExprTrait for FnCallExpr<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize {
        let fn_call_node_id = dag.push_node(
            DAGNode::FnCallNode(
                DAGFnCallNode::new(
                    self.args.len(),
                    self.ssa_ident.clone(),
                    icfg_builder.get_fn_cfg_id(&self.ssa_ident),
                    self.result_type.as_ref().expect("Expected result type in FnCallExpr").clone()
                )
            )
        );

        let args_ids = self.args.iter().fold(vec![], |mut acc, arg| {
            acc.push(arg.compile_into_dag(dag, ident_node_id_map, icfg_builder));
            acc
        });

        for arg_id in args_ids.iter() {
            dag.add_edge(fn_call_node_id, *arg_id);
        }

        fn_call_node_id
    }

    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<ValueType, CompileError> {
        let symbol_fn = match program_symbol_table.lookup_fn(&self.ssa_ident) {
            Ok(symbol_fn) => {
                let ret_type = symbol_fn.get_ret_type().clone();
                self.result_type = Some(ret_type);

                symbol_fn
            }
            Err(msg) => {
                return Err(CompileError::new(ReportedError::new(msg, self.metadata.into())));
            }
        };

        print_todo("Compare amount of args and each arg value type");

        Ok(symbol_fn.get_ret_type().clone())
    }

    fn collect_metadata(&self) -> SrcCharsRange {
        let metadata = self.metadata.into();

        merge_chars_range!(
            metadata,
            self.args
                .iter()
                .map(|arg| arg.collect_metadata())
                .collect()
        )
    }
}
