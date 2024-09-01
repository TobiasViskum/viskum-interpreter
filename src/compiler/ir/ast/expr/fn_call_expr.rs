use ahash::AHashMap;

use crate::{
    compiler::{
        ds::{
            ssa_ident::SSAIdent,
            symbol_table::{ NativeSymbolFn, SymbolFn, UserNativeSymbolFn },
            value::{ Value, ValueType },
        },
        error_handler::{ CompileError, ReportedError, SrcCharsRange },
        ir::{
            ast::stmt::ExprStmt,
            icfg::{
                dag::{
                    DAGConstNode,
                    DAGDerefNode,
                    DAGNativeFnCallNode,
                    DAGNode,
                    DAGUserFnCallNode,
                    DAG,
                },
                icfg_builder::ICFGBuilder,
            },
        },
        parser::token::TokenMetadata,
        print_todo,
        traits::{ Dissasemble, ExprTrait, LinearControlFlow, NativeFnTrait },
        ProgramSymbolTablePhase1,
    },
    macros::merge_chars_range,
};

use super::Expr;

#[derive(Debug)]
pub struct FnCallExpr<'ast> {
    ssa_ident: SSAIdent,
    metadata: TokenMetadata,
    args: Vec<Expr<'ast>>,
    // Set during typechecking
    args_types: Vec<ValueType>,
    result_type: Option<ValueType>,
    is_native_fn: Option<UserNativeSymbolFn>,
}

impl<'ast> FnCallExpr<'ast> {
    pub fn new(ssa_ident: SSAIdent, metadata: TokenMetadata, args: Vec<Expr<'ast>>) -> Self {
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
            if i != self.args.len() - 1 {
                string_builder += ", ";
            }
        }
        string_builder += ")";
        string_builder
    }
}

impl<'ast> ExprTrait for FnCallExpr<'ast> {
    fn get_result_type(&self) -> ValueType {
        self.result_type.as_ref().expect("TC").clone()
    }

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder,
        declaring_ssa_ident: Option<&SSAIdent>
    ) -> usize {
        let fn_call_node_id = if let Some(native_fn) = self.is_native_fn {
            let mut args_types = match native_fn {
                UserNativeSymbolFn::PrintFn(_) => {
                    let mut args_types = vec![ValueType::String];
                    args_types.extend(self.args_types.clone());
                    args_types
                }
            };

            let args_ids = self.args
                .iter()
                .enumerate()
                .fold(vec![], |mut acc, (i, arg)| {
                    let result_type = arg.get_result_type();

                    let mut arg_id = arg.compile_into_dag(
                        dag,
                        ident_node_id_map,
                        icfg_builder,
                        declaring_ssa_ident
                    );

                    if let ValueType::Ptr(inner_value_type) = result_type {
                        let deref_node_id = dag.push_node(
                            DAGNode::DerefNode(DAGDerefNode::new(*inner_value_type.clone()))
                        );
                        args_types[i + 1] = *inner_value_type.clone();

                        dag.add_edge(deref_node_id, arg_id);
                        arg_id = deref_node_id;
                    }

                    acc.push(arg_id);
                    acc
                });

            let fn_call_node_id = match native_fn {
                UserNativeSymbolFn::PrintFn(f) => {
                    let mut iter = args_types.iter().enumerate();
                    iter.next();
                    let fmt_args = iter
                        .map(|(i, arg_type)|
                            format!("{}{}", arg_type.to_llvm_type().to_fmt_str(), if
                                i != args_types.len() - 1
                            {
                                " "
                            } else {
                                ""
                            })
                        )
                        .collect::<String>();

                    let fmt_string_id = dag.push_node(
                        DAGNode::ConstNode(
                            DAGConstNode::new(Value::String(format!("{}\\0A", fmt_args).into()))
                        )
                    );

                    let dag_call_node = DAGNode::NativeFnCallNode(
                        DAGNativeFnCallNode::new(native_fn, args_types)
                    );
                    let fn_call_node_id = dag.push_node(dag_call_node);

                    dag.add_edge(fn_call_node_id, fmt_string_id);

                    fn_call_node_id
                }
            };

            for arg_id in args_ids.iter() {
                dag.add_edge(fn_call_node_id, *arg_id);
            }

            fn_call_node_id
        } else {
            let dag_call_node = DAGNode::UserFnCallNode(
                DAGUserFnCallNode::new(
                    self.args_types.clone(),
                    self.ssa_ident.clone(),
                    self.result_type.as_ref().expect("Expected result type in FnCallExpr").clone()
                )
            );

            dag.push_node(dag_call_node)
        };

        fn_call_node_id
    }

    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<(ValueType, Option<SSAIdent>), CompileError> {
        let symbol_fn = match program_symbol_table.lookup_fn(&self.ssa_ident) {
            Ok(symbol_fn) => {
                if let SymbolFn::NativeSymbolFn(native_fn) = &symbol_fn {
                    self.is_native_fn = Some(*native_fn);
                }
                let reported_error = self.args
                    .iter_mut()
                    .filter_map(|arg| {
                        match arg.type_check(program_symbol_table) {
                            Ok((v, _)) => {
                                self.args_types.push(v);
                                None
                            }
                            Err(err) => Some(err.take_errs()),
                        }
                    })
                    .reduce(|mut acc, errs| {
                        acc.extend(errs);
                        acc
                    })
                    .map(|errs| CompileError::new_multiple(errs));

                if let Some(compile_err) = reported_error {
                    return Err(compile_err);
                }

                print_todo("Compare args len and types here");

                let ret_type = symbol_fn.get_ret_type();
                self.result_type = Some(ret_type.clone());

                ret_type
            }
            Err(msg) => {
                return Err(CompileError::new(ReportedError::new(msg, self.metadata.into())));
            }
        };

        Ok((symbol_fn, None))
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
