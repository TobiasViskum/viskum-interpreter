pub mod cfg;
pub mod dag;
pub mod icfg_builder;

use ahash::AHashMap;
use cfg::{ CFGNodeType, CFG };

use crate::{
    compiler::{
        ds::{ register_allocator::RegisterAllocator, vm_builder::VMBuilder },
        traits::{ Dissasemble, LoadConstants, ParseConnectedNodes },
    },
    vm::instructions::Instruction,
};

pub type CFGId = usize;

#[derive(Debug)]
pub struct ICFG {
    cfgs: Vec<CFG>,
    entry_cfg: usize,
}

impl ICFG {
    pub fn new() -> Self {
        Self {
            cfgs: Vec::new(),
            entry_cfg: 0,
        }
    }

    pub fn print(&self) {
        println!("{}", "\n----- ICFG -----\n");
        println!("{}", self.dissasemble());
        println!("{}", "\n---------------\n")
    }

    pub fn push_cfg(&mut self, cfg: CFG) -> CFGId {
        self.cfgs.push(cfg);
        self.cfgs.len() - 1
    }

    pub fn set_entry_cfg(&mut self, entry_id: CFGId) {
        self.entry_cfg = entry_id;
    }

    pub fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let instructions = self.cfgs[0].generate_instructions(register_allocator);
        instructions

        // type CFGNodeId = usize;
        // type InstrId = usize;
        // let mut cfg_node_id_to_instr_id = AHashMap::<CFGNodeId, InstrId>::new();
        // let mut instructions = vec![];
        // println!("{:#?}", cfg_node_id_to_instr_id);
        // type GotoNodeId = usize;
        // type ConnectedId = usize;
        // let mut goto_node_ids_to_update = AHashMap::<GotoNodeId, ConnectedId>::new();
        // for (node_id, node_type) in self.cfgs[0]
        //     .iter_nodes()
        //     .map(|node| node.get_node_type())
        //     .enumerate() {
        //     if let CFGNodeType::GotoNode(goto_node) = node_type {
        //         let connected_node_id = goto_node.parse_connected_nodes(
        //             self.cfgs[0].get_connected_nodes(node_id)
        //         );

        //         println!("Goto_node_id: {}, linked_cfg_node_id: {}", node_id, connected_node_id);

        //         let goto_instr_id = cfg_node_id_to_instr_id
        //             .get(&node_id)
        //             .expect("Expected goto_instr_id");
        //         let target_instr_id = cfg_node_id_to_instr_id
        //             .get(&connected_node_id)
        //             .expect("Expected target_instr_id");

        //         if
        //             let Instruction::Goto { jmp_pos } = instructions
        //                 .get_mut(*goto_instr_id)
        //                 .expect("Expected Goto instruction")
        //         {
        //             *jmp_pos = *target_instr_id;
        //         }
        //         goto_node_ids_to_update.insert(node_id, connected_node_id);
        //     }
        // }

        // // for (goto_node_id, connected_node_id) in goto_node_ids_to_update.iter() {
        // //     if
        // //         let Some(goto_node) = self.cfgs[0]
        // //             .get_mut_node(goto_node_id)
        // //             .map(|node| node.get_mut_node_type())
        // //     {
        // //     }
        // // }
        // // println!("{:#?}", goto_node_ids_to_update);

        // instructions.push(Instruction::Halt);
        // instructions
    }
}

impl LoadConstants for ICFG {
    fn load_constants(&self, vm_builder: &mut VMBuilder) {
        self.cfgs[0].load_constants(vm_builder)
    }
}

impl Dissasemble for ICFG {
    fn dissasemble(&self) -> String {
        self.cfgs[self.entry_cfg].dissasemble()
    }
}
