use std::rc::Rc;

use ahash::{ AHashMap, AHashSet };

use crate::compiler::{ ds::symbol_table::NativeSymbolFn, traits::NativeFnTrait };

use super::{ llvm_function::Function, llvm_type::{ Type, TypeI32 }, BuildLLVM, RawOperand };

struct LLVMArray {
    items_type: Type,
    items: Vec<RawOperand>,
}

#[derive(PartialEq, Eq, Hash)]
struct LLVMArrayKey {
    items_type: Type,
    items_count: usize,
}

pub struct Module {
    // constants
    // globals
    string_constants: AHashMap<String, usize>,
    native_fns: AHashSet<NativeSymbolFn>,
    arrays: AHashMap<LLVMArrayKey, (LLVMArray, usize)>,
    idents_to_idx: AHashMap<Rc<str>, usize>,
    functions: Vec<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            string_constants: AHashMap::new(),
            native_fns: AHashSet::new(),
            idents_to_idx: AHashMap::new(),
            arrays: AHashMap::new(),
            functions: vec![],
        }
    }

    pub fn push_fn(&mut self, func: Function) {
        self.functions.push(func)
    }

    pub fn insert_ident(&mut self, ident: Rc<str>, idx: usize) {
        self.idents_to_idx.insert(ident, idx);
    }

    pub fn lookup_ident_idx(&self, ident: &Rc<str>) -> usize {
        *self.idents_to_idx.get(ident).expect("Expected ident in lookup_ident")
    }

    pub fn push_native_fn(&mut self, native_fn: NativeSymbolFn) {
        if !self.native_fns.contains(&native_fn) {
            self.native_fns.insert(native_fn);
        }
    }

    pub fn push_array(&mut self, items_type: Type, items: Vec<RawOperand>) -> usize {
        let llvm_array_key = LLVMArrayKey {
            items_count: items.len(),
            items_type: items_type.clone(),
        };
        if let Some((_, idx)) = self.arrays.get(&llvm_array_key) {
            *idx
        } else {
            let idx = self.arrays.len();
            self.arrays.insert(llvm_array_key, (LLVMArray { items_type: items_type, items }, idx));
            idx
        }
    }

    pub fn add_string_const(&mut self, mut string: String) -> usize {
        string += "\\00";
        if let Some(str_idx) = self.string_constants.get(&string) {
            *str_idx
        } else {
            let idx = self.string_constants.len();
            self.string_constants.insert(string, idx);
            idx
        }
    }

    pub fn get_const_string_idx(&self, string: &String) -> usize {
        *self.string_constants.get(&format!("{}\\00", string)).expect("Expected const string")
    }
}

fn get_llvm_string_len(str: &str) -> usize {
    let eol_substring = "\\0A";

    let null_term_substring = "\\00";

    let eol_count = str
        .as_bytes()
        .windows(eol_substring.len())
        .filter(|&w| w == eol_substring.as_bytes())
        .count();

    let null_term_count = str
        .as_bytes()
        .windows(null_term_substring.len())
        .filter(|&w| w == null_term_substring.as_bytes())
        .count();

    str.len() - eol_count * 2 - null_term_count * 2
}

impl BuildLLVM for Module {
    fn build(&self) -> String {
        let mut string_builder = "".to_string();

        for (i, (llvm_array, items)) in self.arrays.values().enumerate() {
            let mut array_string_builder = format!(
                "@.array.{} = private unnamed_addr constant [{} x {}] [",
                i,
                llvm_array.items.len(),
                llvm_array.items_type.build()
            );

            for (i, item) in llvm_array.items.iter().enumerate() {
                array_string_builder += llvm_array.items_type.build().as_str();
                array_string_builder += " ";
                array_string_builder += item.build().as_str();

                if i != llvm_array.items.len() - 1 {
                    array_string_builder += ", ";
                }
            }

            array_string_builder += format!(
                "], align {}\n",
                llvm_array.items_type.get_byte_size()
            ).as_str();
        }

        string_builder += "\n";

        for (string_constant, idx) in self.string_constants.iter() {
            string_builder += format!(
                "@.str.{} = private unnamed_addr constant [{} x i8] c\"{}\", align 1\n",
                idx,
                get_llvm_string_len(string_constant),
                string_constant
            ).as_str();
        }

        string_builder += "\n";

        for function in self.functions.iter() {
            string_builder += "\n";
            string_builder += function.build().as_str();
        }

        string_builder += "\n";

        for native_fn in self.native_fns.iter() {
            string_builder += format!("{}", native_fn.declare_llvm()).as_str();
        }

        string_builder
    }
}
