
def sayHello() Int
    mut b := 2 + 9

    c := b + 1

    ret c
end

def hi(x Int) Int
    ret 2 + 9 * 8 + x
end

def main() Int
    mut a := 1 + 2 - (2 + 8) * 3 + 72
    b := a + 2
    
    mut d := 0
    cond := 2 == 2
    if cond do
        d = 2
        print("Case 1")
    elif a == 2 do
        d = 4
        print("Case 2")
    else
        d = 0
        print("Case 3")
    end

    a = 0

    mut array := [0, 1, 2, 3, 3]
  
    print("array at 4 is:", array[4])

    array[4] = 4;

    print("array at 4 is:", array[4])


    loop a = a + 1
        if a == 100 do
            print("A is now 100")
            break
        else
            print("A is:", a)
            continue
        end
    end


    k := sayHello() + 20 + sayHello()
    print(k)


    str := "Hello world"
    str := "Hello world"
    str := "My name is Tobias. This is my name!"

    isEqual := 100 == a
    print("My name is Tobias. I am", 18, "years old.", str, "Result is:", a, "isEqual is:", isEqual)

    myName := "Tobias"
    brotherName := "Tobias"
    hasSameName := myName == brotherName

    print("Does", myName, "has the same name as", brotherName, ":", hasSameName)

    mut op String
    print("Uninitialized variable:", op, 2)
    op = "Hello"
    print(op)

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



enum UnaryExpr
    op UnaryOp,
    rhs Expr
end

derive @auto() // GetWith, Debug
enum Expr
    BinaryExpr(BinaryExpr)
    UnaryExpr(UnaryExpr)
with resultType ValueType end

struct Hello
    expr Expr,
    resultType ValueType
end

impl GetWith for Expr
    typedef With ( 
        resultType ValueType
    )

    def getWith(self) With
        ret (
            resultType: self.with.resultType
        )
    end
end

impl DefaultWith for Expr
    typedef With ( 
        resultType ValueType
    )

    def defaultWith() With
        ret (
            resultType: Option.None
        )
    end
end

expr := Expr.BinaryExpr(BinaryExpr.new(2, "add", 3)) with ( resultType: ValueType.Int )
( resultType ) := expr.getWith()
    
    */