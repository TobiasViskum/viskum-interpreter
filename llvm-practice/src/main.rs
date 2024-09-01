#[derive(Debug)]
pub enum Op {
    NoOp,
    Add,
    Sub,
}

fn main() {
    let a = 2;
    let b = 4;
    let kk = [8, 8, 9283];

    let d = [2, 3, 4, 5, 6, 1000][5];

    println!("{:?}", d);

    let mut MAIN_PROGRAM_STRING = String::new();
    MAIN_PROGRAM_STRING += "I am making a progamming langauge";

    let hello = "Hello";
    let world = "World!";
    let a = 88888;
    let op = Op::Add;
    let b = 99999;
    let new2 = format!("{} {}", hello, world);
    println!("Result: {} {} {} {:?}", new2, a, b, op);
}
