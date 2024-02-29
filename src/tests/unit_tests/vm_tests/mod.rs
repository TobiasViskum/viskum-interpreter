#[cfg(test)]
mod test {
    use crate::{ vm::{ instructions::Instruction, VM }, * };

    #[test]
    fn test_add() {
        let program = vec![
            Instruction::Load { reg: 0, value: 5 },
            Instruction::Load { reg: 1, value: 10 },
            Instruction::Add { dest: 2, src1: 0, src2: 1 }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(2), &15);
    }

    #[test]
    fn test_sub() {
        let program = vec![
            Instruction::Load { reg: 0, value: 5 },
            Instruction::Load { reg: 1, value: 10 },
            Instruction::Sub { dest: 2, src1: 0, src2: 1 }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(2), &-5);
    }

    #[test]
    fn test_mul() {
        let program = vec![
            Instruction::Load { reg: 0, value: 5 },
            Instruction::Load { reg: 1, value: 10 },
            Instruction::Mul { dest: 2, src1: 0, src2: 1 }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(2), &50);
    }

    #[test]
    fn test_div() {
        let program = vec![
            Instruction::Load { reg: 0, value: 5 },
            Instruction::Load { reg: 1, value: 10 },
            Instruction::Div { dest: 2, src1: 1, src2: 0 }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(2), &2);
    }

    #[test]
    fn test_add_sub() {
        let program = vec![
            Instruction::Load { reg: 0, value: 5 },
            Instruction::Load { reg: 1, value: 10 },
            Instruction::Add { dest: 2, src1: 0, src2: 1 },
            Instruction::Sub { dest: 3, src1: 2, src2: 0 }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(3), &10);
    }
}
