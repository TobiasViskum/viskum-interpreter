@str.fmt.d = private unnamed_addr constant [7 x i8] c"D: %d\0A\00"
@str.fmt.sum = private unnamed_addr constant [9 x i8] c"Sum: %d\0A\00"
@str.fmt.result = private unnamed_addr constant [12 x i8] c"Result: %d\0A\00"

define i32 @add(i32 noundef %a, i32 noundef %b) {
    %result = add i32 %a, %b
    ret i32 %result
}

define i32 @main() {
    %d_ptr = alloca i32
    %a_ptr = alloca i32
    %sum_ptr = alloca i32
    store i32 0, ptr %a_ptr
    store i32 0, ptr %d_ptr
    store i32 0, ptr %sum_ptr
    br label %main_loop_body

main_loop_body:
    store i32 0, ptr %a_ptr
    %d2 = load i32, ptr %d_ptr
    %newd = add i32 %d2, 1
    store i32 %newd, ptr %d_ptr
    %ldr_newd = load i32, ptr %d_ptr

    %cmpeq = icmp eq i32 %ldr_newd, 700000

    br i1 %cmpeq, label %term, label %else_cond

else_cond:
    %a2 = load i32, ptr %a_ptr
    %cmple = icmp sle i32 %a2, 15

    br i1 %cmple, label %else_body, label %main_loop_body
else_body:
    %sum2 = load i32, ptr %sum_ptr
    %a3 = load i32, ptr %a_ptr
    %new_sum = add i32 %sum2, %a3
    store i32 %new_sum, ptr %sum_ptr

    %a4 = load i32, ptr %a_ptr
    %new_a = add i32 %a4, 1
    store i32 %new_a, ptr %a_ptr
    br label %else_cond
term:
    %final_d = load i32, ptr %d_ptr
    call i32 (ptr, ...) @printf(ptr noundef @str.fmt.d, i32 noundef %final_d)

    %final_sum = load i32, ptr %sum_ptr
    call i32 (ptr, ...) @printf(ptr noundef @str.fmt.sum, i32 noundef %final_sum)

    %add_result = call i32 @add(i32 noundef 100, i32 noundef 3)
    call i32 (ptr, ...) @printf(ptr noundef @str.fmt.result, i32 noundef %add_result)

    ret i32 0;
}

declare i32 @printf(ptr noundef, ...)