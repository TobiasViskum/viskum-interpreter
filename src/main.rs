#![feature(macro_metavar_expr)]

mod macros;
mod compiler;

use compiler::Compiler;

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

    let (vm_data, llvm_main_fn) = {
        let mut compiler = Compiler::new();

        match compiler.compile_entry() {
            Some(v) => ((), ()),
            None => compiler.log_errors(),
        }
    };
}
