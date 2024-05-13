// type Hello = i32


mut a := 0
mut b := 0

if a == 1 {
    k := 8
    a = 0
}

loop {
   
    if a < 3 {
        loop {
            d := 8
            if b == 8 {
                c := 2
                break
                d := 9
                a = 8
            }
            b = b + 2
        }
    } else {
        break
        b = 0
    }
    a = a + 1
}
if b == 8 {
    a = 9
}

/*



0: ProgramStart
1: S0, Decision(2, 7)
2: S1, Decision(3, 4)
3: S2, Process(7)
4: S1, Process(5)
5: S1, Process(1)
6: S0, Process(7)
7: ProgramEnd

DEFINE
STARTSCOPE
ASSIGN
ENDSCOPE

let decision_node

a := 2 + 2 * 8 
cd := 4

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