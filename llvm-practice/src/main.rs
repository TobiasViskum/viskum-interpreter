#[derive(Debug)]
pub enum Op {
    NoOp,
    Add,
    Sub,
}

fn main() {
    let hello = "Hello";
    let world = "World!";
    let a = 88888;
    let op = Op::Add;
    let b = 99999;
    let new2 = format!("{} {}", hello, world);
    println!("Result: {} {} {} {:?}", new2, a, b, op);
}
