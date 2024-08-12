mod compiler;
mod macros;
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

    let (registers, instructions, dbg_instructions) = {
        let mut compiler = Compiler::new();

        compiler.compile_entry()
    };

    let is_debug = match &std::env::args().collect::<Vec<_>>().get(2) {
        Some(str) => *str == "--debug",
        None => false,
    };

    if is_debug {
        drop(instructions);
        let mut vm = VM::new(registers);
        vm.debug_run(dbg_instructions);
        println!("{:#?}", vm.print_regs());
    } else {
        drop(dbg_instructions);
        VM::new(registers).run(instructions);
    }
}
