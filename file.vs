// type Hello = i32

fn fib(n i32) i32 {
    if n < 2 {
        return n
    }
    return fib(n - 2) + fib(n - 1)
}

fib(35)


/*

fn someFunc(n i32) i32 {
    if n < 1 {
        return n
    }
    return someFunc(n - 1)
}
someFunc(5000)



mut a := 2 + 2 * 8 
mut cd := 4

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

*/