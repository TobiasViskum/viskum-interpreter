mod dag_node;
mod helper_methods;

pub use dag_node::*;

use crate::{
    compiler::{
        ds::{ register_allocator::{ self, RegisterAllocator }, vm_builder::VMBuilder },
        traits::{ DAGNodeTrait, Dissasemble, GenerateBytecode },
    },
    vm::{ self, instructions::{ Instruction, Reg } },
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

impl DAGNodeTrait for DAGNode {
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

impl DAG {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            entry_node_id: 0,
        }
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

    pub fn generate_instruction(
        &self,
        node_id: usize,
        instructions: &mut Vec<Instruction>,
        register_allocator: &mut RegisterAllocator
    ) -> Reg {
        let node = self.nodes.get(node_id).expect("Expected dag node");

        if
            let Some(next_stmt_node_id) = self
                .get_connected_node_ids(node_id)
                .get(node.expected_connected_nodes())
        {
            self.generate_instruction(*next_stmt_node_id, instructions, register_allocator);
        }

        let dst_reg = match node {
            DAGNode::BinaryNode(binary_node) => {
                self.generate_binary_instruction(
                    node_id,
                    instructions,
                    register_allocator,
                    binary_node.get_op()
                )
            }
            DAGNode::UnaryNode(unary_node) => {
                self.generate_unary_instruction(
                    node_id,
                    instructions,
                    register_allocator,
                    unary_node.get_op()
                )
            }
            DAGNode::GroupNode(group_node) => {
                self.generate_group_instruction(node_id, instructions, register_allocator)
            }
            DAGNode::ConstNode(const_node) => {
                self.generate_const_instruction(register_allocator, const_node.get_value())
            }
            DAGNode::DefineNode(define_node) => {
                self.generate_define_instruction(node_id, instructions, register_allocator)
            }
            _ => todo!(),
        };

        dst_reg
    }
}

impl GenerateBytecode for DAG {
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

    fn generate_instructions(
        &self,
        register_allocator: &mut RegisterAllocator
    ) -> Vec<Instruction> {
        let mut instructions = vec![];

        self.generate_instruction(self.entry_node_id, &mut instructions, register_allocator);

        instructions
    }
}

impl Dissasemble for DAG {
    fn dissasemble(&self) -> String {
        self.dissasemble_node(self.entry_node_id)
    }
}

// Dissasemble helper methods
impl DAG {
    fn dissasemble_node(&self, node_id: usize) -> String {
        let mut connected_nodes = self.get_connected_node_ids(node_id);

        match &self.nodes[node_id] {
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
            DAGNode::FnCallNode(fn_call_node) => {
                self.dissasemble_fn_call_node(fn_call_node, &mut connected_nodes)
            }
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
        where T: Dissasemble
    {
        let mut string_builder = if connected_nodes.len() == 1 {
            let mut string_builder = self.dissasemble_node(connected_nodes.pop().unwrap());
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
        let mut string_builder = if connected_nodes.len() == 3 {
            let mut string_builder = self.dissasemble_node(connected_nodes.pop().unwrap());
            string_builder += "\n";
            string_builder
        } else {
            String::new()
        };

        let rhs = self.dissasemble_node(connected_nodes.pop().unwrap());
        let lhs = self.dissasemble_node(connected_nodes.pop().unwrap());
        string_builder += format!("{} {} {}", lhs, binary_node.dissasemble(), rhs).as_str();
        string_builder
    }

    fn dissasemble_unary_node(
        &self,
        unary_node: &DAGUnaryNode,
        connected_nodes: &mut Vec<usize>
    ) -> String {
        let mut string_builder = if connected_nodes.len() == 2 {
            self.dissasemble_node(connected_nodes.pop().unwrap())
        } else {
            String::new()
        };

        let rhs = self.dissasemble_node(connected_nodes.pop().unwrap());
        string_builder += format!("{}{}", unary_node.dissasemble(), rhs).as_str();
        string_builder
    }

    fn dissasemble_group_node(&self, connected_nodes: &mut Vec<usize>) -> String {
        let mut string_builder = if connected_nodes.len() == 2 {
            self.dissasemble_node(connected_nodes.pop().unwrap())
        } else {
            String::new()
        };
        let group = self.dissasemble_node(connected_nodes.pop().unwrap());
        string_builder += format!("({})", group).as_str();
        string_builder
    }

    fn dissasemble_define_node(
        &self,
        define_node: &DAGDefineNode,
        connected_nodes: &mut Vec<usize>
    ) -> String {
        let mut string_builder = if connected_nodes.len() == 3 {
            let mut string_builder = self.dissasemble_node(connected_nodes.pop().unwrap());
            string_builder += "\n";
            string_builder
        } else {
            String::new()
        };

        let (ident, value) = if connected_nodes.len() == 1 {
            let ident = self.dissasemble_node(connected_nodes.pop().unwrap());
            (ident, None)
        } else {
            let value = Some(self.dissasemble_node(connected_nodes.pop().unwrap()));
            let ident = self.dissasemble_node(connected_nodes.pop().unwrap());
            (ident, value)
        };

        let node_string = if let Some(value) = value {
            format!("{} {} {}", ident, define_node.dissasemble(), value)
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
        let (ident, value) = {
            let value = self.dissasemble_node(connected_nodes.pop().unwrap());
            let ident = self.dissasemble_node(connected_nodes.pop().unwrap());
            (ident, value)
        };

        format!("{} {} {}", ident, assign_node.dissasemble(), value)
    }

    fn dissasemble_fn_call_node(
        &self,
        fn_call_node: &DAGFnCallNode,
        connected_nodes: &mut Vec<usize>
    ) -> String {
        let mut string_builder = fn_call_node.dissasemble();
        for (i, connected_node_id) in connected_nodes.iter().enumerate() {
            let arg = self.dissasemble_node(*connected_node_id);
            string_builder += arg.as_str();
            if i != connected_nodes.len() - 1 {
                string_builder += ", ";
            }
        }

        string_builder += ")";
        string_builder
    }
}
