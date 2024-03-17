// #[cfg(test)]
// mod test {
//     use crate::{ value::Value, vm::{ instructions::{ VMInstruction, VMInstructionSrc }, VM } };

//     #[test]
//     fn test_add() {
//         let program = vec![
//             VMInstruction::Load { reg: 0, src: VMInstructionSrc::Constant(Value::Int32(5)) },
//             VMInstruction::Load { reg: 1, src: VMInstructionSrc::Constant(Value::Int32(10)) },
//             VMInstruction::Add {
//                 dest: 0,
//                 src1: VMInstructionSrc::Register(0),
//                 src2: VMInstructionSrc::Register(1),
//             }
//         ];

//         let mut vm = VM::new(program);
//         vm.run();

//         assert_eq!(vm._get_register(0), &Value::Int32(15));
//     }

//     #[test]
//     fn test_sub() {
//         let program = vec![
//             VMInstruction::Load { reg: 0, src: VMInstructionSrc::Constant(Value::Int32(5)) },
//             VMInstruction::Load { reg: 1, src: VMInstructionSrc::Constant(Value::Int32(10)) },
//             VMInstruction::Sub {
//                 dest: 0,
//                 src1: VMInstructionSrc::Register(0),
//                 src2: VMInstructionSrc::Register(1),
//             }
//         ];

//         let mut vm = VM::new(program);
//         vm.run();

//         assert_eq!(vm._get_register(0), &Value::Int32(-5));
//     }

//     #[test]
//     fn test_mul() {
//         let program = vec![
//             VMInstruction::Load { reg: 0, src: VMInstructionSrc::Constant(Value::Int32(5)) },
//             VMInstruction::Load { reg: 1, src: VMInstructionSrc::Constant(Value::Int32(10)) },
//             VMInstruction::Mul {
//                 dest: 0,
//                 src1: VMInstructionSrc::Register(0),
//                 src2: VMInstructionSrc::Register(1),
//             }
//         ];

//         let mut vm = VM::new(program);
//         vm.run();

//         assert_eq!(vm._get_register(0), &Value::Int32(50));
//     }

//     #[test]
//     fn test_div() {
//         let program = vec![
//             VMInstruction::Load { reg: 0, src: VMInstructionSrc::Constant(Value::Int32(5)) },
//             VMInstruction::Load { reg: 1, src: VMInstructionSrc::Constant(Value::Int32(10)) },
//             VMInstruction::Div {
//                 dest: 0,
//                 src1: VMInstructionSrc::Register(1),
//                 src2: VMInstructionSrc::Register(0),
//             }
//         ];

//         let mut vm = VM::new(program);
//         vm.run();

//         assert_eq!(vm._get_register(0), &Value::Int32(2));
//     }
// }
