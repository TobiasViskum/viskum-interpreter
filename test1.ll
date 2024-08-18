
define i32 @main() {
    %1 = alloca i32, align 4
    %2 = alloca i32, align 4
    %3 = alloca i32, align 4
    %4 = alloca i32, align 4
    %5 = alloca i32, align 4
    %6 = add nsw i32 1, 2
    %7 = sub nsw i32 %6, 2
    %8 = add nsw i32 %7, 1
    store i32 %8, ptr %1, align 4
    %9 = load i32, ptr %1
    %10 = add nsw i32 %9, 2
    store i32 %10, ptr %2, align 4
    %11 = load i32, ptr %1
    %12 = icmp eq i32 %11, 45
    br i1 %12, label %lbl3, label %lbl6
    lbl3:
    store i32 2, ptr %5, align 4
    br label %lbl14
    lbl6:
    %13 = load i32, ptr %1
    %14 = icmp eq i32 %13, 2
    br i1 %14, label %lbl8, label %lbl11
    lbl8:
    store i32 4, ptr %5, align 4
    br label %lbl14
    lbl11:
    store i32 0, ptr %5, align 4
    br label %lbl14
    lbl14:
    store i32 0, ptr %1, align 4
    br label %lbl17
    lbl17:
    %15 = load i32, ptr %1
    %16 = add nsw i32 %15, 1
    store i32 %16, ptr %1, align 4
    %17 = load i32, ptr %1
    %18 = icmp eq i32 %17, 100
    br i1 %18, label %lbl20, label %lbl23
    lbl20:
    br label %lbl25
    br label %lbl23
    lbl23:
    br label %lbl17
    lbl25:
    ret i32 0
}
