use std::{ fs::File, rc::Rc, io::Write };

use ahash::AHashMap;

mod llvm_instruction;
mod llvm_var;
mod llvm_function;
mod llvm_module;
mod llvm_operand;
mod llvm_type;

pub use llvm_function::Function;
pub use llvm_module::Module;
pub use llvm_operand::Operand;
pub use llvm_type::*;
pub use llvm_var::Var;

use super::{ ds::symbol_table::NativeSymbolFn, ir::icfg::{ cfg::CFG, ICFG } };

pub trait BuildLLVM {
    fn build(&self) -> String;
}

pub struct LLVMBuilder<'icfg> {
    main_module: Module,
    next_ssa_key: usize,
    icfg: &'icfg ICFG,
}

impl<'icfg> LLVMBuilder<'icfg> {
    pub fn new(icfg: &'icfg ICFG) -> Self {
        Self {
            main_module: Module::new(),
            next_ssa_key: 1,
            icfg,
        }
    }

    pub fn get_mod(&self) -> &Module {
        &self.main_module
    }

    pub fn get_mut_mod(&mut self) -> &mut Module {
        &mut self.main_module
    }

    pub fn get_cfg(&self, cfg_id: usize) -> &CFG {
        self.icfg.get_cfg(cfg_id)
    }

    pub fn build(&self) -> String {
        self.main_module.build()
    }

    pub fn output(&self) {
        let entry_file = &std::env::args().collect::<Vec<_>>()[1];
        let out_file = entry_file.replace(".vs", ".ll");

        let mut file = File::create(out_file).unwrap();

        write!(file, "{}", self.build()).unwrap();

        // Command::new("clang").arg("file.ll").arg("-o").arg("file").output().unwrap();
    }

    pub fn get_latest_ssa_key(&self) -> usize {
        self.next_ssa_key - 1
    }

    pub fn req_var_ssa_key(&mut self, ident: Rc<str>) -> usize {
        let ssa_key = self.req_ssa_key();
        self.get_mut_mod().insert_ident(ident, ssa_key);
        ssa_key
    }

    pub fn get_var_ssa_key(&self, ident: Rc<str>) -> usize {
        self.get_mod().lookup_ident_idx(&ident)
    }

    pub fn req_ssa_key(&mut self) -> usize {
        self.next_ssa_key += 1;
        self.next_ssa_key - 1
    }
}
