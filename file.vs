mut d := 0
a := 18
if a == 18 {
    d = 9
} else {
    d = 3
}
d = 0


/*
a := 2 * 9
b := 8 + 9 * a
n := (4 * 5 - 10) / a + -6
1 + 8
c := 170 == b
mut d := a
k := d + 2
d = a
cc := d + a


loop {
    d = d + 1
    if d == 10 {
        break
    }
}


while d <= 10 {
    d = d + 1;
}


mut a := 2

if a < 10 do a = 3 else a = 4

while a <= 10 do
    mut prev := a++
end

import std/procs/display

proc iota begin

    derive enum(derivedEnum DerivedEnum)

    end

end

impl<'ast> LinearControlFlow for Stmts<'ast>
    fn compileIntoDag(&self, dag &mut DAG, identNodeIdMap &mut HashMap<SSAKey, Uint>) Uint
        linearStmts := self.stmts.iter().filterMap(|stmt| stmt.asLinearControlFlow()).collect()

        for linearStmt in linearStmts
            nodeId := linearStmt.compileIntoDag(dag, identNodeIdMap)
            dag.setEntryNodeId(nodeId)
        end

        dag.getEntryNodeId()
    end
end

impl<'ast> LinearControlFlow for Stmts<'ast> {
    fn compile_into_dag(
        &self,
        dag: &mut DAG,
        ident_node_id_map: &mut AHashMap<SSAKey, usize>
    ) -> usize {
        let linear_stmts = self.stmts.iter().filter_map(|stmt| stmt.as_linear_control_flow()).collect::<Vec<_>>();

        for linear_stmt in linear_stmts {
            let node_id = linear_stmt.compile_into_dag(dag, ident_node_id_map);
            dag.set_entry_node_id(node_id);
        }

        dag.get_entry_node_id()
    }
}


proc iota {
     derive enum(derivedEnum DerivedEnum) {
        {
            mut acc := 0
            mut variants := derivedEnum.getVariants()
            variants.forEach(fn(variant) {
                variant.setValue(acc++)
            })
        }
        write {
            impl [< derivedEnum.getName() >] {
                [< derivedEnum.getVariants().forEach(fn(variant) {
                    write {
                        fn get[< variant.getName().firstUpperCase() >]() Self {
                            print("Hello world!")
                        }
                    }
                }) >]
            }
        }
    }
}

proc iota begin
    derive enum(derivedEnum DerivedEnum) begin
        do mut acc := 0
            mut variants := derivedEnum.getVariants()
            variants.forEach(fn(variant)
                variant.setValue(acc++)
            end)
        end

        writeBegin

        impl {derivedEnum.getName()}
            {derivedEnum.getVariants().forEach(fn(variant)
                writeBegin
                fn get{variant.getName().firstUpperCase()}() Self
                    print("Hello world!")
                end
                writeEnd
            end)}
        end

        writeEnd
    end
end


proc deriveGetMethods begin
    derive struct(mut derivedStruct DerivedStruct) begin
        beginWrite
        impl {derivedStruct.getName()}
            {derivedStruct.getFields().forEach(fn(mut field)
                beginWrite
                fn get{field.getName().firstUpperCase()}(self) {field.getType()}
                    self.{field.getName()}
                end
                endWrite
            end)}
        end
        endWrite
    end
end

derive @display, @deriveGetMethods
struct User
    name String,
    age UInt

    construct(self, name String, age UInt)
        self.name = name
        self.age = UInt
    end
end

impl User
    fn birthday(mut self) UInt
        self.age++
    end
end


derive @display, @iota
enum OpCodes
    NO_OP,
    ADD,
    SUB,
    MUL,
    DIV,
    OPS_COUNT
end

derive @deriveGetMethods
struct User
    name String,
    age UInt

    construct(mut self, name String, age UInt)
        self.name = name
        self.age = age
    end
end

impl User
    fn birthday(mut self)
        self.age++
    end
end

fn main()
    mut user := new User()
    user.birthday()
    user.getName()
    user.getAge()

    println("Hello {user.getName()}!")
    assertEq(Opcodes.NO_OP, 0)
    assertEq(Opcodes.OPS_COUNT, 5)

    a := do
        mut acc := 0
        for i in 0..100 do
            acc += i
        end
        acc
    end

    println("a = {a}")
end


!!2 == !1
a := 8 + 9 * 8
a := 2 + a
mut a := a + 2
a := 9 + a

if a == 4 {
    a = 1
} else {
    a = 0
}

1 + a * a + a + 8

1 + 9 * a

mut a := (2 * 2 + 2)
b := (2 + 3 + 6)
c := a + 9

if c > 14 {
    a = a + 1
} else {
    a = a + 2
}

fn hi() {
    k := 2 + 6 * 1 + 1
    d := k + 1
}


use crate.hello as hello

fib3 := crate.hello.fib(3)

enum Something {
    A,
    B,
    C,
    D
}

struct Node<'arena> {
    id Int,
    linked Option<&'arena Node<'arena>>
}

struct Arena {
    'lifetime: Node
}

struct Graph {
    'a = 'this.arena.lifetime

    nodes Vec<&'a Node<'a>>,
    arena Arena
}

impl Graph {
    pub fn setMsg(&mut self, msg String) {
        self.msg = msg
    }

    pub fn alloc(&mut self, item Node) {
        allocatedNode = self.arena.alloc(item)
        self.nodes.push()
    }
}

fn fib(hello &mut Graph) {
    a := hello
    b := hello

    a.setMsg("Hello")
}

fn getA() {
    something := Something.A // Now this is of type Something

    match something {
        Something.A -> true,
        _ -> false
    }
}

fn fib(n i32) i32 {
    if n < 2 {
        return n
    }
    return fib(n - 2) + fib(n - 1)
}

fib(35)

arr &mut [i32] := &mut [2, 9, 8]

fn someFunc(n i32) i32 {
    if n < 1 {
        return n
    }
    return someFunc(n - 1)
}
someFunc(5000)

fn computeValue() ref i32 {
    return &3
}

valueRef := computeValue()
drop(valueRef)
valueRef += 1
// ERROR: valueRef was dropped

mut a := 2 + 2 * 8
mut cd := 4

[1, 2, 3, 4].forEach(|num mut ref i32| {
    num++
})
// [2, 3, 4, 5]


if (1 + 1) * 2 == cd {
    if 2 == 3 {
        a = 3
    } else {
        if 1 == 19 {
            a = 8
        } else if 8 == 2 {
            a = 16
        } else {
            a = 9
        }
    }
} else {
    a = -2
}

a
cd




fn cmp(x i32, y i32) {
    if x == y {
        return x
    }
    return cmp(x - y, x - y)
}

cmp(10, 2)








if 2 == 2 {
    a = 3
}

pub fn add(..values i32) i32 {
    mut summed i32

    for v in values {
        summed += v
    }

    return summed
}

pub class Calculator {
    pub value i32: 0
    hasError bool: false

    pub fn addToValue(mut self, value i32) Self {
        self.value += value
    }
}

calculator := Calculator()
calculator.addToValue(3)
print(calculator.value) // 3
calculator.addToValue(-2)
print(calculator.value)

mut a := 1 * 4 + 9
a = 3
2 + a

{

    mut a := 2 + 2
    a + 1
    {
        a = 3
        a + 1
    }
    true

}
a = 0

false
true
{
    a + 2
    c := 9
    {
        a + c + a
        b := 2
        a + b + c
        a = 19
        a = -2
    }
}
a + 2

from main.calculator import Calculator, Add, Sub, Div, Mul

from calculator import Calculator, Add

newImportNamespace Calculator: main.calculator
from Calculator import Calculator, Add, Sub, Div

pub class Calculator {
    pub result int

    pub fn add(x &int, y &int) int {
        return x + y
    }
}

impl Calculator {

}

calculator Calculator

if 2 > 1 {
    calculator = Calculator()
}

print(calculator)

fn calc(x int, y int) int {
    return match bool {
        true => x + y,
        false => x - y
    }
}


hello(calc)

fn hello(func: Fn(x int, y int) int) {
    func(2, 8)
}

impl Hello {
    pub fn doSomething() -> string {
        if true {
            return "Hello"
        } else {
            return "Not hello"
        }
    }
}

impl i32 {
    pub fn toFormattedString() -> string {
        return "The value is: {self.toString()}"
    }
}

a := calc(2, 1)

fn add(x Int, y Int) Int {
    x + y
}

fn add(x Int, y Int) Int
    x + y
end

fn sub(x Int, y Int) Int {
    x - y
}

fn calc(x Int, y Int) Int {
    add(x, y) + sub(x, y)
}

// converts into this:

fn add(x Int, y Int) Int
fn sub(x Int, y Int) Int
fn calc(x Int, y Int) Int

fn add(x Int, y Int) Int {
    x + y
}
fn sub(x Int, y Int) Int {
    x - y
}
fn calc(x Int, y Int) Int {
    add(x, y) + sub(x, y)
}

a := calc(2, 1)


user := User
fn main()

end

fn main {

}


*/
