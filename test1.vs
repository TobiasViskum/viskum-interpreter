def main() Int
    mut a := 1 + 2 - (2 + 8) * 3
    b := a + 2

    mut d := 0
    if a == 45 do
        d = 2
    elif a == 2 do
        d = 4
    else
        d = 0
    end

    a = 0

    loop a = a +1
        if a == 100 do
            break
        end
    end

    k := sayHello() + 20 + sayHello()

    str := "Hello world"



    a = 0
end

def sayHello() Int
    mut b := 2 + 9

    c := b + 1

    ret c + hi()
end

def hi() Int
    ret 2
end

/*  
    k := sayHello() + 20 + sayHello()

compiler := Compiler()
compiler.sayHello()

class Compiler
    programSymbolTablePhase1 ProgramSymbolTablePhase1,

    pub def new() Self
        programSymbolTablePhase1 = ProgramSymbolTablePhase1.new()

        Self(programSymbolTablePhase1)
    end

    pub def dissasemble(self) ()
        print(self.programSymbolTablePhase1.dissasemble())
    end

    pub def dissasemble(self) ()
        print(self.programSymbolTablePhase1.dissasemble())
    end
end

impl Compiler
    pub def sayHello()
        print("Hello world!")
    end
end
    
    */