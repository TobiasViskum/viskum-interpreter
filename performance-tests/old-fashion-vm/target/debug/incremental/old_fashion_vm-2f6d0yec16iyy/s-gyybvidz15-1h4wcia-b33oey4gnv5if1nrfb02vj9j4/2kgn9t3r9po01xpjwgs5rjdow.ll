; ModuleID = '2kgn9t3r9po01xpjwgs5rjdow'
source_filename = "2kgn9t3r9po01xpjwgs5rjdow"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::fmt::rt::Argument<'_>" = type { %"core::fmt::rt::ArgumentType<'_>" }
%"core::fmt::rt::ArgumentType<'_>" = type { [1 x i64], ptr }

@vtable.0 = private constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h587d665a83ae016fE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc121ff4d98570cd0E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc121ff4d98570cd0E" }>, align 8, !dbg !0
@0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_3ba7eeeabd3d9c4a56f56d0cfe62277d = private unnamed_addr constant <{ [11 x i8] }> <{ [11 x i8] c"src/main.rs" }>, align 1
@alloc_cab6a8c52aee07dcb2e518ae814e64ca = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_3ba7eeeabd3d9c4a56f56d0cfe62277d, [16 x i8] c"\0B\00\00\00\00\00\00\00#\02\00\00\0D\00\00\00" }>, align 8
@alloc_76e85d556671283cdae58d4085a447dd = private unnamed_addr constant <{ [9 x i8] }> <{ [9 x i8] c"Elapsed: " }>, align 1
@alloc_a7ca73645ffd836c8c79eebb7036e3e3 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c", d: " }>, align 1
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc_db4e22af1b89f14da7d7a85e765fa7f0 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_76e85d556671283cdae58d4085a447dd, [8 x i8] c"\09\00\00\00\00\00\00\00", ptr @alloc_a7ca73645ffd836c8c79eebb7036e3e3, [8 x i8] c"\05\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_5c13356c806bec7c6e998277df19f8f5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_3ba7eeeabd3d9c4a56f56d0cfe62277d, [16 x i8] c"\0B\00\00\00\00\00\00\00*\02\00\00\15\00\00\00" }>, align 8

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6e7a26fd65c49ce2E(ptr %f) unnamed_addr #0 !dbg !43 {
start:
  %f.dbg.spill = alloca [8 x i8], align 8
  %result.dbg.spill = alloca [0 x i8], align 1
  %dummy.dbg.spill = alloca [0 x i8], align 1
  call void @llvm.dbg.declare(metadata ptr %dummy.dbg.spill, metadata !56, metadata !DIExpression()), !dbg !65
  call void @llvm.dbg.declare(metadata ptr %result.dbg.spill, metadata !51, metadata !DIExpression()), !dbg !67
  store ptr %f, ptr %f.dbg.spill, align 8, !dbg !65
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill, metadata !50, metadata !DIExpression()), !dbg !68
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hfeafe9d5bfb2618aE(ptr %f), !dbg !69
  call void asm sideeffect "", "~{memory}"(), !dbg !70, !srcloc !71
  ret void, !dbg !72
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden i64 @_ZN3std2rt10lang_start17hdb4182d1d8dea5ddE(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #1 !dbg !73 {
start:
  %v.dbg.spill = alloca [8 x i8], align 8
  %sigpipe.dbg.spill = alloca [1 x i8], align 1
  %argv.dbg.spill = alloca [8 x i8], align 8
  %argc.dbg.spill = alloca [8 x i8], align 8
  %main.dbg.spill = alloca [8 x i8], align 8
  %_8 = alloca [8 x i8], align 8
  %_5 = alloca [8 x i8], align 8
  store ptr %main, ptr %main.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %main.dbg.spill, metadata !81, metadata !DIExpression()), !dbg !87
  store i64 %argc, ptr %argc.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %argc.dbg.spill, metadata !82, metadata !DIExpression()), !dbg !88
  store ptr %argv, ptr %argv.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %argv.dbg.spill, metadata !83, metadata !DIExpression()), !dbg !89
  store i8 %sigpipe, ptr %sigpipe.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata ptr %sigpipe.dbg.spill, metadata !84, metadata !DIExpression()), !dbg !90
  store ptr %main, ptr %_8, align 8, !dbg !91
; call std::rt::lang_start_internal
  %0 = call i64 @_ZN3std2rt19lang_start_internal17hd3ad173a069f2ae5E(ptr align 1 %_8, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe), !dbg !92
  store i64 %0, ptr %_5, align 8, !dbg !92
  %v = load i64, ptr %_5, align 8, !dbg !93
  store i64 %v, ptr %v.dbg.spill, align 8, !dbg !93
  call void @llvm.dbg.declare(metadata ptr %v.dbg.spill, metadata !85, metadata !DIExpression()), !dbg !94
  ret i64 %v, !dbg !95
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc121ff4d98570cd0E"(ptr align 8 %_1) unnamed_addr #2 !dbg !96 {
start:
  %self.dbg.spill = alloca [8 x i8], align 8
  %_1.dbg.spill = alloca [8 x i8], align 8
  %self = alloca [1 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !102, metadata !DIExpression(DW_OP_deref)), !dbg !103
  call void @llvm.dbg.declare(metadata ptr %self, metadata !104, metadata !DIExpression()), !dbg !124
  %_4 = load ptr, ptr %_1, align 8, !dbg !126
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6e7a26fd65c49ce2E(ptr %_4), !dbg !127
; call <() as std::process::Termination>::report
  %0 = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h07af27c420ff937eE"(), !dbg !127
  store i8 %0, ptr %self, align 1, !dbg !127
  store ptr %self, ptr %self.dbg.spill, align 8, !dbg !128
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !129, metadata !DIExpression()), !dbg !138
  %_6 = load i8, ptr %self, align 1, !dbg !140
  %_0 = zext i8 %_6 to i32, !dbg !140
  ret i32 %_0, !dbg !141
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h4a28adf5b2518ed4E(ptr sret([48 x i8]) align 8 %_0, ptr align 8 %pieces, ptr align 8 %args) unnamed_addr #2 !dbg !142 {
start:
  %args.dbg.spill = alloca [8 x i8], align 8
  %pieces.dbg.spill = alloca [8 x i8], align 8
  store ptr %pieces, ptr %pieces.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %pieces.dbg.spill, metadata !293, metadata !DIExpression()), !dbg !295
  store ptr %args, ptr %args.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %args.dbg.spill, metadata !294, metadata !DIExpression()), !dbg !296
  store ptr %pieces, ptr %_0, align 8, !dbg !297
  %0 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !297
  store i64 3, ptr %0, align 8, !dbg !297
  %1 = load ptr, ptr @0, align 8, !dbg !297
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !297
  %3 = getelementptr inbounds i8, ptr %_0, i64 32, !dbg !297
  store ptr %1, ptr %3, align 8, !dbg !297
  %4 = getelementptr inbounds i8, ptr %3, i64 8, !dbg !297
  store i64 %2, ptr %4, align 8, !dbg !297
  %5 = getelementptr inbounds i8, ptr %_0, i64 16, !dbg !297
  store ptr %args, ptr %5, align 8, !dbg !297
  %6 = getelementptr inbounds i8, ptr %5, i64 8, !dbg !297
  store i64 2, ptr %6, align 8, !dbg !297
  ret void, !dbg !298
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h587d665a83ae016fE"(ptr %_1) unnamed_addr #2 !dbg !299 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !308, metadata !DIExpression()), !dbg !313
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !309, metadata !DIExpression()), !dbg !313
  %0 = load ptr, ptr %_1, align 8, !dbg !313
; call core::ops::function::FnOnce::call_once
  %_0 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h36d2cfd27f5a30f4E(ptr %0), !dbg !313
  ret i32 %_0, !dbg !313
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h36d2cfd27f5a30f4E(ptr %0) unnamed_addr #2 personality ptr @rust_eh_personality !dbg !314 {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
  call void @llvm.dbg.declare(metadata ptr %_1, metadata !318, metadata !DIExpression()), !dbg !320
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !319, metadata !DIExpression()), !dbg !320
; invoke std::rt::lang_start::{{closure}}
  %_0 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc121ff4d98570cd0E"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup, !dbg !320

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8, !dbg !320
  %3 = getelementptr inbounds i8, ptr %1, i64 8, !dbg !320
  %4 = load i32, ptr %3, align 8, !dbg !320
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0, !dbg !320
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1, !dbg !320
  resume { ptr, i32 } %6, !dbg !320

cleanup:                                          ; preds = %start
  %7 = landingpad { ptr, i32 }
          cleanup
  %8 = extractvalue { ptr, i32 } %7, 0
  %9 = extractvalue { ptr, i32 } %7, 1
  store ptr %8, ptr %1, align 8
  %10 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %9, ptr %10, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %_0, !dbg !320
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hfeafe9d5bfb2618aE(ptr %_1) unnamed_addr #2 !dbg !321 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !323, metadata !DIExpression()), !dbg !327
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !324, metadata !DIExpression()), !dbg !327
  call void %_1(), !dbg !327
  ret void, !dbg !327
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hf9fa3e9e1339a5e9E"(ptr align 8 %_1) unnamed_addr #2 !dbg !328 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !334, metadata !DIExpression()), !dbg !337
  ret void, !dbg !337
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h07af27c420ff937eE"() unnamed_addr #2 !dbg !338 {
start:
  %_1.dbg.spill = alloca [0 x i8], align 1
  %self.dbg.spill = alloca [0 x i8], align 1
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !343, metadata !DIExpression()), !dbg !345
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !344, metadata !DIExpression()), !dbg !345
  ret i8 0, !dbg !346
}

; old_fashion_vm::main
; Function Attrs: uwtable
define internal void @_ZN14old_fashion_vm4main17h1ffeda9f8dd8f7f9E() unnamed_addr #1 !dbg !347 {
start:
  %f.dbg.spill.i2 = alloca [8 x i8], align 8
  %x.dbg.spill.i3 = alloca [8 x i8], align 8
  %_3.i4 = alloca [16 x i8], align 8
  %f.dbg.spill.i = alloca [8 x i8], align 8
  %x.dbg.spill.i = alloca [8 x i8], align 8
  %_3.i = alloca [16 x i8], align 8
  %_19 = alloca [16 x i8], align 8
  %_17 = alloca [16 x i8], align 8
  %_15 = alloca [16 x i8], align 8
  %_14 = alloca [32 x i8], align 8
  %_12 = alloca [48 x i8], align 8
  %a = alloca [4 x i8], align 4
  %d = alloca [4 x i8], align 4
  %now = alloca [16 x i8], align 8
  call void @llvm.dbg.declare(metadata ptr %now, metadata !351, metadata !DIExpression()), !dbg !373
  call void @llvm.dbg.declare(metadata ptr %d, metadata !369, metadata !DIExpression()), !dbg !374
  call void @llvm.dbg.declare(metadata ptr %a, metadata !371, metadata !DIExpression()), !dbg !375
; call std::time::Instant::now
  %0 = call { i64, i32 } @_ZN3std4time7Instant3now17hb6f6e81efdc2ae6cE(), !dbg !376
  %1 = extractvalue { i64, i32 } %0, 0, !dbg !376
  %2 = extractvalue { i64, i32 } %0, 1, !dbg !376
  store i64 %1, ptr %now, align 8, !dbg !376
  %3 = getelementptr inbounds i8, ptr %now, i64 8, !dbg !376
  store i32 %2, ptr %3, align 8, !dbg !376
  store i32 0, ptr %d, align 4, !dbg !377
  br label %bb2, !dbg !378

bb2.loopexit:                                     ; preds = %bb6
  br label %bb2, !dbg !379

bb2:                                              ; preds = %bb2.loopexit, %start
  %_3 = load i32, ptr %d, align 4, !dbg !379
  %4 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %_3, i32 1), !dbg !379
  %_4.0 = extractvalue { i32, i1 } %4, 0, !dbg !379
  %_4.1 = extractvalue { i32, i1 } %4, 1, !dbg !379
  br i1 %_4.1, label %panic, label %bb3, !dbg !379

bb3:                                              ; preds = %bb2
  store i32 %_4.0, ptr %d, align 4, !dbg !380
  %_5 = load i32, ptr %d, align 4, !dbg !381
  %5 = icmp eq i32 %_5, 7000, !dbg !381
  br i1 %5, label %bb4, label %bb5, !dbg !381

panic:                                            ; preds = %bb2
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h7d670d0484c6d554E(ptr align 8 @alloc_cab6a8c52aee07dcb2e518ae814e64ca) #7, !dbg !379
  unreachable, !dbg !379

bb4:                                              ; preds = %bb3
; call std::time::Instant::elapsed
  %6 = call { i64, i32 } @_ZN3std4time7Instant7elapsed17hb8dd834e031d308bE(ptr align 8 %now), !dbg !382
  %7 = extractvalue { i64, i32 } %6, 0, !dbg !382
  %8 = extractvalue { i64, i32 } %6, 1, !dbg !382
  store i64 %7, ptr %_17, align 8, !dbg !382
  %9 = getelementptr inbounds i8, ptr %_17, i64 8, !dbg !382
  store i32 %8, ptr %9, align 8, !dbg !382
  store ptr %_17, ptr %x.dbg.spill.i3, align 8
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill.i3, metadata !383, metadata !DIExpression()), !dbg !401
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill.i3, metadata !403, metadata !DIExpression()), !dbg !414
  store ptr @"_ZN57_$LT$core..time..Duration$u20$as$u20$core..fmt..Debug$GT$3fmt17hdcd30f78c11066d9E", ptr %f.dbg.spill.i2, align 8, !dbg !416
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill.i2, metadata !413, metadata !DIExpression()), !dbg !417
  store ptr %_17, ptr %_3.i4, align 8, !dbg !418
  %10 = getelementptr inbounds i8, ptr %_3.i4, i64 8, !dbg !418
  store ptr @"_ZN57_$LT$core..time..Duration$u20$as$u20$core..fmt..Debug$GT$3fmt17hdcd30f78c11066d9E", ptr %10, align 8, !dbg !418
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_15, ptr align 8 %_3.i4, i64 16, i1 false), !dbg !419
  store ptr %d, ptr %x.dbg.spill.i, align 8
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill.i, metadata !420, metadata !DIExpression()), !dbg !429
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill.i, metadata !431, metadata !DIExpression()), !dbg !442
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E", ptr %f.dbg.spill.i, align 8, !dbg !444
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill.i, metadata !441, metadata !DIExpression()), !dbg !445
  store ptr %d, ptr %_3.i, align 8, !dbg !446
  %11 = getelementptr inbounds i8, ptr %_3.i, i64 8, !dbg !446
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E", ptr %11, align 8, !dbg !446
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_19, ptr align 8 %_3.i, i64 16, i1 false), !dbg !447
  %12 = getelementptr inbounds [2 x %"core::fmt::rt::Argument<'_>"], ptr %_14, i64 0, i64 0, !dbg !448
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %12, ptr align 8 %_15, i64 16, i1 false), !dbg !448
  %13 = getelementptr inbounds [2 x %"core::fmt::rt::Argument<'_>"], ptr %_14, i64 0, i64 1, !dbg !448
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %13, ptr align 8 %_19, i64 16, i1 false), !dbg !448
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h4a28adf5b2518ed4E(ptr sret([48 x i8]) align 8 %_12, ptr align 8 @alloc_db4e22af1b89f14da7d7a85e765fa7f0, ptr align 8 %_14), !dbg !448
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr align 8 %_12), !dbg !448
  ret void, !dbg !449

bb5:                                              ; preds = %bb3
  store i32 0, ptr %a, align 4, !dbg !450
  br label %bb6, !dbg !451

bb6:                                              ; preds = %bb8, %bb5
  %_8 = load i32, ptr %a, align 4, !dbg !452
  %_7 = icmp sle i32 %_8, 15, !dbg !452
  br i1 %_7, label %bb7, label %bb2.loopexit, !dbg !452

bb7:                                              ; preds = %bb6
  %_9 = load i32, ptr %a, align 4, !dbg !453
  %14 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %_9, i32 1), !dbg !453
  %_10.0 = extractvalue { i32, i1 } %14, 0, !dbg !453
  %_10.1 = extractvalue { i32, i1 } %14, 1, !dbg !453
  br i1 %_10.1, label %panic1, label %bb8, !dbg !453

bb8:                                              ; preds = %bb7
  store i32 %_10.0, ptr %a, align 4, !dbg !454
  br label %bb6, !dbg !451

panic1:                                           ; preds = %bb7
; call core::panicking::panic_const::panic_const_add_overflow
  call void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h7d670d0484c6d554E(ptr align 8 @alloc_5c13356c806bec7c6e998277df19f8f5) #7, !dbg !453
  unreachable, !dbg !453
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare void @llvm.dbg.declare(metadata, metadata, metadata) #3

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare i64 @_ZN3std2rt19lang_start_internal17hd3ad173a069f2ae5E(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #1

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E"(ptr align 4, ptr align 8) unnamed_addr #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #4

; <core::time::Duration as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN57_$LT$core..time..Duration$u20$as$u20$core..fmt..Debug$GT$3fmt17hdcd30f78c11066d9E"(ptr align 8, ptr align 8) unnamed_addr #1

; Function Attrs: uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #1

; std::time::Instant::now
; Function Attrs: uwtable
declare { i64, i32 } @_ZN3std4time7Instant3now17hb6f6e81efdc2ae6cE() unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #3

; core::panicking::panic_const::panic_const_add_overflow
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking11panic_const24panic_const_add_overflow17h7d670d0484c6d554E(ptr align 8) unnamed_addr #5

; std::time::Instant::elapsed
; Function Attrs: uwtable
declare { i64, i32 } @_ZN3std4time7Instant7elapsed17hb8dd834e031d308bE(ptr align 8) unnamed_addr #1

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr align 8) unnamed_addr #1

define i32 @main(i32 %0, ptr %1) unnamed_addr #6 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17hdb4182d1d8dea5ddE(ptr @_ZN14old_fashion_vm4main17h1ffeda9f8dd8f7f9E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #4 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #7 = { noreturn }

!llvm.module.flags = !{!24, !25, !26, !27}
!llvm.ident = !{!28}
!llvm.dbg.cu = !{!29}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable}", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "<std::rt::lang_start::{closure_env#0}<()> as core::ops::function::Fn<()>>::{vtable_type}", file: !2, size: 384, align: 64, flags: DIFlagArtificial, elements: !4, vtableHolder: !14, templateParams: !23, identifier: "c49c5dfa0fb7169b1f367873f6a835c4")
!4 = !{!5, !8, !10, !11, !12, !13}
!5 = !DIDerivedType(tag: DW_TAG_member, name: "drop_in_place", scope: !3, file: !2, baseType: !6, size: 64, align: 64)
!6 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const ()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!7 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!8 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !3, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!9 = !DIBasicType(name: "usize", size: 64, encoding: DW_ATE_unsigned)
!10 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !3, file: !2, baseType: !9, size: 64, align: 64, offset: 128)
!11 = !DIDerivedType(tag: DW_TAG_member, name: "__method3", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 192)
!12 = !DIDerivedType(tag: DW_TAG_member, name: "__method4", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 256)
!13 = !DIDerivedType(tag: DW_TAG_member, name: "__method5", scope: !3, file: !2, baseType: !6, size: 64, align: 64, offset: 320)
!14 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<()>", scope: !15, file: !2, size: 64, align: 64, elements: !18, templateParams: !23, identifier: "f69c469a97ad09c39b97bc28dd79df14")
!15 = !DINamespace(name: "lang_start", scope: !16)
!16 = !DINamespace(name: "rt", scope: !17)
!17 = !DINamespace(name: "std", scope: null)
!18 = !{!19}
!19 = !DIDerivedType(tag: DW_TAG_member, name: "main", scope: !14, file: !2, baseType: !20, size: 64, align: 64)
!20 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn()", baseType: !21, size: 64, align: 64, dwarfAddressSpace: 0)
!21 = !DISubroutineType(types: !22)
!22 = !{null}
!23 = !{}
!24 = !{i32 8, !"PIC Level", i32 2}
!25 = !{i32 7, !"PIE Level", i32 2}
!26 = !{i32 2, !"Dwarf Version", i32 4}
!27 = !{i32 2, !"Debug Info Version", i32 3}
!28 = !{!"rustc version 1.80.0 (051478957 2024-07-21)"}
!29 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !30, producer: "clang LLVM (rustc version 1.80.0 (051478957 2024-07-21))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, enums: !31, globals: !42, splitDebugInlining: false, nameTableKind: None)
!30 = !DIFile(filename: "src/main.rs/@/2kgn9t3r9po01xpjwgs5rjdow", directory: "/Users/tobiasviskum/Programming/Rust/viskum-language/performance-tests/old-fashion-vm")
!31 = !{!32}
!32 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !33, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagEnumClass, elements: !37)
!33 = !DINamespace(name: "rt", scope: !34)
!34 = !DINamespace(name: "fmt", scope: !35)
!35 = !DINamespace(name: "core", scope: null)
!36 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!37 = !{!38, !39, !40, !41}
!38 = !DIEnumerator(name: "Left", value: 0, isUnsigned: true)
!39 = !DIEnumerator(name: "Right", value: 1, isUnsigned: true)
!40 = !DIEnumerator(name: "Center", value: 2, isUnsigned: true)
!41 = !DIEnumerator(name: "Unknown", value: 3, isUnsigned: true)
!42 = !{!0}
!43 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h6e7a26fd65c49ce2E", scope: !45, file: !44, line: 151, type: !47, scopeLine: 151, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !53, retainedNodes: !49)
!44 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/sys_common/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "9a938a0945aa66d12453850743d3bf49")
!45 = !DINamespace(name: "backtrace", scope: !46)
!46 = !DINamespace(name: "sys_common", scope: !17)
!47 = !DISubroutineType(types: !48)
!48 = !{null, !20}
!49 = !{!50, !51}
!50 = !DILocalVariable(name: "f", arg: 1, scope: !43, file: !44, line: 151, type: !20)
!51 = !DILocalVariable(name: "result", scope: !52, file: !44, line: 155, type: !7, align: 1)
!52 = distinct !DILexicalBlock(scope: !43, file: !44, line: 155, column: 5)
!53 = !{!54, !55}
!54 = !DITemplateTypeParameter(name: "F", type: !20)
!55 = !DITemplateTypeParameter(name: "T", type: !7)
!56 = !DILocalVariable(name: "dummy", scope: !57, file: !58, line: 337, type: !7, align: 1)
!57 = distinct !DILexicalBlock(scope: !59, file: !58, line: 337, column: 1)
!58 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "36624a7f44e0e372094a9874489ad080")
!59 = distinct !DISubprogram(name: "black_box<()>", linkageName: "_ZN4core4hint9black_box17hd90612ad32d289fcE", scope: !60, file: !58, line: 337, type: !61, scopeLine: 337, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !64, retainedNodes: !63)
!60 = !DINamespace(name: "hint", scope: !35)
!61 = !DISubroutineType(types: !62)
!62 = !{null, !7}
!63 = !{!56}
!64 = !{!55}
!65 = !DILocation(line: 337, column: 27, scope: !57, inlinedAt: !66)
!66 = !DILocation(line: 158, column: 5, scope: !52)
!67 = !DILocation(line: 155, column: 9, scope: !52)
!68 = !DILocation(line: 151, column: 43, scope: !43)
!69 = !DILocation(line: 155, column: 18, scope: !43)
!70 = !DILocation(line: 338, column: 5, scope: !57, inlinedAt: !66)
!71 = !{i32 1986794}
!72 = !DILocation(line: 161, column: 2, scope: !43)
!73 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17hdb4182d1d8dea5ddE", scope: !16, file: !74, line: 152, type: !75, scopeLine: 152, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !64, retainedNodes: !80)
!74 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "d023918fb5f452acdbb300902bf5fc59")
!75 = !DISubroutineType(types: !76)
!76 = !{!77, !20, !77, !78, !36}
!77 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!78 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !79, size: 64, align: 64, dwarfAddressSpace: 0)
!79 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !36, size: 64, align: 64, dwarfAddressSpace: 0)
!80 = !{!81, !82, !83, !84, !85}
!81 = !DILocalVariable(name: "main", arg: 1, scope: !73, file: !74, line: 153, type: !20)
!82 = !DILocalVariable(name: "argc", arg: 2, scope: !73, file: !74, line: 154, type: !77)
!83 = !DILocalVariable(name: "argv", arg: 3, scope: !73, file: !74, line: 155, type: !78)
!84 = !DILocalVariable(name: "sigpipe", arg: 4, scope: !73, file: !74, line: 156, type: !36)
!85 = !DILocalVariable(name: "v", scope: !86, file: !74, line: 158, type: !77, align: 8)
!86 = distinct !DILexicalBlock(scope: !73, file: !74, line: 158, column: 5)
!87 = !DILocation(line: 153, column: 5, scope: !73)
!88 = !DILocation(line: 154, column: 5, scope: !73)
!89 = !DILocation(line: 155, column: 5, scope: !73)
!90 = !DILocation(line: 156, column: 5, scope: !73)
!91 = !DILocation(line: 159, column: 10, scope: !73)
!92 = !DILocation(line: 158, column: 17, scope: !73)
!93 = !DILocation(line: 158, column: 12, scope: !73)
!94 = !DILocation(line: 158, column: 12, scope: !86)
!95 = !DILocation(line: 165, column: 2, scope: !73)
!96 = distinct !DISubprogram(name: "{closure#0}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc121ff4d98570cd0E", scope: !15, file: !74, line: 159, type: !97, scopeLine: 159, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !64, retainedNodes: !101)
!97 = !DISubroutineType(types: !98)
!98 = !{!99, !100}
!99 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!100 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!101 = !{!102}
!102 = !DILocalVariable(name: "main", scope: !96, file: !74, line: 153, type: !20, align: 8)
!103 = !DILocation(line: 153, column: 5, scope: !96)
!104 = !DILocalVariable(name: "self", arg: 1, scope: !105, file: !106, line: 2048, type: !108)
!105 = distinct !DILexicalBlock(scope: !107, file: !106, line: 2048, column: 5)
!106 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/process.rs", directory: "", checksumkind: CSK_MD5, checksum: "9e51e22eb3333ae012d05fdfbdaeaf7a")
!107 = distinct !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217haa9d3ac0a3b1d416E", scope: !108, file: !106, line: 2048, type: !120, scopeLine: 2048, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !23, declaration: !122, retainedNodes: !123)
!108 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !109, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !110, templateParams: !23, identifier: "26ed9badc2529fac4b11f00e1f76ebd2")
!109 = !DINamespace(name: "process", scope: !17)
!110 = !{!111}
!111 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !108, file: !2, baseType: !112, size: 8, align: 8, flags: DIFlagPrivate)
!112 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !113, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !118, templateParams: !23, identifier: "e8a72f2f2ac5c333219035ecc5a8a7e1")
!113 = !DINamespace(name: "process_common", scope: !114)
!114 = !DINamespace(name: "process", scope: !115)
!115 = !DINamespace(name: "unix", scope: !116)
!116 = !DINamespace(name: "pal", scope: !117)
!117 = !DINamespace(name: "sys", scope: !17)
!118 = !{!119}
!119 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !112, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagPrivate)
!120 = !DISubroutineType(types: !121)
!121 = !{!99, !108}
!122 = !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217haa9d3ac0a3b1d416E", scope: !108, file: !106, line: 2048, type: !120, scopeLine: 2048, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!123 = !{!104}
!124 = !DILocation(line: 2048, column: 19, scope: !105, inlinedAt: !125)
!125 = !DILocation(line: 159, column: 92, scope: !96)
!126 = !DILocation(line: 159, column: 77, scope: !96)
!127 = !DILocation(line: 159, column: 18, scope: !96)
!128 = !DILocation(line: 2049, column: 9, scope: !105, inlinedAt: !125)
!129 = !DILocalVariable(name: "self", arg: 1, scope: !130, file: !131, line: 638, type: !135)
!130 = distinct !DILexicalBlock(scope: !132, file: !131, line: 638, column: 5)
!131 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/sys/pal/unix/process/process_common.rs", directory: "", checksumkind: CSK_MD5, checksum: "f12d6cc5fbe6e47291b02b1d467e8da3")
!132 = distinct !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys3pal4unix7process14process_common8ExitCode6as_i3217h1f8c0d2d13a7a529E", scope: !112, file: !131, line: 638, type: !133, scopeLine: 638, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !23, declaration: !136, retainedNodes: !137)
!133 = !DISubroutineType(types: !134)
!134 = !{!99, !135}
!135 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::sys::pal::unix::process::process_common::ExitCode", baseType: !112, size: 64, align: 64, dwarfAddressSpace: 0)
!136 = !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys3pal4unix7process14process_common8ExitCode6as_i3217h1f8c0d2d13a7a529E", scope: !112, file: !131, line: 638, type: !133, scopeLine: 638, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!137 = !{!129}
!138 = !DILocation(line: 638, column: 19, scope: !130, inlinedAt: !139)
!139 = !DILocation(line: 2049, column: 16, scope: !105, inlinedAt: !125)
!140 = !DILocation(line: 639, column: 9, scope: !130, inlinedAt: !139)
!141 = !DILocation(line: 159, column: 100, scope: !96)
!142 = distinct !DISubprogram(name: "new_v1<3, 2>", linkageName: "_ZN4core3fmt9Arguments6new_v117h4a28adf5b2518ed4E", scope: !144, file: !143, line: 349, type: !281, scopeLine: 349, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !23, declaration: !291, retainedNodes: !292)
!143 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "d0edb1b569d2fa74945fd472d62c28dc")
!144 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !34, file: !2, size: 384, align: 64, flags: DIFlagPublic, elements: !145, templateParams: !23, identifier: "b46d88cc0bf2e6afccf1ddcc24dfbeb3")
!145 = !{!146, !157, !203}
!146 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !144, file: !2, baseType: !147, size: 128, align: 64, flags: DIFlagPrivate)
!147 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 128, align: 64, elements: !148, templateParams: !23, identifier: "4e66b00a376d6af5b8765440fb2839f")
!148 = !{!149, !156}
!149 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !147, file: !2, baseType: !150, size: 64, align: 64)
!150 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !151, size: 64, align: 64, dwarfAddressSpace: 0)
!151 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !152, templateParams: !23, identifier: "9277eecd40495f85161460476aacc992")
!152 = !{!153, !155}
!153 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !151, file: !2, baseType: !154, size: 64, align: 64)
!154 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !36, size: 64, align: 64, dwarfAddressSpace: 0)
!155 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !151, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!156 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !147, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!157 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !144, file: !2, baseType: !158, size: 128, align: 64, offset: 256, flags: DIFlagPrivate)
!158 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !159, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !160, templateParams: !23, identifier: "d774420e8e3096a8e0063cd51ebf0c01")
!159 = !DINamespace(name: "option", scope: !35)
!160 = !{!161}
!161 = !DICompositeType(tag: DW_TAG_variant_part, scope: !158, file: !2, size: 128, align: 64, elements: !162, templateParams: !23, identifier: "42af0665edf854dcb9ea52e84c3cefe8", discriminator: !202)
!162 = !{!163, !198}
!163 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !161, file: !2, baseType: !164, size: 128, align: 64, extraData: i128 0)
!164 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !158, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !165, identifier: "fc691c729a9e3e4d57fa2678a9cd73d")
!165 = !{!166}
!166 = !DITemplateTypeParameter(name: "T", type: !167)
!167 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !2, size: 128, align: 64, elements: !168, templateParams: !23, identifier: "18b2c68086ffbcbd7f221423f493c07f")
!168 = !{!169, !197}
!169 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !167, file: !2, baseType: !170, size: 64, align: 64)
!170 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !171, size: 64, align: 64, dwarfAddressSpace: 0)
!171 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !33, file: !2, size: 448, align: 64, flags: DIFlagPublic, elements: !172, templateParams: !23, identifier: "3c94cc8264d3d8b055b5acbf4afe9590")
!172 = !{!173, !174, !176, !177, !179, !196}
!173 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !171, file: !2, baseType: !9, size: 64, align: 64, offset: 256, flags: DIFlagPublic)
!174 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !171, file: !2, baseType: !175, size: 32, align: 32, offset: 320, flags: DIFlagPublic)
!175 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_UTF)
!176 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !171, file: !2, baseType: !32, size: 8, align: 8, offset: 384, flags: DIFlagPublic)
!177 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !171, file: !2, baseType: !178, size: 32, align: 32, offset: 352, flags: DIFlagPublic)
!178 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!179 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !171, file: !2, baseType: !180, size: 128, align: 64, flags: DIFlagPublic)
!180 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !33, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !181, templateParams: !23, identifier: "7bbb02f6e78a3559b539405c91777483")
!181 = !{!182}
!182 = !DICompositeType(tag: DW_TAG_variant_part, scope: !180, file: !2, size: 128, align: 64, elements: !183, templateParams: !23, identifier: "5d0823612efc111239df02572d876ea", discriminator: !194)
!183 = !{!184, !188, !192}
!184 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !182, file: !2, baseType: !185, size: 128, align: 64, extraData: i128 0)
!185 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !180, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !186, templateParams: !23, identifier: "544eb65eb435c61322c3979318601571")
!186 = !{!187}
!187 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !185, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!188 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !182, file: !2, baseType: !189, size: 128, align: 64, extraData: i128 1)
!189 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !180, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !190, templateParams: !23, identifier: "a926f0d235e4de79f584934dbdbca4fc")
!190 = !{!191}
!191 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !189, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!192 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !182, file: !2, baseType: !193, size: 128, align: 64, extraData: i128 2)
!193 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !180, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, identifier: "4f14f535fef5d63a8c4b0fca5a7e7c0")
!194 = !DIDerivedType(tag: DW_TAG_member, scope: !180, file: !2, baseType: !195, size: 64, align: 64, flags: DIFlagArtificial)
!195 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!196 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !171, file: !2, baseType: !180, size: 128, align: 64, offset: 128, flags: DIFlagPublic)
!197 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !167, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!198 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !161, file: !2, baseType: !199, size: 128, align: 64)
!199 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !158, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !200, templateParams: !165, identifier: "989f9335e9b530569cebf2d4c10723d8")
!200 = !{!201}
!201 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !199, file: !2, baseType: !167, size: 128, align: 64, flags: DIFlagPublic)
!202 = !DIDerivedType(tag: DW_TAG_member, scope: !158, file: !2, baseType: !195, size: 64, align: 64, flags: DIFlagArtificial)
!203 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !144, file: !2, baseType: !204, size: 128, align: 64, offset: 128, flags: DIFlagPrivate)
!204 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !2, size: 128, align: 64, elements: !205, templateParams: !23, identifier: "6ec08f8a7448e7974bb363f956a6ce2")
!205 = !{!206, !280}
!206 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !204, file: !2, baseType: !207, size: 64, align: 64)
!207 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !208, size: 64, align: 64, dwarfAddressSpace: 0)
!208 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !33, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !209, templateParams: !23, identifier: "ecd976d121f35b1f1bf62ffde9d03f36")
!209 = !{!210}
!210 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !208, file: !2, baseType: !211, size: 128, align: 64, flags: DIFlagPrivate)
!211 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !33, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !212, templateParams: !23, identifier: "ff3f2e7a1fa7ea1eec8f058c6c7ec6b4")
!212 = !{!213}
!213 = !DICompositeType(tag: DW_TAG_variant_part, scope: !211, file: !2, size: 128, align: 64, elements: !214, templateParams: !23, identifier: "6ca585353d9c5045547be772bdd68196", discriminator: !279)
!214 = !{!215, !275}
!215 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !213, file: !2, baseType: !216, size: 128, align: 64)
!216 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !211, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !217, templateParams: !23, identifier: "21d35b48df9bcc19905f29e285e9d5a1")
!217 = !{!218, !222}
!218 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !216, file: !2, baseType: !219, size: 64, align: 64, flags: DIFlagPrivate)
!219 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::rt::{extern#0}::Opaque", baseType: !220, size: 64, align: 64, dwarfAddressSpace: 0)
!220 = !DICompositeType(tag: DW_TAG_structure_type, name: "Opaque", scope: !221, file: !2, align: 8, elements: !23, identifier: "87190b3403781efddea9ff07bab29af3")
!221 = !DINamespace(name: "{extern#0}", scope: !33)
!222 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !216, file: !2, baseType: !223, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!223 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&core::fmt::rt::{extern#0}::Opaque, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !224, size: 64, align: 64, dwarfAddressSpace: 0)
!224 = !DISubroutineType(types: !225)
!225 = !{!226, !219, !243}
!226 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !227, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !228, templateParams: !23, identifier: "6c840619c30f4ae36491fb7dced63bc")
!227 = !DINamespace(name: "result", scope: !35)
!228 = !{!229}
!229 = !DICompositeType(tag: DW_TAG_variant_part, scope: !226, file: !2, size: 8, align: 8, elements: !230, templateParams: !23, identifier: "288eb4c9a506330b8b455aa98e665788", discriminator: !242)
!230 = !{!231, !238}
!231 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !229, file: !2, baseType: !232, size: 8, align: 8, extraData: i128 0)
!232 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !226, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !233, templateParams: !235, identifier: "6613f4563358f09920d070f9e159e1d5")
!233 = !{!234}
!234 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !232, file: !2, baseType: !7, align: 8, offset: 8, flags: DIFlagPublic)
!235 = !{!55, !236}
!236 = !DITemplateTypeParameter(name: "E", type: !237)
!237 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !34, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "3a48e56d0aef43fb722d6937f8f560dd")
!238 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !229, file: !2, baseType: !239, size: 8, align: 8, extraData: i128 1)
!239 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !226, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !240, templateParams: !235, identifier: "cd0715ab4b3caa165e90f0874d4b294b")
!240 = !{!241}
!241 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !239, file: !2, baseType: !237, align: 8, offset: 8, flags: DIFlagPublic)
!242 = !DIDerivedType(tag: DW_TAG_member, scope: !226, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagArtificial)
!243 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !244, size: 64, align: 64, dwarfAddressSpace: 0)
!244 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !34, file: !2, size: 512, align: 64, flags: DIFlagPublic, elements: !245, templateParams: !23, identifier: "e64159529f90e6c93501e1c8b44635e1")
!245 = !{!246, !247, !248, !249, !263, !264}
!246 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !244, file: !2, baseType: !178, size: 32, align: 32, offset: 416, flags: DIFlagPrivate)
!247 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !244, file: !2, baseType: !175, size: 32, align: 32, offset: 384, flags: DIFlagPrivate)
!248 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !244, file: !2, baseType: !32, size: 8, align: 8, offset: 448, flags: DIFlagPrivate)
!249 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !244, file: !2, baseType: !250, size: 128, align: 64, flags: DIFlagPrivate)
!250 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !159, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !251, templateParams: !23, identifier: "562a25f644cfbb3ec8203ae3d455f2e")
!251 = !{!252}
!252 = !DICompositeType(tag: DW_TAG_variant_part, scope: !250, file: !2, size: 128, align: 64, elements: !253, templateParams: !23, identifier: "8ed3b56007451f33e768ffbb6948a6c4", discriminator: !262)
!253 = !{!254, !258}
!254 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !252, file: !2, baseType: !255, size: 128, align: 64, extraData: i128 0)
!255 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !250, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !256, identifier: "3040cd18654cf144208f719c3a1aa714")
!256 = !{!257}
!257 = !DITemplateTypeParameter(name: "T", type: !9)
!258 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !252, file: !2, baseType: !259, size: 128, align: 64, extraData: i128 1)
!259 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !250, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !260, templateParams: !256, identifier: "485854eed0ea9812f1a7f394ec88ec1d")
!260 = !{!261}
!261 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !259, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!262 = !DIDerivedType(tag: DW_TAG_member, scope: !250, file: !2, baseType: !195, size: 64, align: 64, flags: DIFlagArtificial)
!263 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !244, file: !2, baseType: !250, size: 128, align: 64, offset: 128, flags: DIFlagPrivate)
!264 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !244, file: !2, baseType: !265, size: 128, align: 64, offset: 256, flags: DIFlagPrivate)
!265 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 128, align: 64, elements: !266, templateParams: !23, identifier: "7314e09b4ef07e046a853e7dcce5165f")
!266 = !{!267, !270}
!267 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !265, file: !2, baseType: !268, size: 64, align: 64)
!268 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !269, size: 64, align: 64, dwarfAddressSpace: 0)
!269 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !23, identifier: "8d7cf6ee3dd70fbcd6a3056bca62fea8")
!270 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !265, file: !2, baseType: !271, size: 64, align: 64, offset: 64)
!271 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !272, size: 64, align: 64, dwarfAddressSpace: 0)
!272 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 384, align: 64, elements: !273)
!273 = !{!274}
!274 = !DISubrange(count: 6, lowerBound: 0)
!275 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !213, file: !2, baseType: !276, size: 128, align: 64, extraData: i128 0)
!276 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !211, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !277, templateParams: !23, identifier: "578cfa2846b688ed99eab6949d37fe79")
!277 = !{!278}
!278 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !276, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPrivate)
!279 = !DIDerivedType(tag: DW_TAG_member, scope: !211, file: !2, baseType: !195, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)
!280 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !204, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!281 = !DISubroutineType(types: !282)
!282 = !{!144, !283, !287}
!283 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[&str; 3]", baseType: !284, size: 64, align: 64, dwarfAddressSpace: 0)
!284 = !DICompositeType(tag: DW_TAG_array_type, baseType: !151, size: 384, align: 64, elements: !285)
!285 = !{!286}
!286 = !DISubrange(count: 3, lowerBound: 0)
!287 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[core::fmt::rt::Argument; 2]", baseType: !288, size: 64, align: 64, dwarfAddressSpace: 0)
!288 = !DICompositeType(tag: DW_TAG_array_type, baseType: !208, size: 256, align: 64, elements: !289)
!289 = !{!290}
!290 = !DISubrange(count: 2, lowerBound: 0)
!291 = !DISubprogram(name: "new_v1<3, 2>", linkageName: "_ZN4core3fmt9Arguments6new_v117h4a28adf5b2518ed4E", scope: !144, file: !143, line: 349, type: !281, scopeLine: 349, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !23)
!292 = !{!293, !294}
!293 = !DILocalVariable(name: "pieces", arg: 1, scope: !142, file: !143, line: 350, type: !283)
!294 = !DILocalVariable(name: "args", arg: 2, scope: !142, file: !143, line: 351, type: !287)
!295 = !DILocation(line: 350, column: 9, scope: !142)
!296 = !DILocation(line: 351, column: 9, scope: !142)
!297 = !DILocation(line: 354, column: 9, scope: !142)
!298 = !DILocation(line: 355, column: 6, scope: !142)
!299 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h587d665a83ae016fE", scope: !301, file: !300, line: 250, type: !304, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !310, retainedNodes: !307)
!300 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "abc772494ea8033dad5cae2e40e54b10")
!301 = !DINamespace(name: "FnOnce", scope: !302)
!302 = !DINamespace(name: "function", scope: !303)
!303 = !DINamespace(name: "ops", scope: !35)
!304 = !DISubroutineType(types: !305)
!305 = !{!99, !306}
!306 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!307 = !{!308, !309}
!308 = !DILocalVariable(arg: 1, scope: !299, file: !300, line: 250, type: !306)
!309 = !DILocalVariable(arg: 2, scope: !299, file: !300, line: 250, type: !7)
!310 = !{!311, !312}
!311 = !DITemplateTypeParameter(name: "Self", type: !14)
!312 = !DITemplateTypeParameter(name: "Args", type: !7)
!313 = !DILocation(line: 250, column: 5, scope: !299)
!314 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h36d2cfd27f5a30f4E", scope: !301, file: !300, line: 250, type: !315, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !310, retainedNodes: !317)
!315 = !DISubroutineType(types: !316)
!316 = !{!99, !14}
!317 = !{!318, !319}
!318 = !DILocalVariable(arg: 1, scope: !314, file: !300, line: 250, type: !14)
!319 = !DILocalVariable(arg: 2, scope: !314, file: !300, line: 250, type: !7)
!320 = !DILocation(line: 250, column: 5, scope: !314)
!321 = distinct !DISubprogram(name: "call_once<fn(), ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17hfeafe9d5bfb2618aE", scope: !301, file: !300, line: 250, type: !47, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !325, retainedNodes: !322)
!322 = !{!323, !324}
!323 = !DILocalVariable(arg: 1, scope: !321, file: !300, line: 250, type: !20)
!324 = !DILocalVariable(arg: 2, scope: !321, file: !300, line: 250, type: !7)
!325 = !{!326, !312}
!326 = !DITemplateTypeParameter(name: "Self", type: !20)
!327 = !DILocation(line: 250, column: 5, scope: !321)
!328 = distinct !DISubprogram(name: "drop_in_place<std::rt::lang_start::{closure_env#0}<()>>", linkageName: "_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hf9fa3e9e1339a5e9E", scope: !330, file: !329, line: 542, type: !331, scopeLine: 542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !335, retainedNodes: !333)
!329 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "a85d519116fde26821981e2ebfa0ecba")
!330 = !DINamespace(name: "ptr", scope: !35)
!331 = !DISubroutineType(types: !332)
!332 = !{null, !306}
!333 = !{!334}
!334 = !DILocalVariable(arg: 1, scope: !328, file: !329, line: 542, type: !306)
!335 = !{!336}
!336 = !DITemplateTypeParameter(name: "T", type: !14)
!337 = !DILocation(line: 542, column: 1, scope: !328)
!338 = distinct !DISubprogram(name: "report", linkageName: "_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h07af27c420ff937eE", scope: !339, file: !106, line: 2421, type: !340, scopeLine: 2421, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !23, retainedNodes: !342)
!339 = !DINamespace(name: "{impl#57}", scope: !109)
!340 = !DISubroutineType(types: !341)
!341 = !{!108, !7}
!342 = !{!343, !344}
!343 = !DILocalVariable(name: "self", scope: !338, file: !106, line: 2421, type: !7, align: 1)
!344 = !DILocalVariable(arg: 1, scope: !338, file: !106, line: 2421, type: !7)
!345 = !DILocation(line: 2421, column: 15, scope: !338)
!346 = !DILocation(line: 2423, column: 6, scope: !338)
!347 = distinct !DISubprogram(name: "main", linkageName: "_ZN14old_fashion_vm4main17h1ffeda9f8dd8f7f9E", scope: !349, file: !348, line: 543, type: !21, scopeLine: 543, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagMainSubprogram, unit: !29, templateParams: !23, retainedNodes: !350)
!348 = !DIFile(filename: "src/main.rs", directory: "/Users/tobiasviskum/Programming/Rust/viskum-language/performance-tests/old-fashion-vm", checksumkind: CSK_MD5, checksum: "5ff7c03b07d38e6dfef89c7d2b12b571")
!349 = !DINamespace(name: "old_fashion_vm", scope: null)
!350 = !{!351, !369, !371}
!351 = !DILocalVariable(name: "now", scope: !352, file: !348, line: 544, type: !353, align: 8)
!352 = distinct !DILexicalBlock(scope: !347, file: !348, line: 544, column: 5)
!353 = !DICompositeType(tag: DW_TAG_structure_type, name: "Instant", scope: !354, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !355, templateParams: !23, identifier: "d14c7078c42be7726d41c8202563cbc1")
!354 = !DINamespace(name: "time", scope: !17)
!355 = !{!356}
!356 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !353, file: !2, baseType: !357, size: 128, align: 64, flags: DIFlagPrivate)
!357 = !DICompositeType(tag: DW_TAG_structure_type, name: "Instant", scope: !358, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !359, templateParams: !23, identifier: "7c35bf343e9b9c05266cccd4564ff6b5")
!358 = !DINamespace(name: "time", scope: !115)
!359 = !{!360}
!360 = !DIDerivedType(tag: DW_TAG_member, name: "t", scope: !357, file: !2, baseType: !361, size: 128, align: 64, flags: DIFlagPrivate)
!361 = !DICompositeType(tag: DW_TAG_structure_type, name: "Timespec", scope: !358, file: !2, size: 128, align: 64, flags: DIFlagProtected, elements: !362, templateParams: !23, identifier: "6500e5d48cee2cc91274e9c1fe40debd")
!362 = !{!363, !365}
!363 = !DIDerivedType(tag: DW_TAG_member, name: "tv_sec", scope: !361, file: !2, baseType: !364, size: 64, align: 64, flags: DIFlagPrivate)
!364 = !DIBasicType(name: "i64", size: 64, encoding: DW_ATE_signed)
!365 = !DIDerivedType(tag: DW_TAG_member, name: "tv_nsec", scope: !361, file: !2, baseType: !366, size: 32, align: 32, offset: 64, flags: DIFlagPrivate)
!366 = !DICompositeType(tag: DW_TAG_structure_type, name: "Nanoseconds", scope: !358, file: !2, size: 32, align: 32, flags: DIFlagPrivate, elements: !367, templateParams: !23, identifier: "5e5de8b2a01c3fbd9b75723da138e506")
!367 = !{!368}
!368 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !366, file: !2, baseType: !178, size: 32, align: 32, flags: DIFlagPrivate)
!369 = !DILocalVariable(name: "d", scope: !370, file: !348, line: 545, type: !99, align: 4)
!370 = distinct !DILexicalBlock(scope: !352, file: !348, line: 545, column: 5)
!371 = !DILocalVariable(name: "a", scope: !372, file: !348, line: 551, type: !99, align: 4)
!372 = distinct !DILexicalBlock(scope: !370, file: !348, line: 551, column: 13)
!373 = !DILocation(line: 544, column: 9, scope: !352)
!374 = !DILocation(line: 545, column: 9, scope: !370)
!375 = !DILocation(line: 551, column: 17, scope: !372)
!376 = !DILocation(line: 544, column: 15, scope: !347)
!377 = !DILocation(line: 545, column: 17, scope: !352)
!378 = !DILocation(line: 546, column: 5, scope: !370)
!379 = !DILocation(line: 547, column: 13, scope: !370)
!380 = !DILocation(line: 547, column: 9, scope: !370)
!381 = !DILocation(line: 548, column: 12, scope: !370)
!382 = !DILocation(line: 559, column: 38, scope: !370)
!383 = !DILocalVariable(name: "x", arg: 1, scope: !384, file: !385, line: 116, type: !388)
!384 = distinct !DISubprogram(name: "new_debug<core::time::Duration>", linkageName: "_ZN4core3fmt2rt8Argument9new_debug17ha2b41630653ea5a7E", scope: !208, file: !385, line: 116, type: !386, scopeLine: 116, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !398, declaration: !397, retainedNodes: !400)
!385 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/fmt/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "97b307aeb2cfde091afce8909100596c")
!386 = !DISubroutineType(types: !387)
!387 = !{!208, !388}
!388 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::time::Duration", baseType: !389, size: 64, align: 64, dwarfAddressSpace: 0)
!389 = !DICompositeType(tag: DW_TAG_structure_type, name: "Duration", scope: !390, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !391, templateParams: !23, identifier: "ed7c5b6f4c1cc125245dc31be7c52d1c")
!390 = !DINamespace(name: "time", scope: !35)
!391 = !{!392, !393}
!392 = !DIDerivedType(tag: DW_TAG_member, name: "secs", scope: !389, file: !2, baseType: !195, size: 64, align: 64, flags: DIFlagPrivate)
!393 = !DIDerivedType(tag: DW_TAG_member, name: "nanos", scope: !389, file: !2, baseType: !394, size: 32, align: 32, offset: 64, flags: DIFlagPrivate)
!394 = !DICompositeType(tag: DW_TAG_structure_type, name: "Nanoseconds", scope: !390, file: !2, size: 32, align: 32, flags: DIFlagPrivate, elements: !395, templateParams: !23, identifier: "4b22771ae00e7ac5aecb0d2ebb9344a0")
!395 = !{!396}
!396 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !394, file: !2, baseType: !178, size: 32, align: 32, flags: DIFlagPrivate)
!397 = !DISubprogram(name: "new_debug<core::time::Duration>", linkageName: "_ZN4core3fmt2rt8Argument9new_debug17ha2b41630653ea5a7E", scope: !208, file: !385, line: 116, type: !386, scopeLine: 116, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !398)
!398 = !{!399}
!399 = !DITemplateTypeParameter(name: "T", type: !389)
!400 = !{!383}
!401 = !DILocation(line: 116, column: 36, scope: !384, inlinedAt: !402)
!402 = distinct !DILocation(line: 559, column: 5, scope: !370)
!403 = !DILocalVariable(name: "x", arg: 1, scope: !404, file: !385, line: 92, type: !388)
!404 = distinct !DILexicalBlock(scope: !405, file: !385, line: 92, column: 5)
!405 = distinct !DISubprogram(name: "new<core::time::Duration>", linkageName: "_ZN4core3fmt2rt8Argument3new17h9e5c4f2814c3c7bdE", scope: !208, file: !385, line: 92, type: !406, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !398, declaration: !411, retainedNodes: !412)
!406 = !DISubroutineType(types: !407)
!407 = !{!208, !388, !408}
!408 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&core::time::Duration, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !409, size: 64, align: 64, dwarfAddressSpace: 0)
!409 = !DISubroutineType(types: !410)
!410 = !{!226, !388, !243}
!411 = !DISubprogram(name: "new<core::time::Duration>", linkageName: "_ZN4core3fmt2rt8Argument3new17h9e5c4f2814c3c7bdE", scope: !208, file: !385, line: 92, type: !406, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !398)
!412 = !{!403, !413}
!413 = !DILocalVariable(name: "f", arg: 2, scope: !404, file: !385, line: 92, type: !408)
!414 = !DILocation(line: 92, column: 19, scope: !404, inlinedAt: !415)
!415 = distinct !DILocation(line: 117, column: 9, scope: !384, inlinedAt: !402)
!416 = !DILocation(line: 117, column: 22, scope: !384, inlinedAt: !402)
!417 = !DILocation(line: 92, column: 29, scope: !404, inlinedAt: !415)
!418 = !DILocation(line: 103, column: 21, scope: !404, inlinedAt: !415)
!419 = !DILocation(line: 102, column: 13, scope: !404, inlinedAt: !415)
!420 = !DILocalVariable(name: "x", arg: 1, scope: !421, file: !385, line: 112, type: !424)
!421 = distinct !DISubprogram(name: "new_display<i32>", linkageName: "_ZN4core3fmt2rt8Argument11new_display17hd99ce12318bfeebdE", scope: !208, file: !385, line: 112, type: !422, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !426, declaration: !425, retainedNodes: !428)
!422 = !DISubroutineType(types: !423)
!423 = !{!208, !424}
!424 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&i32", baseType: !99, size: 64, align: 64, dwarfAddressSpace: 0)
!425 = !DISubprogram(name: "new_display<i32>", linkageName: "_ZN4core3fmt2rt8Argument11new_display17hd99ce12318bfeebdE", scope: !208, file: !385, line: 112, type: !422, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !426)
!426 = !{!427}
!427 = !DITemplateTypeParameter(name: "T", type: !99)
!428 = !{!420}
!429 = !DILocation(line: 112, column: 40, scope: !421, inlinedAt: !430)
!430 = distinct !DILocation(line: 559, column: 5, scope: !370)
!431 = !DILocalVariable(name: "x", arg: 1, scope: !432, file: !385, line: 92, type: !424)
!432 = distinct !DILexicalBlock(scope: !433, file: !385, line: 92, column: 5)
!433 = distinct !DISubprogram(name: "new<i32>", linkageName: "_ZN4core3fmt2rt8Argument3new17hafe3e816142f8314E", scope: !208, file: !385, line: 92, type: !434, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !29, templateParams: !426, declaration: !439, retainedNodes: !440)
!434 = !DISubroutineType(types: !435)
!435 = !{!208, !424, !436}
!436 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&i32, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !437, size: 64, align: 64, dwarfAddressSpace: 0)
!437 = !DISubroutineType(types: !438)
!438 = !{!226, !424, !243}
!439 = !DISubprogram(name: "new<i32>", linkageName: "_ZN4core3fmt2rt8Argument3new17hafe3e816142f8314E", scope: !208, file: !385, line: 92, type: !434, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, templateParams: !426)
!440 = !{!431, !441}
!441 = !DILocalVariable(name: "f", arg: 2, scope: !432, file: !385, line: 92, type: !436)
!442 = !DILocation(line: 92, column: 19, scope: !432, inlinedAt: !443)
!443 = distinct !DILocation(line: 113, column: 9, scope: !421, inlinedAt: !430)
!444 = !DILocation(line: 113, column: 22, scope: !421, inlinedAt: !430)
!445 = !DILocation(line: 92, column: 29, scope: !432, inlinedAt: !443)
!446 = !DILocation(line: 103, column: 21, scope: !432, inlinedAt: !443)
!447 = !DILocation(line: 102, column: 13, scope: !432, inlinedAt: !443)
!448 = !DILocation(line: 559, column: 5, scope: !370)
!449 = !DILocation(line: 560, column: 2, scope: !347)
!450 = !DILocation(line: 551, column: 25, scope: !370)
!451 = !DILocation(line: 553, column: 13, scope: !372)
!452 = !DILocation(line: 553, column: 19, scope: !372)
!453 = !DILocation(line: 554, column: 21, scope: !372)
!454 = !DILocation(line: 554, column: 17, scope: !372)
