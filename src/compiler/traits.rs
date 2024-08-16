use std::{ fmt::Debug, rc::Rc };

use ahash::AHashMap;
use llvm_builder::{ Function, LLVMBuilder, LLVMType, Operand };

use crate::vm::instructions::Instruction;

use super::{
    ds::{
        register_allocator::RegisterAllocator,
        symbol_table::{ SSAKey, Symbol, SymbolFunction, SymbolTableRef, SymbolVariable },
        value::ValueType,
        vm_builder::VMBuilder,
    },
    error_handler::{ CompileError, ErrorHandler, SrcCharsRange },
    ir::{ ast::stmt::GotoNodeIds, icfg::{ dag::DAG, icfg_builder::{ CFGBuilder }, ICFG } },
};

pub trait Dissasemble {
    fn dissasemble(&self) -> String;
}

pub trait SymbolTableActions {
    fn lookup_with_key(&self, ssa_key: &SSAKey) -> Option<&Symbol>;

    fn lookup_as_fn(&self, ident: &Rc<str>) -> Result<&SymbolFunction, String>;

    fn lookup_as_var(&self, ident: &Rc<str>) -> Result<&SymbolVariable, String>;

    fn lookup(&self, ident: &Rc<str>) -> Option<(&SSAKey, &Symbol)>;

    fn insert(&mut self, ident: Rc<str>, symbol: Symbol) -> SSAKey;
}

pub trait SymbolTableAlloc {
    fn alloc_symbol_table(&mut self, return_type: Option<ValueType>) -> SymbolTableRef;
}

pub trait OpTrait: Dissasemble + Debug + Clone + Copy {
    fn get_op_len(&self) -> usize;

    fn get_can_constant_fold(&self) -> bool;

    fn build_llvm(&self) -> String;
}

pub trait ExprTrait where Self: Dissasemble + Debug {
    fn type_check(&mut self, symbol_table_ref: &SymbolTableRef) -> Result<ValueType, CompileError>;

    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize;

    fn collect_metadata(&self) -> SrcCharsRange;
}

pub trait LinearControlFlow {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize;
}

pub trait StmtTrait {
    fn compile_into_icfg(
        &self,
        icfg: &mut ICFG,
        cfg_builder: &mut CFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    );

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
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

pub trait DAGNodeGenerateLLVM: DAGNodeTrait {
    fn generate_llvm<T>(
        &self,
        node_id: usize,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder,
        dag: &DAG
    ) -> Operand
        where T: LLVMType;
}
pub trait CFGNodeTrait: ParseConnectedNodes {
    fn generate_instructions(&self, register_allocator: &mut RegisterAllocator) -> Vec<Instruction>;

    fn build_llvm(&self, func: &mut Function, llvm_builder: &mut LLVMBuilder);
}
