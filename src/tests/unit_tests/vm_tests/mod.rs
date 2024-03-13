#[cfg(test)]
mod test {
    use crate::{ value::Value, vm::{ instructions::{ Instruction, InstructionSrc }, VM } };

    #[test]
    fn test_add() {
        let program = vec![
            Instruction::Load { reg: 0, value: Value::Int32(5) },
            Instruction::Load { reg: 1, value: Value::Int32(10) },
            Instruction::Add {
                dest: 0,
                src1: InstructionSrc::Register(0),
                src2: InstructionSrc::Register(1),
            }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(0), &Value::Int32(15));
    }

    #[test]
    fn test_sub() {
        let program = vec![
            Instruction::Load { reg: 0, value: Value::Int32(5) },
            Instruction::Load { reg: 1, value: Value::Int32(10) },
            Instruction::Sub {
                dest: 0,
                src1: InstructionSrc::Register(0),
                src2: InstructionSrc::Register(1),
            }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(0), &Value::Int32(-5));
    }

    #[test]
    fn test_mul() {
        let program = vec![
            Instruction::Load { reg: 0, value: Value::Int32(5) },
            Instruction::Load { reg: 1, value: Value::Int32(10) },
            Instruction::Mul {
                dest: 0,
                src1: InstructionSrc::Register(0),
                src2: InstructionSrc::Register(1),
            }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(0), &Value::Int32(50));
    }

    #[test]
    fn test_div() {
        let program = vec![
            Instruction::Load { reg: 0, value: Value::Int32(5) },
            Instruction::Load { reg: 1, value: Value::Int32(10) },
            Instruction::Div {
                dest: 0,
                src1: InstructionSrc::Register(1),
                src2: InstructionSrc::Register(0),
            }
        ];

        let mut vm = VM::new(program);
        vm.run();

        assert_eq!(vm.get_register(0), &Value::Int32(2));
    }
}
