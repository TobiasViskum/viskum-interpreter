use std::cell::RefCell;
use std::rc::Rc;

use ahash::{ AHashMap, HashSet, HashSetExt };
use indexmap::{ map::MutableKeys, IndexMap };

use crate::value::{ Function, Value };
use crate::vm::instructions::{ InstructionSrc, StackLocation };
use crate::{ constants::REGISTERS, operations::ComparisonOp, vm::instructions::Instruction };
use crate::compiler::vm_symbol_table::VMSymbolTable;

use self::cfg_node::{
    CFGConnectorNode,
    CFGDecisionNode,
    CFGGotoNode,
    CFGNodeState,
    CFGProcessNode,
    CFGReturnNode,
};
use self::dag::DAG;

pub mod cfg_node;
pub mod dag;

#[derive(Debug)]
pub enum CFGNode {
    // Connector {
    //     node: CFGConnectorNode,
    //     scope: usize,
    // },
    FunctionConnector {
        node: CFGConnectorNode,
        fn_name: Rc<str>,
        args: Vec<Rc<str>>,
        args_count: usize,
        scope: usize,
    },
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
    Return {
        node: CFGReturnNode,
        scope: usize,
    },
    CfgStart {
        next_id: usize,
        scope: usize,
    },
    CfgEnd {
        scope: usize,
    },
}

impl CFGNode {
    pub fn get_scope(&self) -> usize {
        match self {
            Self::Return { scope, .. } => *scope,
            Self::FunctionConnector { scope, .. } => *scope,
            Self::Process { scope, .. } => *scope,
            Self::Decision { scope, .. } => *scope,
            Self::Goto { scope, .. } => *scope,
            Self::CfgStart { scope, .. } => *scope,
            Self::CfgEnd { scope, .. } => *scope,
        }
    }

    pub fn dissassemble(&self) -> String {
        match self {
            Self::FunctionConnector { node, scope, .. } =>
                format!("S{}, FunctionConnector({})", scope, node.cfg_id),
            Self::Process { node, scope } => format!("S{}, Process({})", scope, node.next_id),
            Self::Decision { node, scope } =>
                format!("S{}, Decision({}, {})", scope, node.true_branch_id, node.false_branch_id),
            Self::Goto { node, scope } => format!("S{}, Goto({})", scope, node.goto_node_id),
            Self::Return { scope, .. } => format!("S{}, Return", scope),
            Self::CfgStart { .. } => format!("CfgStart"),
            Self::CfgEnd { .. } => format!("CfgEnd"),
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

impl CFG {
    pub fn dde(&mut self, visited_cfgs: &mut HashSet<usize>) -> Vec<usize> {
        let mut visited_nodes: HashSet<usize> = HashSet::new();

        self.mark_node_alive(0, &mut visited_nodes, visited_cfgs)
    }

    fn mark_node_alive(
        &mut self,
        node_id: usize,
        visited_nodes: &mut HashSet<usize>,
        visited_cfgs: &mut HashSet<usize>
    ) -> Vec<usize> {
        let mut linked_cfgs: Vec<usize> = Vec::new();
        let mut linked_ids: Vec<usize> = Vec::with_capacity(2);

        match self.get_mut_node(node_id).unwrap() {
            CFGNode::Return { node, .. } => {
                (*node).state = CFGNodeState::Alive;
            }
            CFGNode::FunctionConnector { node, .. } => {
                (*node).state = CFGNodeState::Alive;
                linked_ids.push(node.next_id);
                if !visited_cfgs.contains(&node.cfg_id) {
                    visited_cfgs.insert(node.cfg_id);
                    linked_cfgs.push(node.cfg_id);
                }
            }
            CFGNode::CfgStart { next_id, .. } => {
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
            CFGNode::CfgEnd { .. } => {}
        }

        for linked_id in linked_ids {
            if !visited_nodes.contains(&linked_id) {
                visited_nodes.insert(linked_id);
                linked_cfgs.extend(self.mark_node_alive(linked_id, visited_nodes, visited_cfgs));
            }
        }

        linked_cfgs
    }

    #[profiler::function_tracker]
    pub fn optimize_and_generate_bytecode(&mut self) -> Vec<Instruction> {
        // self.constant_folding();

        // self.eliminate_dead_code();

        // let registers_maps = &mut RegistersMap::new();

        self.dde(&mut HashSet::new());

        let mut vm_symbol_table = VMSymbolTable::new();

        // self.dissassemble();

        let instructions = RefCell::new(Vec::new());

        self.generate_bytecode(&mut vm_symbol_table, &instructions, None);

        instructions.take()
    }

    fn get_node_scopes(&self, start_node_id: usize, end_node_id: usize) -> (usize, usize) {
        let start_scope = self.nodes.get(start_node_id).unwrap().get_scope();

        let end_scope = self.nodes.get(end_node_id).unwrap().get_scope();

        // println!("start: {}, end:{}", start_scope, end_scope);

        // start_scope.abs_diff(end_scope)

        (start_scope, end_scope)
    }

    pub fn generate_bytecode(
        &self,
        vm_symbol_table: &mut VMSymbolTable,
        instructions: &RefCell<Vec<Instruction>>,
        initial_variables: Option<Vec<Rc<str>>>
    ) -> Vec<(usize, Vec<Rc<str>>, usize)> {
        let mut function_connector_ids = vec![];

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

            if i == 1 {
                if let Some(ref initial_variables) = initial_variables {
                    let mut reg = 0;
                    for var in initial_variables {
                        let (stack_pos, is_relative) = vm_symbol_table.insert_variable(var);
                        instructions.borrow_mut().push(Instruction::Define {
                            stack_loc: StackLocation::new(stack_pos, is_relative),
                            src: InstructionSrc::Register { reg },
                        });
                        vm_symbol_table.free_register(reg);
                        reg += 1;
                    }
                }
            }

            match self.nodes.get(i).unwrap() {
                CFGNode::FunctionConnector { node, scope, fn_name, args_count, args } => {
                    if node.state == CFGNodeState::Dead {
                        i += 1;
                        continue;
                    }
                    if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                        instructions.borrow_mut().push(pop_instruction);
                    }
                    function_connector_ids.push((
                        node.cfg_id,
                        args.clone(),
                        instructions.borrow().len(),
                    ));
                    let (stack_pos, is_relative) = vm_symbol_table.insert_variable(fn_name);

                    instructions.borrow_mut().push(Instruction::Define {
                        stack_loc: StackLocation::new(stack_pos, is_relative),
                        src: InstructionSrc::Constant {
                            val: Value::Function(Function::new(*args_count)),
                        },
                    });
                }
                CFGNode::CfgStart { scope, .. } => {
                    if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                        instructions.borrow_mut().push(pop_instruction);
                    }
                }
                CFGNode::Process { node, scope } => {
                    if node.state == CFGNodeState::Dead {
                        i += 1;
                        continue;
                    }
                    if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                        instructions.borrow_mut().push(pop_instruction);
                    }

                    instructions
                        .borrow_mut()
                        .extend(node.dag.generate_dag_bytecode(vm_symbol_table));

                    // self.adjust_scope(i, &instructions, vm_symbol_table);
                }
                CFGNode::Decision { node, scope } => {
                    if node.state == CFGNodeState::Dead {
                        i += 1;
                        continue;
                    }
                    if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                        instructions.borrow_mut().push(pop_instruction);
                    }

                    if let Some(condition) = &node.condition {
                        let comparison_op = condition.get_comparison_op();

                        let generated_condition_bytecode =
                            condition.generate_dag_bytecode(vm_symbol_table);

                        let true_pos =
                            instructions.borrow().len() + generated_condition_bytecode.len() + 1;

                        let jmp_instruction = match comparison_op {
                            ComparisonOp::Equal =>
                                Instruction::JE {
                                    true_pos,
                                    false_pos: 0,
                                },
                            ComparisonOp::NotEqual =>
                                Instruction::JNE {
                                    true_pos,
                                    false_pos: 0,
                                },
                            ComparisonOp::Greater =>
                                Instruction::JG {
                                    true_pos,
                                    false_pos: 0,
                                },
                            ComparisonOp::GreaterEqual =>
                                Instruction::JGE {
                                    true_pos,
                                    false_pos: 0,
                                },
                            ComparisonOp::Less =>
                                Instruction::JL {
                                    true_pos,
                                    false_pos: 0,
                                },
                            ComparisonOp::LessEqual =>
                                Instruction::JLE {
                                    true_pos,
                                    false_pos: 0,
                                },
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
                CFGNode::Return { node, scope } => {
                    if node.state == CFGNodeState::Dead {
                        i += 1;
                        continue;
                    }
                    if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                        instructions.borrow_mut().push(pop_instruction);
                    }

                    match &node.return_value {
                        Some(dag) => {
                            let bytecode = dag.generate_dag_bytecode_as_return(vm_symbol_table);

                            instructions.borrow_mut().extend(bytecode);
                        }
                        None => {
                            panic!(
                                "Defined vars before return: (not implemented if return_value is None). May cause A LOT of cache misses if this isn't dealt with. Therefore I'm panicking"
                            );

                            instructions.borrow_mut().push(Instruction::Return {
                                src: InstructionSrc::Constant { val: Value::Void },
                            });
                        }
                    }
                }
                CFGNode::Goto { node, scope } => {
                    if node.state == CFGNodeState::Dead {
                        i += 1;
                        continue;
                    }
                    if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                        instructions.borrow_mut().push(pop_instruction);
                    }

                    let (start_scope, end_scope) = self.get_node_scopes(i, node.goto_node_id);
                    println!("{} {}", start_scope, end_scope);
                    let pop_amount = vm_symbol_table.get_pop_amount_in_scope_diff(
                        start_scope,
                        end_scope
                    );

                    if
                        let Some(jmp_pos) = node_id_to_instruction_index
                            .borrow()
                            .get(&node.goto_node_id)
                    {
                        if pop_amount > 0 {
                            instructions.borrow_mut().push(Instruction::JmpPop {
                                pos: *jmp_pos,
                                amount: pop_amount,
                            });
                        } else {
                            instructions.borrow_mut().push(Instruction::Jmp { pos: *jmp_pos });
                        }
                    } else {
                        let goto_index = instructions.borrow_mut().len();
                        push_listener(node.goto_node_id, goto_index);
                        if pop_amount > 0 {
                            instructions.borrow_mut().push(Instruction::JmpPop {
                                pos: 0,
                                amount: pop_amount,
                            });
                        } else {
                            instructions.borrow_mut().push(Instruction::Jmp { pos: 0 });
                        }
                    }
                }
                CFGNode::CfgEnd { scope } => {
                    if let Some(pop_instruction) = vm_symbol_table.adjust_scope(*scope) {
                        instructions.borrow_mut().push(pop_instruction);
                    }
                    break;
                }
            }

            i += 1;
        }

        function_connector_ids
    }
}

#[derive(Debug)]
pub struct CFG {
    nodes: Vec<CFGNode>,
}

impl Default for CFG {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
}

impl CFG {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
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
            CFGNode::Return { .. } => {
                next_node_id();
            }
            CFGNode::FunctionConnector { .. } => {}
            CFGNode::Goto { .. } => {
                next_node_id();
            }
            CFGNode::Decision { .. } => {}
            CFGNode::Process { .. } => {}
            CFGNode::CfgStart { .. } => {}
            CFGNode::CfgEnd { .. } => {}
        }

        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn get_node(&self, i: usize) -> Option<&CFGNode> {
        self.nodes.get(i)
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
    //                     CFGNode::CfgStart(next_id) => {
    //                         next_node_id = *next_id;
    //                     }
    //                     CFGNode::CfgEnd => {
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
    //             CFGNode::CfgStart { .. } => {
    //                 environment.start_scope();
    //             }
    //             CFGNode::CfgEnd => {
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
    //                     CFGNode::CfgStart(next_id) => {
    //                         next_node_id = *next_id;
    //                     }
    //                     CFGNode::CfgEnd => {
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
