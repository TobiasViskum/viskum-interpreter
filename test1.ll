@.str.0 = private unnamed_addr constant [12 x i8] c"Hello world\00", align 1

define i32 @hi1() {
    ret i32 2
}

define i32 @sayHello1() {
    %1 = alloca i32, align 4
    %2 = alloca i32, align 4
    %3 = add nsw i32 2, 9
    store i32 %3, ptr %1, align 4
    %4 = load i32, ptr %1
    %5 = add nsw i32 %4, 1
    store i32 %5, ptr %2, align 4
    %6 = load i32, ptr %2
    %8 = call i32 @hi1()
    %9 = add nsw i32 %6, %8
    ret i32 %9
}

define i32 @main() {
    %10 = alloca i32, align 4
    %11 = alloca i32, align 4
    %12 = alloca i32, align 4
    %13 = alloca i32, align 4
    %14 = alloca ptr, align 4
    %15 = add nsw i32 1, 2
    %16 = add nsw i32 2, 8
    %17 = mul nsw i32 %16, 3
    %18 = sub nsw i32 %15, %17
    store i32 %18, ptr %10, align 4
    %19 = load i32, ptr %10
    %20 = add nsw i32 %19, 2
    store i32 %20, ptr %11, align 4
    store i32 0, ptr %12, align 4
    %21 = load i32, ptr %10
    %22 = icmp eq i32 %21, 45
    br i1 %22, label %lbl3, label %lbl6
    lbl3:
    store i32 2, ptr %12, align 4
    br label %lbl14
    lbl6:
    %23 = load i32, ptr %10
    %24 = icmp eq i32 %23, 2
    br i1 %24, label %lbl8, label %lbl11
    lbl8:
    store i32 4, ptr %12, align 4
    br label %lbl14
    lbl11:
    store i32 0, ptr %12, align 4
    br label %lbl14
    lbl14:
    store i32 0, ptr %10, align 4
    br label %lbl17
    lbl17:
    %25 = load i32, ptr %10
    %26 = add nsw i32 %25, 1
    store i32 %26, ptr %10, align 4
    %27 = load i32, ptr %10
    %28 = icmp eq i32 %27, 100
    br i1 %28, label %lbl20, label %lbl23
    lbl20:
    br label %lbl25
    br label %lbl23
    lbl23:
    br label %lbl17
    lbl25:
    %30 = call i32 @sayHello1()
    %31 = add nsw i32 %30, 20
    %33 = call i32 @sayHello1()
    %34 = add nsw i32 %31, %33
    store i32 %34, ptr %13, align 4
    store ptr @.str.0, ptr %14, align 4
    store i32 0, ptr %10, align 4
    ret i32 0
}
