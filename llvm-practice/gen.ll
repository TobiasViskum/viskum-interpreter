; ModuleID = 'gen.c'
source_filename = "gen.c"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx15.0.0"

@.str = private unnamed_addr constant [12 x i8] c"U runsdfdsf\00", align 1
@__const.main.myname = private unnamed_addr constant [7 x i8] c"tobias\00", align 1
@.str.1 = private unnamed_addr constant [12 x i8] c"Hello word\0A\00", align 1
@__const.main.name1 = private unnamed_addr constant [7 x i8] c"Tobias\00", align 1
@__const.main.name2 = private unnamed_addr constant [8 x i8] c"Andreas\00", align 1
@.str.2 = private unnamed_addr constant [11 x i8] c"Is eq: %d\0A\00", align 1
@.str.3 = private unnamed_addr constant [3 x i8] c"%s\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.5 = private unnamed_addr constant [1 x i8] zeroinitializer, align 1
@.str.6 = private unnamed_addr constant [7 x i8] c"World!\00", align 1
@.str.7 = private unnamed_addr constant [7 x i8] c"%s%s%s\00", align 1
@.str.8 = private unnamed_addr constant [27 x i8] c"Concatenated String: %d%s\0A\00", align 1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @add(i32 noundef %0, i32 noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 %0, ptr %3, align 4
  store i32 %1, ptr %4, align 4
  %5 = load i32, ptr %3, align 4
  %6 = load i32, ptr %4, align 4
  %7 = add nsw i32 %5, %6
  ret i32 %7
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca [50 x i8], align 1
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca [5 x i32], align 4
  %6 = alloca [5 x i32], align 4
  %7 = alloca [7 x i8], align 1
  %8 = alloca [7 x i8], align 1
  %9 = alloca [8 x i8], align 1
  %10 = alloca i32, align 4
  %11 = alloca ptr, align 8
  %12 = alloca ptr, align 8
  %13 = alloca ptr, align 8
  %14 = alloca ptr, align 8
  %15 = alloca i32, align 4
  %16 = alloca ptr, align 8
  store i32 0, ptr %1, align 4
  store i32 1, ptr %3, align 4
  store i32 9, ptr %4, align 4
  %17 = getelementptr inbounds [5 x i32], ptr %5, i64 0, i64 0
  %18 = load i32, ptr %3, align 4
  %19 = load i32, ptr %4, align 4
  %20 = add nsw i32 %18, %19
  store i32 %20, ptr %17, align 4
  %21 = getelementptr inbounds i32, ptr %17, i64 1
  store i32 20, ptr %21, align 4
  %22 = getelementptr inbounds i32, ptr %21, i64 1
  store i32 30, ptr %22, align 4
  %23 = getelementptr inbounds i32, ptr %22, i64 1
  store i32 40, ptr %23, align 4
  %24 = getelementptr inbounds i32, ptr %23, i64 1
  store i32 50, ptr %24, align 4
  %25 = getelementptr inbounds [5 x i32], ptr %6, i64 0, i64 0
  store i32 10, ptr %25, align 4
  %26 = getelementptr inbounds [5 x i32], ptr %6, i64 0, i64 1
  store i32 20, ptr %26, align 4
  %27 = getelementptr inbounds [5 x i32], ptr %6, i64 0, i64 2
  store i32 30, ptr %27, align 4
  %28 = getelementptr inbounds [5 x i32], ptr %6, i64 0, i64 3
  store i32 40, ptr %28, align 4
  %29 = getelementptr inbounds [5 x i32], ptr %6, i64 0, i64 4
  store i32 40, ptr %29, align 4
  %30 = call i32 (ptr, ...) @printf(ptr noundef @.str)
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %7, ptr align 1 @__const.main.myname, i64 7, i1 false)
  %31 = getelementptr inbounds [7 x i8], ptr %7, i64 0, i64 0
  store i8 84, ptr %31, align 1
  %32 = getelementptr inbounds [7 x i8], ptr %7, i64 0, i64 1
  store i8 79, ptr %32, align 1
  %33 = getelementptr inbounds [7 x i8], ptr %7, i64 0, i64 2
  store i8 66, ptr %33, align 1
  %34 = call i32 (ptr, ...) @printf(ptr noundef @.str.1)
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %8, ptr align 1 @__const.main.name1, i64 7, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %9, ptr align 1 @__const.main.name2, i64 8, i1 false)
  %35 = getelementptr inbounds [8 x i8], ptr %9, i64 0, i64 0
  %36 = getelementptr inbounds [7 x i8], ptr %8, i64 0, i64 0
  %37 = call i32 @strcmp(ptr noundef %35, ptr noundef %36)
  store i32 %37, ptr %10, align 4
  %38 = load i32, ptr %10, align 4
  %39 = call i32 (ptr, ...) @printf(ptr noundef @.str.2, i32 noundef %38)
  %40 = getelementptr inbounds [7 x i8], ptr %7, i64 0, i64 0
  %41 = call i32 (ptr, ...) @printf(ptr noundef @.str.3, ptr noundef %40)
  store ptr @.str.4, ptr %11, align 8
  store ptr @.str.4, ptr %12, align 8
  store ptr @.str.5, ptr %13, align 8
  store ptr @.str.6, ptr %14, align 8
  %42 = getelementptr inbounds [50 x i8], ptr %2, i64 0, i64 0
  %43 = load ptr, ptr %11, align 8
  %44 = load ptr, ptr %13, align 8
  %45 = load ptr, ptr %14, align 8
  %46 = call i32 (ptr, i32, i64, ptr, ...) @__sprintf_chk(ptr noundef %42, i32 noundef 0, i64 noundef 50, ptr noundef @.str.7, ptr noundef %43, ptr noundef %44, ptr noundef %45)
  %47 = getelementptr inbounds [50 x i8], ptr %2, i64 0, i64 0
  %48 = call i64 @strlen(ptr noundef %47)
  %49 = trunc i64 %48 to i32
  store i32 %49, ptr %15, align 4
  %50 = load i32, ptr %15, align 4
  %51 = add nsw i32 %50, 1
  %52 = sext i32 %51 to i64
  %53 = mul i64 %52, 1
  %54 = call ptr @malloc(i64 noundef %53) #6
  store ptr %54, ptr %16, align 8
  %55 = load ptr, ptr %16, align 8
  %56 = getelementptr inbounds [50 x i8], ptr %2, i64 0, i64 0
  %57 = load ptr, ptr %16, align 8
  %58 = call i64 @llvm.objectsize.i64.p0(ptr %57, i1 false, i1 true, i1 false)
  %59 = call ptr @__strcpy_chk(ptr noundef %55, ptr noundef %56, i64 noundef %58) #7
  %60 = load i32, ptr %15, align 4
  %61 = load ptr, ptr %16, align 8
  %62 = call i32 (ptr, ...) @printf(ptr noundef @.str.8, i32 noundef %60, ptr noundef %61)
  ret i32 0
}

declare i32 @printf(ptr noundef, ...) #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2

declare i32 @strcmp(ptr noundef, ptr noundef) #1

declare i32 @__sprintf_chk(ptr noundef, i32 noundef, i64 noundef, ptr noundef, ...) #1

declare i64 @strlen(ptr noundef) #1

; Function Attrs: allocsize(0)
declare ptr @malloc(i64 noundef) #3

; Function Attrs: nounwind
declare ptr @__strcpy_chk(ptr noundef, ptr noundef, i64 noundef) #4

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.objectsize.i64.p0(ptr, i1 immarg, i1 immarg, i1 immarg) #5

attributes #0 = { noinline nounwind optnone ssp uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #1 = { "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { allocsize(0) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #4 = { nounwind "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #5 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #6 = { allocsize(0) }
attributes #7 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 1}
!4 = !{!"Homebrew clang version 18.1.6"}
