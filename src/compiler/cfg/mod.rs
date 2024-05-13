use std::{ borrow::{ Borrow, BorrowMut }, cell::RefCell, collections::HashMap };

use ahash::{ AHashMap, HashSet, HashSetExt };
use indexmap::{ map::MutableKeys, IndexMap };

use crate::{
    constants::REGISTERS,
    operations::{ BinaryOp, ComparisonOp },
    value::Value,
    vm::instructions::Instruction,
};

use self::cfg_node::{ CFGBreakNode, CFGDecisionNode, CFGGotoNode, CFGNodeState, CFGProcessNode };
pub mod cfg_node;
pub mod dag;

#[derive(Debug)]
pub enum CFGNode {
    Process {
        node: CFGProcessNode,
        scope: usize,
    },
    Decision {
        node: CFGDecisionNode,
        scope: usize,
    },

    Goto {
        node: CFGGotoNode,
        scope: usize,
    },
    ProgramStart {
        next_id: usize,
        scope: usize,
    },
    ProgramEnd {
        scope: usize,
    },
}

impl CFGNode {
    pub fn get_scope(&self) -> usize {
        match self {
            Self::Process { scope, .. } => *scope,
            Self::Decision { scope, .. } => *scope,
            Self::Goto { scope, .. } => *scope,
            Self::ProgramStart { scope, .. } => *scope,
            Self::ProgramEnd { scope, .. } => *scope,
        }
    }

    pub fn dissassemble(&self) -> String {
        match self {
            Self::Process { node, scope } => format!("S{}, Process({})", scope, node.next_id),
            Self::Decision { node, scope } =>
                format!("S{}, Decision({}, {})", scope, node.true_branch_id, node.false_branch_id),
            Self::Goto { node, scope } => format!("S{}, Goto({})", scope, node.goto_node_id),

            Self::ProgramStart { .. } => format!("ProgramStart"),
            Self::ProgramEnd { .. } => format!("ProgramEnd"),
        }
    }
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

// #[derive(Debug)]
// pub struct IREnvironment {
//     definitions: Vec<AHashMap<(String, usize), (Option<Value>, ChangedState, DefinitionState)>>,
// }

// impl IREnvironment {
//     pub fn new() -> Self {
//         Self {
//             definitions: vec![],
//         }
//     }

//     pub fn start_scope(&mut self) {
//         self.definitions.push(AHashMap::default())
//     }

//     pub fn end_scope(&mut self) {
//         self.definitions.pop();
//     }

//     pub fn get(&self, lexeme: &String) -> Option<&(Option<Value>, ChangedState, DefinitionState)> {
//         for scope in self.definitions.iter().rev() {
//             let scope_values = scope.iter().collect::<Vec<_>>();
//             for i in (0..scope_values.len()).rev() {
//                 let ((name, _), value) = scope_values[i];

//                 if name == lexeme {
//                     return Some(value);
//                 }
//             }
//         }

//         None
//     }

//     pub fn overwrite(&mut self, lexeme: &String, new_value: Option<Value>) {
//         for scope in self.definitions.iter_mut().rev() {
//             let mut scope_values = scope.iter_mut().collect::<Vec<_>>();
//             for i in (0..scope_values.len()).rev() {
//                 let ((name, _), (value, changed_state, definition_state)) = scope_values
//                     .get_mut(i)
//                     .unwrap();

//                 if name == lexeme {
//                     *value = new_value;
//                     *changed_state = ChangedState::Unchanged;
//                     *definition_state = DefinitionState::IsAssignment;
//                     return;
//                 }
//             }
//         }
//     }

//     pub fn push(
//         &mut self,
//         lexeme: &String,
//         value: Option<Value>,
//         is_definition: DefinitionState,
//         scope: usize
//     ) {
//         let new_subscript = self.get_new_subscript(&lexeme);
//         self.definitions[scope].insert(
//             (lexeme.clone(), new_subscript),
//             (value, ChangedState::Unchanged, is_definition)
//         );
//     }

//     fn get_new_subscript(&self, lexeme: &str) -> usize {
//         for scope in self.definitions.iter().rev() {
//             let scope_keys = scope.keys().collect::<Vec<_>>();

//             for i in (0..scope_keys.len()).rev() {
//                 let (name, subscript) = scope_keys[i];

//                 if name == lexeme {
//                     return *subscript + 1;
//                 }
//             }
//         }

//         0
//     }
// }

// #[derive(Debug)]
// pub struct JMPInstructionIndexes {
//     pub continue_jmps: Vec<usize>,
//     pub break_jmps: Vec<usize>,
//     pub return_jmps: Vec<usize>,
// }

// impl JMPInstructionIndexes {
//     pub fn new() -> Self {
//         Self {
//             continue_jmps: vec![],
//             break_jmps: vec![],
//             return_jmps: vec![],
//         }
//     }

//     pub fn get_total_instruction_len(&self) -> usize {
//         self.continue_jmps.len() + self.break_jmps.len() + self.return_jmps.len()
//     }

//     pub fn extend(&mut self, other: JMPInstructionIndexes) {
//         self.continue_jmps.extend(other.continue_jmps);
//         self.break_jmps.extend(other.break_jmps);
//         self.return_jmps.extend(other.return_jmps);
//     }

//     pub fn apply_offset(&mut self, increment: usize) {
//         for v in &mut self.continue_jmps {
//             *v += increment;
//         }
//         for v in &mut self.break_jmps {
//             *v += increment;
//         }
//         for v in &mut self.return_jmps {
//             *v += increment;
//         }
//     }
// }

#[derive(Debug)]
pub struct VMSymbolTable {
    available_registers: Vec<usize>,
    scope_begin: Vec<usize>,
    stack_counter: usize,
    stack_table: IndexMap<String, Vec<usize>>, // Vec<usize> refers to SSA. The outermost index is the newest of that var. E.g. the stack position of a_1 is on top of a_0
    previous_scope: usize,
}

impl VMSymbolTable {
    pub fn new() -> Self {
        let available_registers = (0..REGISTERS).rev().collect::<Vec<_>>();

        Self {
            available_registers,
            scope_begin: Vec::new(),
            stack_counter: 0,
            stack_table: IndexMap::new(),
            previous_scope: 0,
        }
    }

    pub fn get_pop_amount_in_scope_diff(&self, scope_diff: usize) -> usize {
        if scope_diff == 0 {
            return 0;
        }

        let new_stack_height = self.scope_begin.get(self.scope_begin.len() - scope_diff).unwrap();

        let mut amount_to_pop = 0;

        for (_, stack_positions) in self.stack_table.iter() {
            if let Some(top_stack_pos) = stack_positions.last() {
                if *top_stack_pos > new_stack_height - 1 {
                    amount_to_pop += 1;
                }
            }
        }

        amount_to_pop
    }

    pub fn adjust_scope(&mut self, new_scope: usize) -> Option<Instruction> {
        let pop_instruction = if new_scope > self.previous_scope {
            self.scope_begin.push(self.stack_counter);

            None
        } else if new_scope < self.previous_scope {
            let new_stack_height = self.scope_begin.pop().unwrap();
            let difference = self.stack_counter - new_stack_height;
            self.stack_counter = new_stack_height;

            let mut keys_to_update: Vec<usize> = vec![];
            let mut i: usize = 0;
            for (_, stack_positions) in self.stack_table.iter() {
                if let Some(top_stack_pos) = stack_positions.last() {
                    if *top_stack_pos > new_stack_height - 1 {
                        keys_to_update.push(i);
                    }
                }
                i += 1;
            }

            for key in keys_to_update {
                if let Some((_, stack_positions)) = self.stack_table.get_index_mut2(key) {
                    for i in (0..stack_positions.len()).rev() {
                        let stack_pos = stack_positions.get(i).unwrap();
                        if *stack_pos > new_stack_height - 1 {
                            stack_positions.pop();
                        } else {
                            break;
                        }
                    }
                }
            }

            if difference == 0 {
                None
            } else {
                Some(Instruction::Pop { amount: difference })
            }
        } else {
            None
        };

        self.previous_scope = new_scope;
        pop_instruction
    }

    pub fn free_register(&mut self, register: usize) {
        self.available_registers.push(register)
    }

    pub fn assign_register(&mut self) -> usize {
        self.available_registers.pop().expect("Internal error: All registers are in use")
    }

    fn increment_stack_counter(&mut self) -> usize {
        let index = self.stack_counter;
        self.stack_counter += 1;
        index
    }

    pub fn insert_variable(&mut self, lexeme: &String) -> usize {
        if self.stack_table.contains_key(lexeme) {
            let pos = self.increment_stack_counter();
            let mut_value = self.stack_table.get_mut(lexeme).unwrap();
            mut_value.push(pos);
            pos
        } else {
            let pos = self.increment_stack_counter();
            self.stack_table.insert(lexeme.clone(), vec![pos]);
            pos
        }
    }

    pub fn get_variable_stack_pos(&self, lexeme: &String) -> usize {
        *self.stack_table.get(lexeme).unwrap().last().unwrap()
    }
}

impl CFG {
    fn dde(&mut self) {
        let mut visited_nodes: HashSet<usize> = HashSet::new();
        self.mark_node_alive(0, &mut visited_nodes);
    }

    fn mark_node_alive(&mut self, node_id: usize, visited_nodes: &mut HashSet<usize>) {
        let mut linked_ids: Vec<usize> = Vec::with_capacity(2);

        match self.get_mut_node(node_id).unwrap() {
            CFGNode::ProgramStart { next_id, .. } => {
                linked_ids.push(*next_id);
            }
            CFGNode::Process { node, .. } => {
                (*node).state = CFGNodeState::Alive;
                linked_ids.push(node.next_id);
            }
            CFGNode::Decision { node, .. } => {
                (*node).state = CFGNodeState::Alive;
                linked_ids.push(node.true_branch_id);
                linked_ids.push(node.false_branch_id);
            }
            CFGNode::Goto { node, .. } => {
                (*node).state = CFGNodeState::Alive;
                linked_ids.push(node.goto_node_id);
            }
            CFGNode::ProgramEnd { .. } => {}
        }

        for linked_id in linked_ids {
            if !visited_nodes.contains(&linked_id) {
                visited_nodes.insert(linked_id);
                self.mark_node_alive(linked_id, visited_nodes);
            }
        }
    }

    #[profiler::function_tracker]
    pub fn optimize_and_generate_bytecode(&mut self) -> Vec<Instruction> {
        // self.constant_folding();

        // self.eliminate_dead_code();

        // let registers_maps = &mut RegistersMap::new();

        self.dde();

        let mut vm_symbol_table = VMSymbolTable::new();

        // self.dissassemble();

        self.generate_bytecode(&mut vm_symbol_table)
    }

    fn adjust_scope(
        &mut self,
        node_id: usize,
        instructions: &RefCell<Vec<Instruction>>,
        vm_symbol_table: &mut VMSymbolTable
    ) {
        match self.nodes.get(node_id).unwrap() {
            CFGNode::ProgramStart { .. } => {}
            CFGNode::Process { node, scope } => {
                if node.state == CFGNodeState::Dead {
                    return;
                } else if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                    // instructions.borrow_mut().push(pop_instruction);
                }
            }
            CFGNode::Decision { node, scope } => {
                if node.state == CFGNodeState::Dead {
                    return;
                } else if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                    // instructions.borrow_mut().push(pop_instruction);
                }
            }
            CFGNode::Goto { node, scope } => {
                if node.state == CFGNodeState::Dead {
                    return;
                } else if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                    // instructions.borrow_mut().push(pop_instruction);
                }
            }
            CFGNode::ProgramEnd { .. } => {
                return;
            }
        }
    }

    fn get_scope_diff_between_nodes(&self, start_node_id: usize, end_node_id: usize) -> usize {
        let start_scope = self.nodes.get(start_node_id).unwrap().get_scope();

        let end_scope = self.nodes.get(end_node_id).unwrap().get_scope();

        start_scope.abs_diff(end_scope)
    }

    fn generate_bytecode(&mut self, vm_symbol_table: &mut VMSymbolTable) -> Vec<Instruction> {
        let instructions = RefCell::new(Vec::new());

        let node_id_to_instruction_index: RefCell<AHashMap<usize, usize>> = RefCell::new(
            AHashMap::new()
        );
        let goto_listerners: RefCell<AHashMap<usize, Vec<usize>>> = RefCell::new(AHashMap::new());

        let push_listener = |node_id: usize, instruction_listener: usize| {
            let contains_key = goto_listerners.borrow().contains_key(&node_id);
            if contains_key {
                (*goto_listerners.borrow_mut().get_mut(&node_id).unwrap()).push(
                    instruction_listener
                )
            } else {
                goto_listerners.borrow_mut().insert(node_id, vec![instruction_listener]);
            }
        };

        let fire_listeners = |node_id: usize| {
            let change_pos = |instruction_index: &usize| {
                let instructions_len = instructions.borrow().len();

                match instructions.borrow_mut().get_mut(*instruction_index).unwrap() {
                    Instruction::Jmp { pos } => {
                        *pos = instructions_len;
                    }
                    Instruction::JmpPop { pos, .. } => {
                        *pos = instructions_len;
                    }
                    | Instruction::JE { false_pos, .. }
                    | Instruction::JNE { false_pos, .. }
                    | Instruction::JG { false_pos, .. }
                    | Instruction::JGE { false_pos, .. }
                    | Instruction::JL { false_pos, .. }
                    | Instruction::JLE { false_pos, .. } => {
                        *false_pos = instructions_len;
                    }
                    _ => {}
                }
            };

            if let Some(instruction_indexes) = goto_listerners.borrow().get(&node_id) {
                for instruction_index in instruction_indexes {
                    change_pos(instruction_index);
                }
            }

            goto_listerners.borrow_mut().remove(&node_id);
        };

        let mut i = 0;

        loop {
            fire_listeners(i);

            match self.nodes.get(i).unwrap() {
                CFGNode::ProgramStart { .. } => {}
                CFGNode::Process { node, scope } => {
                    if node.state == CFGNodeState::Dead {
                        i += 1;
                        continue;
                    }
                    vm_symbol_table.adjust_scope(*scope);

                    instructions
                        .borrow_mut()
                        .extend(node.dag.generate_dag_bytecode(vm_symbol_table));
                }
                CFGNode::Decision { node, scope } => {
                    if node.state == CFGNodeState::Dead {
                        i += 1;
                        continue;
                    }
                    vm_symbol_table.adjust_scope(*scope);

                    if let Some(condition) = &node.condition {
                        let comparison_op = condition.get_comparison_op();

                        let generated_condition_bytecode =
                            condition.generate_dag_bytecode(vm_symbol_table);

                        let true_pos =
                            instructions.borrow().len() + generated_condition_bytecode.len() + 1;

                        let jmp_instruction = match comparison_op {
                            ComparisonOp::Equal => Instruction::JE { true_pos, false_pos: 0 },
                            ComparisonOp::NotEqual => Instruction::JNE { true_pos, false_pos: 0 },
                            ComparisonOp::Greater => Instruction::JG { true_pos, false_pos: 0 },
                            ComparisonOp::GreaterEqual =>
                                Instruction::JGE { true_pos, false_pos: 0 },
                            ComparisonOp::Less => Instruction::JL { true_pos, false_pos: 0 },
                            ComparisonOp::LessEqual => Instruction::JLE { true_pos, false_pos: 0 },
                        };

                        node_id_to_instruction_index
                            .borrow_mut()
                            .insert(i, instructions.borrow().len());

                        instructions.borrow_mut().extend(generated_condition_bytecode);
                        instructions.borrow_mut().push(jmp_instruction);

                        push_listener(node.false_branch_id, instructions.borrow_mut().len() - 1);
                    } else {
                        node_id_to_instruction_index
                            .borrow_mut()
                            .insert(i, instructions.borrow().len());
                    }
                }
                CFGNode::Goto { node, scope } => {
                    if node.state == CFGNodeState::Dead {
                        i += 1;
                        continue;
                    }
                    vm_symbol_table.adjust_scope(*scope);

                    let scope_difference = self.get_scope_diff_between_nodes(i, node.goto_node_id);
                    let pop_amount = vm_symbol_table.get_pop_amount_in_scope_diff(scope_difference);

                    if
                        let Some(jmp_pos) = node_id_to_instruction_index
                            .borrow()
                            .get(&node.goto_node_id)
                    {
                        if pop_amount > 0 {
                            instructions
                                .borrow_mut()
                                .push(Instruction::JmpPop { pos: *jmp_pos, amount: pop_amount });
                        } else {
                            instructions.borrow_mut().push(Instruction::Jmp { pos: *jmp_pos });
                        }
                    } else {
                        let goto_index = instructions.borrow_mut().len();
                        push_listener(node.goto_node_id, goto_index);
                        if pop_amount > 0 {
                            instructions
                                .borrow_mut()
                                .push(Instruction::JmpPop { pos: 0, amount: pop_amount });
                        } else {
                            instructions.borrow_mut().push(Instruction::Jmp { pos: 0 });
                        }
                    }
                }
                CFGNode::ProgramEnd { scope } => {
                    if let Some(pop_instruction) = vm_symbol_table.adjust_scope(0) {
                        instructions.borrow_mut().push(pop_instruction);
                    }
                    vm_symbol_table.adjust_scope(*scope);
                    instructions.borrow_mut().push(Instruction::Halt);
                    break;
                }
            }

            i += 1;
        }

        instructions.take()
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

    pub fn dissassemble(&self) {
        let mut i = 0;

        for node in &self.nodes {
            println!("{}:{}{}", i, " ".repeat(3 - i.to_string().len()), node.dissassemble());
            i += 1;
        }
    }

    pub fn add_node(&mut self, node: CFGNode, next_node_id: &impl Fn() -> usize) -> usize {
        match node {
            CFGNode::Goto { .. } => {
                next_node_id();
            }
            CFGNode::Decision { .. } => {}
            CFGNode::Process { .. } => {}
            CFGNode::ProgramStart { .. } => {}
            CFGNode::ProgramEnd { .. } => {}
        }

        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn get_mut_node(&mut self, i: usize) -> Option<&mut CFGNode> {
        self.nodes.get_mut(i)
    }

    // // Implement iterator for CFG
    // fn for_each<F: FnMut(&mut CFGNode)>(&mut self, start_node_id: usize, mut callback: F) {
    //     let mut next_node_id = start_node_id;

    //     loop {
    //         match self.nodes.get_mut(next_node_id) {
    //             Some(node) => {
    //                 callback(node);
    //                 match node {
    //                     CFGNode::Break(_) => {
    //                         break;
    //                     }
    //                     CFGNode::Loop(loop_node) => {
    //                         next_node_id = loop_node.node_id_after_loop;
    //                     }
    //                     CFGNode::Decision(decision_node) => {
    //                         next_node_id = decision_node.true_branch_id;
    //                     }
    //                     CFGNode::Process(process_node) => {
    //                         next_node_id = process_node.next_id;
    //                     }
    //                     CFGNode::ProgramStart(next_id) => {
    //                         next_node_id = *next_id;
    //                     }
    //                     CFGNode::ProgramEnd => {
    //                         break;
    //                     }
    //                     CFGNode::ScopeStart(next_id) => {
    //                         next_node_id = *next_id;
    //                     }
    //                     CFGNode::ScopeEnd(next_id) => {
    //                         match next_id {
    //                             Some(next_id) => {
    //                                 next_node_id = *next_id;
    //                             }
    //                             None => {}
    //                         }
    //                     }
    //                 }
    //             }
    //             None => {
    //                 panic!("Invalid next_node_id: {}", next_node_id);
    //             }
    //         }
    //     }
    // }

    // fn constant_folding(&mut self) {
    //     let mut environment = IREnvironment::new();

    //     let mut scope = 0;

    //     self.for_each(0, |node| {
    //         match node {
    //             CFGNode::Break { .. } => {}
    //             CFGNode::Loop { .. } => {}
    //             CFGNode::Decision { .. } => {}
    //             CFGNode::ProgramStart { .. } => {
    //                 environment.start_scope();
    //             }
    //             CFGNode::ProgramEnd => {
    //                 environment.end_scope();
    //             }
    //             CFGNode::Process { ref mut node, .. } => {
    //                 node.dag.constant_folding(&mut environment, scope);
    //             }
    //         }
    //     });
    // }

    // pub fn generate_bytecode(
    //     &mut self,
    //     start_node_id: usize,
    //     registers_map: &mut RegistersMap,
    //     offset_len: usize
    // ) -> (Vec<Instruction>, JMPInstructionIndexes) {
    //     let mut instructions = Vec::with_capacity(64);
    //     let mut next_node_id = start_node_id;
    //     let mut jmp_instructions = JMPInstructionIndexes::new();

    //     loop {
    //         match self.nodes.get(next_node_id) {
    //             Some(node) => {
    //                 // println!("node: {:#?}", node);
    //                 match node {
    //                     CFGNode::Goto { node, scope } => panic!("Goto not supported yet"),
    //                     CFGNode::Loop { node, .. } => {
    //                         next_node_id = node.node_id_after_loop;

    //                         println!("loop_node_id_after: {}", node.node_id_after_loop);

    //                         let (mut temp_instructions, temp_jmp_instructions) =
    //                             self.generate_bytecode(
    //                                 node.loop_begin_node_id,
    //                                 registers_map,
    //                                 offset_len
    //                             );

    //                         jmp_instructions.extend(temp_jmp_instructions);
    //                         jmp_instructions.apply_offset(instructions.len());

    //                         println!("temp_instructions: {:#?}", temp_instructions);
    //                         println!("jmp_instrs: {:#?}", jmp_instructions);

    //                         let len = temp_instructions.len();
    //                         for break_index in &jmp_instructions.break_jmps {
    //                             match temp_instructions.get_mut(*break_index).unwrap() {
    //                                 Instruction::Jmp { ref mut pos } => {
    //                                     *pos = len + instructions.len();
    //                                 }
    //                                 _ => panic!("Expected Jmp instruction"),
    //                             };
    //                         }

    //                         let scope_end_instructions = temp_instructions.pop().unwrap();

    //                         temp_instructions.push(Instruction::Jmp {
    //                             pos: instructions.len() + 1,
    //                         });

    //                         temp_instructions.push(scope_end_instructions);

    //                         instructions.extend(temp_instructions);
    //                     }
    //                     CFGNode::Decision { node, .. } => {
    //                         let node_id_after_if_branches = node.false_branch_id;

    //                         let (temp_instructions, temp_jump_instructions) =
    //                             self.generate_bytecode_for_if_stmt(
    //                                 next_node_id,
    //                                 registers_map,
    //                                 instructions.len() + offset_len
    //                             );

    //                         jmp_instructions.extend(temp_jump_instructions);

    //                         instructions.extend(temp_instructions);

    //                         next_node_id = node_id_after_if_branches;
    //                     }
    //                     CFGNode::Process { node, .. } => {
    //                         next_node_id = node.next_id;

    //                         let node_instructions = node.dag.generate_dag_bytecode(registers_map);
    //                         instructions.extend(node_instructions);
    //                     }
    //                     CFGNode::ProgramStart(next_id) => {
    //                         next_node_id = *next_id;
    //                     }
    //                     CFGNode::ProgramEnd => {
    //                         instructions.push(Instruction::Halt);
    //                         break;
    //                     }
    //                 }
    //             }
    //             None => {
    //                 panic!("Invalid next_node_id: {}", next_node_id);
    //             }
    //         }
    //     }

    //     (instructions, jmp_instructions)
    // }

    // pub fn generate_bytecode_for_if_stmt(
    //     &mut self,
    //     decision_node_id: usize,
    //     registers_map: &mut RegistersMap,
    //     all_instructions_len: usize
    // ) -> (Vec<Instruction>, JMPInstructionIndexes) {
    //     // Get comparison op for later (to avoid borrow rule errors)
    //     let comparison_op = match self.nodes.get_mut(decision_node_id).unwrap() {
    //         CFGNode::Decision { node, .. } =>
    //             match &mut node.condition {
    //                 Some(dag) => { Some(dag.ensure_comparison_to_entry_node()) }
    //                 _ => None,
    //             }
    //         _ => panic!("Expected decision node"),
    //     };

    //     let (instructions, jmp_instructions) = match self.nodes.get_mut(decision_node_id).unwrap() {
    //         CFGNode::Decision { node, .. } => {
    //             let mut if_stmt_instructions: Vec<Instruction> = vec![];

    //             let node_instructions = match &node.condition {
    //                 Some(dag) => { dag.generate_dag_bytecode(registers_map) }
    //                 None => vec![],
    //             };

    //             let offset_len = all_instructions_len + node_instructions.len() + 1;

    //             let (true_branch_id, false_branch_id) = (node.true_branch_id, node.false_branch_id);

    //             let (true_branch_instructions, mut jmp_instructions) = self.generate_bytecode(
    //                 true_branch_id,
    //                 registers_map,
    //                 offset_len
    //             );

    //             if_stmt_instructions.extend(node_instructions);

    //             let true_pos =
    //                 all_instructions_len +
    //                 if_stmt_instructions.len() +
    //                 jmp_instructions.get_total_instruction_len() +
    //                 1;
    //             let false_pos = true_pos + true_branch_instructions.len() + 1;

    //             let jump_instruction = match comparison_op {
    //                 Some(comparison_op) =>
    //                     match comparison_op {
    //                         ComparisonOp::Equal => { Instruction::JE { true_pos, false_pos } }
    //                         ComparisonOp::NotEqual => { Instruction::JNE { true_pos, false_pos } }
    //                         ComparisonOp::Greater => { Instruction::JG { true_pos, false_pos } }
    //                         ComparisonOp::GreaterEqual => {
    //                             Instruction::JGE { true_pos, false_pos }
    //                         }
    //                         ComparisonOp::Less => { Instruction::JL { true_pos, false_pos } }
    //                         ComparisonOp::LessEqual => { Instruction::JLE { true_pos, false_pos } }
    //                     }
    //                 None => Instruction::Jmp { pos: true_pos },
    //             };

    //             if_stmt_instructions.push(jump_instruction);

    //             if_stmt_instructions.extend(true_branch_instructions);

    //             jmp_instructions.apply_offset(if_stmt_instructions.len() - 1);

    //             // if let Some(false_branch_id) = false_branch_id {
    //             let (temp_instructions, temp_jmp_instructions) = self.generate_bytecode_for_if_stmt(
    //                 false_branch_id,
    //                 registers_map,
    //                 all_instructions_len +
    //                     if_stmt_instructions.len() +
    //                     1 +
    //                     jmp_instructions.get_total_instruction_len()
    //             );

    //             jmp_instructions.extend(temp_jmp_instructions);

    //             if_stmt_instructions.push(Instruction::Jmp {
    //                 pos: all_instructions_len +
    //                 if_stmt_instructions.len() +
    //                 temp_instructions.len() +
    //                 jmp_instructions.get_total_instruction_len() +
    //                 1,
    //             });
    //             if_stmt_instructions.extend(temp_instructions);
    //             // } else {
    //             //     if_stmt_instructions.push(Instruction::Jmp {
    //             //         pos: all_instructions_len +
    //             //         if_stmt_instructions.len() +
    //             //         jmp_instructions.get_total_instruction_len() +
    //             //         1,
    //             //     });
    //             // }

    //             (if_stmt_instructions, jmp_instructions)
    //         }
    //         _ => panic!("Expected decision node"),
    //     };

    //     (instructions, jmp_instructions)
    // }
}
