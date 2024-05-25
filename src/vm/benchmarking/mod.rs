#[cfg(test)]
mod test {
    use std::time::Instant;

    use crate::{
        value::{ Function, Value },
        vm::{ instructions::{ Instruction, InstructionSrc, StackLocation }, VM },
    };

    #[test]
    fn fib_recusive_new() {
        let mut next_loc: usize = 0;
        let mut get_next_loc = || {
            let prev = next_loc;
            next_loc += 1;
            prev
        };

        // let mut vm = VM::new(main_instructions);

        let now = Instant::now();
        // vm.run();
        println!("Elapsed: {:?}", now.elapsed())
    }
}

/*

----------------------

DEFINE [0 + x] R0
CMP [0 + x] C(2)
JL 3 4
RETURN_POP [0 + x] 1
SUB R1 [0 + x] C(1)
LOAD R0 R1
CALL [1 + x] [0]
SUB R2 [0 + x] C(2)
LOAD R0 R2
CALL [2 + x] [0]
ADD R0 [2 + x] [1 + x]
RETURN_POP R0 3

DEFINE [0 + x] R0
CMP [0 + x] C(2)
JL 3 4
RETURN_POP [0 + x] 1
SUB R1 [0 + x] C(1)
LOAD R0 R1
CALL [1 + x] [0]
SUB R2 [0 + x] C(2)
LOAD R0 R2
CALL [2 + x] [0]
ADD R0 [2 + x] [1 + x]
RETURN_POP R0 3
POP 3

*/

/*

----------------------



----------------------

*/
