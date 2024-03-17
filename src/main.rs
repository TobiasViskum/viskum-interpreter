use compiler::Compiler;
use error_handler::ErrorHandler;
use parser::Parser;
use vm::VM;

#[cfg(test)]
mod tests;

mod error_handler;
mod parser;
mod vm;
mod compiler;
mod ast;
mod value;
mod constants;
mod operations;
mod util;

/*
MUL R0 2 3
ADD R1 1 R0
MUL R0 4 5
ADD R2 R1 R0
ADD R0 3 6
MUL R1 6 R0
ADD R0 R2 R1
ADD R1 1 1
DIV R2 4 R1
ADD R1 R0 R2
*/

/*
MUL R9 2 3
ADD R8 1 R9
MUL R7 4 5
ADD R6 R8 R7
ADD R5 3 6
MUL R4 6 R5
ADD R3 R6 R4
ADD R2 1 1
DIV R1 4 R2
ADD R0 R3 R1
*/

#[profiler::start(true)]
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <source file>", args[0]);
        std::process::exit(1);
    }

    let file_content = match std::fs::read_to_string(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            println!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };
    let file_str = file_content.as_str();

    // let src1 = "(-2 * ((1 + -(2 * 3) + 4 * (3 + 1 + 1)) * -8 + 2 + 6) + 1 + 1 - 1) / 2 - 4";
    // let src2 = "2 + true";
    let src3 =
        "mut i32 a := 2 + 8; 2 + a; 3 * a + a; 1 + a; i32 b := a * 8; a = 3; i32 c := a + b; i32 d := 2 * a * b * c + 4 * 8";

    let src = file_str;

    let error_handler = &mut ErrorHandler::new();

    let src_chars = src.chars().collect::<Vec<_>>();
    let mut parser = Parser::new(&src_chars, error_handler);

    let ast = parser.parse_to_ast();

    if error_handler.has_error() {
        error_handler.print_errors(src);
        std::process::exit(1);
    }

    let mut compiler = Compiler::new(error_handler);

    let instructions = compiler.compile(ast);

    if let Some(instructions) = instructions {
        let mut vm = VM::new(instructions);

        vm.run();
    } else {
        error_handler.print_errors(src);
    }
}
