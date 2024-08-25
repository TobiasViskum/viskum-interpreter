use std::rc::Rc;

pub trait NativeFnTrait {
    fn get_ident() -> Rc<str>;
}

#[derive(Debug)]
pub enum NativeFn {
    PrintFn(PrintFn),
}

#[derive(Debug)]
pub struct PrintFn;

impl NativeFnTrait for PrintFn {
    fn get_ident() -> Rc<str> {
        "print".into()
    }
}
