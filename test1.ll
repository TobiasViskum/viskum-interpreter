@.str.0 = private unnamed_addr constant [12 x i8] c"Hello world\00", align 1
@.str.1 = private unnamed_addr constant [18 x i8] c"My name is Tobias\00", align 1


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
    %15 = alloca ptr, align 4
    %16 = add nsw i32 1, 2
    %17 = add nsw i32 2, 8
    %18 = mul nsw i32 %17, 3
    %19 = sub nsw i32 %16, %18
    store i32 %19, ptr %10, align 4
    %20 = load i32, ptr %10
    %21 = add nsw i32 %20, 2
    store i32 %21, ptr %11, align 4
    store i32 0, ptr %12, align 4
    %22 = load i32, ptr %10
    %23 = icmp eq i32 %22, 45
    br i1 %23, label %lbl3, label %lbl6
    lbl3:
    store i32 2, ptr %12, align 4
    br label %lbl14
    lbl6:
    %24 = load i32, ptr %10
    %25 = icmp eq i32 %24, 2
    br i1 %25, label %lbl8, label %lbl11
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
    %26 = load i32, ptr %10
    %27 = add nsw i32 %26, 1
    store i32 %27, ptr %10, align 4
    %28 = load i32, ptr %10
    %29 = icmp eq i32 %28, 100
    br i1 %29, label %lbl20, label %lbl23
    lbl20:
    br label %lbl25
    br label %lbl23
    lbl23:
    br label %lbl17
    lbl25:
    %31 = call i32 @sayHello1()
    %32 = add nsw i32 %31, 20
    %34 = call i32 @sayHello1()
    %35 = add nsw i32 %32, %34
    store i32 %35, ptr %13, align 4
    store ptr @.str.0, ptr %15, align 4
    store ptr @.str.1, ptr %15, align 4
    store i32 0, ptr %10, align 4
    ret i32 0
}
