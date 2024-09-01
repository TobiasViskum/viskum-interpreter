
@.str.25 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.17 = private unnamed_addr constant [7 x i8] c"Tobias\00", align 1
@.str.9 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.16 = private unnamed_addr constant [25 x i8] c"%s %d %s %s %s %d %s %d\0A\00", align 1
@.str.11 = private unnamed_addr constant [36 x i8] c"My name is Tobias. This is my name!\00", align 1
@.str.5 = private unnamed_addr constant [7 x i8] c"%s %d\0A\00", align 1
@.str.10 = private unnamed_addr constant [12 x i8] c"Hello world\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@.str.0 = private unnamed_addr constant [7 x i8] c"Case 1\00", align 1
@.str.19 = private unnamed_addr constant [21 x i8] c"has the same name as\00", align 1
@.str.22 = private unnamed_addr constant [1 x i8] c"\00", align 1
@.str.6 = private unnamed_addr constant [15 x i8] c"array at 5 is:\00", align 1
@.str.14 = private unnamed_addr constant [11 x i8] c"Result is:\00", align 1
@.str.7 = private unnamed_addr constant [13 x i8] c"A is now 100\00", align 1
@.str.8 = private unnamed_addr constant [6 x i8] c"A is:\00", align 1
@.str.13 = private unnamed_addr constant [11 x i8] c"years old.\00", align 1
@.str.18 = private unnamed_addr constant [5 x i8] c"Does\00", align 1
@.str.21 = private unnamed_addr constant [19 x i8] c"%s %s %s %s %s %d\0A\00", align 1
@.str.24 = private unnamed_addr constant [10 x i8] c"%s %s %d\0A\00", align 1
@.str.23 = private unnamed_addr constant [24 x i8] c"Uninitialized variable:\00", align 1
@.str.2 = private unnamed_addr constant [7 x i8] c"Case 2\00", align 1
@.str.3 = private unnamed_addr constant [7 x i8] c"Case 3\00", align 1
@.str.15 = private unnamed_addr constant [12 x i8] c"isEqual is:\00", align 1
@.str.4 = private unnamed_addr constant [15 x i8] c"array at 4 is:\00", align 1
@.str.12 = private unnamed_addr constant [24 x i8] c"My name is Tobias. I am\00", align 1
@.str.20 = private unnamed_addr constant [2 x i8] c":\00", align 1


define i32 @hi1() {
    %v1 = mul nsw i32 9, 8
    %v2 = add nsw i32 2, %v1
    ret i32 %v2
}

define i32 @sayHello1() {
    %v3 = alloca i32, align 4
    %v4 = alloca i32, align 4
    %v5 = add nsw i32 2, 9
    store i32 %v5, ptr %v3, align 4
    %v6 = load i32, ptr %v3
    %v7 = add nsw i32 %v6, 1
    store i32 %v7, ptr %v4, align 4
    %v8 = load i32, ptr %v4
    ret i32 %v8
}

define i32 @main() {
    %v9 = alloca i32, align 4
    %v10 = alloca i32, align 4
    %v11 = alloca i32, align 4
    %v12 = alloca i1, align 1
    %v13 = alloca [5 x i32], align 4
    %v14 = alloca i32, align 4
    %v15 = alloca ptr, align 8
    %v16 = alloca ptr, align 8
    %v17 = alloca ptr, align 8
    %v18 = alloca i1, align 1
    %v19 = alloca ptr, align 8
    %v20 = alloca ptr, align 8
    %v21 = alloca i1, align 1
    %v22 = alloca ptr, align 8
    %v23 = add nsw i32 1, 2
    %v24 = add nsw i32 2, 8
    %v25 = mul nsw i32 %v24, 3
    %v26 = sub nsw i32 %v23, %v25
    %v27 = add nsw i32 %v26, 72
    store i32 %v27, ptr %v9, align 4
    %v28 = load i32, ptr %v9
    %v29 = add nsw i32 %v28, 2
    store i32 %v29, ptr %v10, align 4
    store i32 0, ptr %v11, align 4
    %v30 = icmp eq i32 2, 2
    store i1 %v30, ptr %v12, align 4
    %v31 = load i1, ptr %v12
    br i1 %v31, label %lbl3, label %lbl6
    lbl3:
    store i32 2, ptr %v11, align 4
    %v32 = call i32 (ptr, ...) @printf(ptr noundef @.str.1, ptr noundef @.str.0)
    br label %lbl14
    lbl6:
    %v33 = load i32, ptr %v9
    %v34 = icmp eq i32 %v33, 2
    br i1 %v34, label %lbl8, label %lbl11
    lbl8:
    store i32 4, ptr %v11, align 4
    %v35 = call i32 (ptr, ...) @printf(ptr noundef @.str.1, ptr noundef @.str.2)
    br label %lbl14
    lbl11:
    store i32 0, ptr %v11, align 4
    %v36 = call i32 (ptr, ...) @printf(ptr noundef @.str.1, ptr noundef @.str.3)
    br label %lbl14
    lbl14:
    %v37 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 4
    store i32 3, ptr %v37, align 4
    %v38 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 3
    store i32 3, ptr %v38, align 4
    %v39 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 2
    store i32 2, ptr %v39, align 4
    %v40 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 1
    store i32 1, ptr %v40, align 4
    %v41 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 0
    store i32 0, ptr %v41, align 4
    %v42 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 4
    %v43 = load i32, ptr %v42
    %v44 = call i32 (ptr, ...) @printf(ptr noundef @.str.5, ptr noundef @.str.4, i32 noundef %v43)
    %v45 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 4
    store i32 4, ptr %v45, align 4
    %v46 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 4
    %v47 = load i32, ptr %v46
    %v48 = call i32 (ptr, ...) @printf(ptr noundef @.str.5, ptr noundef @.str.4, i32 noundef %v47)
    %v49 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 5
    store i32 2, ptr %v49, align 4
    %v50 = getelementptr inbounds [5 x i32], ptr %v13, i64 0, i64 5
    %v51 = load i32, ptr %v50
    %v52 = call i32 (ptr, ...) @printf(ptr noundef @.str.5, ptr noundef @.str.6, i32 noundef %v51)
    br label %lbl17
    lbl17:
    %v53 = load i32, ptr %v9
    %v54 = add nsw i32 %v53, 1
    store i32 %v54, ptr %v9, align 4
    %v55 = load i32, ptr %v9
    %v56 = icmp eq i32 %v55, 100
    br i1 %v56, label %lbl20, label %lbl24
    lbl20:
    %v57 = call i32 (ptr, ...) @printf(ptr noundef @.str.1, ptr noundef @.str.7)
    br label %lbl30
    br label %lbl28
    lbl24:
    %v58 = load i32, ptr %v9
    %v59 = call i32 (ptr, ...) @printf(ptr noundef @.str.5, ptr noundef @.str.8, i32 noundef %v58)
    br label %lbl17
    br label %lbl28
    lbl28:
    br label %lbl17
    lbl30:
    %v60 = call i32 @sayHello1()
    %v61 = add nsw i32 %v60, 20
    %v62 = call i32 @sayHello1()
    %v63 = add nsw i32 %v61, %v62
    store i32 %v63, ptr %v14, align 4
    %v64 = load i32, ptr %v14
    %v65 = call i32 (ptr, ...) @printf(ptr noundef @.str.9, i32 noundef %v64)
    store ptr @.str.10, ptr %v17, align 4
    store ptr @.str.10, ptr %v17, align 4
    store ptr @.str.11, ptr %v17, align 4
    %v66 = load i32, ptr %v9
    %v67 = icmp eq i32 100, %v66
    store i1 %v67, ptr %v18, align 4
    %v68 = load ptr, ptr %v17
    %v69 = load i32, ptr %v9
    %v70 = load i1, ptr %v18
    %v71 = call i32 (ptr, ...) @printf(ptr noundef @.str.16, ptr noundef @.str.12, i32 noundef 18, ptr noundef @.str.13, ptr noundef %v68, ptr noundef @.str.14, i32 noundef %v69, ptr noundef @.str.15, i1 noundef %v70)
    store ptr @.str.17, ptr %v19, align 4
    store ptr @.str.17, ptr %v20, align 4
    %v72 = load ptr, ptr %v19
    %v73 = load ptr, ptr %v20
    %v74 = call i32 @strcmp(ptr noundef %v72, ptr noundef %v73)
    %v75 = icmp eq i32 %v74, 0
    store i1 %v75, ptr %v21, align 4
    %v76 = load ptr, ptr %v19
    %v77 = load ptr, ptr %v20
    %v78 = load i1, ptr %v21
    %v79 = call i32 (ptr, ...) @printf(ptr noundef @.str.21, ptr noundef @.str.18, ptr noundef %v76, ptr noundef @.str.19, ptr noundef %v77, ptr noundef @.str.20, i1 noundef %v78)
    store ptr @.str.22, ptr %v22, align 4
    %v80 = load ptr, ptr %v22
    %v81 = call i32 (ptr, ...) @printf(ptr noundef @.str.24, ptr noundef @.str.23, ptr noundef %v80, i32 noundef 2)
    store ptr @.str.25, ptr %v22, align 4
    %v82 = load ptr, ptr %v22
    %v83 = call i32 (ptr, ...) @printf(ptr noundef @.str.1, ptr noundef %v82)
    store i32 0, ptr %v9, align 4
    ret i32 0
}

declare i32 @printf(ptr noundef, ...)
declare i32 @strcmp(ptr noundef, ptr noundef)