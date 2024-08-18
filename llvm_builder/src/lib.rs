mod llvm_builder;

use std::{ fs::File, process::Command, rc::Rc, io::Write };

use ahash::AHashMap;
pub use llvm_builder::{
    llvm_function::Function,
    llvm_module::Module,
    llvm_operand::Operand,
    llvm_var::Var,
    llvm_type::*,
};

pub trait BuildLLVM {
    fn build(&self) -> String;
}

pub struct LLVMBuilder {
    mods: Vec<Module>,
    idents: AHashMap<Rc<str>, usize>,
    next_ssa_key: usize,
}

impl LLVMBuilder {
    pub fn new() -> Self {
        Self {
            mods: vec![],
            idents: AHashMap::new(),
            next_ssa_key: 1,
        }
    }

    pub fn build(&self) -> String {
        self.mods[0].build()
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
        self.idents.insert(ident, ssa_key);
        ssa_key
    }

    pub fn get_var_ssa_key(&self, ident: Rc<str>) -> usize {
        *self.idents.get(&ident).unwrap()
    }

    pub fn req_ssa_key(&mut self) -> usize {
        self.next_ssa_key += 1;
        self.next_ssa_key - 1
    }

    pub fn push_mod(&mut self, module: Module) {
        self.mods.push(module);
    }
}
