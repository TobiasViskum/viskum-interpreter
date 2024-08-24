; ModuleID = 'gen.c'
source_filename = "gen.c"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx15.0.0"

%struct.Vector = type { i32, i32, ptr }

@.str = private unnamed_addr constant [9 x i8] c"Len: %d\0A\00", align 1
@.str.1 = private unnamed_addr constant [14 x i8] c"Capacity: %d\0A\00", align 1
@.str.2 = private unnamed_addr constant [8 x i8] c"Items:\0A\00", align 1
@.str.3 = private unnamed_addr constant [14 x i8] c"vec[%d] = %d\0A\00", align 1
@.str.4 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.5 = private unnamed_addr constant [2 x i8] c" \00", align 1
@.str.6 = private unnamed_addr constant [7 x i8] c"World!\00", align 1
@__const.main.hello2 = private unnamed_addr constant [10 x i8] c"scubidii\00\00", align 1
@.str.7 = private unnamed_addr constant [7 x i8] c"%s%s%s\00", align 1
@.str.8 = private unnamed_addr constant [27 x i8] c"Concatenated String: %d%s\0A\00", align 1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define zeroext i1 @hello() #0 {
  ret i1 true
}

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define ptr @new_vec() #0 {
  %1 = alloca ptr, align 8
  %2 = alloca ptr, align 8
  %3 = call ptr @malloc(i64 noundef 16) #8
  store ptr %3, ptr %1, align 8
  %4 = load ptr, ptr %1, align 8
  %5 = icmp eq ptr %4, null
  br i1 %5, label %6, label %7

6:                                                ; preds = %0
  call void @exit(i32 noundef 1) #9
  unreachable

7:                                                ; preds = %0
  %8 = load ptr, ptr %1, align 8
  %9 = getelementptr inbounds %struct.Vector, ptr %8, i32 0, i32 0
  store i32 0, ptr %9, align 8
  %10 = load ptr, ptr %1, align 8
  %11 = getelementptr inbounds %struct.Vector, ptr %10, i32 0, i32 1
  store i32 4, ptr %11, align 4
  %12 = load ptr, ptr %1, align 8
  %13 = getelementptr inbounds %struct.Vector, ptr %12, i32 0, i32 1
  %14 = load i32, ptr %13, align 4
  %15 = sext i32 %14 to i64
  %16 = mul i64 %15, 4
  %17 = call ptr @malloc(i64 noundef %16) #8
  store ptr %17, ptr %2, align 8
  %18 = load ptr, ptr %2, align 8
  %19 = icmp eq ptr %18, null
  br i1 %19, label %20, label %21

20:                                               ; preds = %7
  call void @exit(i32 noundef 1) #9
  unreachable

21:                                               ; preds = %7
  %22 = load ptr, ptr %2, align 8
  %23 = load ptr, ptr %1, align 8
  %24 = getelementptr inbounds %struct.Vector, ptr %23, i32 0, i32 2
  store ptr %22, ptr %24, align 8
  %25 = load ptr, ptr %1, align 8
  ret ptr %25
}

; Function Attrs: allocsize(0)
declare ptr @malloc(i64 noundef) #1

; Function Attrs: noreturn
declare void @exit(i32 noundef) #2

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @push_item(ptr noundef %0, i32 noundef %1) #0 {
  %3 = alloca ptr, align 8
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  %6 = alloca ptr, align 8
  %7 = alloca i32, align 4
  %8 = alloca i32, align 4
  %9 = alloca i32, align 4
  store ptr %0, ptr %3, align 8
  store i32 %1, ptr %4, align 4
  store i32 888, ptr %5, align 4
  %10 = load ptr, ptr %3, align 8
  %11 = getelementptr inbounds %struct.Vector, ptr %10, i32 0, i32 0
  %12 = load i32, ptr %11, align 8
  %13 = load ptr, ptr %3, align 8
  %14 = getelementptr inbounds %struct.Vector, ptr %13, i32 0, i32 1
  %15 = load i32, ptr %14, align 4
  %16 = icmp eq i32 %12, %15
  br i1 %16, label %17, label %40

17:                                               ; preds = %2
  %18 = load ptr, ptr %3, align 8
  %19 = getelementptr inbounds %struct.Vector, ptr %18, i32 0, i32 1
  %20 = load i32, ptr %19, align 4
  %21 = mul nsw i32 %20, 2
  %22 = load ptr, ptr %3, align 8
  %23 = getelementptr inbounds %struct.Vector, ptr %22, i32 0, i32 1
  store i32 %21, ptr %23, align 4
  %24 = load ptr, ptr %3, align 8
  %25 = getelementptr inbounds %struct.Vector, ptr %24, i32 0, i32 2
  %26 = load ptr, ptr %25, align 8
  %27 = load ptr, ptr %3, align 8
  %28 = getelementptr inbounds %struct.Vector, ptr %27, i32 0, i32 1
  %29 = load i32, ptr %28, align 4
  %30 = sext i32 %29 to i64
  %31 = mul i64 %30, 4
  %32 = call ptr @realloc(ptr noundef %26, i64 noundef %31) #10
  store ptr %32, ptr %6, align 8
  %33 = load ptr, ptr %6, align 8
  %34 = icmp eq ptr %33, null
  br i1 %34, label %35, label %36

35:                                               ; preds = %17
  call void @exit(i32 noundef 1) #9
  unreachable

36:                                               ; preds = %17
  %37 = load ptr, ptr %6, align 8
  %38 = load ptr, ptr %3, align 8
  %39 = getelementptr inbounds %struct.Vector, ptr %38, i32 0, i32 2
  store ptr %37, ptr %39, align 8
  br label %40

40:                                               ; preds = %36, %2
  store i32 999, ptr %7, align 4
  %41 = load i32, ptr %4, align 4
  %42 = load ptr, ptr %3, align 8
  %43 = getelementptr inbounds %struct.Vector, ptr %42, i32 0, i32 2
  %44 = load ptr, ptr %43, align 8
  %45 = load ptr, ptr %3, align 8
  %46 = getelementptr inbounds %struct.Vector, ptr %45, i32 0, i32 0
  %47 = load i32, ptr %46, align 8
  %48 = sext i32 %47 to i64
  %49 = getelementptr inbounds i32, ptr %44, i64 %48
  store i32 %41, ptr %49, align 4
  store i32 777, ptr %8, align 4
  %50 = load ptr, ptr %3, align 8
  %51 = getelementptr inbounds %struct.Vector, ptr %50, i32 0, i32 0
  %52 = load i32, ptr %51, align 8
  %53 = add nsw i32 %52, 1
  store i32 %53, ptr %51, align 8
  store i32 666, ptr %9, align 4
  ret i32 0
}

; Function Attrs: allocsize(1)
declare ptr @realloc(ptr noundef, i64 noundef) #3

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define void @dissasemble_vec(ptr noundef %0) #0 {
  %2 = alloca ptr, align 8
  %3 = alloca i32, align 4
  store ptr %0, ptr %2, align 8
  %4 = load ptr, ptr %2, align 8
  %5 = getelementptr inbounds %struct.Vector, ptr %4, i32 0, i32 0
  %6 = load i32, ptr %5, align 8
  %7 = call i32 (ptr, ...) @printf(ptr noundef @.str, i32 noundef %6)
  %8 = load ptr, ptr %2, align 8
  %9 = getelementptr inbounds %struct.Vector, ptr %8, i32 0, i32 1
  %10 = load i32, ptr %9, align 4
  %11 = call i32 (ptr, ...) @printf(ptr noundef @.str.1, i32 noundef %10)
  %12 = call i32 (ptr, ...) @printf(ptr noundef @.str.2)
  store i32 0, ptr %3, align 4
  br label %13

13:                                               ; preds = %29, %1
  %14 = load i32, ptr %3, align 4
  %15 = load ptr, ptr %2, align 8
  %16 = getelementptr inbounds %struct.Vector, ptr %15, i32 0, i32 0
  %17 = load i32, ptr %16, align 8
  %18 = icmp slt i32 %14, %17
  br i1 %18, label %19, label %32

19:                                               ; preds = %13
  %20 = load i32, ptr %3, align 4
  %21 = load ptr, ptr %2, align 8
  %22 = getelementptr inbounds %struct.Vector, ptr %21, i32 0, i32 2
  %23 = load ptr, ptr %22, align 8
  %24 = load i32, ptr %3, align 4
  %25 = sext i32 %24 to i64
  %26 = getelementptr inbounds i32, ptr %23, i64 %25
  %27 = load i32, ptr %26, align 4
  %28 = call i32 (ptr, ...) @printf(ptr noundef @.str.3, i32 noundef %20, i32 noundef %27)
  br label %29

29:                                               ; preds = %19
  %30 = load i32, ptr %3, align 4
  %31 = add nsw i32 %30, 1
  store i32 %31, ptr %3, align 4
  br label %13, !llvm.loop !5

32:                                               ; preds = %13
  ret void
}

declare i32 @printf(ptr noundef, ...) #4

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca ptr, align 8
  %3 = alloca [50 x i8], align 1
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca [10 x i8], align 1
  %8 = alloca i32, align 4
  %9 = alloca ptr, align 8
  store i32 0, ptr %1, align 4
  %10 = call ptr @new_vec()
  store ptr %10, ptr %2, align 8
  %11 = load ptr, ptr %2, align 8
  %12 = call i32 @push_item(ptr noundef %11, i32 noundef 2)
  %13 = load ptr, ptr %2, align 8
  %14 = call i32 @push_item(ptr noundef %13, i32 noundef 6)
  %15 = load ptr, ptr %2, align 8
  %16 = call i32 @push_item(ptr noundef %15, i32 noundef 343)
  %17 = load ptr, ptr %2, align 8
  %18 = call i32 @push_item(ptr noundef %17, i32 noundef 39)
  %19 = load ptr, ptr %2, align 8
  %20 = call i32 @push_item(ptr noundef %19, i32 noundef 90)
  store ptr @.str.4, ptr %4, align 8
  store ptr @.str.5, ptr %5, align 8
  store ptr @.str.6, ptr %6, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %7, ptr align 1 @__const.main.hello2, i64 10, i1 false)
  %21 = getelementptr inbounds [10 x i8], ptr %7, i64 0, i64 2
  store i8 88, ptr %21, align 1
  %22 = getelementptr inbounds [50 x i8], ptr %3, i64 0, i64 0
  %23 = load ptr, ptr %4, align 8
  %24 = load ptr, ptr %5, align 8
  %25 = load ptr, ptr %6, align 8
  %26 = call i32 (ptr, i32, i64, ptr, ...) @__sprintf_chk(ptr noundef %22, i32 noundef 0, i64 noundef 50, ptr noundef @.str.7, ptr noundef %23, ptr noundef %24, ptr noundef %25)
  %27 = getelementptr inbounds [50 x i8], ptr %3, i64 0, i64 0
  %28 = call i64 @strlen(ptr noundef %27)
  %29 = trunc i64 %28 to i32
  store i32 %29, ptr %8, align 4
  %30 = load i32, ptr %8, align 4
  %31 = add nsw i32 %30, 1
  %32 = sext i32 %31 to i64
  %33 = mul i64 %32, 1
  %34 = call ptr @malloc(i64 noundef %33) #8
  store ptr %34, ptr %9, align 8
  %35 = load ptr, ptr %9, align 8
  %36 = getelementptr inbounds [50 x i8], ptr %3, i64 0, i64 0
  %37 = load ptr, ptr %9, align 8
  %38 = call i64 @llvm.objectsize.i64.p0(ptr %37, i1 false, i1 true, i1 false)
  %39 = call ptr @__strcpy_chk(ptr noundef %35, ptr noundef %36, i64 noundef %38) #11
  %40 = load i32, ptr %8, align 4
  %41 = load ptr, ptr %9, align 8
  %42 = call i32 (ptr, ...) @printf(ptr noundef @.str.8, i32 noundef %40, ptr noundef %41)
  %43 = load ptr, ptr %2, align 8
  call void @dissasemble_vec(ptr noundef %43)
  ret i32 0
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #5

declare i32 @__sprintf_chk(ptr noundef, i32 noundef, i64 noundef, ptr noundef, ...) #4

declare i64 @strlen(ptr noundef) #4

; Function Attrs: nounwind
declare ptr @__strcpy_chk(ptr noundef, ptr noundef, i64 noundef) #6

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.objectsize.i64.p0(ptr, i1 immarg, i1 immarg, i1 immarg) #7

attributes #0 = { noinline nounwind optnone ssp uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #1 = { allocsize(0) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #2 = { noreturn "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #3 = { allocsize(1) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #4 = { "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #5 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { nounwind "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #7 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #8 = { allocsize(0) }
attributes #9 = { noreturn }
attributes #10 = { allocsize(1) }
attributes #11 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 1}
!3 = !{i32 7, !"frame-pointer", i32 1}
!4 = !{!"Homebrew clang version 18.1.6"}
!5 = distinct !{!5, !6}
!6 = !{!"llvm.loop.mustprogress"}
