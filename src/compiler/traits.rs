use std::{ fmt::Debug, rc::Rc };

use ahash::AHashMap;
use crate::compiler::llvm_builder::{ Function, LLVMBuilder, Module, Operand };

use crate::vm::instructions::Instruction;

use super::{
    ds::{
        register_allocator::RegisterAllocator,
        ssa_ident::SSAIdent,
        value::ValueType,
        vm_builder::VMBuilder,
    },
    error_handler::{ CompileError, ErrorHandler, SrcCharsRange },
    ir::{
        ast::stmt::GotoNodeIds,
        icfg::{ cfg::CFG, dag::DAG, icfg_builder::{ CFGBuilder, ICFGBuilder }, ICFG },
    },
    ProgramSymbolTablePhase1,
};

pub trait Dissasemble {
    fn dissasemble(&self) -> String;
}

pub trait AstDissasemble {
    fn ast_dissasemble(
        &self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        scope_depth: usize
    ) -> String;
}

pub trait OpTrait: Dissasemble + Debug + Clone + Copy {
    fn get_op_len(&self) -> usize;

    fn get_can_constant_fold(&self) -> bool;

    fn build_llvm(&self) -> String;
}

pub trait ExprTrait where Self: Dissasemble + Debug {
    fn type_check(
        &mut self,
        program_symbol_table: &ProgramSymbolTablePhase1
    ) -> Result<ValueType, CompileError>;

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize;

    fn collect_metadata(&self) -> SrcCharsRange;
}

pub trait LinearControlFlow {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAIdent, usize>,
        icfg_builder: &mut ICFGBuilder
    ) -> usize;
}

pub trait StmtTrait {
    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    );

    fn validate_stmt(
        &mut self,
        program_symbol_table: &mut ProgramSymbolTablePhase1,
        error_handler: &mut ErrorHandler
    );

    fn is_linear_control_flow(&self) -> bool;

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow>;
}

pub trait LoadConstants {
    fn load_constants(&self, vm_builder: &mut VMBuilder);
}

pub trait ParseConnectedNodes {
    type ConnectedNodes;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes;
}

pub trait DAGNodeTrait: ParseConnectedNodes {
    fn expected_connected_nodes(&self) -> usize;
}

pub trait AllocLLVM {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, module: &mut Module, func: &mut Function);
}

pub trait DAGNodeGenerateLLVM: DAGNodeTrait {
    fn generate_llvm(
        &self,
        node_id: usize,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand;

    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, module: &mut Module, func: &mut Function);
}
pub trait CFGNodeTrait: ParseConnectedNodes {
    fn generate_instructions(&self, register_allocator: &mut RegisterAllocator) -> Vec<Instruction>;
}

pub trait GenerateLLVM: AllocLLVM {
    fn build_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function);
}

pub trait CFGNodeGenerateLLVM: CFGNodeTrait {
    fn build_llvm(
        &self,
        node_id: usize,
        llvm_builder: &mut LLVMBuilder,
        func: &mut Function,
        cfg: &CFG
    );
}
