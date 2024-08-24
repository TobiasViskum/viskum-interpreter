mod dag_nodes;
mod helper_methods;

// pub use dag_node::*;
pub use dag_nodes::*;
use crate::compiler::llvm_builder::{ Function, LLVMBuilder, Module, Operand, TypeI32 };

use crate::{
    compiler::{
        ds::{ register_allocator::RegisterAllocator, vm_builder::VMBuilder },
        traits::{ AllocLLVM, DAGNodeGenerateLLVM, DAGNodeTrait, Dissasemble, LoadConstants },
    },
    vm::instructions::{ Instruction, Reg },
};

#[derive(Debug)]
pub struct DAGEdge {
    origin: usize,
    dest: usize,
}

impl DAGEdge {
    pub fn new(origin: usize, dest: usize) -> Self {
        Self { origin, dest }
    }

    pub fn get_origin(&self) -> usize {
        self.origin
    }

    pub fn get_dest(&self) -> usize {
        self.dest
    }
}

#[derive(Debug)]
pub enum DAGNode {
    BinaryNode(DAGBinaryNode),
    UnaryNode(DAGUnaryNode),
    GroupNode(DAGGroupNode),
    FnCallNode(DAGFnCallNode),
    DefineNode(DAGDefineNode),
    AssignNode(DAGAssignNode),
    ConstNode(DAGConstNode),
    IdentNode(DAGIdentNode),
}

impl DAGNode {
    fn expected_connected_nodes(&self) -> usize {
        match self {
            Self::BinaryNode(binary_node) => binary_node.expected_connected_nodes(),
            Self::UnaryNode(unary_node) => unary_node.expected_connected_nodes(),
            Self::GroupNode(group_node) => group_node.expected_connected_nodes(),
            Self::FnCallNode(fn_call_node) => fn_call_node.expected_connected_nodes(),
            Self::DefineNode(define_node) => define_node.expected_connected_nodes(),
            Self::AssignNode(assign_node) => assign_node.expected_connected_nodes(),
            Self::ConstNode(const_node) => const_node.expected_connected_nodes(),
            Self::IdentNode(ident_node) => ident_node.expected_connected_nodes(),
        }
    }
}

#[derive(Debug)]
pub struct DAG {
    nodes: Vec<DAGNode>,
    edges: Vec<DAGEdge>,
    entry_node_id: usize,
}

impl AllocLLVM for DAG {
    fn alloc_llvm(&self, llvm_builder: &mut LLVMBuilder, module: &mut Module, func: &mut Function) {
        self.nodes.iter().for_each(|node| {
            match node {
                DAGNode::BinaryNode(node) => node.alloc_llvm(llvm_builder, module, func),
                DAGNode::UnaryNode(node) => node.alloc_llvm(llvm_builder, module, func),
                DAGNode::GroupNode(node) => node.alloc_llvm(llvm_builder, module, func),
                DAGNode::DefineNode(node) => node.alloc_llvm(llvm_builder, module, func),
                DAGNode::AssignNode(node) => node.alloc_llvm(llvm_builder, module, func),
                DAGNode::ConstNode(node) => node.alloc_llvm(llvm_builder, module, func),
                DAGNode::IdentNode(node) => node.alloc_llvm(llvm_builder, module, func),
                DAGNode::FnCallNode(node) => node.alloc_llvm(llvm_builder, module, func),
            }
        })
    }
}

impl DAG {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            entry_node_id: 0,
        }
    }

    pub fn build_llvm(&self, llvm_builder: &mut LLVMBuilder, func: &mut Function) -> Operand {
        self.generate_llvm(self.entry_node_id, func, llvm_builder)
    }

    pub fn get_entry_node_id(&self) -> usize {
        self.entry_node_id
    }

    pub fn set_entry_node_id(&mut self, entry_node_id: usize) {
        self.entry_node_id = entry_node_id;
    }

    pub fn add_edge(&mut self, origin: usize, dest: usize) {
        self.edges.push(DAGEdge::new(origin, dest));
    }

    pub fn push_node(&mut self, dag_node: DAGNode) -> usize {
        self.nodes.push(dag_node);
        self.nodes.len() - 1
    }

    pub fn get_connected_node_ids(&self, node_id: usize) -> Vec<usize> {
        let mut connected_nodes = vec![];
        for edge in self.edges.iter() {
            if edge.get_origin() == node_id {
                connected_nodes.push(edge.get_dest());
            }
        }
        connected_nodes
    }

    fn generate_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator,
        is_condition: bool
    ) -> Reg {
        let node = self.nodes.get(node_id).expect("Expected dag node");

        if
            let Some(next_stmt_node_id) = self
                .get_connected_node_ids(node_id)
                .get(node.expected_connected_nodes())
        {
            let possibly_dead_reg = self.generate_instruction(
                *next_stmt_node_id,
                instructions,
                register_allocator,
                is_condition
            );
            register_allocator.free_temp_reg(possibly_dead_reg);
        }

        let dst_reg = match node {
            DAGNode::FnCallNode(fn_call_node) => { todo!() }
            DAGNode::BinaryNode(binary_node) => {
                self.generate_binary_instruction(
                    node_id,
                    instructions,
                    register_allocator,
                    binary_node,
                    is_condition
                )
            }
            DAGNode::UnaryNode(unary_node) => {
                self.generate_unary_instruction(
                    node_id,
                    instructions,
                    register_allocator,
                    unary_node,
                    is_condition
                )
            }
            DAGNode::GroupNode(group_node) => {
                self.generate_group_instruction(
                    node_id,
                    instructions,
                    register_allocator,
                    group_node,
                    is_condition
                )
            }

            DAGNode::AssignNode(assign_node) => {
                self.generate_assign_instruction(
                    node_id,
                    instructions,
                    register_allocator,
                    assign_node,
                    is_condition
                )
            }
            DAGNode::DefineNode(define_node) => {
                self.generate_define_instruction(
                    node_id,
                    instructions,
                    register_allocator,
                    define_node,
                    is_condition
                )
            }
            DAGNode::ConstNode(const_node) => {
                self.generate_const_instruction(register_allocator, const_node.get_value())
            }
            DAGNode::IdentNode(ident_node) => {
                register_allocator.get_var_reg(ident_node.get_ssa_key().get_ident())
            }

            // DAGNode::FnCallNode(fn_call_node) => { todo!() }
        };

        dst_reg
    }

    pub fn generate_instructions_as_condition(
        &self,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator
    ) {
        let possibly_dead_reg = self.generate_instruction(
            self.entry_node_id,
            instructions,
            register_allocator,
            true
        );

        register_allocator.free_temp_reg(possibly_dead_reg);
    }

    fn generate_llvm(
        &self,
        node_id: usize,
        func: &mut Function,
        llvm_builder: &mut LLVMBuilder
    ) -> Operand {
        let node = self.nodes.get(node_id).expect("Expected dag node");

        if
            let Some(next_stmt_node_id) = self
                .get_connected_node_ids(node_id)
                .get(node.expected_connected_nodes())
        {
            self.generate_llvm(*next_stmt_node_id, func, llvm_builder);
        }

        match node {
            DAGNode::BinaryNode(binary_node) => {
                binary_node.generate_llvm(node_id, func, llvm_builder, self)
            }
            DAGNode::ConstNode(const_node) => {
                const_node.generate_llvm(node_id, func, llvm_builder, self)
            }
            DAGNode::DefineNode(define_node) => {
                define_node.generate_llvm(node_id, func, llvm_builder, self)
            }
            DAGNode::IdentNode(ident_node) => {
                ident_node.generate_llvm(node_id, func, llvm_builder, self)
            }
            DAGNode::AssignNode(assign_node) => {
                assign_node.generate_llvm(node_id, func, llvm_builder, self)
            }
            DAGNode::FnCallNode(fn_call_node) => {
                fn_call_node.generate_llvm(node_id, func, llvm_builder, self)
            }
            DAGNode::GroupNode(group_node) => {
                group_node.generate_llvm(node_id, func, llvm_builder, self)
            }
            _ => panic!("SDf"),
        }
    }

    pub fn generate_instructions(
        &self,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator
    ) {
        let possibly_dead_reg = self.generate_instruction(
            self.entry_node_id,
            instructions,
            register_allocator,
            false
        );

        register_allocator.free_temp_reg(possibly_dead_reg);
    }
}

impl LoadConstants for DAG {
    fn load_constants(&self, vm_builder: &mut VMBuilder) {
        self.nodes.iter().for_each(|node| {
            if let DAGNode::ConstNode(const_node) = node {
                if let Some(simple_const) = const_node.get_value().get_as_simple_const() {
                    vm_builder.alloc_const_reg(simple_const);
                } else {
                    // vm_builder.alloc_heap(complex_const);
                }
            }
        })
    }
}

impl Dissasemble for DAG {
    fn dissasemble(&self) -> String {
        self.dissasemble_node(self.entry_node_id)
    }
}

// Dissasemble helper methods
macro_rules! dissasemble_next {
    ($self:ident, $connected_nodes:ident) => {
        $self.dissasemble_node($connected_nodes.pop().unwrap())
    };
}

impl DAG {
    fn dissasemble_node(&self, node_id: usize) -> String {
        let mut connected_nodes = self.get_connected_node_ids(node_id);

        match &self.nodes[node_id] {
            DAGNode::FnCallNode(fn_call_node) => { "".to_string() }
            DAGNode::BinaryNode(binary_node) => {
                self.dissasemble_binary_node(binary_node, &mut connected_nodes)
            }
            DAGNode::UnaryNode(unary_node) => {
                self.dissasemble_unary_node(unary_node, &mut connected_nodes)
            }
            DAGNode::GroupNode(_) => { self.dissasemble_group_node(&mut connected_nodes) }
            DAGNode::DefineNode(define_node) => {
                self.dissasemble_define_node(define_node, &mut connected_nodes)
            }
            DAGNode::AssignNode(assign_node) => {
                self.dissasemble_assign_node(assign_node, &mut connected_nodes)
            }
            // DAGNode::FnCallNode(fn_call_node) => {
            //     self.dissasemble_fn_call_node(fn_call_node, &mut connected_nodes)
            // }
            DAGNode::ConstNode(node) => {
                self.dissasemble_const_or_ident_node(node, &mut connected_nodes)
            }
            DAGNode::IdentNode(node) => {
                self.dissasemble_const_or_ident_node(node, &mut connected_nodes)
            }
        }
    }

    fn dissasemble_const_or_ident_node<T>(
        &self,
        node: &T,
        connected_nodes: &mut Vec<usize>
    ) -> String
        where T: Dissasemble + DAGNodeTrait
    {
        let mut string_builder = if connected_nodes.len() == node.expected_connected_nodes() + 1 {
            let mut string_builder = dissasemble_next!(self, connected_nodes);
            string_builder += "\n";
            string_builder
        } else {
            String::new()
        };

        string_builder += node.dissasemble().as_str();

        string_builder
    }

    fn dissasemble_binary_node(
        &self,
        binary_node: &DAGBinaryNode,
        connected_nodes: &mut Vec<usize>
    ) -> String {
        let mut string_builder = if
            connected_nodes.len() == binary_node.expected_connected_nodes() + 1
        {
            let mut string_builder = dissasemble_next!(self, connected_nodes);
            string_builder += "\n";
            string_builder
        } else {
            String::new()
        };

        let rhs = dissasemble_next!(self, connected_nodes);
        let lhs = dissasemble_next!(self, connected_nodes);
        string_builder += format!("{} {} {}", lhs, binary_node.dissasemble(), rhs).as_str();
        string_builder
    }

    fn dissasemble_unary_node(
        &self,
        unary_node: &DAGUnaryNode,
        connected_nodes: &mut Vec<usize>
    ) -> String {
        let mut string_builder = if
            connected_nodes.len() == unary_node.expected_connected_nodes() + 1
        {
            dissasemble_next!(self, connected_nodes)
        } else {
            String::new()
        };

        let rhs = dissasemble_next!(self, connected_nodes);
        string_builder += format!("{}{}", unary_node.dissasemble(), rhs).as_str();
        string_builder
    }

    fn dissasemble_group_node(&self, connected_nodes: &mut Vec<usize>) -> String {
        let mut string_builder = if connected_nodes.len() == 2 {
            dissasemble_next!(self, connected_nodes)
        } else {
            String::new()
        };

        string_builder += format!("({})", dissasemble_next!(self, connected_nodes)).as_str();
        string_builder
    }

    fn dissasemble_define_node(
        &self,
        define_node: &DAGDefineNode,
        connected_nodes: &mut Vec<usize>
    ) -> String {
        let mut string_builder = if connected_nodes.len() == 2 {
            let mut string_builder = dissasemble_next!(self, connected_nodes);
            string_builder += "\n";
            string_builder
        } else {
            String::new()
        };

        let value = if connected_nodes.len() == 0 {
            None
        } else {
            Some(dissasemble_next!(self, connected_nodes))
        };

        let ident = define_node.get_ssa_key().dissasemble();

        let node_string = if let Some(value) = value {
            format!(
                "{}{} {} {}",
                if define_node.get_is_mutable() {
                    "mut "
                } else {
                    ""
                },
                ident,
                define_node.dissasemble(),
                value
            )
        } else {
            format!("{}", ident)
        };

        string_builder += node_string.as_str();
        string_builder
    }

    fn dissasemble_assign_node(
        &self,
        assign_node: &DAGAssignNode,
        connected_nodes: &mut Vec<usize>
    ) -> String {
        let mut string_builder = if
            connected_nodes.len() == assign_node.expected_connected_nodes() + 1
        {
            let mut string_builder = dissasemble_next!(self, connected_nodes);
            string_builder += "\n";
            string_builder
        } else {
            String::new()
        };

        let (value, ident) = (
            dissasemble_next!(self, connected_nodes),
            dissasemble_next!(self, connected_nodes),
        );

        string_builder += format!("{} {} {}", ident, assign_node.dissasemble(), value).as_str();

        string_builder
    }

    // fn dissasemble_fn_call_node(
    //     &self,
    //     fn_call_node: &DAGFnCallNode,
    //     connected_nodes: &mut Vec<usize>
    // ) -> String {
    //     let mut string_builder = fn_call_node.dissasemble();
    //     for (i, connected_node_id) in connected_nodes.iter().enumerate() {
    //         let arg = self.dissasemble_node(*connected_node_id);
    //         string_builder += arg.as_str();
    //         if i != connected_nodes.len() - 1 {
    //             string_builder += ", ";
    //         }
    //     }

    //     string_builder += ")";
    //     string_builder
    // }
}
