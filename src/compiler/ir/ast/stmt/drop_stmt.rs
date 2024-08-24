use ahash::AHashMap;

use crate::compiler::{
    ds::symbol_table::{ SSAIdent, SymbolTableRef },
    error_handler::ErrorHandler,
    ir::icfg::{
        dag::{ DAGDropNode, DAGIdentNode, DAGNode, DAG },
        icfg_builder::{ CFGBuilder },
        ICFG,
    },
    traits::{ LinearControlFlow, StmtTrait },
    Dissasemble,
};

use super::{ GotoNodeIds };

#[derive(Debug)]
pub struct DropStmt {
    vars: Vec<SSAIdent>,
    // strings
    // other heap allocated objects
}

impl DropStmt {
    pub fn new(vars: Vec<SSAIdent>) -> Self {
        Self {
            vars,
        }
    }

    pub fn get_vars(&self) -> &Vec<SSAIdent> {
        &self.vars
    }
}

impl StmtTrait for DropStmt {
    fn compile_into_icfg(
        &self,
        icfg: &mut ICFG,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) {
        todo!()
    }

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    ) {}

    fn is_linear_control_flow(&self) -> bool {
        true
    }

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow> {
        Some(self)
    }
}

impl LinearControlFlow for DropStmt {
    fn compile_into_dag(&self, dag: &mut DAG, _: &mut AHashMap<SSAIdent, usize>) -> usize {
        let drops_count = self.vars.len();

        let drop_node_id = dag.push_node(DAGNode::DropNode(DAGDropNode::new(drops_count)));

        for ssa_key in self.vars.iter() {
            let var_drop_node_id = dag.push_node(
                DAGNode::IdentNode(DAGIdentNode::new(ssa_key.clone()))
            );

            dag.add_edge(drop_node_id, var_drop_node_id);
        }

        dag.set_entry_node_id(drop_node_id);

        drop_node_id
    }
}

impl Dissasemble for DropStmt {
    fn dissasemble(&self) -> String {
        let mut string_builder = String::from("drop(");
        for (i, ssa_key) in self.vars.iter().enumerate() {
            if i != 0 {
                string_builder += ", ";
            }
            string_builder += ssa_key.dissasemble().as_str();
        }
        string_builder += ")\n";
        string_builder
    }
}
