use ahash::AHashMap;

use crate::compiler::{
    ds::symbol_table::SSAKey,
    ir::icfg::{ cfg::{ CFGNode, CFGNodeType, CFGProcessNode }, dag::DAG },
};

use super::{ ICFGBuilder, LinearBasicBlock };

impl ICFGBuilder {
    pub(super) fn setup_or_get_mut_building_linear_basic_block(&mut self) -> &mut LinearBasicBlock {
        if self.building_linear_basic_block.is_none() {
            self.building_linear_basic_block = Some(LinearBasicBlock::new());
        }

        self.building_linear_basic_block.as_mut().unwrap()
    }

    pub(super) fn push_linear_block_if_exists(&mut self) {
        match self.building_linear_basic_block.take() {
            Some(linear_basic_block) => {
                let dag = linear_basic_block.take_dag();
                self.building_cfg.push_node(
                    CFGNode::new(CFGNodeType::ProcessNode(CFGProcessNode::new(dag)))
                );
            }
            None => {}
        }
    }
}
