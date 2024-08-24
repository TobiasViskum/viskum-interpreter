#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

/* clang gen.c -o gen && ./gen && clang -S -emit-llvm gen.c */
typedef struct {
    int len;
    int capacity;
    int* items;
} Vector;

bool hello() {
    return 3;
}

Vector* new_vec() {
    Vector* vec = (Vector*)malloc(sizeof(Vector));
    
    if (vec == NULL) {
        exit(1);
    }
    
    vec->len = 0;
    vec->capacity = 4;

    int* items = (int*)malloc(vec->capacity * sizeof(int));

    if (items == NULL) {
        exit(1);
    }

    vec->items = items;

    return vec;
}

int push_item(Vector* vec, int item) {
    int b = 888;
    if (vec->len == vec->capacity) {
        vec->capacity = vec->capacity * 2;

        int* new_items = (int*)realloc(vec->items, vec->capacity * sizeof(int));

        if (new_items == NULL) {
            exit(1);
        }

        vec->items = new_items;
    }

    int a = 999;

    vec->items[vec->len] = item;
    int c = 777;
    vec->len++;
    int k = 666;

    return 0;
}

void dissasemble_vec(Vector* vec) {
    printf("Len: %d\n", vec->len);
    printf("Capacity: %d\n", vec->capacity);
    printf("Items:\n");
    for (int i = 0; i < vec->len; i++) {
        printf("vec[%d] = %d\n", i, vec->items[i]);
    }
}



int main() {
    Vector* vec = new_vec();
    push_item(vec, 2);
    push_item(vec, 6);
    push_item(vec, 343);
    push_item(vec, 39);
    push_item(vec, 90);

    char temp[50];

    char* hello = "Hello";
    char* space = " ";
    char* world = "World!";
    
    sprintf(temp, "%s%s%s", hello, space, world);
    int len = strlen(temp);
    char* final = (char*)malloc((len + 1) * sizeof(char));
    strcpy(final, temp);
    printf("Concatenated String: %d%s\n", len, final);


    dissasemble_vec(vec);

    return 0;
}

/*

pub mod Math
    pub add(x Int, y Int) Int
        ret x + y
    end

    pub sub(x Int, y Int) Int
        ret x - y
    end
end

struct Hello
    add fn(x Int, y Int) Int,
    sub fn(x Int, y Int) Int,
end

impl Hello
    pub new(add Add, sub Sub) Self
        Self {
            add: add,
            sub: sub
        }
    end

    pub callAdd(self, args (x Int, y Int)) Int
        self.add(args.x, args.y)
    end
end

add(x Int, y Int) Int
    ret x + y
end

sub(x Int, y Int) Int
    ret x - y
end

!Allow GlobalMut
mut INSTRS = [2, 3, 4]

proc iota() begin
    derive enum(mut derivedEnum DerivedEnum) begin
        do mut i = 0
            for variant in derivedEnum.getMutVariants() do
                variant.setValue(i++)
            end
        end
    end

    @write(
    
    impl {derivedEnum.getName()}
        pub new() Self


        end
    end

    )
end

std.macros.enum.@iota()
enum Instrs
    NO_OP,
    ADD,
    SUB,
    MUL,
    OPS_COUNT
end

struct Person
    name String
    age Uint.8
end

newAdd(args ...Int) Int
    import std.math

    x math.Solutions = math.@begin(
        a = 1
        c = 2
        d = 3
        f(x) = 2x + 2
        g(x) = 2x^2 + 4x + 1
        ret f(x) = g(x)
    )


    integrated math.Function = math.integrate(f) # math.integrate(f math.Function)



    ret args.reduce((acc, arg)
        acc += arg
    end)
end

add(x Int, y Int) Result<(result Int, success Bool), (errMsg String)>
    result = x + y
    ret Result.Ok((result, success: true))
end

main() ()

end

main() Int
    mut ints Int[]

    i = mut i Int :: loop
        if i++ > 2 do
            break i
        end
    end

    print("I was {i}")

    vec [Int: 3] = [2, 3, 4]
    mut vec2 Int[] = [2, 3, 4]

    newRes = newAdd(2, 3, 9, 8, 9, 1, 2) #
    
    if vec.len() == 3 do goto terminate: end

    mut a = add(2, 3)
    d = sub(8, 2)

    thread.new((x Int)
        a = std.copy(a)
        b = std.copy(d)
    end)

    a = do
        result = ((x Int) Int ret x**2 end)(2)
        k1 = 2
        k2 = 3
        2 + 3
    end

    cmp = if a > 2 do true else false end

    a = if a > 2 and b < 2 do
        2 + 2
    else
        2 + 9
    end
    
    user = "Tobias"
    print("Hello {user}, how's your day?")

    label terminate:
    ret a + d
end

declare print(String)

macro @printf(str String, args ...) begin
    print(@format(str, args))
end

macro macroAssert<T>(cond1 T, cond2 T, str String, args ...) begin
    macroAssert(cond1, cond2, @format(str, args))
end

macro @format(str String, args ...) begin
    @macroAssert(str.placeholderArgsLen(), args.len(), "Expected equal amount of args, but got ({}, {})", str.placeholderArgsLen(), args.len())
    @formatArgs(str, args)
end

macro @formatArgs(str String, args ...) begin
    @write()
end



*/