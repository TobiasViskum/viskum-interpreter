mut d := 1
{
    d = 1
    {
        d = 9
    }
    d = 2
}
2 + 9

if a == 18 {
    d = 99
} else {
    d = 0
}

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

mut a := 2

if a < 10 do a = 3 else a = 4

while a <= 10 do
    mut prev := a++
    prev = 2
end

enum HelloWorld
    str1 = 1,
    str2 = 2,
    str3 = 3
    



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

fn add(x int, y int) int {
    x + y
}

fn sub(x int, y int) int {
    x - y
}

fn calc(x int, y int) int {
    add(x, y) + sub(x, y)
}

// converts into this:

fn add(x int, y int) int
fn sub(x int, y int) int
fn calc(x int, y int) int

fn add(x int, y int) int {
    x + y
}
fn sub(x int, y int) int {
    x - y
}
fn calc(x int, y int) int {
    add(x, y) + sub(x, y)
}

a := calc(2, 1)

*/