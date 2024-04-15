use ahash::AHashMap;

use crate::{ constants::REGISTERS, value_v2::Value, vm::instructions::Instruction };

use self::cfg_node::CFGProcessNode;
pub mod cfg_node;
pub mod dag;

#[derive(Debug)]
pub enum CFGNode {
    Process(CFGProcessNode), // This is essentially a statement e.g. "mut i32 a := 8"
    // Decision, // This is an if-statement (maybe other conditional statements as well)
    ScopeStart(usize),
    ScopeEnd(usize),
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

    pub fn add_node(&mut self, node: CFGNode) {
        self.nodes.push(node);
    }

    // Implement iterator for CFG
    fn for_each<F: FnMut(&mut CFGNode)>(&mut self, mut callback: F) {
        let mut next_node_id = 0;

        loop {
            match self.nodes.get_mut(next_node_id) {
                Some(node) => {
                    callback(node);
                    match node {
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
                            next_node_id = *next_id;
                        }
                    }
                }
                None => {
                    panic!("Invalid next_node_id: {}", next_node_id);
                }
            }
        }
    }

    pub fn generate_bytecode(&mut self) -> Vec<Instruction> {
        let mut registers_maps = RegistersMap::new();
        let mut instructions: Vec<Instruction> = Vec::with_capacity(64);

        self.for_each(|node| {
            match node {
                CFGNode::Process(ref mut process_node) => {
                    let node_instructions = process_node.dag.generate_bytecode(&mut registers_maps);
                    instructions.extend(node_instructions)
                }
                CFGNode::ScopeStart(_) => {
                    registers_maps.start_scope();
                    instructions.push(Instruction::StartScope)
                }
                CFGNode::ScopeEnd(_) => {
                    registers_maps.end_scope();
                    instructions.push(Instruction::EndScope)
                }
                CFGNode::ProgramStart(_) => {}
                CFGNode::ProgramEnd => {
                    instructions.push(Instruction::Halt);
                }
            }
        });

        instructions

        /*
        MUL S0:R0 1 4
        ADD S0:R1 S0:R0 9
        DEFINE S0:R0 S0:R1
        ADD S0:R1 2 2
        STARTSCOPE

            DEFINE S1:R0 2
            ADD S1:R1 S1:R0 1
            STARTSCOPE

                ASSIGN S2:R0 3
                ADD S2:R1 S2:R0 1

            ENDSCOPE
            LOAD S1:R2 true

        ENDSCOPE
        LOAD S0:R2 false
        LOAD S0:R3 true
        STARTSCOPE

            ADD S1:R0 S0:R0 2
            DEFINE S1:R1 9
            STARTSCOPE

                ADD S2:R0 S0:R0 S1:R1
                MUL S2:R1 2 8
                ADD S2:R2 S2:R0 S2:R1
                DEFINE S2:R1 2

            ENDSCOPE

        ENDSCOPE
        ADD S0:R4 S0:R0 2
        HALT
        */
    }

    #[profiler::function_tracker]
    pub fn optimize_and_generate_bytecode(&mut self) -> Vec<Instruction> {
        self.constant_folding();

        // self.eliminate_dead_code();

        self.generate_bytecode()
    }

    fn constant_folding(&mut self) {
        let mut environment = IREnvironment::new();

        let mut scope = 0;

        self.for_each(|node| {
            match node {
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
