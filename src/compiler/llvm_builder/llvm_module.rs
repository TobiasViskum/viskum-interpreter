use super::{ llvm_function::Function, llvm_type::{ Type, TypeI32 }, BuildLLVM };

pub struct Module {
    // constants
    // globals
    string_constants: Vec<String>,
    functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            string_constants: vec![],
            functions: vec![],
        }
    }

    pub fn push_func(&mut self, func: Function) {
        self.functions.push(func);
    }

    pub fn add_string_const(&mut self, string: String) -> usize {
        self.string_constants.push(string);
        self.string_constants.len() - 1
    }
}

impl BuildLLVM for Module {
    fn build(&self) -> String {
        let mut string_builder = "".to_string();

        for (i, string_constant) in self.string_constants.iter().enumerate() {
            string_builder += format!(
                "@.str.{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1",
                i,
                string_constant.len() + 1,
                string_constant
            ).as_str();
        }

        string_builder += "\n";

        for function in self.functions.iter() {
            string_builder += "\n";
            string_builder += function.build().as_str();
        }

        string_builder
    }
}
