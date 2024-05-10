use ahash::AHashMap;

use crate::{
    constants::REGISTERS,
    operations::{ BinaryOp, ComparisonOp },
    value::Value,
    vm::instructions::Instruction,
};

use self::cfg_node::{ CFGDecisionNode, CFGProcessNode };
pub mod cfg_node;
pub mod dag;

#[derive(Debug)]
pub enum CFGNode {
    Process(CFGProcessNode), // This is essentially a statement e.g. "mut i32 a := 8"
    Decision(CFGDecisionNode),
    ScopeStart(usize),
    ScopeEnd(Option<usize>),
    ProgramStart(usize),
    ProgramEnd,
}

#[derive(Debug, PartialEq)]
pub enum ChangedState {
    MaybeChanged,
    Unchanged,
}

#[derive(Debug, PartialEq)]
pub enum DefinitionState {
    IsDefinition,
    IsAssignment,
}

#[derive(Debug)]
pub struct RegistersMap {
    registers_maps: Vec<(AHashMap<String, usize>, Vec<usize>)>,
}

impl RegistersMap {
    pub fn new() -> Self {
        let available_registers = (0..REGISTERS).rev().collect::<Vec<_>>();

        Self {
            registers_maps: vec![(AHashMap::default(), available_registers)],
        }
    }

    pub fn start_scope(&mut self) {
        let available_registers = (0..REGISTERS).rev().collect::<Vec<_>>();

        self.registers_maps.push((AHashMap::default(), available_registers));
    }

    pub fn end_scope(&mut self) {
        self.registers_maps.pop();
    }

    pub fn assign_register(&mut self) -> (usize, usize) {
        let scope = self.registers_maps.len() - 1;
        let (_, available_registers) = self.registers_maps.last_mut().unwrap();

        let register = available_registers.pop().unwrap();

        (register, scope)
    }

    pub fn assign_variable_register(&mut self, variable: String) -> (usize, usize) {
        let scope = self.registers_maps.len() - 1;
        let (current_scope, available_registers) = self.registers_maps.last_mut().unwrap();

        let register = available_registers.pop().unwrap();

        current_scope.insert(variable, register);

        (register, scope)
    }

    pub fn get_register(&self, variable: &String) -> Option<(usize, usize)> {
        for i in (0..self.registers_maps.len()).rev() {
            let (current_scope, _) = &self.registers_maps[i];

            if let Some(register) = current_scope.get(variable) {
                return Some((*register, i));
            }
        }

        None
    }

    pub fn free_register(&mut self, register: usize, scope: usize) {
        let (_, available_registers) = self.registers_maps.get_mut(scope).unwrap();

        available_registers.push(register);
    }
}

#[derive(Debug)]
pub struct IREnvironment {
    definitions: Vec<AHashMap<(String, usize), (Option<Value>, ChangedState, DefinitionState)>>,
}

impl IREnvironment {
    pub fn new() -> Self {
        Self {
            definitions: vec![],
        }
    }

    pub fn start_scope(&mut self) {
        self.definitions.push(AHashMap::default())
    }

    pub fn end_scope(&mut self) {
        self.definitions.pop();
    }

    pub fn get(&self, lexeme: &String) -> Option<&(Option<Value>, ChangedState, DefinitionState)> {
        for scope in self.definitions.iter().rev() {
            let scope_values = scope.iter().collect::<Vec<_>>();
            for i in (0..scope_values.len()).rev() {
                let ((name, _), value) = scope_values[i];

                if name == lexeme {
                    return Some(value);
                }
            }
        }

        None
    }

    pub fn overwrite(&mut self, lexeme: &String, new_value: Option<Value>) {
        for scope in self.definitions.iter_mut().rev() {
            let mut scope_values = scope.iter_mut().collect::<Vec<_>>();
            for i in (0..scope_values.len()).rev() {
                let ((name, _), (value, changed_state, definition_state)) = scope_values
                    .get_mut(i)
                    .unwrap();

                if name == lexeme {
                    *value = new_value;
                    *changed_state = ChangedState::Unchanged;
                    *definition_state = DefinitionState::IsAssignment;
                    return;
                }
            }
        }
    }

    pub fn push(
        &mut self,
        lexeme: &String,
        value: Option<Value>,
        is_definition: DefinitionState,
        scope: usize
    ) {
        let new_subscript = self.get_new_subscript(&lexeme);
        self.definitions[scope].insert(
            (lexeme.clone(), new_subscript),
            (value, ChangedState::Unchanged, is_definition)
        );
    }

    fn get_new_subscript(&self, lexeme: &str) -> usize {
        for scope in self.definitions.iter().rev() {
            let scope_keys = scope.keys().collect::<Vec<_>>();

            for i in (0..scope_keys.len()).rev() {
                let (name, subscript) = scope_keys[i];

                if name == lexeme {
                    return *subscript + 1;
                }
            }
        }

        0
    }
}

#[derive(Debug)]
pub struct CFG {
    nodes: Vec<CFGNode>,
}

impl CFG {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: CFGNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn get_mut_node(&mut self, i: usize) -> Option<&mut CFGNode> {
        self.nodes.get_mut(i)
    }

    // Implement iterator for CFG
    fn for_each<F: FnMut(&mut CFGNode)>(&mut self, start_node_id: usize, mut callback: F) {
        let mut next_node_id = start_node_id;

        loop {
            match self.nodes.get_mut(next_node_id) {
                Some(node) => {
                    callback(node);
                    match node {
                        CFGNode::Decision(decision_node) => {
                            next_node_id = decision_node.true_branch_id;
                        }
                        CFGNode::Process(process_node) => {
                            next_node_id = process_node.next_id;
                        }
                        CFGNode::ProgramStart(next_id) => {
                            next_node_id = *next_id;
                        }
                        CFGNode::ProgramEnd => {
                            break;
                        }
                        CFGNode::ScopeStart(next_id) => {
                            next_node_id = *next_id;
                        }
                        CFGNode::ScopeEnd(next_id) => {
                            match next_id {
                                Some(next_id) => {
                                    next_node_id = *next_id;
                                }
                                None => {}
                            }
                        }
                    }
                }
                None => {
                    panic!("Invalid next_node_id: {}", next_node_id);
                }
            }
        }
    }

    pub fn generate_bytecode_for_if_stmt(
        &mut self,
        decision_node_id: usize,
        registers_map: &mut RegistersMap,
        all_instructions_len: usize
    ) -> (Vec<Instruction>, usize) {
        // Get comparison op for later (to avoid borrow rule errors)
        let comparison_op = match self.nodes.get_mut(decision_node_id).unwrap() {
            CFGNode::Decision(decision_node) =>
                match &mut decision_node.condition {
                    Some(dag) => { Some(dag.ensure_comparison_to_entry_node()) }
                    _ => None,
                }
            _ => panic!("Expected decision node"),
        };

        let (instructions, next_node_id) = match self.nodes.get_mut(decision_node_id).unwrap() {
            CFGNode::Decision(decision_node) => {
                let mut if_stmt_instructions: Vec<Instruction> = vec![];

                let node_instructions = match &decision_node.condition {
                    Some(dag) => { dag.generate_dag_bytecode(registers_map) }
                    None => vec![],
                };

                let offset_len = all_instructions_len + node_instructions.len() + 1;

                let (true_branch_id, false_branch_id) = (
                    decision_node.true_branch_id,
                    decision_node.false_branch_id,
                );

                let (true_branch_instructions, mut next_node_id) = self.generate_bytecode(
                    true_branch_id,
                    registers_map,
                    offset_len
                );

                if_stmt_instructions.extend(node_instructions);

                let true_pos = all_instructions_len + if_stmt_instructions.len() + 1;
                let false_pos = true_pos + true_branch_instructions.len() + 1;

                let jump_instruction = match comparison_op {
                    Some(comparison_op) =>
                        match comparison_op {
                            ComparisonOp::Equal => { Instruction::JE { true_pos, false_pos } }
                            ComparisonOp::NotEqual => { Instruction::JNE { true_pos, false_pos } }
                            ComparisonOp::Greater => { Instruction::JG { true_pos, false_pos } }
                            ComparisonOp::GreaterEqual => {
                                Instruction::JGE { true_pos, false_pos }
                            }
                            ComparisonOp::Less => { Instruction::JL { true_pos, false_pos } }
                            ComparisonOp::LessEqual => { Instruction::JLE { true_pos, false_pos } }
                        }
                    None => Instruction::Jmp { pos: true_pos },
                };

                if_stmt_instructions.push(jump_instruction);
                if_stmt_instructions.extend(true_branch_instructions);

                if let Some(false_branch_id) = false_branch_id {
                    let (temp_instructions, new_next_node_id) = self.generate_bytecode_for_if_stmt(
                        false_branch_id,
                        registers_map,
                        all_instructions_len + if_stmt_instructions.len() + 1
                    );

                    next_node_id = new_next_node_id;

                    if_stmt_instructions.push(Instruction::Jmp {
                        pos: all_instructions_len +
                        if_stmt_instructions.len() +
                        temp_instructions.len() +
                        1,
                    });
                    if_stmt_instructions.extend(temp_instructions);
                } else {
                    if_stmt_instructions.push(Instruction::Jmp {
                        pos: all_instructions_len + if_stmt_instructions.len() + 1,
                    });
                }

                (if_stmt_instructions, next_node_id)
            }
            _ => panic!("Expected decision node"),
        };

        (instructions, next_node_id)

        // let (instructions, next_node_id) = match self.nodes.get(decision_node_id).unwrap() {
        //     CFGNode::Decision(decision_node) => {
        //         let mut if_stmt_instructions: Vec<Instruction> = vec![];

        //         let node_instructions = match &decision_node.condition {
        //             Some(dag) => {
        //                 println!("entry_node_id: {:?}", dag.nodes.get(&dag.get_entry_node_id()));
        //                 dag.generate_dag_bytecode(registers_map)
        //             }
        //             None => vec![],
        //         };

        //         let offset_len = all_instructions_len + node_instructions.len() + 1;
        //         if_stmt_instructions.extend(node_instructions);

        //         let (true_branch_instructions, next_node_id) = self.generate_bytecode(
        //             decision_node.true_branch_id,
        //             registers_map,
        //             offset_len
        //         );

        //         let true_pos = all_instructions_len + if_stmt_instructions.len() + 1;
        //         let false_pos = true_pos + true_branch_instructions.len() + 1;

        //         let jump_instruction = match comparison_op {
        //             Some(comparison_op) =>
        //                 match comparison_op {
        //                     ComparisonOp::Equal => { Instruction::JE { true_pos, false_pos } }
        //                     ComparisonOp::NotEqual => { Instruction::JNE { true_pos, false_pos } }
        //                     ComparisonOp::Greater => { Instruction::JG { true_pos, false_pos } }
        //                     ComparisonOp::GreaterEqual => {
        //                         Instruction::JGE { true_pos, false_pos }
        //                     }
        //                     ComparisonOp::Less => { Instruction::JL { true_pos, false_pos } }
        //                     ComparisonOp::LessEqual => { Instruction::JLE { true_pos, false_pos } }
        //                 }

        //             None => Instruction::Jmp { pos: true_pos },
        //         };

        //         if_stmt_instructions.push(jump_instruction);
        //         if_stmt_instructions.extend(true_branch_instructions);

        //         if let Some(false_branch_id) = decision_node.false_branch_id {
        //             let (temp_instructions, new_next_node_id) = self.generate_bytecode_for_if_stmt(
        //                 false_branch_id,
        //                 registers_map,
        //                 all_instructions_len + if_stmt_instructions.len() + 1
        //             );

        //             next_node_id = new_next_node_id;

        //             if_stmt_instructions.push(Instruction::Jmp {
        //                 pos: all_instructions_len +
        //                 if_stmt_instructions.len() +
        //                 temp_instructions.len() +
        //                 1,
        //             });
        //             if_stmt_instructions.extend(temp_instructions);
        //         } else {
        //             if_stmt_instructions.push(Instruction::Jmp {
        //                 pos: all_instructions_len + if_stmt_instructions.len() + 1,
        //             });
        //         }

        //         (if_stmt_instructions, next_node_id)
        //     }
        //     _ => panic!("Expected decision node"),
        // };

        // (instructions, next_node_id)

        // match self.nodes.get_mut(decision_node_id).unwrap() {
        //     CFGNode::Decision(decision_node) => {
        //         let jump_instruction = match &mut decision_node.condition {
        //             Some(dag) => {
        //                 println!("dag before: {:#?}", dag);

        //                 let comparison_op = dag.ensure_comparison_to_entry_node();

        //                 println!("dag efter: {:#?}", dag);

        //                 match comparison_op {
        //                     ComparisonOp::Equal => { Instruction::JE { true_pos, false_pos } }
        //                     ComparisonOp::NotEqual => { Instruction::JNE { true_pos, false_pos } }
        //                     ComparisonOp::Greater => { Instruction::JG { true_pos, false_pos } }
        //                     ComparisonOp::GreaterEqual => {
        //                         Instruction::JGE { true_pos, false_pos }
        //                     }
        //                     ComparisonOp::Less => { Instruction::JL { true_pos, false_pos } }
        //                     ComparisonOp::LessEqual => { Instruction::JLE { true_pos, false_pos } }
        //                 }
        //             }
        //             None => Instruction::Jmp { pos: true_pos },
        //         };

        //         if_stmt_instructions.push(jump_instruction);
        //         if_stmt_instructions.extend(true_branch_instructions);

        //         if let Some(false_branch_id) = decision_node.false_branch_id {
        //             let (temp_instructions, new_next_node_id) = self.generate_bytecode_for_if_stmt(
        //                 false_branch_id,
        //                 registers_map,
        //                 all_instructions_len + if_stmt_instructions.len() + 1
        //             );

        //             next_node_id = new_next_node_id;

        //             if_stmt_instructions.push(Instruction::Jmp {
        //                 pos: all_instructions_len +
        //                 if_stmt_instructions.len() +
        //                 temp_instructions.len() +
        //                 1,
        //             });
        //             if_stmt_instructions.extend(temp_instructions);
        //         } else {
        //             if_stmt_instructions.push(Instruction::Jmp {
        //                 pos: all_instructions_len + if_stmt_instructions.len() + 1,
        //             });
        //         }
        //     }
        //     _ => panic!("Expected decision node"),
        // }
    }

    pub fn generate_bytecode(
        &mut self,
        start_node_id: usize,
        registers_map: &mut RegistersMap,
        offset_len: usize
    ) -> (Vec<Instruction>, usize) {
        let mut instructions = Vec::with_capacity(64);
        let mut next_node_id = start_node_id;

        loop {
            match self.nodes.get(next_node_id) {
                Some(node) => {
                    match node {
                        CFGNode::Decision(_) => {
                            let (temp_instructions, new_next_node_id) =
                                self.generate_bytecode_for_if_stmt(
                                    next_node_id,
                                    registers_map,
                                    instructions.len() + offset_len
                                );

                            instructions.extend(temp_instructions);

                            next_node_id = new_next_node_id;
                        }
                        CFGNode::Process(process_node) => {
                            next_node_id = process_node.next_id;

                            let node_instructions =
                                process_node.dag.generate_dag_bytecode(registers_map);
                            instructions.extend(node_instructions);
                        }
                        CFGNode::ProgramStart(next_id) => {
                            next_node_id = *next_id;
                        }
                        CFGNode::ProgramEnd => {
                            instructions.push(Instruction::Halt);
                            break;
                        }
                        CFGNode::ScopeStart(next_id) => {
                            next_node_id = *next_id;

                            registers_map.start_scope();
                            instructions.push(Instruction::StartScope);
                        }
                        CFGNode::ScopeEnd(next_id) => {
                            registers_map.end_scope();
                            instructions.push(Instruction::EndScope);

                            match next_id {
                                Some(next_id) => {
                                    next_node_id = *next_id;
                                }
                                None => {
                                    break;
                                }
                            }
                        }
                    }
                }
                None => {
                    panic!("Invalid next_node_id: {}", next_node_id);
                }
            }
        }

        (instructions, next_node_id + 1)
    }

    #[profiler::function_tracker]
    pub fn optimize_and_generate_bytecode(&mut self) -> Vec<Instruction> {
        // self.constant_folding();

        // self.eliminate_dead_code();

        let registers_maps = &mut RegistersMap::new();
        self.generate_bytecode(0, registers_maps, 0).0
    }

    fn constant_folding(&mut self) {
        let mut environment = IREnvironment::new();

        let mut scope = 0;

        self.for_each(0, |node| {
            match node {
                CFGNode::Decision(_) => {}
                CFGNode::ProgramStart(_) => {
                    environment.start_scope();
                }
                CFGNode::ProgramEnd => {
                    environment.end_scope();
                }
                CFGNode::Process(ref mut process_node) => {
                    process_node.dag.constant_folding(&mut environment, scope);
                }
                CFGNode::ScopeStart(_) => {
                    environment.start_scope();
                    scope += 1;
                }
                CFGNode::ScopeEnd(_) => {
                    environment.end_scope();
                    scope -= 1;
                }
            }
        });
    }
}
