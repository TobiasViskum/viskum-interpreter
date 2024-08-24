; ModuleID = 'gen.c'
source_filename = "gen.c"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx15.0.0"

@__const.main.myname = private unnamed_addr constant [7 x i8] c"tobias\00", align 1
@.str = private unnamed_addr constant [3 x i8] c"%s\00", align 1
@.str.1 = private unnamed_addr constant [6 x i8] c"Hello\00", align 1
@.str.2 = private unnamed_addr constant [1 x i8] zeroinitializer, align 1
@.str.3 = private unnamed_addr constant [7 x i8] c"World!\00", align 1
@.str.4 = private unnamed_addr constant [7 x i8] c"%s%s%s\00", align 1
@.str.5 = private unnamed_addr constant [27 x i8] c"Concatenated String: %d%s\0A\00", align 1

; Function Attrs: noinline nounwind optnone ssp uwtable(sync)
define i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca [50 x i8], align 1
  %3 = alloca [7 x i8], align 1
  %4 = alloca ptr, align 8
  %5 = alloca ptr, align 8
  %6 = alloca ptr, align 8
  %7 = alloca ptr, align 8
  %8 = alloca i32, align 4
  %9 = alloca ptr, align 8
  store i32 0, ptr %1, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %3, ptr align 1 @__const.main.myname, i64 7, i1 false)
  %10 = getelementptr inbounds [7 x i8], ptr %3, i64 0, i64 0
  store i8 84, ptr %10, align 1
  %11 = getelementptr inbounds [7 x i8], ptr %3, i64 0, i64 1
  store i8 79, ptr %11, align 1
  %12 = getelementptr inbounds [7 x i8], ptr %3, i64 0, i64 2
  store i8 66, ptr %12, align 1
  %13 = getelementptr inbounds [7 x i8], ptr %3, i64 0, i64 0
  %14 = call i32 (ptr, ...) @printf(ptr noundef @.str, ptr noundef %13)
  store ptr @.str.1, ptr %4, align 8
  store ptr @.str.1, ptr %5, align 8
  store ptr @.str.2, ptr %6, align 8
  store ptr @.str.3, ptr %7, align 8
  %15 = getelementptr inbounds [50 x i8], ptr %2, i64 0, i64 0
  %16 = load ptr, ptr %4, align 8
  %17 = load ptr, ptr %6, align 8
  %18 = load ptr, ptr %7, align 8
  %19 = call i32 (ptr, i32, i64, ptr, ...) @__sprintf_chk(ptr noundef %15, i32 noundef 0, i64 noundef 50, ptr noundef @.str.4, ptr noundef %16, ptr noundef %17, ptr noundef %18)
  %20 = getelementptr inbounds [50 x i8], ptr %2, i64 0, i64 0
  %21 = call i64 @strlen(ptr noundef %20)
  %22 = trunc i64 %21 to i32
  store i32 %22, ptr %8, align 4
  %23 = load i32, ptr %8, align 4
  %24 = add nsw i32 %23, 1
  %25 = sext i32 %24 to i64
  %26 = mul i64 %25, 1
  %27 = call ptr @malloc(i64 noundef %26) #6
  store ptr %27, ptr %9, align 8
  %28 = load ptr, ptr %9, align 8
  %29 = getelementptr inbounds [50 x i8], ptr %2, i64 0, i64 0
  %30 = load ptr, ptr %9, align 8
  %31 = call i64 @llvm.objectsize.i64.p0(ptr %30, i1 false, i1 true, i1 false)
  %32 = call ptr @__strcpy_chk(ptr noundef %28, ptr noundef %29, i64 noundef %31) #7
  %33 = load i32, ptr %8, align 4
  %34 = load ptr, ptr %9, align 8
  %35 = call i32 (ptr, ...) @printf(ptr noundef @.str.5, i32 noundef %33, ptr noundef %34)
  ret i32 0
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #1

declare i32 @printf(ptr noundef, ...) #2

declare i32 @__sprintf_chk(ptr noundef, i32 noundef, i64 noundef, ptr noundef, ...) #2

declare i64 @strlen(ptr noundef) #2

; Function Attrs: allocsize(0)
declare ptr @malloc(i64 noundef) #3

; Function Attrs: nounwind
declare ptr @__strcpy_chk(ptr noundef, ptr noundef, i64 noundef) #4

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.objectsize.i64.p0(ptr, i1 immarg, i1 immarg, i1 immarg) #5

attributes #0 = { noinline nounwind optnone ssp uwtable(sync) "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #2 = { "frame-pointer"="non-leaf" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="apple-m1" "target-features"="+aes,+complxnum,+crc,+dotprod,+fp-armv8,+fp16fml,+fullfp16,+jsconv,+lse,+neon,+pauth,+ras,+rcpc,+rdm,+sha2,+sha3,+v8.1a,+v8.2a,+v8.3a,+v8.4a,+v8.5a,+v8a,+zcm,+zcz" }
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
