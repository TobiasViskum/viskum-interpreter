@.str.0 = private unnamed_addr constant [12 x i8] c"Hello world\00", align 1
@.str.1 = private unnamed_addr constant [18 x i8] c"My name is Tobias\00", align 1
@.str.2 = private unnamed_addr constant [15 x i8] c"Hello again :)\00", align 1


define i32 @hi1() {
    %1 = mul nsw i32 9, 8
    %2 = add nsw i32 2, %1
    ret i32 %2
}

define i32 @sayHello1() {
    %3 = alloca i32, align 4
    %4 = alloca i32, align 4
    %5 = add nsw i32 2, 9
    store i32 %5, ptr %3, align 4
    %6 = load i32, ptr %3
    %7 = add nsw i32 %6, 1
    store i32 %7, ptr %4, align 4
    %8 = load i32, ptr %4
    %13 = call i32 @hi1()
    %14 = add nsw i32 %8, %13
    ret i32 %14
}

define i32 @main() {
    %15 = alloca i32, align 4
    %16 = alloca i32, align 4
    %17 = alloca i32, align 4
    %18 = alloca i32, align 4
    %19 = alloca ptr, align 4
    %20 = alloca ptr, align 4
    %21 = alloca ptr, align 4
    %22 = add nsw i32 1, 2
    %23 = add nsw i32 2, 8
    %24 = mul nsw i32 %23, 3
    %25 = sub nsw i32 %22, %24
    store i32 %25, ptr %15, align 4
    %26 = load i32, ptr %15
    %27 = add nsw i32 %26, 2
    store i32 %27, ptr %16, align 4
    store i32 0, ptr %17, align 4
    %28 = load i32, ptr %15
    %29 = icmp eq i32 %28, 45
    br i1 %29, label %lbl3, label %lbl6
    lbl3:
    store i32 2, ptr %17, align 4
    br label %lbl14
    lbl6:
    %30 = load i32, ptr %15
    %31 = icmp eq i32 %30, 2
    br i1 %31, label %lbl8, label %lbl11
    lbl8:
    store i32 4, ptr %17, align 4
    br label %lbl14
    lbl11:
    store i32 0, ptr %17, align 4
    br label %lbl14
    lbl14:
    store i32 0, ptr %15, align 4
    br label %lbl17
    lbl17:
    %32 = load i32, ptr %15
    %33 = add nsw i32 %32, 1
    store i32 %33, ptr %15, align 4
    %34 = load i32, ptr %15
    %35 = icmp eq i32 %34, 100
    br i1 %35, label %lbl20, label %lbl23
    lbl20:
    br label %lbl28
    br label %lbl26
    lbl23:
    br label %lbl17
    br label %lbl26
    lbl26:
    br label %lbl17
    lbl28:
    %40 = call i32 @sayHello1()
    %41 = add nsw i32 %40, 20
    %46 = call i32 @sayHello1()
    %47 = add nsw i32 %41, %46
    store i32 %47, ptr %18, align 4
    store ptr @.str.0, ptr %21, align 4
    store ptr @.str.1, ptr %21, align 4
    store ptr @.str.2, ptr %21, align 4
    store i32 0, ptr %15, align 4
    ret i32 0
}
