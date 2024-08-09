use std::{ fmt::Debug, rc::Rc };

use ahash::AHashMap;

use crate::vm::instructions::Instruction;

use super::{
    ds::{
        register_allocator::RegisterAllocator,
        symbol_table::{ SSAKey, Symbol, SymbolFunction, SymbolTableRef, SymbolVariable },
        value::ValueType,
        vm_builder::VMBuilder,
    },
    error_handler::{ CompileError, ErrorHandler, SrcCharsRange },
    ir::{ ast::stmt::{ GotoNodeIds, NodeIdsRange }, icfg::{ dag::DAG, icfg_builder::ICFGBuilder } },
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
    type ReturnTypeCompileIntoICFG;

    fn compile_into_icfg(
        &self,
        icfg_builder: &mut ICFGBuilder,
        goto_node_ids: &mut GotoNodeIds
    ) -> Self::ReturnTypeCompileIntoICFG;

    fn validate_stmt(
        &mut self,
        symbol_table_ref: &mut SymbolTableRef,
        error_handler: &mut ErrorHandler
    );

    fn is_linear_control_flow(&self) -> bool;

    fn as_linear_control_flow(&self) -> Option<&dyn LinearControlFlow>;
}

pub trait GenerateBytecode {
    fn load_constants(&self, vm_builder: &mut VMBuilder);

    fn generate_instructions(&self, vm_builder: &mut RegisterAllocator) -> Vec<Instruction>;
}

pub trait DAGNodeTrait {
    type ConnectedNodes;

    fn parse_connected_nodes(&self, connected_nodes: Vec<usize>) -> Self::ConnectedNodes;

    fn expected_connected_nodes(&self) -> usize;
}
