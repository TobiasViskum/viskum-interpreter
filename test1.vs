
def sayHello() Int
    mut b := 2 + 9

    c := b + 1

    ret c + hi()
end

def hi() Int
    ret 2 + 9 * 8
end


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


    loop a = a + 1
        if a == 100 do
            break
        else
            continue
        end
    end

    k := sayHello() + 20 + sayHello()


    str := "Hello world"
    str := "My name is Tobias"
    str := "Hello again :)"




    a = 0
end



/*  
    k := sayHello() + 20 + sayHello()

compiler := Compiler()
compiler.sayHello()

enum Expr
    BinaryExpr(BinaryExpr),
end

pub class Lexer chars Char[] tokens Token[] current Uint
    pub def new(chars Char[]) Self
        print("Created new lexer!")

        ret Self(
            chars,
            tokens: [], 
            current: 0 
        )
    end

    pub def scanTokens(self) Token[]
        while !self.isAtEnd() do
            self.scanToken()
        end

        ret self.tokens
    end

    def advance(mut self) Char
        self.chars[self.current++]
    end
end

class Compiler
    programSymbolTablePhase1 ProgramSymbolTablePhase1,

    pub def new() Self
        programSymbolTablePhase1 = ProgramSymbolTablePhase1.new()

        ret Self(programSymbolTablePhase1)
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

 Expands into:

    proc math(
        tt Tokens[]
    )
        derive new
        class FunctionDef
            name String
            value Expr

            pub def new(name String, value Expr) Self
                ret Self(name, value)
            end


        end
        
    end

    result = @math(
        f(x) = 2x + 3
        g(x) = 4x - 4
        ret solve(f(x) = g(x))
    )
    print("Result is {result}")

    do 


        def f(x Float) Float
            ret 2 * x + 3
        end

        def g(x Float) Float
            ret 4 * x - 4
        end


        x := mut i Float = -10000 :: while i < 1000 do
            f(i) == g(i) then break Some(i)
            f(i + 1) == g(i + 1) then break Some(i + 1)
            i += 2
        end or None

        x
    end
    
    */