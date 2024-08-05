mod macros;
mod compiler;
mod vm;

use compiler::Compiler;
use vm::VM;

pub const U16_MAX: usize = u16::MAX as usize;

fn main() {
    /*
    Rewrite to this later:

    struct VMData {
        instructions: OptimizedInstructions,
        constant_pool: Vec<Value>
    }

    let (vm_data, llvm_main_fn) = {
        let compiler = Compiler::new();

        match compiler.compile_entry() {
            Some(v) => v,
            None => std::process:exit(1);
        }; 
    }; // Option<(VMData, LLVMFnMain)>

    VM::new().run(VMData);

    llvm_main_fn.compile_and_execute();
    */

    let (registers, instructions) = {
        let mut compiler = Compiler::new();

        compiler.compile_entry()
    };

    let mut vm = VM::new(registers);
    vm.run(instructions);
    println!("{:#?}", vm.print_regs())
}
