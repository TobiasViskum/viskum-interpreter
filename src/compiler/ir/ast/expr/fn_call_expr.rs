use ahash::AHashMap;

use crate::{
    compiler::{
        ds::{ ssa_ident::SSAIdent, symbol_table::{ NativeSymbolFn, SymbolFn }, value::ValueType },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::{
            ast::stmt::ExprStmt,
            icfg::{ dag::{ DAGFnCallNode, DAGNode, DAG }, icfg_builder::ICFGBuilder },
        },
        parser::token::TokenMetadata,
        print_todo,
        traits::{ Dissasemble, ExprTrait, LinearControlFlow, NativeFnTrait },
        ProgramSymbolTablePhase1,
    },
    macros::merge_chars_range,
};

#[derive(Debug)]
pub struct FnCallExpr<'ast> {
    ssa_ident: SSAIdent,
    metadata: TokenMetadata,
    args: Vec<ExprStmt<'ast>>,
    // Set during typechecking
    args_types: Vec<ValueType>,
    result_type: Option<ValueType>,
    is_native_fn: Option<NativeSymbolFn>,
}

impl<'ast> FnCallExpr<'ast> {
    pub fn new(ssa_ident: SSAIdent, metadata: TokenMetadata, args: Vec<ExprStmt<'ast>>) -> Self {
        Self {
            ssa_ident,
            metadata,
            args,
            args_types: vec![],
            result_type: None,
            is_native_fn: None,
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
        if let Some(native_fn) = self.is_native_fn {
            panic!("Support for native functions not implemented: {}", native_fn.get_ident());
        }

        let fn_call_node_id = dag.push_node(
            DAGNode::FnCallNode(
                DAGFnCallNode::new(
                    self.args_types.clone(),
                    self.ssa_ident.clone(),
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
                if let SymbolFn::NativeSymbolFn(native_fn) = &symbol_fn {
                    self.is_native_fn = Some(*native_fn);
                }
                let ret_type = symbol_fn.get_ret_type();
                self.result_type = Some(ret_type.clone());

                ret_type
            }
            Err(msg) => {
                return Err(CompileError::new(ReportedError::new(msg, self.metadata.into())));
            }
        };

        print_todo("Compare amount of args and each arg value type");

        Ok(symbol_fn)
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
