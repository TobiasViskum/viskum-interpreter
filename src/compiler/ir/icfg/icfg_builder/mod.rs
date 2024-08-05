use ahash::AHashMap;

use crate::compiler::{ ds::symbol_table::SSAKey, traits::LinearControlFlow, Dissasemble };

use super::{ cfg::{ CFGNode, CFGNodeType, CFGProcessNode, CFG }, dag::DAG, ICFG };

mod private_methods;

#[derive(Debug)]
pub struct LinearBasicBlock {
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

#[derive(Debug)]
pub struct ICFGBuilder {
    icfg: ICFG,
    building_cfg: CFG,
    prev_node_id: Option<usize>,
    building_linear_basic_block: Option<LinearBasicBlock>,
}

impl ICFGBuilder {
    pub fn new() -> Self {
        Self {
            icfg: ICFG::new(),
            building_cfg: CFG::new(),
            prev_node_id: None,
            building_linear_basic_block: None,
        }
    }

    pub fn take_icfg(mut self) -> ICFG {
        self.push_linear_block_if_exists();
        self.icfg.push_cfg(self.building_cfg);
        self.icfg
    }

    pub fn push_cfg_node(&mut self, cfg_node: CFGNode) -> usize {
        self.push_linear_block_if_exists();
        self.building_cfg.push_node(cfg_node)
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

    pub fn print_icfg(&self) {
        println!("{}", self.icfg.dissasemble())
    }

    pub fn get_prev_node_id(&self) -> Option<usize> {
        self.prev_node_id
    }

    pub fn set_prev_node_id(&mut self, new_prev_node_id: usize) {
        self.prev_node_id = Some(new_prev_node_id);
    }

    // pub fn push_dag_into_basic_block(&mut self, dag: DAG) {
    //     match self.building_cfg.last_mut_node() {
    //         Some(node) => {
    //             match node.get_mut_node_type() {
    //                 CFGNodeType::ProcessNode(node) => {
    //                     node.get_mut_dag();
    //                 }
    //                 _ => self.push_process_node_from_dag(dag),
    //             };
    //         }
    //         _ => self.push_process_node_from_dag(dag),
    //     };
    // }

    // fn push_process_node_from_dag(&mut self, dag: DAG) {
    //     self.building_cfg.push_node(
    //         CFGNode::new(CFGNodeType::ProcessNode(CFGProcessNode::new(dag)))
    //     );
    // }
}
