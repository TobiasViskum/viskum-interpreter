#include <stdio.h>

int main() {

    int end = 2

    for (int i = 0; i < 10; i++) {
        printf("Hello world\n %s", i);
    }

    return 0;
}

proc writeSomething(derivedEnum &DerivedEnum) begin writeBegin

    impl {derivedEnum.getName()}

    end

writeEnd end

proc iota() begin
    derive enum(mut derivedEnum DerivedEnum)
        do mut acc := 0
            derivedEnum.getVariants().forEach(fn(variant)
                variant.setValue(acc++)
            end)
        end

        @writeSomething(derivedEnum)
    end
end

cond := if 2 == 2 do true else false
if cond do
    println("cond {cond}")
end

proc display() begin
    derive enum(derivedEnum DerivedEnum)

        fn display(enum Enum) String
            if enum.getValue() != None do "{enum.getName()}: {enum.}" else "{enum.getName()}" end
        end

    end
end

proc iota() begin
    derive enum(mut derivedEnum DerivedEnum)
        do mut acc := 0
            derivedEnum.getVariants().forEach(fn(mut variant EnumVariant)
                variant.value = acc++
            end)
        end
    end
end

import crate/procs/iota
import std/network/HttpServer

derive @iota()
enum OpCodes
    NO_OP,
    ADD,
    SUB,
    DIV,
    MUL,
    OPS_COUNT
end

const PORT Int = 5000

fn main() Int
    op := OpCodes.selectRandom()
    op.getValue()
    op.fromValue(2)

    server := new HttpServer()

    server.get("/")

    server.listen(PORT, fn(result)
        match result
            ListenResult.Ok -> dbg.print("Listening on port {PORT}"),
            ListenResult.Err(err) -> do

            end
        end
    end)

    if op == OpCodes.ADD do
        println("Random op was ADD")
    else
        println ("Random op was { op.display() } which is equal to { op.value() }")
    end
end
