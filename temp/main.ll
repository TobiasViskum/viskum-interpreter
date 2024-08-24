@.str.fmt = private unnamed_addr constant [7 x i8] c"I: %d\0A\00", align 1
@.str.helloworld = private unnamed_addr constant [14 x i8] c"Hello, world\0A\00", align 1

define i32 @main() {
    %i1 = alloca i32
    store i32 0, ptr %i1
    br label %cond

cond:
    %i2 = load i32, ptr %i1
    %cmp = icmp sle i32 %i2, 10
    br i1 %cmp, label %body, label %end

body:
    call i32 (ptr, ...) @printf(ptr noundef @.str.fmt, i32 noundef %i2)

    br label %inc

inc:
    %newI = add i32 %i2, 1
    store i32 %newI, ptr %i1
    br label %cond

end:
    ret i8 0
}


declare i32 @printf(ptr noundef, ...)


; clang -o main main.ll && ./main  

; clang -o test test.c && ./test && clang -S -emit-llvm test.c