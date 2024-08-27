use std::rc::Rc;

use crate::compiler::{ llvm_builder::BuildLLVM, Dissasemble };

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct SSAIdent {
    ident: Rc<str>,
    subscript: usize,
}

impl BuildLLVM for SSAIdent {
    fn build(&self) -> String {
        match self.subscript {
            0 => self.ident.to_string(),
            _ => format!("{}{}", self.ident, self.subscript),
        }
    }
}

impl SSAIdent {
    pub fn new(ident: Rc<str>, subscript: usize) -> Self {
        Self { ident, subscript }
    }

    pub fn is(&self, str: &str, subscript: usize) -> bool {
        match (self.ident.as_ref() == str, self.subscript == subscript) {
            (true, true) => true,
            _ => false,
        }
    }

    pub fn get_ident(&self) -> Rc<str> {
        Rc::clone(&self.ident)
    }

    pub fn borrow_ident(&self) -> &Rc<str> {
        &self.ident
    }

    pub fn get_subscript(&self) -> usize {
        self.subscript
    }

    pub fn set_subscript_to_zero(&mut self) {
        self.subscript = 0;
    }
}

impl Dissasemble for SSAIdent {
    fn dissasemble(&self) -> String {
        let mut subscript = String::new();
        for char in self.subscript.to_string().chars() {
            subscript += match char {
                '0' => "₀",
                '1' => "₁",
                '2' => "₂",
                '3' => "₃",
                '4' => "₄",
                '5' => "₅",
                '6' => "₆",
                '7' => "₇",
                '8' => "₈",
                '9' => "₉",
                c => panic!("Invalid subscript character: {} (only chars 0-9 is supported)", c),
            };
        }

        format!("{}{}", self.ident, subscript)
    }
}
