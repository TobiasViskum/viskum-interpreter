use std::cell::RefCell;

use ahash::{ HashSet, HashSetExt };

use crate::{
    ast::Ast,
    compiler::vm_symbol_table::VMSymbolTable,
    value::Value,
    vm::{ instructions::{ Instruction, InstructionSrc }, Instructions },
};

use self::generate_cfg::generate_cfgs;

use super::cfg::{ CFGNode, CFG };

pub(super) mod helper_structs;
mod generate_cfg;

#[derive(Debug)]
pub struct ICFG {
    cfgs: Vec<CFG>,
    entry_cfg: usize,
}

impl ICFG {
    pub fn from_ast(ast: Ast) -> Self {
        let mut icfg = ICFG { cfgs: vec![], entry_cfg: 0 };

        generate_cfgs(&mut icfg, ast.main_scope, 0);

        icfg
    }

    pub fn push_cfg(&mut self, cfg: CFG) -> usize {
        let cfg_id = self.cfgs.len();
        self.cfgs.push(cfg);
        self.entry_cfg = cfg_id;
        cfg_id
    }

    #[profiler::function_tracker]
    pub fn optimize_and_generate_bytecode(&mut self) -> Vec<Instruction> {
        self.dde();

        let mut bytecode = self.generate_bytecode();
        bytecode.push(Instruction::Halt);

        bytecode
    }

    fn dissassemble(&self) {
        let mut i = 0;
        for cfg in &self.cfgs {
            println!("\nCfg {}:", i);
            cfg.dissassemble();
            i += 1;
        }
    }

    fn generate_bytecode(&mut self) -> Vec<Instruction> {
        #[cfg(debug_assertions)]
        {
            self.dissassemble();
        }

        let mut vm_symbol_table = VMSymbolTable::new();

        let instructions = RefCell::new(Vec::new());

        let function_connector_ids = self.cfgs
            .get(self.entry_cfg)
            .unwrap()
            .generate_bytecode(&mut vm_symbol_table, &instructions, None);

        for (cfg_id, args, instruction_id) in &function_connector_ids {
            let function_instructions: RefCell<Vec<Instruction>> = RefCell::new(Vec::new());

            // let args = match self.cfgs.get(self.entry_cfg).unwrap().get_node(node_id).unwrap() {
            //     CFGNode
            // }

            vm_symbol_table.begin_func_compilation();

            self.cfgs[*cfg_id].generate_bytecode(
                &mut vm_symbol_table,
                &function_instructions,
                Some(args.clone())
            );
            vm_symbol_table.end_func_compilation();

            match instructions.borrow_mut().get_mut(*instruction_id).unwrap() {
                Instruction::Define { src, .. } =>
                    match src {
                        InstructionSrc::Constant { val } =>
                            match val {
                                Value::Function(func) => {
                                    (*func).instructions = Instructions::from(
                                        function_instructions.take()
                                    );
                                }
                                _ => panic!("Expected function"),
                            }
                        _ => panic!("Expected Constant"),
                    }
                _ => panic!("Expected "),
            }
        }

        instructions.take()
    }

    fn dde(&mut self) {
        let mut visited_cfgs: HashSet<usize> = HashSet::new();

        let mut cfgs_to_analyze: Vec<usize> = vec![self.entry_cfg];

        while cfgs_to_analyze.len() != 0 {
            let mut cfg_ids_to_push: Vec<usize> = vec![];
            for cfg_id in &cfgs_to_analyze {
                let cfg = self.cfgs.get_mut(*cfg_id).unwrap();
                cfg_ids_to_push.extend(cfg.dde(&mut visited_cfgs));
            }
            cfgs_to_analyze.clear();

            cfgs_to_analyze.extend(cfg_ids_to_push);
        }
    }
}
