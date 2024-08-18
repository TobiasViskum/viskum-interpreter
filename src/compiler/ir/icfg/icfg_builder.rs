use ahash::AHashMap;

use crate::compiler::{
    ds::{ symbol_table::SSAKey, value::ValueType },
    ir::ast::stmt::FunctionStmt,
    traits::LinearControlFlow,
    Dissasemble,
};

use super::{
    cfg::{ CFGEdge, CFGNode, CFGNodeType, CFGProcessNode, CFGTerminateNode, CFG },
    dag::DAG,
    ICFG,
};

#[derive(Debug)]
pub struct ICFGBuilder {
    fn_to_cfg_id: AHashMap<SSAKey, usize>,
    icfg: ICFG,
}

impl ICFGBuilder {
    pub fn new() -> Self {
        Self {
            fn_to_cfg_id: AHashMap::new(),
            icfg: ICFG::new(),
        }
    }

    pub fn take_cfg(self) -> ICFG {
        self.icfg
    }

    pub fn set_entry_cfg(&mut self, entry_cfg_id: usize) {
        self.icfg.set_entry_cfg(entry_cfg_id)
    }

    pub fn push_cfg(&mut self, cfg: CFG) -> usize {
        self.icfg.push_cfg(cfg);
        self.icfg.cfgs.len() - 1
    }
}

#[derive(Debug)]
pub struct CFGBuilder {
    building_cfg: CFG,
    current_cfg_node_id: usize,
    building_linear_basic_block: Option<LinearBasicBlock>,
    next_label_id: usize,
}

impl CFGBuilder {
    pub fn new() -> Self {
        let mut cfg = CFG::new("main".to_string(), ValueType::Int);
        cfg.push_node(CFGNode::new(CFGNodeType::TerminateNode(CFGTerminateNode::new_start())));
        cfg.add_edge(0, 1);

        Self {
            current_cfg_node_id: 0,
            building_cfg: cfg,
            building_linear_basic_block: None,
            next_label_id: 1,
        }
    }

    pub fn debug(&self) {
        for (i, node) in self.building_cfg.iter_nodes().enumerate() {
            println!("Node {}: {}", i, node.dissasemble());
        }
    }

    pub fn req_label_id(&mut self) -> usize {
        self.next_label_id += 1;
        self.next_label_id - 1
    }

    pub fn end_cfg(mut self) -> CFG {
        self.push_linear_block_if_exists();
        self.push_cfg_node(CFGNode::new(CFGNodeType::TerminateNode(CFGTerminateNode::new_end())));
        self.debug();

        self.building_cfg.print_nodes();
        self.building_cfg
    }

    pub fn end_entry_cfg(mut self) -> CFG {
        self.push_linear_block_if_exists();
        self.push_cfg_node(CFGNode::new(CFGNodeType::TerminateNode(CFGTerminateNode::new_end())));
        self.debug();

        self.building_cfg.print_nodes();
        self.building_cfg
    }

    pub fn get_next_cfg_node_id(&self) -> usize {
        self.current_cfg_node_id + 1
    }

    pub fn get_current_cfg_node_id(&self) -> usize {
        self.current_cfg_node_id
    }

    pub fn push_cfg_edge(&mut self, origin: usize, dest: usize) {
        self.building_cfg.add_edge(origin, dest)
    }

    pub fn push_cfg_node(&mut self, cfg_node: CFGNode) -> usize {
        self.push_linear_block_if_exists();
        self.building_cfg.push_node(cfg_node);
        self.current_cfg_node_id += 1;
        self.current_cfg_node_id
    }

    pub fn build_into_linear_basic_block(&mut self, linear_stmt: &dyn LinearControlFlow) {
        let linear_basic_block = self.setup_or_get_mut_building_linear_basic_block();
        let (dag, ident_node_id_map) = linear_basic_block.get_linear_block_data();

        let dag_node_id = linear_stmt.compile_into_dag(dag, ident_node_id_map);

        if let Some(prev_node_id) = linear_basic_block.get_prev_dag_node_id() {
            linear_basic_block.get_dag().add_edge(dag_node_id, prev_node_id);
        }
        linear_basic_block.set_prev_dag_node_id(dag_node_id)
    }

    pub fn print_cfg_nodes(&self) {
        self.building_cfg.print_nodes()
    }

    pub fn push_linear_block_if_exists(&mut self) {
        match self.building_linear_basic_block.take() {
            Some(linear_basic_block) => {
                let dag = linear_basic_block.take_dag();
                self.building_cfg.push_node(
                    CFGNode::new(CFGNodeType::ProcessNode(CFGProcessNode::new(dag)))
                );
                self.current_cfg_node_id += 1;

                self.push_cfg_edge(self.current_cfg_node_id, self.current_cfg_node_id + 1)
            }
            None => {}
        }
    }

    fn setup_or_get_mut_building_linear_basic_block(&mut self) -> &mut LinearBasicBlock {
        if self.building_linear_basic_block.is_none() {
            self.building_linear_basic_block = Some(LinearBasicBlock::new());
        }

        self.building_linear_basic_block.as_mut().unwrap()
    }
}

#[derive(Debug)]
struct LinearBasicBlock {
    dag: DAG,
    ident_node_id_map: AHashMap<SSAKey, usize>,
    prev_dag_node_id: Option<usize>,
}

impl LinearBasicBlock {
    pub fn new() -> Self {
        Self {
            dag: DAG::new(),
            ident_node_id_map: AHashMap::new(),
            prev_dag_node_id: None,
        }
    }

    pub fn take_dag(self) -> DAG {
        self.dag
    }

    pub fn get_linear_block_data(&mut self) -> (&mut DAG, &mut AHashMap<SSAKey, usize>) {
        (&mut self.dag, &mut self.ident_node_id_map)
    }

    pub fn get_prev_dag_node_id(&self) -> Option<usize> {
        self.prev_dag_node_id
    }

    pub fn set_prev_dag_node_id(&mut self, new_prev_dag_node_id: usize) {
        self.prev_dag_node_id = Some(new_prev_dag_node_id);
    }

    pub fn get_dag(&mut self) -> &mut DAG {
        &mut self.dag
    }
}
