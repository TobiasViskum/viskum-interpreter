define i32 @main() {
    %1 = add nsw i32 2, 2
    %2 = sub nsw i32 %1, 2
    %3 = add nsw i32 %2, 2
    %4 = alloca i32, align 4
    store i32 %3, ptr %4, align 4
    %5 = load i32, ptr %4
    %6 = add nsw i32 %5, 2
    %7 = alloca i32, align 4
    store i32 %6, ptr %7, align 4
    ret i32 0
}

