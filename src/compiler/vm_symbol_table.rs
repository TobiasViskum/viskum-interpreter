use std::{ collections::BTreeSet, rc::Rc };

use indexmap::IndexMap;

use crate::{ constants::REGISTERS, vm::instructions::Instruction };

#[derive(Debug)]
pub struct VMSymbolTable {
    pub available_registers: BTreeSet<usize>, // Available registers
    pub reserved_registers: BTreeSet<usize>, // Reserved registers (for func calls)
    scope_begin: Vec<usize>,
    function_begin: Vec<usize>,
    stack_height: usize,
    stack_table: IndexMap<Rc<str>, Vec<usize>>, // Vec<usize> refers to SSA. The outermost index is the newest of that var. E.g. the stack position of a_1 is on top of a_0
    previous_scope: usize,
}

impl VMSymbolTable {
    pub fn new() -> Self {
        // let available_registers = (0..REGISTERS).rev().collect::<Vec<_>>();
        let mut available_registers = BTreeSet::new();

        for i in 0..REGISTERS {
            available_registers.insert(i);
        }

        Self {
            available_registers,
            reserved_registers: BTreeSet::new(),
            scope_begin: Vec::new(),
            function_begin: Vec::new(),
            stack_height: 0,
            stack_table: IndexMap::new(),
            previous_scope: 0,
        }
    }

    pub fn begin_func_compilation(&mut self) {
        self.function_begin.push(self.stack_height)
    }

    pub fn end_func_compilation(&mut self) {
        self.function_begin.pop();
    }

    pub fn get_pop_amount_in_fn(&self) -> usize {
        let new_stack_height = self.function_begin.last().unwrap();

        let mut amount_to_pop = 0;

        for (_, stack_positions) in self.stack_table.iter() {
            for stack_pos in stack_positions.iter() {
                if *stack_pos > *new_stack_height - 1 {
                    amount_to_pop += 1;
                }
            }
        }

        amount_to_pop
    }

    pub fn get_pop_amount_in_scope_diff(&self, start_scope: usize, end_scope: usize) -> usize {
        let scope_diff = start_scope.abs_diff(end_scope);

        if scope_diff == 0 {
            return 0;
        }

        println!("len: {}", self.scope_begin.len());
        println!("diff: {}", scope_diff);
        let new_stack_height = self.scope_begin.get(self.scope_begin.len() - scope_diff).unwrap();

        let mut amount_to_pop = 0;

        for (_, stack_positions) in self.stack_table.iter() {
            for stack_pos in stack_positions.iter() {
                if *stack_pos > *new_stack_height - 1 {
                    amount_to_pop += 1;
                }
            }
        }

        amount_to_pop
    }

    pub fn adjust_scope(&mut self, new_scope: usize) -> Option<Instruction> {
        let pop_instruction = if new_scope > self.previous_scope {
            for _ in self.previous_scope..new_scope {
                self.scope_begin.push(self.stack_height);
            }

            None
        } else if new_scope < self.previous_scope {
            let mut new_stack_height: usize = self.stack_height;
            for _ in new_scope..self.previous_scope {
                new_stack_height = self.scope_begin.pop().unwrap();
            }

            let difference = self.stack_height - new_stack_height;

            if difference == 0 {
                return None;
            }

            self.stack_height = new_stack_height;

            let mut keys_to_update: Vec<usize> = vec![];
            let mut i: usize = 0;
            for (_, stack_positions) in self.stack_table.iter() {
                if let Some(top_stack_pos) = stack_positions.last() {
                    if new_stack_height != 0 && *top_stack_pos > new_stack_height - 1 {
                        keys_to_update.push(i);
                    }
                }
                i += 1;
            }

            for key in keys_to_update {
                if let Some((_, stack_positions)) = self.stack_table.get_index_mut(key) {
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

            Some(Instruction::Pop { amount: difference })
        } else {
            None
        };

        self.previous_scope = new_scope;
        pop_instruction
    }

    pub fn free_register(&mut self, register: usize) {
        self.available_registers.insert(register);
    }

    pub fn reserve_registers(&mut self, amount: usize) {
        for i in 0..amount {
            let first_reg = *self.available_registers.first().unwrap();

            if first_reg != i {
                panic!("Could not reserve registers for function call");
            } else {
                self.available_registers.remove(&first_reg);
                self.reserved_registers.insert(first_reg);
            }
        }
    }

    pub fn get_reserved_register(&mut self) -> usize {
        let reserved_reg = *self.reserved_registers
            .first()
            .expect("Unable to get reserved register");
        self.reserved_registers.remove(&reserved_reg);
        self.available_registers.insert(reserved_reg);

        reserved_reg
    }

    pub fn assign_register(&mut self) -> usize {
        let register = *self.available_registers.first().unwrap();
        self.available_registers.remove(&register);

        register
    }

    fn increment_stack_counter(&mut self) -> usize {
        let index = self.stack_height;
        self.stack_height += 1;
        index
    }

    pub fn insert_call_result(&mut self) -> (usize, bool) {
        let lexeme: &Rc<str> = &"@call".into();
        let stack_pos = if self.stack_table.contains_key(lexeme) {
            let pos = self.increment_stack_counter();
            let mut_value = self.stack_table.get_mut(lexeme).unwrap();
            mut_value.push(pos);
            pos
        } else {
            let pos = self.increment_stack_counter();
            self.stack_table.insert(Rc::clone(lexeme), vec![pos]);
            pos
        };

        if let Some(func_begin) = self.function_begin.last() {
            if stack_pos > *func_begin - 1 {
                (stack_pos - *func_begin, true)
            } else {
                (stack_pos, false)
            }
        } else {
            (stack_pos, false)
        }
    }

    pub fn insert_variable(&mut self, lexeme: &Rc<str>) -> (usize, bool) {
        let stack_pos = if self.stack_table.contains_key(lexeme) {
            let pos = self.increment_stack_counter();
            let mut_value = self.stack_table.get_mut(lexeme).unwrap();
            mut_value.push(pos);
            pos
        } else {
            let pos = self.increment_stack_counter();
            self.stack_table.insert(Rc::clone(lexeme), vec![pos]);
            pos
        };

        if let Some(func_begin) = self.function_begin.last() {
            if stack_pos > *func_begin - 1 {
                (stack_pos - *func_begin, true)
            } else {
                (stack_pos, false)
            }
        } else {
            (stack_pos, false)
        }
    }

    pub fn get_variable_stack_pos(&self, lexeme: &Rc<str>) -> (usize, bool) {
        let stack_pos = *self.stack_table.get(lexeme).unwrap().last().unwrap();

        if let Some(func_begin) = self.function_begin.last() {
            if stack_pos > *func_begin - 1 {
                (stack_pos - *func_begin, true)
            } else {
                (stack_pos, false)
            }
        } else {
            (stack_pos, false)
        }
    }
}
