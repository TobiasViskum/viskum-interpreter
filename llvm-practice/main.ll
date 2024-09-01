; ModuleID = 'main.205e55082b145553-cgu.0'
source_filename = "main.205e55082b145553-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::fmt::rt::Argument<'_>" = type { %"core::fmt::rt::ArgumentType<'_>" }
%"core::fmt::rt::ArgumentType<'_>" = type { [1 x i64], ptr }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2da35ab277f4fe50E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h10256255c7a161d1E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h10256255c7a161d1E" }>, align 8
@alloc_424341b75ece3fa20496f1c69b9fb0ac = private unnamed_addr constant <{ [111 x i8] }> <{ [111 x i8] c"unsafe precondition(s) violated: ptr::write_bytes requires that the destination pointer is aligned and non-null" }>, align 1
@alloc_fad0cd83b7d1858a846a172eb260e593 = private unnamed_addr constant <{ [42 x i8] }> <{ [42 x i8] c"is_aligned_to: align is not a power-of-two" }>, align 1
@alloc_041983ee8170efdaaf95ba67fd072d26 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_fad0cd83b7d1858a846a172eb260e593, [8 x i8] c"*\00\00\00\00\00\00\00" }>, align 8
@0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_97fb9151f1124c12c97ae1401d646f1d = private unnamed_addr constant <{ [81 x i8] }> <{ [81 x i8] c"/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/const_ptr.rs" }>, align 1
@alloc_b93ebc201e5633c55c5ca8e09a9c752e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_97fb9151f1124c12c97ae1401d646f1d, [16 x i8] c"Q\00\00\00\00\00\00\00\86\06\00\00\0D\00\00\00" }>, align 8
@alloc_20b3d155afd5c58c42e598b7e6d186ef = private unnamed_addr constant <{ [93 x i8] }> <{ [93 x i8] c"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null" }>, align 1
@alloc_ec595fc0e82ef92fc59bd74f68296eae = private unnamed_addr constant <{ [73 x i8] }> <{ [73 x i8] c"assertion failed: 0 < pointee_size && pointee_size <= isize::MAX as usize" }>, align 1
@alloc_bfadc9315544832b1b9e6c1e1ca7ad8c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_97fb9151f1124c12c97ae1401d646f1d, [16 x i8] c"Q\00\00\00\00\00\00\00p\03\00\00\09\00\00\00" }>, align 8
@alloc_1f65c820ca68ae2f599734dcc2c915bc = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/alloc/layout.rs" }>, align 1
@alloc_312fbec039d269105e1e1b37557eaa16 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_1f65c820ca68ae2f599734dcc2c915bc, [16 x i8] c"P\00\00\00\00\00\00\00\C3\01\00\00)\00\00\00" }>, align 8
@__rust_no_alloc_shim_is_unstable = external global i8
@1 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] c"\01\00\00\00\00\00\00\80", [8 x i8] undef }>, align 8
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc_3cf8a28b1a0b9f6efeedeb779c4e30d8 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr inttoptr (i64 1 to ptr), [8 x i8] zeroinitializer, ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_c9c83b5dacde997ee6686b983822ba47 = private unnamed_addr constant <{ [33 x i8] }> <{ [33 x i8] c"I am making a progamming langauge" }>, align 1
@alloc_3edef0b68cfa9c8c95e6d4fe1a68842b = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"Hello" }>, align 1
@alloc_788cd4e9a7412aa082bf4e248b638dbd = private unnamed_addr constant <{ [6 x i8] }> <{ [6 x i8] c"World!" }>, align 1
@alloc_0242e8ee118de705af76c627590b82cc = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c" " }>, align 1
@alloc_4e0023beeca5f8e5d06a41f60e7c1e6e = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr inttoptr (i64 1 to ptr), [8 x i8] zeroinitializer, ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_fa49c7126bbf089d50625d8296ac7a1f = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"Result: " }>, align 1
@alloc_d44aaeff7ed57f2fd407cc45b24d7fa9 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_fa49c7126bbf089d50625d8296ac7a1f, [8 x i8] c"\08\00\00\00\00\00\00\00", ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00", ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00", ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_e5af32aee833d2e1c4d09a8b77fd9715 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"NoOp" }>, align 1
@alloc_5fbefac694924822db8fd0f7c11bfc6e = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c"Add" }>, align 1
@alloc_f855d6e9fed35f8dfc3fb4617c12f82f = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c"Sub" }>, align 1

; <alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<&T,core::slice::iter::Iter<T>>>::spec_extend
; Function Attrs: uwtable
define internal void @"_ZN132_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17h92cd9dd083bb00dbE"(ptr noalias noundef align 8 dereferenceable(24) %self, ptr noundef nonnull %0, ptr noundef %1) unnamed_addr #0 {
start:
  %iterator = alloca [16 x i8], align 8
  store ptr %0, ptr %iterator, align 8
  %2 = getelementptr inbounds i8, ptr %iterator, i64 8
  store ptr %1, ptr %2, align 8
; call core::slice::iter::Iter<T>::make_slice
  %3 = call { ptr, i64 } @"_ZN4core5slice4iter13Iter$LT$T$GT$10make_slice17h75bbfc7d1aa544ddE"(ptr noalias noundef readonly align 8 dereferenceable(16) %iterator)
  %slice.0 = extractvalue { ptr, i64 } %3, 0
  %slice.1 = extractvalue { ptr, i64 } %3, 1
; call alloc::vec::Vec<T,A>::append_elements
  call void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$15append_elements17he130bd9716784b40E"(ptr noalias noundef align 8 dereferenceable(24) %self, ptr noundef %slice.0, i64 noundef %slice.1)
  ret void
}

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hba6d22205787d239E(ptr noundef nonnull %f) unnamed_addr #1 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h1d2788fa1499e7ddE(ptr noundef nonnull %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !3
  ret void
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17hfc21bb30c4fbffc2E(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_8 = alloca [8 x i8], align 8
  %_5 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %_5)
  call void @llvm.lifetime.start.p0(i64 8, ptr %_8)
  store ptr %main, ptr %_8, align 8
; call std::rt::lang_start_internal
  %0 = call noundef i64 @_ZN3std2rt19lang_start_internal17hd3ad173a069f2ae5E(ptr noundef nonnull align 1 %_8, ptr noalias noundef readonly align 8 dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  store i64 %0, ptr %_5, align 8
  %v = load i64, ptr %_5, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 8, ptr %_8)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_5)
  ret i64 %v
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h10256255c7a161d1E"(ptr noalias noundef readonly align 8 dereferenceable(8) %_1) unnamed_addr #2 {
start:
  %self = alloca [1 x i8], align 1
  call void @llvm.lifetime.start.p0(i64 1, ptr %self)
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hba6d22205787d239E(ptr noundef nonnull %_4)
; call <() as std::process::Termination>::report
  %0 = call noundef i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hfeb601ef834eda5dE"()
  store i8 %0, ptr %self, align 1
  %_6 = load i8, ptr %self, align 1, !noundef !4
  %_0 = zext i8 %_6 to i32
  call void @llvm.lifetime.end.p0(i64 1, ptr %self)
  ret i32 %_0
}

; <&T as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd63a69400007f2aeE"(ptr noalias noundef readonly align 8 dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !4, !align !5, !noundef !4
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !4
; call <str as core::fmt::Display>::fmt
  %_0 = call noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd248aa3f582d4052E"(ptr noalias noundef nonnull readonly align 1 %_3.0, i64 noundef %_3.1, ptr noalias noundef align 8 dereferenceable(64) %f)
  ret i1 %_0
}

; core::intrinsics::write_bytes::precondition_check
; Function Attrs: inlinehint nounwind uwtable
define internal void @_ZN4core10intrinsics11write_bytes18precondition_check17hbdd3a4488707f69aE(ptr noundef %addr, i64 noundef %align) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = alloca [4 x i8], align 4
  %_8 = alloca [48 x i8], align 8
  %_6 = ptrtoint ptr %addr to i64
  %1 = icmp eq i64 %_6, 0
  br i1 %1, label %bb3, label %bb4

bb3:                                              ; preds = %start
  br label %bb2

bb4:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 4, ptr %0)
  %2 = call i64 @llvm.ctpop.i64(i64 %align)
  %3 = trunc i64 %2 to i32
  store i32 %3, ptr %0, align 4
  %_9 = load i32, ptr %0, align 4, !noundef !4
  call void @llvm.lifetime.end.p0(i64 4, ptr %0)
  %4 = icmp eq i32 %_9, 1
  br i1 %4, label %bb5, label %bb6

bb2:                                              ; preds = %bb5, %bb3
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h0ca77474896e89daE(ptr noalias noundef nonnull readonly align 1 @alloc_424341b75ece3fa20496f1c69b9fb0ac, i64 noundef 111) #21
  unreachable

bb5:                                              ; preds = %bb4
  %_13 = sub i64 %align, 1
  %_12 = and i64 %_6, %_13
  %_3 = icmp eq i64 %_12, 0
  br i1 %_3, label %bb1, label %bb2

bb6:                                              ; preds = %bb4
  call void @llvm.lifetime.start.p0(i64 48, ptr %_8)
  store ptr @alloc_041983ee8170efdaaf95ba67fd072d26, ptr %_8, align 8
  %5 = getelementptr inbounds i8, ptr %_8, i64 8
  store i64 1, ptr %5, align 8
  %6 = load ptr, ptr @0, align 8, !align !6, !noundef !4
  %7 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %8 = getelementptr inbounds i8, ptr %_8, i64 32
  store ptr %6, ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 %7, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %_8, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %10, i64 8
  store i64 0, ptr %11, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17hf3031f82c202a80dE(ptr noalias nocapture noundef readonly align 8 dereferenceable(48) %_8, ptr noalias noundef readonly align 8 dereferenceable(24) @alloc_b93ebc201e5633c55c5ca8e09a9c752e) #22
          to label %unreachable unwind label %terminate

bb1:                                              ; preds = %bb5
  ret void

terminate:                                        ; preds = %bb6
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17he766a33bd515ac66E() #23
  unreachable

unreachable:                                      ; preds = %bb6
  unreachable
}

; core::intrinsics::unlikely
; Function Attrs: nounwind uwtable
define internal noundef zeroext i1 @_ZN4core10intrinsics8unlikely17h0152b546a6961350E(i1 noundef zeroext %b) unnamed_addr #4 {
start:
  ret i1 %b
}

; core::cmp::impls::<impl core::cmp::Ord for usize>::cmp
; Function Attrs: inlinehint uwtable
define internal noundef i8 @"_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17haf4664b167a967cbE"(ptr noalias noundef readonly align 8 dereferenceable(8) %self, ptr noalias noundef readonly align 8 dereferenceable(8) %other) unnamed_addr #2 {
start:
  %_3 = load i64, ptr %self, align 8, !noundef !4
  %_4 = load i64, ptr %other, align 8, !noundef !4
  %0 = icmp ult i64 %_3, %_4
  %1 = icmp ne i64 %_3, %_4
  %2 = select i1 %1, i8 1, i8 0
  %_0 = select i1 %0, i8 -1, i8 %2
  ret i8 %_0
}

; core::cmp::max_by
; Function Attrs: inlinehint uwtable
define internal noundef i64 @_ZN4core3cmp6max_by17he5bc204448e63f16E(i64 noundef %0, i64 noundef %1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %2 = alloca [16 x i8], align 8
  %_9 = alloca [1 x i8], align 1
  %_4 = alloca [1 x i8], align 1
  %_0 = alloca [8 x i8], align 8
  %v2 = alloca [8 x i8], align 8
  %v1 = alloca [8 x i8], align 8
  store i64 %0, ptr %v1, align 8
  store i64 %1, ptr %v2, align 8
  store i8 1, ptr %_9, align 1
  call void @llvm.lifetime.start.p0(i64 1, ptr %_4)
; invoke core::ops::function::FnOnce::call_once
  %3 = invoke noundef i8 @_ZN4core3ops8function6FnOnce9call_once17h2743d8e375eb3596E(ptr noalias noundef readonly align 8 dereferenceable(8) %v1, ptr noalias noundef readonly align 8 dereferenceable(8) %v2)
          to label %bb1 unwind label %cleanup, !range !7

bb6:                                              ; preds = %cleanup
  br label %bb10

cleanup:                                          ; preds = %start
  %4 = landingpad { ptr, i32 }
          cleanup
  %5 = extractvalue { ptr, i32 } %4, 0
  %6 = extractvalue { ptr, i32 } %4, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %2)
  store ptr %5, ptr %2, align 8
  %7 = getelementptr inbounds i8, ptr %2, i64 8
  store i32 %6, ptr %7, align 8
  br label %bb6

bb1:                                              ; preds = %start
  store i8 %3, ptr %_4, align 1
  %_8 = load i8, ptr %_4, align 1, !range !7, !noundef !4
  switch i8 %_8, label %bb2 [
    i8 -1, label %bb4
    i8 0, label %bb4
    i8 1, label %bb3
  ]

bb2:                                              ; preds = %bb1
  unreachable

bb4:                                              ; preds = %bb1, %bb1
  %8 = load i64, ptr %v2, align 8, !noundef !4
  store i64 %8, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 1, ptr %_4)
  %9 = load i8, ptr %_9, align 1, !range !8, !noundef !4
  %10 = trunc i8 %9 to i1
  br i1 %10, label %bb8, label %bb5

bb3:                                              ; preds = %bb1
  store i8 0, ptr %_9, align 1
  %11 = load i64, ptr %v1, align 8, !noundef !4
  store i64 %11, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 1, ptr %_4)
  br label %bb5

bb5:                                              ; preds = %bb3, %bb8, %bb4
  %12 = load i64, ptr %_0, align 8, !noundef !4
  ret i64 %12

bb8:                                              ; preds = %bb4
  br label %bb5

bb10:                                             ; preds = %bb6
  %13 = load i8, ptr %_9, align 1, !range !8, !noundef !4
  %14 = trunc i8 %13 to i1
  br i1 %14, label %bb9, label %bb7

bb7:                                              ; preds = %bb9, %bb10
  %15 = load ptr, ptr %2, align 8, !noundef !4
  %16 = getelementptr inbounds i8, ptr %2, i64 8
  %17 = load i32, ptr %16, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %2)
  %18 = insertvalue { ptr, i32 } poison, ptr %15, 0
  %19 = insertvalue { ptr, i32 } %18, i32 %17, 1
  resume { ptr, i32 } %19

bb9:                                              ; preds = %bb10
  br label %bb7
}

; core::fmt::num::<impl core::fmt::Debug for i32>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h5ebe2532ddb9230eE"(ptr noalias noundef readonly align 4 dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #2 {
start:
  %_0 = alloca [1 x i8], align 1
  %0 = getelementptr inbounds i8, ptr %f, i64 52
  %_4 = load i32, ptr %0, align 4, !noundef !4
  %_3 = and i32 %_4, 16
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %2 = getelementptr inbounds i8, ptr %f, i64 52
  %_6 = load i32, ptr %2, align 4, !noundef !4
  %_5 = and i32 %_6, 32
  %3 = icmp eq i32 %_5, 0
  br i1 %3, label %bb4, label %bb3

bb1:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
  %4 = call noundef zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17hdb5970dd58d96e1dE"(ptr noalias noundef readonly align 4 dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(64) %f)
  %5 = zext i1 %4 to i8
  store i8 %5, ptr %_0, align 1
  br label %bb6

bb4:                                              ; preds = %bb2
; call core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
  %6 = call noundef zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E"(ptr noalias noundef readonly align 4 dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(64) %f)
  %7 = zext i1 %6 to i8
  store i8 %7, ptr %_0, align 1
  br label %bb5

bb3:                                              ; preds = %bb2
; call core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
  %8 = call noundef zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc15a4a0cd502e3bcE"(ptr noalias noundef readonly align 4 dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(64) %f)
  %9 = zext i1 %8 to i8
  store i8 %9, ptr %_0, align 1
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  br label %bb6

bb6:                                              ; preds = %bb1, %bb5
  %10 = load i8, ptr %_0, align 1, !range !8, !noundef !4
  %11 = trunc i8 %10 to i1
  ret i1 %11
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2da35ab277f4fe50E"(ptr noundef %_1) unnamed_addr #2 {
start:
  %_2 = alloca [0 x i8], align 1
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call core::ops::function::FnOnce::call_once
  %_0 = call noundef i32 @_ZN4core3ops8function6FnOnce9call_once17hc1318dd7ed2121aaE(ptr noundef nonnull %0)
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h1d2788fa1499e7ddE(ptr noundef nonnull %_1) unnamed_addr #2 {
start:
  %_2 = alloca [0 x i8], align 1
  call void %_1()
  ret void
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal noundef i8 @_ZN4core3ops8function6FnOnce9call_once17h2743d8e375eb3596E(ptr noalias noundef readonly align 8 dereferenceable(8) %0, ptr noalias noundef readonly align 8 dereferenceable(8) %1) unnamed_addr #2 {
start:
  %_2 = alloca [16 x i8], align 8
  store ptr %0, ptr %_2, align 8
  %2 = getelementptr inbounds i8, ptr %_2, i64 8
  store ptr %1, ptr %2, align 8
  %3 = load ptr, ptr %_2, align 8, !nonnull !4, !align !6, !noundef !4
  %4 = getelementptr inbounds i8, ptr %_2, i64 8
  %5 = load ptr, ptr %4, align 8, !nonnull !4, !align !6, !noundef !4
; call core::cmp::impls::<impl core::cmp::Ord for usize>::cmp
  %_0 = call noundef i8 @"_ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17haf4664b167a967cbE"(ptr noalias noundef readonly align 8 dereferenceable(8) %3, ptr noalias noundef readonly align 8 dereferenceable(8) %5), !range !7
  ret i8 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_ZN4core3ops8function6FnOnce9call_once17hc1318dd7ed2121aaE(ptr noundef nonnull %0) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %_0 = invoke noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h10256255c7a161d1E"(ptr noalias noundef readonly align 8 dereferenceable(8) %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8, !noundef !4
  %3 = getelementptr inbounds i8, ptr %1, i64 8
  %4 = load i32, ptr %3, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %1)
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1
  resume { ptr, i32 } %6

cleanup:                                          ; preds = %start
  %7 = landingpad { ptr, i32 }
          cleanup
  %8 = extractvalue { ptr, i32 } %7, 0
  %9 = extractvalue { ptr, i32 } %7, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %1)
  store ptr %8, ptr %1, align 8
  %10 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %9, ptr %10, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hcea270a94e177a16E(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %0, i64 noundef %1) unnamed_addr #2 {
start:
  %_2 = alloca [16 x i8], align 8
  store ptr %0, ptr %_2, align 8
  %2 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 %1, ptr %2, align 8
  %3 = load ptr, ptr %_2, align 8, !nonnull !4, !align !5, !noundef !4
  %4 = getelementptr inbounds i8, ptr %_2, i64 8
  %5 = load i64, ptr %4, align 8, !noundef !4
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  call void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h12890283f9e53a37E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %3, i64 noundef %5)
  ret void
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h6e310eb1b38c7843E"(ptr noalias noundef align 8 dereferenceable(24) %_1) unnamed_addr #0 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb6b2f82722c8c530E"(ptr noalias noundef align 8 dereferenceable(24) %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb6b2f82722c8c530E"(ptr noalias noundef align 8 dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h770afccffcf33d3eE"(ptr noalias noundef align 8 dereferenceable(24) %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  invoke void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc6571d6d0c59ea66E"(ptr noalias noundef align 8 dereferenceable(16) %_1) #24
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %0)
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc6571d6d0c59ea66E"(ptr noalias noundef align 8 dereferenceable(16) %_1)
  ret void

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() #23
  unreachable

bb1:                                              ; preds = %bb3
  %8 = load ptr, ptr %0, align 8, !noundef !4
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  %10 = load i32, ptr %9, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %0)
  %11 = insertvalue { ptr, i32 } poison, ptr %8, 0
  %12 = insertvalue { ptr, i32 } %11, i32 %10, 1
  resume { ptr, i32 } %12
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc6571d6d0c59ea66E"(ptr noalias noundef align 8 dereferenceable(16) %_1) unnamed_addr #0 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8043150a46d436d6E"(ptr noalias noundef align 8 dereferenceable(16) %_1)
  ret void
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h2e0681300b3e3fe2E"(ptr noalias noundef align 8 dereferenceable(8) %_1) unnamed_addr #2 {
start:
  ret void
}

; core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
; Function Attrs: inlinehint nounwind uwtable
define internal void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h3dcb9f87e643d327E"(ptr noundef %ptr) unnamed_addr #3 {
start:
  %_4 = ptrtoint ptr %ptr to i64
  %0 = icmp eq i64 %_4, 0
  br i1 %0, label %bb1, label %bb2

bb1:                                              ; preds = %start
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h0ca77474896e89daE(ptr noalias noundef nonnull readonly align 1 @alloc_20b3d155afd5c58c42e598b7e6d186ef, i64 noundef 93) #21
  unreachable

bb2:                                              ; preds = %start
  ret void
}

; core::ptr::const_ptr::<impl *const T>::sub_ptr
; Function Attrs: inlinehint uwtable
define internal noundef i64 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h54566039e89830feE"(ptr noundef %self, ptr noundef %origin) unnamed_addr #2 {
start:
  %0 = alloca [8 x i8], align 8
  br label %bb3

bb3:                                              ; preds = %start
  br label %bb4

bb1:                                              ; No predecessors!
  unreachable

bb2:                                              ; No predecessors!
  unreachable

bb4:                                              ; preds = %bb3
  br label %bb5

bb5:                                              ; preds = %bb4
  call void @llvm.lifetime.start.p0(i64 8, ptr %0)
  %1 = ptrtoint ptr %self to i64
  %2 = ptrtoint ptr %origin to i64
  %3 = sub nuw i64 %1, %2
  %4 = udiv exact i64 %3, 1
  store i64 %4, ptr %0, align 8
  %_0 = load i64, ptr %0, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 8, ptr %0)
  ret i64 %_0

bb6:                                              ; No predecessors!
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hfb97fcfe2d432218E(ptr noalias noundef nonnull readonly align 1 @alloc_ec595fc0e82ef92fc59bd74f68296eae, i64 noundef 73, ptr noalias noundef readonly align 8 dereferenceable(24) @alloc_bfadc9315544832b1b9e6c1e1ca7ad8c) #22
  unreachable
}

; core::alloc::layout::Layout::array::inner
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h1e2e6b6a7c48ef5dE(i64 noundef %element_size, i64 noundef %align, i64 noundef %n) unnamed_addr #2 {
start:
  %_20 = alloca [8 x i8], align 8
  %_13 = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %0 = icmp eq i64 %element_size, 0
  br i1 %0, label %bb5, label %bb1

bb5:                                              ; preds = %bb4, %start
  br label %bb8

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr %_13)
  store i64 %align, ptr %_13, align 8
  %_14 = load i64, ptr %_13, align 8, !range !9, !noundef !4
  %_15 = icmp uge i64 %_14, 1
  %_16 = icmp ule i64 %_14, -9223372036854775808
  %_17 = and i1 %_15, %_16
  call void @llvm.assume(i1 %_17)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_13)
  %_11 = sub i64 %_14, 1
  %_6 = sub i64 9223372036854775807, %_11
  %_7 = icmp eq i64 %element_size, 0
  br i1 %_7, label %panic, label %bb2

bb2:                                              ; preds = %bb1
  %_5 = udiv i64 %_6, %element_size
  %_4 = icmp ugt i64 %n, %_5
  br i1 %_4, label %bb3, label %bb4

panic:                                            ; preds = %bb1
; call core::panicking::panic_const::panic_const_div_by_zero
  call void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h6a572ca1aae6312bE(ptr noalias noundef readonly align 8 dereferenceable(24) @alloc_312fbec039d269105e1e1b37557eaa16) #22
  unreachable

bb4:                                              ; preds = %bb2
  br label %bb5

bb3:                                              ; preds = %bb2
  %1 = load i64, ptr @0, align 8, !range !10, !noundef !4
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %1, ptr %_0, align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %2, ptr %3, align 8
  br label %bb6

bb8:                                              ; preds = %bb5
  %array_size = mul nuw i64 %element_size, %n
  call void @llvm.lifetime.start.p0(i64 8, ptr %_20)
  store i64 %align, ptr %_20, align 8
  %_21 = load i64, ptr %_20, align 8, !range !9, !noundef !4
  %_22 = icmp uge i64 %_21, 1
  %_23 = icmp ule i64 %_21, -9223372036854775808
  %_24 = and i1 %_22, %_23
  call void @llvm.assume(i1 %_24)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_20)
  %4 = icmp uge i64 %_21, 1
  call void @llvm.assume(i1 %4)
  %5 = icmp ule i64 %_21, -9223372036854775808
  call void @llvm.assume(i1 %5)
  store i64 %_21, ptr %_0, align 8
  %6 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %array_size, ptr %6, align 8
  br label %bb6

bb7:                                              ; No predecessors!
  unreachable

bb6:                                              ; preds = %bb3, %bb8
  %7 = load i64, ptr %_0, align 8, !range !10, !noundef !4
  %8 = getelementptr inbounds i8, ptr %_0, i64 8
  %9 = load i64, ptr %8, align 8
  %10 = insertvalue { i64, i64 } poison, i64 %7, 0
  %11 = insertvalue { i64, i64 } %10, i64 %9, 1
  ret { i64, i64 } %11
}

; core::alloc::layout::Layout::dangling
; Function Attrs: inlinehint uwtable
define internal noundef nonnull ptr @_ZN4core5alloc6layout6Layout8dangling17h837658e2318fa047E(ptr noalias noundef readonly align 8 dereferenceable(16) %self) unnamed_addr #2 {
start:
  %_5 = alloca [8 x i8], align 8
  %self1 = load i64, ptr %self, align 8, !range !9, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_5)
  store i64 %self1, ptr %_5, align 8
  %_6 = load i64, ptr %_5, align 8, !range !9, !noundef !4
  %_7 = icmp uge i64 %_6, 1
  %_8 = icmp ule i64 %_6, -9223372036854775808
  %_9 = and i1 %_7, %_8
  call void @llvm.assume(i1 %_9)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_5)
  %ptr = getelementptr i8, ptr null, i64 %_6
  br label %bb3

bb3:                                              ; preds = %start
  ret ptr %ptr

bb1:                                              ; No predecessors!
  unreachable

bb2:                                              ; No predecessors!
  unreachable
}

; core::slice::iter::Iter<T>::make_slice
; Function Attrs: alwaysinline uwtable
define internal { ptr, i64 } @"_ZN4core5slice4iter13Iter$LT$T$GT$10make_slice17h75bbfc7d1aa544ddE"(ptr noalias noundef readonly align 8 dereferenceable(16) %self) unnamed_addr #5 {
start:
  %len = alloca [8 x i8], align 8
  %self1 = load ptr, ptr %self, align 8, !nonnull !4, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %len)
  br label %bb2

bb2:                                              ; preds = %start
  %self2 = getelementptr inbounds i8, ptr %self, i64 8
  %end = load ptr, ptr %self2, align 8, !nonnull !4, !noundef !4
; call core::ptr::const_ptr::<impl *const T>::sub_ptr
  %0 = call noundef i64 @"_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$7sub_ptr17h54566039e89830feE"(ptr noundef %end, ptr noundef %self1)
  store i64 %0, ptr %len, align 8
  br label %bb3

bb3:                                              ; preds = %bb2
  br label %bb7

bb1:                                              ; No predecessors!
  unreachable

bb7:                                              ; preds = %bb3
  %_18.1 = load i64, ptr %len, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 8, ptr %len)
  %1 = insertvalue { ptr, i64 } poison, ptr %self1, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_18.1, 1
  ret { ptr, i64 } %2

bb5:                                              ; No predecessors!
  unreachable

bb6:                                              ; No predecessors!
  unreachable
}

; core::option::Option<T>::map_or_else
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17h866827687608a716E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 1 %0, i64 %1, ptr noalias noundef readonly align 8 dereferenceable(48) %default) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %2 = alloca [16 x i8], align 8
  %_10 = alloca [1 x i8], align 1
  %_9 = alloca [1 x i8], align 1
  %self = alloca [16 x i8], align 8
  store ptr %0, ptr %self, align 8
  %3 = getelementptr inbounds i8, ptr %self, i64 8
  store i64 %1, ptr %3, align 8
  store i8 1, ptr %_10, align 1
  store i8 1, ptr %_9, align 1
  %4 = load ptr, ptr %self, align 8, !noundef !4
  %5 = ptrtoint ptr %4 to i64
  %6 = icmp eq i64 %5, 0
  %_4 = select i1 %6, i64 0, i64 1
  switch i64 %_4, label %bb1 [
    i64 0, label %bb2
    i64 1, label %bb3
  ]

bb1:                                              ; preds = %start
  unreachable

bb2:                                              ; preds = %start
  store i8 0, ptr %_10, align 1
; invoke alloc::fmt::format::{{closure}}
  invoke void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h0d0e7b9263ef693aE"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 8 dereferenceable(48) %default)
          to label %bb5 unwind label %cleanup

bb3:                                              ; preds = %start
  %t.0 = load ptr, ptr %self, align 8, !nonnull !4, !align !5, !noundef !4
  %7 = getelementptr inbounds i8, ptr %self, i64 8
  %t.1 = load i64, ptr %7, align 8, !noundef !4
  store i8 0, ptr %_9, align 1
; invoke core::ops::function::FnOnce::call_once
  invoke void @_ZN4core3ops8function6FnOnce9call_once17hcea270a94e177a16E(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %t.0, i64 noundef %t.1)
          to label %bb4 unwind label %cleanup

bb11:                                             ; preds = %cleanup
  %8 = load i8, ptr %_9, align 1, !range !8, !noundef !4
  %9 = trunc i8 %8 to i1
  br i1 %9, label %bb10, label %bb7

cleanup:                                          ; preds = %bb3, %bb2
  %10 = landingpad { ptr, i32 }
          cleanup
  %11 = extractvalue { ptr, i32 } %10, 0
  %12 = extractvalue { ptr, i32 } %10, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %2)
  store ptr %11, ptr %2, align 8
  %13 = getelementptr inbounds i8, ptr %2, i64 8
  store i32 %12, ptr %13, align 8
  br label %bb11

bb5:                                              ; preds = %bb2
  br label %bb6

bb6:                                              ; preds = %bb9, %bb4, %bb5
  ret void

bb4:                                              ; preds = %bb3
  %14 = load i8, ptr %_10, align 1, !range !8, !noundef !4
  %15 = trunc i8 %14 to i1
  br i1 %15, label %bb9, label %bb6

bb9:                                              ; preds = %bb4
  br label %bb6

bb7:                                              ; preds = %bb10, %bb11
  %16 = load i8, ptr %_10, align 1, !range !8, !noundef !4
  %17 = trunc i8 %16 to i1
  br i1 %17, label %bb12, label %bb8

bb10:                                             ; preds = %bb11
  br label %bb7

bb8:                                              ; preds = %bb12, %bb7
  %18 = load ptr, ptr %2, align 8, !noundef !4
  %19 = getelementptr inbounds i8, ptr %2, i64 8
  %20 = load i32, ptr %19, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %2)
  %21 = insertvalue { ptr, i32 } poison, ptr %18, 0
  %22 = insertvalue { ptr, i32 } %21, i32 %20, 1
  resume { ptr, i32 } %22

bb12:                                             ; preds = %bb7
  br label %bb8
}

; <T as alloc::slice::hack::ConvertVec>::to_vec
; Function Attrs: inlinehint uwtable
define internal void @"_ZN52_$LT$T$u20$as$u20$alloc..slice..hack..ConvertVec$GT$6to_vec17h5d2e00662e73db50E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %s.0, i64 noundef %s.1) unnamed_addr #2 {
start:
  %_12 = alloca [24 x i8], align 8
  %v = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr %v)
  call void @llvm.lifetime.start.p0(i64 24, ptr %_12)
; call alloc::raw_vec::RawVec<T,A>::try_allocate_in
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17hfa11d2430c197263E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_12, i64 noundef %s.1, i1 noundef zeroext false)
  %_13 = load i64, ptr %_12, align 8, !range !11, !noundef !4
  switch i64 %_13, label %bb2 [
    i64 0, label %bb4
    i64 1, label %bb3
  ]

bb2:                                              ; preds = %start
  unreachable

bb4:                                              ; preds = %start
  %0 = getelementptr inbounds i8, ptr %_12, i64 8
  %res.0 = load i64, ptr %0, align 8, !range !12, !noundef !4
  %1 = getelementptr inbounds i8, ptr %0, i64 8
  %res.1 = load ptr, ptr %1, align 8, !nonnull !4, !noundef !4
  call void @llvm.lifetime.end.p0(i64 24, ptr %_12)
  store i64 %res.0, ptr %v, align 8
  %2 = getelementptr inbounds i8, ptr %v, i64 8
  store ptr %res.1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %v, i64 16
  store i64 0, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %v, i64 8
  %self = load ptr, ptr %4, align 8, !nonnull !4, !noundef !4
  br label %bb7

bb3:                                              ; preds = %start
  %5 = getelementptr inbounds i8, ptr %_12, i64 8
  %err.0 = load i64, ptr %5, align 8, !range !10, !noundef !4
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  %err.1 = load i64, ptr %6, align 8
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h7f9cd8199ddef69cE(i64 noundef %err.0, i64 %err.1) #22
  unreachable

bb7:                                              ; preds = %bb4
  %7 = mul i64 %s.1, 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %self, ptr align 1 %s.0, i64 %7, i1 false)
  %8 = getelementptr inbounds i8, ptr %v, i64 16
  store i64 %s.1, ptr %8, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %v, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr %v)
  ret void

bb5:                                              ; No predecessors!
  unreachable

bb6:                                              ; No predecessors!
  unreachable
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal noundef i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hfeb601ef834eda5dE"() unnamed_addr #2 {
start:
  ret i8 0
}

; alloc::fmt::format
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc3fmt6format17h58dbea2d6d469c4dE(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias nocapture noundef readonly align 8 dereferenceable(48) %args) unnamed_addr #2 {
start:
  %_2 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_2)
  %_6.0 = load ptr, ptr %args, align 8, !nonnull !4, !align !6, !noundef !4
  %0 = getelementptr inbounds i8, ptr %args, i64 8
  %_6.1 = load i64, ptr %0, align 8, !noundef !4
  %1 = getelementptr inbounds i8, ptr %args, i64 16
  %_7.0 = load ptr, ptr %1, align 8, !nonnull !4, !align !6, !noundef !4
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  %_7.1 = load i64, ptr %2, align 8, !noundef !4
  %3 = icmp eq i64 %_6.1, 0
  br i1 %3, label %bb4, label %bb5

bb4:                                              ; preds = %start
  %4 = icmp eq i64 %_7.1, 0
  br i1 %4, label %bb7, label %bb3

bb5:                                              ; preds = %start
  %5 = icmp eq i64 %_6.1, 1
  br i1 %5, label %bb6, label %bb3

bb7:                                              ; preds = %bb4
  store ptr inttoptr (i64 1 to ptr), ptr %_2, align 8
  %6 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 0, ptr %6, align 8
  br label %bb2

bb3:                                              ; preds = %bb6, %bb5, %bb4
  %7 = load ptr, ptr @0, align 8, !align !5, !noundef !4
  %8 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %7, ptr %_2, align 8
  %9 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 %8, ptr %9, align 8
  br label %bb2

bb2:                                              ; preds = %bb3, %bb8, %bb7
  %10 = load ptr, ptr %_2, align 8, !align !5, !noundef !4
  %11 = getelementptr inbounds i8, ptr %_2, i64 8
  %12 = load i64, ptr %11, align 8
; call core::option::Option<T>::map_or_else
  call void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17h866827687608a716E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 1 %10, i64 %12, ptr noalias noundef readonly align 8 dereferenceable(48) %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_2)
  ret void

bb6:                                              ; preds = %bb5
  %13 = icmp eq i64 %_7.1, 0
  br i1 %13, label %bb8, label %bb3

bb8:                                              ; preds = %bb6
  %s = getelementptr inbounds [0 x { ptr, i64 }], ptr %_6.0, i64 0, i64 0
  %14 = getelementptr inbounds [0 x { ptr, i64 }], ptr %_6.0, i64 0, i64 0
  %_13.0 = load ptr, ptr %14, align 8, !nonnull !4, !align !5, !noundef !4
  %15 = getelementptr inbounds i8, ptr %14, i64 8
  %_13.1 = load i64, ptr %15, align 8, !noundef !4
  store ptr %_13.0, ptr %_2, align 8
  %16 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 %_13.1, ptr %16, align 8
  br label %bb2
}

; alloc::fmt::format::{{closure}}
; Function Attrs: inlinehint uwtable
define internal void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h0d0e7b9263ef693aE"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 8 dereferenceable(48) %_1) unnamed_addr #2 {
start:
  %_2 = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr %_2)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_2, ptr align 8 %_1, i64 48, i1 false)
; call alloc::fmt::format::format_inner
  call void @_ZN5alloc3fmt6format12format_inner17h457d6f4cc84042e9E(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias nocapture noundef readonly align 8 dereferenceable(48) %_2)
  call void @llvm.lifetime.end.p0(i64 48, ptr %_2)
  ret void
}

; alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
; Function Attrs: inlinehint uwtable
define internal void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h12890283f9e53a37E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %self.0, i64 noundef %self.1) unnamed_addr #2 {
start:
  %bytes = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr %bytes)
; call <T as alloc::slice::hack::ConvertVec>::to_vec
  call void @"_ZN52_$LT$T$u20$as$u20$alloc..slice..hack..ConvertVec$GT$6to_vec17h5d2e00662e73db50E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %bytes, ptr noalias noundef nonnull readonly align 1 %self.0, i64 noundef %self.1)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %bytes, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr %bytes)
  ret void
}

; alloc::vec::Vec<T,A>::append_elements
; Function Attrs: inlinehint uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$15append_elements17he130bd9716784b40E"(ptr noalias noundef align 8 dereferenceable(24) %self, ptr noundef %other.0, i64 noundef %other.1) unnamed_addr #2 {
start:
; call alloc::vec::Vec<T,A>::reserve
  call void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h14cd26c7af4f2c24E"(ptr noalias noundef align 8 dereferenceable(24) %self, i64 noundef %other.1)
  %0 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %0, align 8, !noundef !4
  %1 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %1, align 8, !nonnull !4, !noundef !4
  %dst = getelementptr inbounds i8, ptr %self1, i64 %len
  br label %bb4

bb4:                                              ; preds = %start
  %2 = mul i64 %other.1, 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %dst, ptr align 1 %other.0, i64 %2, i1 false)
  %3 = getelementptr inbounds i8, ptr %self, i64 16
  %4 = getelementptr inbounds i8, ptr %self, i64 16
  %5 = load i64, ptr %4, align 8, !noundef !4
  %6 = add i64 %5, %other.1
  store i64 %6, ptr %3, align 8
  ret void

bb2:                                              ; No predecessors!
  unreachable

bb3:                                              ; No predecessors!
  unreachable
}

; alloc::vec::Vec<T,A>::extend_from_slice
; Function Attrs: uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17hb14ca464f4c24098E"(ptr noalias noundef align 8 dereferenceable(24) %self, ptr noalias noundef nonnull readonly align 1 %other.0, i64 noundef %other.1) unnamed_addr #0 {
start:
  %end_or_len = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %end_or_len)
  br label %bb3

bb3:                                              ; preds = %start
  %_9 = getelementptr inbounds i8, ptr %other.0, i64 %other.1
  store ptr %_9, ptr %end_or_len, align 8
  br label %bb4

bb4:                                              ; preds = %bb3
  %_11 = load ptr, ptr %end_or_len, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 8, ptr %end_or_len)
; call <alloc::vec::Vec<T,A> as alloc::vec::spec_extend::SpecExtend<&T,core::slice::iter::Iter<T>>>::spec_extend
  call void @"_ZN132_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$alloc..vec..spec_extend..SpecExtend$LT$$RF$T$C$core..slice..iter..Iter$LT$T$GT$$GT$$GT$11spec_extend17h92cd9dd083bb00dbE"(ptr noalias noundef align 8 dereferenceable(24) %self, ptr noundef nonnull %other.0, ptr noundef %_11)
  ret void

bb2:                                              ; No predecessors!
  unreachable
}

; alloc::vec::Vec<T,A>::reserve
; Function Attrs: uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h14cd26c7af4f2c24E"(ptr noalias noundef align 8 dereferenceable(24) %self, i64 noundef %additional) unnamed_addr #0 {
start:
  %self1 = alloca [8 x i8], align 8
  %0 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %0, align 8, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %self1)
  br label %bb4

bb4:                                              ; preds = %start
  %1 = load i64, ptr %self, align 8, !noundef !4
  store i64 %1, ptr %self1, align 8
  br label %bb5

bb5:                                              ; preds = %bb4
  %2 = load i64, ptr %self1, align 8, !noundef !4
  %_7 = sub i64 %2, %len
  call void @llvm.lifetime.end.p0(i64 8, ptr %self1)
  %_5 = icmp ugt i64 %additional, %_7
  br i1 %_5, label %bb1, label %bb2

bb3:                                              ; No predecessors!
  unreachable

bb2:                                              ; preds = %bb1, %bb5
  ret void

bb1:                                              ; preds = %bb5
; call alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hf19a3efabb9eed6fE"(ptr noalias noundef align 8 dereferenceable(16) %self, i64 noundef %len, i64 noundef %additional)
  br label %bb2
}

; alloc::alloc::alloc
; Function Attrs: inlinehint uwtable
define internal noundef ptr @_ZN5alloc5alloc5alloc17h1ece6c5426488636E(i64 noundef %0, i64 noundef %1) unnamed_addr #2 {
start:
  %2 = alloca [1 x i8], align 1
  %_11 = alloca [8 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %3, align 8
  br label %bb5

bb5:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 1, ptr %2)
  %4 = load volatile i8, ptr @__rust_no_alloc_shim_is_unstable, align 1
  store i8 %4, ptr %2, align 1
  %_2 = load i8, ptr %2, align 1, !noundef !4
  call void @llvm.lifetime.end.p0(i64 1, ptr %2)
  %5 = getelementptr inbounds i8, ptr %layout, i64 8
  %_3 = load i64, ptr %5, align 8, !noundef !4
  %self = load i64, ptr %layout, align 8, !range !9, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_11)
  store i64 %self, ptr %_11, align 8
  %_12 = load i64, ptr %_11, align 8, !range !9, !noundef !4
  %_13 = icmp uge i64 %_12, 1
  %_14 = icmp ule i64 %_12, -9223372036854775808
  %_15 = and i1 %_13, %_14
  call void @llvm.assume(i1 %_15)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_11)
  %_0 = call noundef ptr @__rust_alloc(i64 noundef %_3, i64 noundef %_12) #25
  ret ptr %_0

bb3:                                              ; No predecessors!
  unreachable

bb4:                                              ; No predecessors!
  unreachable
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %0, i64 noundef %1, i1 noundef zeroext %zeroed) unnamed_addr #2 {
start:
  %_29 = alloca [8 x i8], align 8
  %self3 = alloca [8 x i8], align 8
  %self2 = alloca [8 x i8], align 8
  %_11 = alloca [8 x i8], align 8
  %layout1 = alloca [16 x i8], align 8
  %raw_ptr = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %size = load i64, ptr %3, align 8, !noundef !4
  %4 = icmp eq i64 %size, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %start
; call core::alloc::layout::Layout::dangling
  %data = call noundef nonnull ptr @_ZN4core5alloc6layout6Layout8dangling17h837658e2318fa047E(ptr noalias noundef readonly align 8 dereferenceable(16) %layout)
  br label %bb10

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr %raw_ptr)
  br i1 %zeroed, label %bb4, label %bb5

bb10:                                             ; preds = %bb2
  store ptr %data, ptr %_0, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 0, ptr %5, align 8
  br label %bb7

bb8:                                              ; No predecessors!
  unreachable

bb9:                                              ; No predecessors!
  unreachable

bb7:                                              ; preds = %bb18, %bb12, %bb10
  %6 = load ptr, ptr %_0, align 8, !noundef !4
  %7 = getelementptr inbounds i8, ptr %_0, i64 8
  %8 = load i64, ptr %7, align 8
  %9 = insertvalue { ptr, i64 } poison, ptr %6, 0
  %10 = insertvalue { ptr, i64 } %9, i64 %8, 1
  ret { ptr, i64 } %10

bb5:                                              ; preds = %bb1
  %11 = load i64, ptr %layout, align 8, !range !9, !noundef !4
  %12 = getelementptr inbounds i8, ptr %layout, i64 8
  %13 = load i64, ptr %12, align 8, !noundef !4
; call alloc::alloc::alloc
  %14 = call noundef ptr @_ZN5alloc5alloc5alloc17h1ece6c5426488636E(i64 noundef %11, i64 noundef %13)
  store ptr %14, ptr %raw_ptr, align 8
  br label %bb6

bb4:                                              ; preds = %bb1
  call void @llvm.lifetime.start.p0(i64 16, ptr %layout1)
  %15 = load i64, ptr %layout, align 8, !range !9, !noundef !4
  %16 = getelementptr inbounds i8, ptr %layout, i64 8
  %17 = load i64, ptr %16, align 8, !noundef !4
  store i64 %15, ptr %layout1, align 8
  %18 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %17, ptr %18, align 8
  %self4 = load i64, ptr %layout, align 8, !range !9, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_29)
  store i64 %self4, ptr %_29, align 8
  %_30 = load i64, ptr %_29, align 8, !range !9, !noundef !4
  %_31 = icmp uge i64 %_30, 1
  %_32 = icmp ule i64 %_30, -9223372036854775808
  %_33 = and i1 %_31, %_32
  call void @llvm.assume(i1 %_33)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_29)
  %19 = call noundef ptr @__rust_alloc_zeroed(i64 noundef %size, i64 noundef %_30) #25
  store ptr %19, ptr %raw_ptr, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout1)
  br label %bb6

bb6:                                              ; preds = %bb4, %bb5
  call void @llvm.lifetime.start.p0(i64 8, ptr %_11)
  call void @llvm.lifetime.start.p0(i64 8, ptr %self2)
  call void @llvm.lifetime.start.p0(i64 8, ptr %self3)
  %ptr = load ptr, ptr %raw_ptr, align 8, !noundef !4
  %_35 = ptrtoint ptr %ptr to i64
  %20 = icmp eq i64 %_35, 0
  br i1 %20, label %bb12, label %bb13

bb12:                                             ; preds = %bb6
  store ptr null, ptr %self3, align 8
  store ptr null, ptr %self2, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %self3)
  call void @llvm.lifetime.end.p0(i64 8, ptr %self2)
  %21 = load ptr, ptr @0, align 8, !noundef !4
  %22 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %21, ptr %_0, align 8
  %23 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %22, ptr %23, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %_11)
  call void @llvm.lifetime.end.p0(i64 8, ptr %raw_ptr)
  br label %bb7

bb13:                                             ; preds = %bb6
  br label %bb15

bb15:                                             ; preds = %bb13
  store ptr %ptr, ptr %self3, align 8
  %v = load ptr, ptr %self3, align 8, !nonnull !4, !noundef !4
  store ptr %v, ptr %self2, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %self3)
  %v5 = load ptr, ptr %self2, align 8, !nonnull !4, !noundef !4
  store ptr %v5, ptr %_11, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %self2)
  %ptr6 = load ptr, ptr %_11, align 8, !nonnull !4, !noundef !4
  call void @llvm.lifetime.end.p0(i64 8, ptr %_11)
  br label %bb18

bb14:                                             ; No predecessors!
  unreachable

bb18:                                             ; preds = %bb15
  store ptr %ptr6, ptr %_0, align 8
  %24 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %size, ptr %24, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %raw_ptr)
  br label %bb7

bb16:                                             ; No predecessors!
  unreachable

bb17:                                             ; No predecessors!
  unreachable
}

; alloc::alloc::Global::grow_impl
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global9grow_impl17ha31b45620eea8e3cE(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull %ptr, i64 noundef %0, i64 noundef %1, i64 noundef %2, i64 noundef %3, i1 noundef zeroext %zeroed) unnamed_addr #2 {
start:
  %_72 = alloca [8 x i8], align 8
  %data12 = alloca [8 x i8], align 8
  %ptr11 = alloca [16 x i8], align 8
  %_63 = alloca [8 x i8], align 8
  %_62 = alloca [8 x i8], align 8
  %ptr10 = alloca [8 x i8], align 8
  %self9 = alloca [8 x i8], align 8
  %self8 = alloca [8 x i8], align 8
  %self7 = alloca [8 x i8], align 8
  %_56 = alloca [8 x i8], align 8
  %_49 = alloca [8 x i8], align 8
  %_43 = alloca [8 x i8], align 8
  %self6 = alloca [16 x i8], align 8
  %_37 = alloca [16 x i8], align 8
  %len = alloca [8 x i8], align 8
  %data = alloca [8 x i8], align 8
  %ptr5 = alloca [8 x i8], align 8
  %self4 = alloca [8 x i8], align 8
  %self3 = alloca [8 x i8], align 8
  %_25 = alloca [8 x i8], align 8
  %new_size = alloca [8 x i8], align 8
  %layout = alloca [16 x i8], align 8
  %self2 = alloca [8 x i8], align 8
  %ptr1 = alloca [8 x i8], align 8
  %raw_ptr = alloca [8 x i8], align 8
  %old_size = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %new_layout = alloca [16 x i8], align 8
  %old_layout = alloca [16 x i8], align 8
  store i64 %0, ptr %old_layout, align 8
  %4 = getelementptr inbounds i8, ptr %old_layout, i64 8
  store i64 %1, ptr %4, align 8
  store i64 %2, ptr %new_layout, align 8
  %5 = getelementptr inbounds i8, ptr %new_layout, i64 8
  store i64 %3, ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %7 = load i64, ptr %6, align 8, !noundef !4
  store i64 %7, ptr %old_size, align 8
  %8 = load i64, ptr %old_size, align 8, !noundef !4
  %9 = icmp eq i64 %8, 0
  br i1 %9, label %bb1, label %bb2

bb1:                                              ; preds = %start
  %10 = load i64, ptr %new_layout, align 8, !range !9, !noundef !4
  %11 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %12 = load i64, ptr %11, align 8, !noundef !4
; call alloc::alloc::Global::alloc_impl
  %13 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %10, i64 noundef %12, i1 noundef zeroext %zeroed)
  %14 = extractvalue { ptr, i64 } %13, 0
  %15 = extractvalue { ptr, i64 } %13, 1
  store ptr %14, ptr %_0, align 8
  %16 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %15, ptr %16, align 8
  br label %bb10

bb2:                                              ; preds = %start
  %self13 = load i64, ptr %old_layout, align 8, !range !9, !noundef !4
  store i64 %self13, ptr %_43, align 8
  %_44 = load i64, ptr %_43, align 8, !range !9, !noundef !4
  %_45 = icmp uge i64 %_44, 1
  %_46 = icmp ule i64 %_44, -9223372036854775808
  %_47 = and i1 %_45, %_46
  call void @llvm.assume(i1 %_47)
  %self14 = load i64, ptr %new_layout, align 8, !range !9, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_49)
  store i64 %self14, ptr %_49, align 8
  %_50 = load i64, ptr %_49, align 8, !range !9, !noundef !4
  %_51 = icmp uge i64 %_50, 1
  %_52 = icmp ule i64 %_50, -9223372036854775808
  %_53 = and i1 %_51, %_52
  call void @llvm.assume(i1 %_53)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_49)
  %_11 = icmp eq i64 %_44, %_50
  br i1 %_11, label %bb3, label %bb4

bb10:                                             ; preds = %bb24, %bb14, %bb25, %bb29, %bb1
  %17 = load ptr, ptr %_0, align 8, !noundef !4
  %18 = getelementptr inbounds i8, ptr %_0, i64 8
  %19 = load i64, ptr %18, align 8
  %20 = insertvalue { ptr, i64 } poison, ptr %17, 0
  %21 = insertvalue { ptr, i64 } %20, i64 %19, 1
  ret { ptr, i64 } %21

bb4:                                              ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 16, ptr %_37)
  call void @llvm.lifetime.start.p0(i64 16, ptr %self6)
  %22 = load i64, ptr %new_layout, align 8, !range !9, !noundef !4
  %23 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %24 = load i64, ptr %23, align 8, !noundef !4
; call alloc::alloc::Global::alloc_impl
  %25 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %22, i64 noundef %24, i1 noundef zeroext %zeroed)
  %26 = extractvalue { ptr, i64 } %25, 0
  %27 = extractvalue { ptr, i64 } %25, 1
  store ptr %26, ptr %self6, align 8
  %28 = getelementptr inbounds i8, ptr %self6, i64 8
  store i64 %27, ptr %28, align 8
  %29 = load ptr, ptr %self6, align 8, !noundef !4
  %30 = ptrtoint ptr %29 to i64
  %31 = icmp eq i64 %30, 0
  %_76 = select i1 %31, i64 1, i64 0
  switch i64 %_76, label %bb5 [
    i64 0, label %bb26
    i64 1, label %bb25
  ]

bb3:                                              ; preds = %bb2
  %32 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %new_size15 = load i64, ptr %32, align 8, !noundef !4
  %33 = load i64, ptr %old_size, align 8, !noundef !4
  %cond = icmp uge i64 %new_size15, %33
  br label %bb12

bb5:                                              ; preds = %bb4
  unreachable

bb26:                                             ; preds = %bb4
  %v.0 = load ptr, ptr %self6, align 8, !nonnull !4, !noundef !4
  %34 = getelementptr inbounds i8, ptr %self6, i64 8
  %v.1 = load i64, ptr %34, align 8, !noundef !4
  store ptr %v.0, ptr %_37, align 8
  %35 = getelementptr inbounds i8, ptr %_37, i64 8
  store i64 %v.1, ptr %35, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %self6)
  %new_ptr.0 = load ptr, ptr %_37, align 8, !nonnull !4, !noundef !4
  %36 = getelementptr inbounds i8, ptr %_37, i64 8
  %new_ptr.1 = load i64, ptr %36, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %_37)
  br label %bb29

bb25:                                             ; preds = %bb4
  call void @llvm.lifetime.end.p0(i64 16, ptr %self6)
  %37 = load ptr, ptr @0, align 8, !noundef !4
  %38 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %37, ptr %_0, align 8
  %39 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %38, ptr %39, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_37)
  br label %bb10

bb29:                                             ; preds = %bb26
  %40 = load i64, ptr %old_size, align 8, !noundef !4
  %41 = mul i64 %40, 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %new_ptr.0, ptr align 1 %ptr, i64 %41, i1 false)
  %42 = load i64, ptr %old_layout, align 8, !range !9, !noundef !4
  %43 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %44 = load i64, ptr %43, align 8, !noundef !4
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h37134dd5886182ffE"(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull %ptr, i64 noundef %42, i64 noundef %44)
  store ptr %new_ptr.0, ptr %_0, align 8
  %45 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %new_ptr.1, ptr %45, align 8
  br label %bb10

bb27:                                             ; No predecessors!
  unreachable

bb28:                                             ; No predecessors!
  unreachable

bb12:                                             ; preds = %bb3
  call void @llvm.assume(i1 %cond)
  call void @llvm.lifetime.start.p0(i64 8, ptr %ptr1)
  call void @llvm.lifetime.start.p0(i64 8, ptr %self2)
  store ptr %ptr, ptr %self2, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %_56)
  store ptr %ptr, ptr %_56, align 8
  %46 = load ptr, ptr %_56, align 8, !noundef !4
  store ptr %46, ptr %ptr1, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %_56)
  call void @llvm.lifetime.end.p0(i64 8, ptr %self2)
  call void @llvm.lifetime.start.p0(i64 16, ptr %layout)
  %47 = load i64, ptr %old_layout, align 8, !range !9, !noundef !4
  %48 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %49 = load i64, ptr %48, align 8, !noundef !4
  store i64 %47, ptr %layout, align 8
  %50 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %49, ptr %50, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %new_size)
  store i64 %new_size15, ptr %new_size, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %self7)
  store ptr %layout, ptr %self7, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %self7)
  call void @llvm.lifetime.start.p0(i64 8, ptr %self8)
  store ptr %layout, ptr %self8, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %self9)
  store i64 %self13, ptr %self9, align 8
  call void @llvm.assume(i1 %_47)
  call void @llvm.lifetime.end.p0(i64 8, ptr %self9)
  call void @llvm.lifetime.end.p0(i64 8, ptr %self8)
  %51 = load ptr, ptr %ptr1, align 8, !noundef !4
  %52 = load i64, ptr %old_size, align 8, !noundef !4
  %53 = call noundef ptr @__rust_realloc(ptr noundef %51, i64 noundef %52, i64 noundef %_44, i64 noundef %new_size15) #25
  store ptr %53, ptr %raw_ptr, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %new_size)
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout)
  call void @llvm.lifetime.end.p0(i64 8, ptr %ptr1)
  call void @llvm.lifetime.start.p0(i64 8, ptr %_25)
  call void @llvm.lifetime.start.p0(i64 8, ptr %self3)
  call void @llvm.lifetime.start.p0(i64 8, ptr %self4)
  call void @llvm.lifetime.start.p0(i64 8, ptr %ptr5)
  %54 = load ptr, ptr %raw_ptr, align 8, !noundef !4
  store ptr %54, ptr %ptr5, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %_63)
  call void @llvm.lifetime.start.p0(i64 8, ptr %ptr10)
  %55 = load ptr, ptr %raw_ptr, align 8, !noundef !4
  store ptr %55, ptr %ptr10, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %_62)
  %56 = load ptr, ptr %raw_ptr, align 8, !noundef !4
  store ptr %56, ptr %_63, align 8
  %57 = load ptr, ptr %_63, align 8, !noundef !4
  %58 = ptrtoint ptr %57 to i64
  store i64 %58, ptr %_62, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %ptr10)
  %59 = load i64, ptr %_62, align 8, !noundef !4
  %60 = icmp eq i64 %59, 0
  br i1 %60, label %bb14, label %bb15

bb14:                                             ; preds = %bb12
  call void @llvm.lifetime.end.p0(i64 8, ptr %_62)
  store ptr null, ptr %self4, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %_63)
  call void @llvm.lifetime.end.p0(i64 8, ptr %ptr5)
  store ptr null, ptr %self3, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %self4)
  call void @llvm.lifetime.end.p0(i64 8, ptr %self3)
  %61 = load ptr, ptr @0, align 8, !noundef !4
  %62 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store ptr %61, ptr %_0, align 8
  %63 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %62, ptr %63, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %_25)
  br label %bb10

bb15:                                             ; preds = %bb12
  call void @llvm.lifetime.end.p0(i64 8, ptr %_62)
  br label %bb17

bb17:                                             ; preds = %bb16, %bb15
  %_65 = load ptr, ptr %raw_ptr, align 8, !noundef !4
  store ptr %_65, ptr %self4, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %_63)
  call void @llvm.lifetime.end.p0(i64 8, ptr %ptr5)
  %v = load ptr, ptr %self4, align 8, !nonnull !4, !noundef !4
  store ptr %v, ptr %self3, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %self4)
  %v16 = load ptr, ptr %self3, align 8, !nonnull !4, !noundef !4
  store ptr %v16, ptr %_25, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %self3)
  %ptr17 = load ptr, ptr %_25, align 8, !nonnull !4, !noundef !4
  call void @llvm.lifetime.end.p0(i64 8, ptr %_25)
  br i1 %zeroed, label %bb6, label %bb7

bb11:                                             ; No predecessors!
  unreachable

bb31:                                             ; No predecessors!
  unreachable

bb32:                                             ; No predecessors!
  unreachable

bb33:                                             ; No predecessors!
  unreachable

bb16:                                             ; No predecessors!
  %64 = load ptr, ptr %_63, align 8, !noundef !4
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h3dcb9f87e643d327E"(ptr noundef %64) #25
  br label %bb17

bb7:                                              ; preds = %bb21, %bb17
  call void @llvm.lifetime.start.p0(i64 8, ptr %data)
  store ptr %ptr17, ptr %data, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %len)
  store i64 %new_size15, ptr %len, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %ptr11)
  call void @llvm.lifetime.start.p0(i64 8, ptr %data12)
  call void @llvm.lifetime.start.p0(i64 8, ptr %_72)
  store ptr %ptr17, ptr %_72, align 8
  %65 = load ptr, ptr %_72, align 8, !noundef !4
  store ptr %65, ptr %data12, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %_72)
  %66 = load ptr, ptr %data12, align 8, !noundef !4
  %67 = load i64, ptr %len, align 8, !noundef !4
  store ptr %66, ptr %ptr11, align 8
  %68 = getelementptr inbounds i8, ptr %ptr11, i64 8
  store i64 %67, ptr %68, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %data12)
  br label %bb24

bb6:                                              ; preds = %bb17
  %self18 = load ptr, ptr %raw_ptr, align 8, !noundef !4
  %69 = load ptr, ptr %raw_ptr, align 8, !noundef !4
  %70 = load i64, ptr %old_size, align 8, !noundef !4
  %self19 = getelementptr inbounds i8, ptr %69, i64 %70
  %71 = load i64, ptr %old_size, align 8, !noundef !4
  %count = sub i64 %new_size15, %71
  br label %bb21

bb21:                                             ; preds = %bb6
  %72 = mul i64 1, %count
  call void @llvm.memset.p0.i64(ptr align 1 %self19, i8 0, i64 %72, i1 false)
  br label %bb7

bb24:                                             ; preds = %bb22, %bb7
  %_75.0 = load ptr, ptr %ptr11, align 8, !noundef !4
  %73 = getelementptr inbounds i8, ptr %ptr11, i64 8
  %_75.1 = load i64, ptr %73, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %ptr11)
  call void @llvm.lifetime.end.p0(i64 8, ptr %len)
  call void @llvm.lifetime.end.p0(i64 8, ptr %data)
  store ptr %_75.0, ptr %_0, align 8
  %74 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %_75.1, ptr %74, align 8
  br label %bb10

bb19:                                             ; No predecessors!
; call core::intrinsics::write_bytes::precondition_check
  call void @_ZN4core10intrinsics11write_bytes18precondition_check17hbdd3a4488707f69aE(ptr noundef %self19, i64 noundef 1) #25
  %75 = mul i64 1, %count
  call void @llvm.memset.p0.i64(ptr align 1 %self19, i8 0, i64 %75, i1 false)
  call void @llvm.lifetime.start.p0(i64 8, ptr %data)
  store ptr %ptr17, ptr %data, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr %len)
  store i64 %new_size15, ptr %len, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %ptr11)
  call void @llvm.lifetime.start.p0(i64 8, ptr %data12)
  call void @llvm.lifetime.start.p0(i64 8, ptr %_72)
  store ptr %ptr17, ptr %_72, align 8
  %76 = load ptr, ptr %_72, align 8, !noundef !4
  store ptr %76, ptr %data12, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %_72)
  %77 = load ptr, ptr %data12, align 8, !noundef !4
  %78 = load i64, ptr %len, align 8, !noundef !4
  store ptr %77, ptr %ptr11, align 8
  %79 = getelementptr inbounds i8, ptr %ptr11, i64 8
  store i64 %78, ptr %79, align 8
  call void @llvm.lifetime.end.p0(i64 8, ptr %data12)
  br label %bb22

bb22:                                             ; preds = %bb19
  %_74 = load ptr, ptr %ptr11, align 8, !noundef !4
  %80 = getelementptr inbounds i8, ptr %ptr11, i64 8
  %81 = load i64, ptr %80, align 8, !noundef !4
; call core::ptr::non_null::NonNull<T>::new_unchecked::precondition_check
  call void @"_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h3dcb9f87e643d327E"(ptr noundef %_74) #25
  br label %bb24
}

; alloc::raw_vec::finish_grow
; Function Attrs: noinline uwtable
define internal void @_ZN5alloc7raw_vec11finish_grow17h3c6fc7aa1af7d81dE(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, i64 noundef %0, i64 %1, ptr noalias nocapture noundef readonly align 8 dereferenceable(24) %current_memory, ptr noalias noundef nonnull align 1 %alloc) unnamed_addr #1 {
start:
  %self3 = alloca [16 x i8], align 8
  %_47 = alloca [8 x i8], align 8
  %_41 = alloca [8 x i8], align 8
  %_34 = alloca [16 x i8], align 8
  %self2 = alloca [16 x i8], align 8
  %old_layout = alloca [16 x i8], align 8
  %memory = alloca [16 x i8], align 8
  %residual = alloca [16 x i8], align 8
  %self = alloca [24 x i8], align 8
  %_5 = alloca [24 x i8], align 8
  %new_layout1 = alloca [16 x i8], align 8
  %new_layout = alloca [16 x i8], align 8
  store i64 %0, ptr %new_layout, align 8
  %2 = getelementptr inbounds i8, ptr %new_layout, i64 8
  store i64 %1, ptr %2, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %new_layout1)
  call void @llvm.lifetime.start.p0(i64 24, ptr %_5)
  call void @llvm.lifetime.start.p0(i64 24, ptr %self)
  %3 = load i64, ptr %new_layout, align 8, !range !10, !noundef !4
  %4 = icmp eq i64 %3, 0
  %_30 = select i1 %4, i64 1, i64 0
  switch i64 %_30, label %bb1 [
    i64 0, label %bb9
    i64 1, label %bb8
  ]

bb1:                                              ; preds = %bb6, %bb9, %start
  unreachable

bb9:                                              ; preds = %start
  %t.0 = load i64, ptr %new_layout, align 8, !range !9, !noundef !4
  %5 = getelementptr inbounds i8, ptr %new_layout, i64 8
  %t.1 = load i64, ptr %5, align 8, !noundef !4
  %6 = getelementptr inbounds i8, ptr %self, i64 8
  store i64 %t.0, ptr %6, align 8
  %7 = getelementptr inbounds i8, ptr %6, i64 8
  store i64 %t.1, ptr %7, align 8
  store i64 0, ptr %self, align 8
  %8 = getelementptr inbounds i8, ptr %self, i64 8
  %v.0 = load i64, ptr %8, align 8, !range !9, !noundef !4
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  %v.1 = load i64, ptr %9, align 8, !noundef !4
  %10 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 %v.0, ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %10, i64 8
  store i64 %v.1, ptr %11, align 8
  store i64 0, ptr %_5, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr %self)
  %12 = getelementptr inbounds i8, ptr %_5, i64 8
  %val.0 = load i64, ptr %12, align 8, !range !9, !noundef !4
  %13 = getelementptr inbounds i8, ptr %12, i64 8
  %val.1 = load i64, ptr %13, align 8, !noundef !4
  store i64 %val.0, ptr %new_layout1, align 8
  %14 = getelementptr inbounds i8, ptr %new_layout1, i64 8
  store i64 %val.1, ptr %14, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr %_5)
  call void @llvm.lifetime.start.p0(i64 16, ptr %memory)
  %15 = getelementptr inbounds i8, ptr %current_memory, i64 8
  %16 = load i64, ptr %15, align 8, !range !10, !noundef !4
  %17 = icmp eq i64 %16, 0
  %_14 = select i1 %17, i64 0, i64 1
  switch i64 %_14, label %bb1 [
    i64 1, label %bb3
    i64 0, label %bb2
  ]

bb8:                                              ; preds = %start
  %18 = load i64, ptr @0, align 8, !range !10, !noundef !4
  %19 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %20 = getelementptr inbounds i8, ptr %self, i64 8
  store i64 %18, ptr %20, align 8
  %21 = getelementptr inbounds i8, ptr %20, i64 8
  store i64 %19, ptr %21, align 8
  store i64 1, ptr %self, align 8
  %22 = getelementptr inbounds i8, ptr %self, i64 8
  %e.0 = load i64, ptr %22, align 8, !range !10, !noundef !4
  %23 = getelementptr inbounds i8, ptr %22, i64 8
  %e.1 = load i64, ptr %23, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_34)
  store i64 %e.0, ptr %_34, align 8
  %24 = getelementptr inbounds i8, ptr %_34, i64 8
  store i64 %e.1, ptr %24, align 8
  %25 = load i64, ptr %_34, align 8, !range !10, !noundef !4
  %26 = getelementptr inbounds i8, ptr %_34, i64 8
  %27 = load i64, ptr %26, align 8
  %28 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 %25, ptr %28, align 8
  %29 = getelementptr inbounds i8, ptr %28, i64 8
  store i64 %27, ptr %29, align 8
  store i64 1, ptr %_5, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_34)
  call void @llvm.lifetime.end.p0(i64 24, ptr %self)
  %30 = getelementptr inbounds i8, ptr %_5, i64 8
  %31 = load i64, ptr %30, align 8, !range !10, !noundef !4
  %32 = getelementptr inbounds i8, ptr %30, i64 8
  %33 = load i64, ptr %32, align 8
  store i64 %31, ptr %residual, align 8
  %34 = getelementptr inbounds i8, ptr %residual, i64 8
  store i64 %33, ptr %34, align 8
  %e.07 = load i64, ptr %residual, align 8, !range !10, !noundef !4
  %35 = getelementptr inbounds i8, ptr %residual, i64 8
  %e.18 = load i64, ptr %35, align 8
  %36 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %e.07, ptr %36, align 8
  %37 = getelementptr inbounds i8, ptr %36, i64 8
  store i64 %e.18, ptr %37, align 8
  store i64 1, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr %_5)
  call void @llvm.lifetime.end.p0(i64 16, ptr %new_layout1)
  br label %bb7

bb3:                                              ; preds = %bb9
  %ptr = load ptr, ptr %current_memory, align 8, !nonnull !4, !noundef !4
  %38 = getelementptr inbounds i8, ptr %current_memory, i64 8
  %39 = load i64, ptr %38, align 8, !range !9, !noundef !4
  %40 = getelementptr inbounds i8, ptr %38, i64 8
  %41 = load i64, ptr %40, align 8, !noundef !4
  store i64 %39, ptr %old_layout, align 8
  %42 = getelementptr inbounds i8, ptr %old_layout, i64 8
  store i64 %41, ptr %42, align 8
  %self4 = load i64, ptr %old_layout, align 8, !range !9, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_41)
  store i64 %self4, ptr %_41, align 8
  %_42 = load i64, ptr %_41, align 8, !range !9, !noundef !4
  %_43 = icmp uge i64 %_42, 1
  %_44 = icmp ule i64 %_42, -9223372036854775808
  %_45 = and i1 %_43, %_44
  call void @llvm.assume(i1 %_45)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_41)
  call void @llvm.lifetime.start.p0(i64 8, ptr %_47)
  store i64 %val.0, ptr %_47, align 8
  %_48 = load i64, ptr %_47, align 8, !range !9, !noundef !4
  %_49 = icmp uge i64 %_48, 1
  %_50 = icmp ule i64 %_48, -9223372036854775808
  %_51 = and i1 %_49, %_50
  call void @llvm.assume(i1 %_51)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_47)
  %cond = icmp eq i64 %_42, %_48
  br label %bb11

bb2:                                              ; preds = %bb9
; call <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %43 = call { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h6ca1b7a3462ca17fE"(ptr noalias noundef nonnull readonly align 1 %alloc, i64 noundef %val.0, i64 noundef %val.1)
  %44 = extractvalue { ptr, i64 } %43, 0
  %45 = extractvalue { ptr, i64 } %43, 1
  store ptr %44, ptr %memory, align 8
  %46 = getelementptr inbounds i8, ptr %memory, i64 8
  store i64 %45, ptr %46, align 8
  br label %bb6

bb11:                                             ; preds = %bb3
  call void @llvm.assume(i1 %cond)
  %47 = load i64, ptr %old_layout, align 8, !range !9, !noundef !4
  %48 = getelementptr inbounds i8, ptr %old_layout, i64 8
  %49 = load i64, ptr %48, align 8, !noundef !4
; call <alloc::alloc::Global as core::alloc::Allocator>::grow
  %50 = call { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17h8187fb1253081182E"(ptr noalias noundef nonnull readonly align 1 %alloc, ptr noundef nonnull %ptr, i64 noundef %47, i64 noundef %49, i64 noundef %val.0, i64 noundef %val.1)
  %51 = extractvalue { ptr, i64 } %50, 0
  %52 = extractvalue { ptr, i64 } %50, 1
  store ptr %51, ptr %memory, align 8
  %53 = getelementptr inbounds i8, ptr %memory, i64 8
  store i64 %52, ptr %53, align 8
  br label %bb6

bb10:                                             ; No predecessors!
  unreachable

bb6:                                              ; preds = %bb2, %bb11
  call void @llvm.lifetime.start.p0(i64 16, ptr %self2)
  %54 = load ptr, ptr %memory, align 8, !noundef !4
  %55 = getelementptr inbounds i8, ptr %memory, i64 8
  %56 = load i64, ptr %55, align 8
  store ptr %54, ptr %self2, align 8
  %57 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %56, ptr %57, align 8
  %58 = load ptr, ptr %self2, align 8, !noundef !4
  %59 = ptrtoint ptr %58 to i64
  %60 = icmp eq i64 %59, 0
  %_54 = select i1 %60, i64 1, i64 0
  switch i64 %_54, label %bb1 [
    i64 0, label %bb14
    i64 1, label %bb13
  ]

bb14:                                             ; preds = %bb6
  %t.05 = load ptr, ptr %self2, align 8, !nonnull !4, !noundef !4
  %61 = getelementptr inbounds i8, ptr %self2, i64 8
  %t.16 = load i64, ptr %61, align 8, !noundef !4
  %62 = getelementptr inbounds i8, ptr %_0, i64 8
  store ptr %t.05, ptr %62, align 8
  %63 = getelementptr inbounds i8, ptr %62, i64 8
  store i64 %t.16, ptr %63, align 8
  store i64 0, ptr %_0, align 8
  br label %bb12

bb13:                                             ; preds = %bb6
  call void @llvm.lifetime.start.p0(i64 16, ptr %self3)
  store i64 %val.0, ptr %self3, align 8
  %64 = getelementptr inbounds i8, ptr %self3, i64 8
  store i64 %val.1, ptr %64, align 8
  %_56.0 = load i64, ptr %self3, align 8, !range !10, !noundef !4
  %65 = getelementptr inbounds i8, ptr %self3, i64 8
  %_56.1 = load i64, ptr %65, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %self3)
  %66 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %_56.0, ptr %66, align 8
  %67 = getelementptr inbounds i8, ptr %66, i64 8
  store i64 %_56.1, ptr %67, align 8
  store i64 1, ptr %_0, align 8
  br label %bb12

bb12:                                             ; preds = %bb13, %bb14
  call void @llvm.lifetime.end.p0(i64 16, ptr %self2)
  call void @llvm.lifetime.end.p0(i64 16, ptr %memory)
  call void @llvm.lifetime.end.p0(i64 16, ptr %new_layout1)
  br label %bb7

bb7:                                              ; preds = %bb8, %bb12
  ret void
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h769bf7a93d043036E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_9 = alloca [24 x i8], align 8
  br label %bb1

bb1:                                              ; preds = %start
  %_3 = load i64, ptr %self, align 8, !noundef !4
  %0 = icmp eq i64 %_3, 0
  br i1 %0, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  br label %bb3

bb4:                                              ; preds = %bb1
  %rhs = load i64, ptr %self, align 8, !noundef !4
  br label %bb7

bb3:                                              ; preds = %bb2
  %1 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 0, ptr %1, align 8
  br label %bb5

bb7:                                              ; preds = %bb4
  %size = mul nuw i64 1, %rhs
  call void @llvm.assume(i1 true)
  call void @llvm.assume(i1 true)
  call void @llvm.lifetime.start.p0(i64 24, ptr %_9)
  %2 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %2, align 8, !nonnull !4, !noundef !4
  store ptr %self1, ptr %_9, align 8
  %3 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %size, ptr %4, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_9, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr %_9)
  br label %bb5

bb6:                                              ; No predecessors!
  unreachable

bb5:                                              ; preds = %bb3, %bb7
  ret void
}

; alloc::raw_vec::RawVec<T,A>::grow_amortized
; Function Attrs: uwtable
define internal { i64, i64 } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h0e9d83d84c8160aaE"(ptr noalias noundef align 8 dereferenceable(16) %self, i64 noundef %len, i64 noundef %additional) unnamed_addr #0 {
start:
  %0 = alloca [1 x i8], align 1
  %_42 = alloca [16 x i8], align 8
  %_32 = alloca [16 x i8], align 8
  %residual4 = alloca [16 x i8], align 8
  %_17 = alloca [24 x i8], align 8
  %self3 = alloca [24 x i8], align 8
  %_15 = alloca [24 x i8], align 8
  %residual = alloca [16 x i8], align 8
  %self2 = alloca [16 x i8], align 8
  %self1 = alloca [16 x i8], align 8
  %_5 = alloca [16 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  br label %bb2

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr %_5)
  call void @llvm.lifetime.start.p0(i64 16, ptr %self1)
  call void @llvm.lifetime.start.p0(i64 16, ptr %self2)
  %1 = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %len, i64 %additional)
  %_25.0 = extractvalue { i64, i1 } %1, 0
  %_25.1 = extractvalue { i64, i1 } %1, 1
  call void @llvm.lifetime.start.p0(i64 1, ptr %0)
  %2 = call i1 @llvm.expect.i1(i1 %_25.1, i1 false)
  %3 = zext i1 %2 to i8
  store i8 %3, ptr %0, align 1
  %4 = load i8, ptr %0, align 1, !range !8, !noundef !4
  %_22 = trunc i8 %4 to i1
  call void @llvm.lifetime.end.p0(i64 1, ptr %0)
  br i1 %_22, label %bb9, label %bb10

bb10:                                             ; preds = %bb2
  %5 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %_25.0, ptr %5, align 8
  store i64 1, ptr %self2, align 8
  %6 = getelementptr inbounds i8, ptr %self2, i64 8
  %v = load i64, ptr %6, align 8, !noundef !4
  %7 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %v, ptr %7, align 8
  store i64 -9223372036854775807, ptr %self1, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %self2)
  %8 = getelementptr inbounds i8, ptr %self1, i64 8
  %v5 = load i64, ptr %8, align 8, !noundef !4
  %9 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 %v5, ptr %9, align 8
  store i64 -9223372036854775807, ptr %_5, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %self1)
  %10 = getelementptr inbounds i8, ptr %_5, i64 8
  %required_cap = load i64, ptr %10, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %_5)
  %_12 = load i64, ptr %self, align 8, !noundef !4
  %v1 = mul i64 %_12, 2
; call core::cmp::max_by
  %cap = call noundef i64 @_ZN4core3cmp6max_by17he5bc204448e63f16E(i64 noundef %v1, i64 noundef %required_cap)
; call core::cmp::max_by
  %cap6 = call noundef i64 @_ZN4core3cmp6max_by17he5bc204448e63f16E(i64 noundef 8, i64 noundef %cap)
  call void @llvm.assume(i1 true)
  call void @llvm.assume(i1 true)
; call core::alloc::layout::Layout::array::inner
  %11 = call { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h1e2e6b6a7c48ef5dE(i64 noundef 1, i64 noundef 1, i64 noundef %cap6)
  %new_layout.0 = extractvalue { i64, i64 } %11, 0
  %new_layout.1 = extractvalue { i64, i64 } %11, 1
  call void @llvm.lifetime.start.p0(i64 24, ptr %_15)
  call void @llvm.lifetime.start.p0(i64 24, ptr %self3)
  call void @llvm.lifetime.start.p0(i64 24, ptr %_17)
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h769bf7a93d043036E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_17, ptr noalias noundef readonly align 8 dereferenceable(16) %self)
  %_19 = getelementptr inbounds i8, ptr %self, i64 16
; call alloc::raw_vec::finish_grow
  call void @_ZN5alloc7raw_vec11finish_grow17h3c6fc7aa1af7d81dE(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %self3, i64 noundef %new_layout.0, i64 %new_layout.1, ptr noalias nocapture noundef readonly align 8 dereferenceable(24) %_17, ptr noalias noundef nonnull align 1 %_19)
  call void @llvm.lifetime.end.p0(i64 24, ptr %_17)
  %_39 = load i64, ptr %self3, align 8, !range !11, !noundef !4
  switch i64 %_39, label %bb3 [
    i64 0, label %bb15
    i64 1, label %bb14
  ]

bb9:                                              ; preds = %bb2
  %12 = load i64, ptr @0, align 8, !range !11, !noundef !4
  %13 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %12, ptr %self2, align 8
  %14 = getelementptr inbounds i8, ptr %self2, i64 8
  store i64 %13, ptr %14, align 8
  %15 = load i64, ptr @0, align 8, !range !10, !noundef !4
  %16 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %15, ptr %self1, align 8
  %17 = getelementptr inbounds i8, ptr %self1, i64 8
  store i64 %16, ptr %17, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %self2)
  %e.09 = load i64, ptr %self1, align 8, !range !10, !noundef !4
  %18 = getelementptr inbounds i8, ptr %self1, i64 8
  %e.110 = load i64, ptr %18, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_32)
  store i64 %e.09, ptr %_32, align 8
  %19 = getelementptr inbounds i8, ptr %_32, i64 8
  store i64 %e.110, ptr %19, align 8
  %20 = load i64, ptr %_32, align 8, !range !10, !noundef !4
  %21 = getelementptr inbounds i8, ptr %_32, i64 8
  %22 = load i64, ptr %21, align 8
  store i64 %20, ptr %_5, align 8
  %23 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 %22, ptr %23, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_32)
  call void @llvm.lifetime.end.p0(i64 16, ptr %self1)
  %24 = load i64, ptr %_5, align 8, !range !10, !noundef !4
  %25 = getelementptr inbounds i8, ptr %_5, i64 8
  %26 = load i64, ptr %25, align 8
  store i64 %24, ptr %residual, align 8
  %27 = getelementptr inbounds i8, ptr %residual, i64 8
  store i64 %26, ptr %27, align 8
  %e.011 = load i64, ptr %residual, align 8, !range !10, !noundef !4
  %28 = getelementptr inbounds i8, ptr %residual, i64 8
  %e.112 = load i64, ptr %28, align 8
  store i64 %e.011, ptr %_0, align 8
  %29 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %e.112, ptr %29, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_5)
  br label %bb6

bb3:                                              ; preds = %bb10
  unreachable

bb15:                                             ; preds = %bb10
  %30 = getelementptr inbounds i8, ptr %self3, i64 8
  %v.0 = load ptr, ptr %30, align 8, !nonnull !4, !noundef !4
  %31 = getelementptr inbounds i8, ptr %30, i64 8
  %v.1 = load i64, ptr %31, align 8, !noundef !4
  %32 = getelementptr inbounds i8, ptr %_15, i64 8
  store ptr %v.0, ptr %32, align 8
  %33 = getelementptr inbounds i8, ptr %32, i64 8
  store i64 %v.1, ptr %33, align 8
  store i64 0, ptr %_15, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr %self3)
  %34 = getelementptr inbounds i8, ptr %_15, i64 8
  %ptr.0 = load ptr, ptr %34, align 8, !nonnull !4, !noundef !4
  %35 = getelementptr inbounds i8, ptr %34, i64 8
  %ptr.1 = load i64, ptr %35, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 24, ptr %_15)
  %36 = getelementptr inbounds i8, ptr %self, i64 8
  store ptr %ptr.0, ptr %36, align 8
  store i64 %cap6, ptr %self, align 8
  %37 = load i64, ptr @1, align 8, !range !13, !noundef !4
  %38 = load i64, ptr getelementptr inbounds (i8, ptr @1, i64 8), align 8
  store i64 %37, ptr %_0, align 8
  %39 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %38, ptr %39, align 8
  br label %bb7

bb14:                                             ; preds = %bb10
  %40 = getelementptr inbounds i8, ptr %self3, i64 8
  %e.0 = load i64, ptr %40, align 8, !range !10, !noundef !4
  %41 = getelementptr inbounds i8, ptr %40, i64 8
  %e.1 = load i64, ptr %41, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_42)
  store i64 %e.0, ptr %_42, align 8
  %42 = getelementptr inbounds i8, ptr %_42, i64 8
  store i64 %e.1, ptr %42, align 8
  %43 = load i64, ptr %_42, align 8, !range !10, !noundef !4
  %44 = getelementptr inbounds i8, ptr %_42, i64 8
  %45 = load i64, ptr %44, align 8
  %46 = getelementptr inbounds i8, ptr %_15, i64 8
  store i64 %43, ptr %46, align 8
  %47 = getelementptr inbounds i8, ptr %46, i64 8
  store i64 %45, ptr %47, align 8
  store i64 1, ptr %_15, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_42)
  call void @llvm.lifetime.end.p0(i64 24, ptr %self3)
  %48 = getelementptr inbounds i8, ptr %_15, i64 8
  %49 = load i64, ptr %48, align 8, !range !10, !noundef !4
  %50 = getelementptr inbounds i8, ptr %48, i64 8
  %51 = load i64, ptr %50, align 8
  store i64 %49, ptr %residual4, align 8
  %52 = getelementptr inbounds i8, ptr %residual4, i64 8
  store i64 %51, ptr %52, align 8
  %e.07 = load i64, ptr %residual4, align 8, !range !10, !noundef !4
  %53 = getelementptr inbounds i8, ptr %residual4, i64 8
  %e.18 = load i64, ptr %53, align 8
  store i64 %e.07, ptr %_0, align 8
  %54 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %e.18, ptr %54, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr %_15)
  br label %bb6

bb7:                                              ; preds = %bb6, %bb15
  %55 = load i64, ptr %_0, align 8, !range !13, !noundef !4
  %56 = getelementptr inbounds i8, ptr %_0, i64 8
  %57 = load i64, ptr %56, align 8
  %58 = insertvalue { i64, i64 } poison, i64 %55, 0
  %59 = insertvalue { i64, i64 } %58, i64 %57, 1
  ret { i64, i64 } %59

bb6:                                              ; preds = %bb9, %bb14
  br label %bb7

bb1:                                              ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::try_allocate_in
; Function Attrs: uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17hfa11d2430c197263E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, i64 noundef %capacity, i1 noundef zeroext %0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %1 = alloca [16 x i8], align 8
  %_26 = alloca [1 x i8], align 1
  %self = alloca [16 x i8], align 8
  %result = alloca [16 x i8], align 8
  %_8 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  %alloc = alloca [0 x i8], align 1
  %init = alloca [1 x i8], align 1
  %2 = zext i1 %0 to i8
  store i8 %2, ptr %init, align 1
  store i8 1, ptr %_26, align 1
  br label %bb1

bb1:                                              ; preds = %start
  %3 = icmp eq i64 %capacity, 0
  br i1 %3, label %bb2, label %bb4

bb2:                                              ; preds = %bb1
  store i8 0, ptr %_26, align 1
; invoke alloc::raw_vec::RawVec<T,A>::new_in
  %4 = invoke { i64, ptr } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17haac1913c85d88fc7E"()
          to label %bb3 unwind label %cleanup

bb4:                                              ; preds = %bb1
  call void @llvm.lifetime.start.p0(i64 16, ptr %layout)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_8)
  call void @llvm.assume(i1 true)
  call void @llvm.assume(i1 true)
; invoke core::alloc::layout::Layout::array::inner
  %5 = invoke { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h1e2e6b6a7c48ef5dE(i64 noundef 1, i64 noundef 1, i64 noundef %capacity)
          to label %bb21 unwind label %cleanup

bb20:                                             ; preds = %cleanup
  %6 = load i8, ptr %_26, align 1, !range !8, !noundef !4
  %7 = trunc i8 %6 to i1
  br i1 %7, label %bb19, label %bb18

cleanup:                                          ; preds = %bb2, %bb8, %bb9, %bb4
  %8 = landingpad { ptr, i32 }
          cleanup
  %9 = extractvalue { ptr, i32 } %8, 0
  %10 = extractvalue { ptr, i32 } %8, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %1)
  store ptr %9, ptr %1, align 8
  %11 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %10, ptr %11, align 8
  br label %bb20

bb21:                                             ; preds = %bb4
  %12 = extractvalue { i64, i64 } %5, 0
  %13 = extractvalue { i64, i64 } %5, 1
  store i64 %12, ptr %_8, align 8
  %14 = getelementptr inbounds i8, ptr %_8, i64 8
  store i64 %13, ptr %14, align 8
  %15 = load i64, ptr %_8, align 8, !range !10, !noundef !4
  %16 = icmp eq i64 %15, 0
  %_9 = select i1 %16, i64 1, i64 0
  switch i64 %_9, label %bb5 [
    i64 0, label %bb7
    i64 1, label %bb6
  ]

bb5:                                              ; preds = %bb12, %bb7, %bb21
  unreachable

bb7:                                              ; preds = %bb21
  %layout.0 = load i64, ptr %_8, align 8, !range !9, !noundef !4
  %17 = getelementptr inbounds i8, ptr %_8, i64 8
  %layout.1 = load i64, ptr %17, align 8, !noundef !4
  store i64 %layout.0, ptr %layout, align 8
  %18 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %layout.1, ptr %18, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_8)
  call void @llvm.lifetime.start.p0(i64 16, ptr %result)
  %19 = load i8, ptr %init, align 1, !range !8, !noundef !4
  %20 = trunc i8 %19 to i1
  %_14 = zext i1 %20 to i64
  switch i64 %_14, label %bb5 [
    i64 0, label %bb9
    i64 1, label %bb8
  ]

bb6:                                              ; preds = %bb21
  %21 = load i64, ptr @0, align 8, !range !10, !noundef !4
  %22 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %23 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %21, ptr %23, align 8
  %24 = getelementptr inbounds i8, ptr %23, i64 8
  store i64 %22, ptr %24, align 8
  store i64 1, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %_8)
  br label %bb16

bb9:                                              ; preds = %bb7
; invoke <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %25 = invoke { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h6ca1b7a3462ca17fE"(ptr noalias noundef nonnull readonly align 1 %alloc, i64 noundef %layout.0, i64 noundef %layout.1)
          to label %bb10 unwind label %cleanup

bb8:                                              ; preds = %bb7
; invoke <alloc::alloc::Global as core::alloc::Allocator>::allocate_zeroed
  %26 = invoke { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h51d97e6b86c1750aE"(ptr noalias noundef nonnull readonly align 1 %alloc, i64 noundef %layout.0, i64 noundef %layout.1)
          to label %bb11 unwind label %cleanup

bb10:                                             ; preds = %bb9
  %27 = extractvalue { ptr, i64 } %25, 0
  %28 = extractvalue { ptr, i64 } %25, 1
  store ptr %27, ptr %result, align 8
  %29 = getelementptr inbounds i8, ptr %result, i64 8
  store i64 %28, ptr %29, align 8
  br label %bb12

bb12:                                             ; preds = %bb11, %bb10
  %30 = load ptr, ptr %result, align 8, !noundef !4
  %31 = ptrtoint ptr %30 to i64
  %32 = icmp eq i64 %31, 0
  %_17 = select i1 %32, i64 1, i64 0
  switch i64 %_17, label %bb5 [
    i64 0, label %bb14
    i64 1, label %bb13
  ]

bb11:                                             ; preds = %bb8
  %33 = extractvalue { ptr, i64 } %26, 0
  %34 = extractvalue { ptr, i64 } %26, 1
  store ptr %33, ptr %result, align 8
  %35 = getelementptr inbounds i8, ptr %result, i64 8
  store i64 %34, ptr %35, align 8
  br label %bb12

bb14:                                             ; preds = %bb12
  %ptr.0 = load ptr, ptr %result, align 8, !nonnull !4, !noundef !4
  %36 = getelementptr inbounds i8, ptr %result, i64 8
  %ptr.1 = load i64, ptr %36, align 8, !noundef !4
  %37 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %capacity, ptr %37, align 8
  %38 = getelementptr inbounds i8, ptr %37, i64 8
  store ptr %ptr.0, ptr %38, align 8
  store i64 0, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %result)
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout)
  br label %bb15

bb13:                                             ; preds = %bb12
  call void @llvm.lifetime.start.p0(i64 16, ptr %self)
  store i64 %layout.0, ptr %self, align 8
  %39 = getelementptr inbounds i8, ptr %self, i64 8
  store i64 %layout.1, ptr %39, align 8
  %_19.0 = load i64, ptr %self, align 8, !range !10, !noundef !4
  %40 = getelementptr inbounds i8, ptr %self, i64 8
  %_19.1 = load i64, ptr %40, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %self)
  %41 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %_19.0, ptr %41, align 8
  %42 = getelementptr inbounds i8, ptr %41, i64 8
  store i64 %_19.1, ptr %42, align 8
  store i64 1, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr %result)
  br label %bb16

bb15:                                             ; preds = %bb3, %bb14
  br label %bb17

bb16:                                             ; preds = %bb6, %bb13
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout)
  br label %bb17

bb17:                                             ; preds = %bb15, %bb16
  ret void

bb3:                                              ; preds = %bb2
  %_5.0 = extractvalue { i64, ptr } %4, 0
  %_5.1 = extractvalue { i64, ptr } %4, 1
  %43 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %_5.0, ptr %43, align 8
  %44 = getelementptr inbounds i8, ptr %43, i64 8
  store ptr %_5.1, ptr %44, align 8
  store i64 0, ptr %_0, align 8
  br label %bb15

bb18:                                             ; preds = %bb19, %bb20
  %45 = load ptr, ptr %1, align 8, !noundef !4
  %46 = getelementptr inbounds i8, ptr %1, i64 8
  %47 = load i32, ptr %46, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %1)
  %48 = insertvalue { ptr, i32 } poison, ptr %45, 0
  %49 = insertvalue { ptr, i32 } %48, i32 %47, 1
  resume { ptr, i32 } %49

bb19:                                             ; preds = %bb20
  br label %bb18
}

; alloc::raw_vec::RawVec<T,A>::new_in
; Function Attrs: uwtable
define internal { i64, ptr } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17haac1913c85d88fc7E"() unnamed_addr #0 {
start:
  br label %bb3

bb3:                                              ; preds = %start
  ret { i64, ptr } { i64 0, ptr getelementptr (i8, ptr null, i64 1) }

bb1:                                              ; No predecessors!
  unreachable

bb2:                                              ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::reserve::do_reserve_and_handle
; Function Attrs: cold uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17hf19a3efabb9eed6fE"(ptr noalias noundef align 8 dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional) unnamed_addr #6 {
start:
  %_4 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_4)
; call alloc::raw_vec::RawVec<T,A>::grow_amortized
  %0 = call { i64, i64 } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h0e9d83d84c8160aaE"(ptr noalias noundef align 8 dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional)
  %1 = extractvalue { i64, i64 } %0, 0
  %2 = extractvalue { i64, i64 } %0, 1
  store i64 %1, ptr %_4, align 8
  %3 = getelementptr inbounds i8, ptr %_4, i64 8
  store i64 %2, ptr %3, align 8
  %4 = load i64, ptr %_4, align 8, !range !13, !noundef !4
  %5 = icmp eq i64 %4, -9223372036854775807
  %_5 = select i1 %5, i64 0, i64 1
  switch i64 %_5, label %bb4 [
    i64 1, label %bb2
    i64 0, label %bb3
  ]

bb4:                                              ; preds = %start
  unreachable

bb2:                                              ; preds = %start
  %err.0 = load i64, ptr %_4, align 8, !range !10, !noundef !4
  %6 = getelementptr inbounds i8, ptr %_4, i64 8
  %err.1 = load i64, ptr %6, align 8
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h7f9cd8199ddef69cE(i64 noundef %err.0, i64 %err.1) #22
  unreachable

bb3:                                              ; preds = %start
  call void @llvm.lifetime.end.p0(i64 16, ptr %_4)
  ret void
}

; <alloc::string::String as core::fmt::Display>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h3a517c0b8c90cbfeE"(ptr noalias noundef readonly align 8 dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #2 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8, !noundef !4
  br label %bb4

bb4:                                              ; preds = %start
; call <str as core::fmt::Display>::fmt
  %_0 = call noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd248aa3f582d4052E"(ptr noalias noundef nonnull readonly align 1 %self1, i64 noundef %len, ptr noalias noundef align 8 dereferenceable(64) %f)
  ret i1 %_0

bb2:                                              ; No predecessors!
  unreachable

bb3:                                              ; No predecessors!
  unreachable
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h37134dd5886182ffE"(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull %ptr, i64 noundef %0, i64 noundef %1) unnamed_addr #2 {
start:
  %_13 = alloca [8 x i8], align 8
  %layout1 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %layout, i64 8
  %_4 = load i64, ptr %3, align 8, !noundef !4
  %4 = icmp eq i64 %_4, 0
  br i1 %4, label %bb2, label %bb1

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr %layout1)
  %5 = load i64, ptr %layout, align 8, !range !9, !noundef !4
  %6 = getelementptr inbounds i8, ptr %layout, i64 8
  %7 = load i64, ptr %6, align 8, !noundef !4
  store i64 %5, ptr %layout1, align 8
  %8 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %7, ptr %8, align 8
  %self2 = load i64, ptr %layout, align 8, !range !9, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_13)
  store i64 %self2, ptr %_13, align 8
  %_14 = load i64, ptr %_13, align 8, !range !9, !noundef !4
  %_15 = icmp uge i64 %_14, 1
  %_16 = icmp ule i64 %_14, -9223372036854775808
  %_17 = and i1 %_15, %_16
  call void @llvm.assume(i1 %_17)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_13)
  call void @__rust_dealloc(ptr noundef %ptr, i64 noundef %_4, i64 noundef %_14) #25
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout1)
  br label %bb2
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate_zeroed
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h51d97e6b86c1750aE"(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %layout.0, i64 noundef %layout.1) unnamed_addr #2 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %layout.0, i64 noundef %layout.1, i1 noundef zeroext true)
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::alloc::Global as core::alloc::Allocator>::grow
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17h8187fb1253081182E"(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull %ptr, i64 noundef %old_layout.0, i64 noundef %old_layout.1, i64 noundef %new_layout.0, i64 noundef %new_layout.1) unnamed_addr #2 {
start:
; call alloc::alloc::Global::grow_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global9grow_impl17ha31b45620eea8e3cE(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull %ptr, i64 noundef %old_layout.0, i64 noundef %old_layout.1, i64 noundef %new_layout.0, i64 noundef %new_layout.1, i1 noundef zeroext false)
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h6ca1b7a3462ca17fE"(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %layout.0, i64 noundef %layout.1) unnamed_addr #2 {
start:
; call alloc::alloc::Global::alloc_impl
  %0 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %layout.0, i64 noundef %layout.1, i1 noundef zeroext false)
  %_0.0 = extractvalue { ptr, i64 } %0, 0
  %_0.1 = extractvalue { ptr, i64 } %0, 1
  %1 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %_0.1, 1
  ret { ptr, i64 } %2
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h770afccffcf33d3eE"(ptr noalias noundef align 8 dereferenceable(24) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8, !noundef !4
  ret void
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8043150a46d436d6E"(ptr noalias noundef align 8 dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_2 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr %_2)
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h769bf7a93d043036E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_2, ptr noalias noundef readonly align 8 dereferenceable(16) %self)
  %0 = getelementptr inbounds i8, ptr %_2, i64 8
  %1 = load i64, ptr %0, align 8, !range !10, !noundef !4
  %2 = icmp eq i64 %1, 0
  %_4 = select i1 %2, i64 0, i64 1
  switch i64 %_4, label %bb5 [
    i64 1, label %bb2
    i64 0, label %bb4
  ]

bb5:                                              ; preds = %start
  unreachable

bb2:                                              ; preds = %start
  %ptr = load ptr, ptr %_2, align 8, !nonnull !4, !noundef !4
  %3 = getelementptr inbounds i8, ptr %_2, i64 8
  %layout.0 = load i64, ptr %3, align 8, !range !9, !noundef !4
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  %layout.1 = load i64, ptr %4, align 8, !noundef !4
  %_7 = getelementptr inbounds i8, ptr %self, i64 16
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h37134dd5886182ffE"(ptr noalias noundef nonnull readonly align 1 %_7, ptr noundef nonnull %ptr, i64 noundef %layout.0, i64 noundef %layout.1)
  br label %bb4

bb4:                                              ; preds = %bb2, %start
  call void @llvm.lifetime.end.p0(i64 24, ptr %_2)
  ret void
}

; main::main
; Function Attrs: uwtable
define internal void @_ZN4main4main17h67bdd25297d83862E() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_70 = alloca [16 x i8], align 8
  %_66 = alloca [16 x i8], align 8
  %_62 = alloca [16 x i8], align 8
  %_58 = alloca [16 x i8], align 8
  %_52 = alloca [16 x i8], align 8
  %_48 = alloca [16 x i8], align 8
  %_43 = alloca [24 x i8], align 8
  %_38 = alloca [16 x i8], align 8
  %_35 = alloca [16 x i8], align 8
  %_33 = alloca [16 x i8], align 8
  %_31 = alloca [16 x i8], align 8
  %_29 = alloca [16 x i8], align 8
  %_28 = alloca [64 x i8], align 8
  %_26 = alloca [48 x i8], align 8
  %_23 = alloca [16 x i8], align 8
  %_21 = alloca [16 x i8], align 8
  %_20 = alloca [32 x i8], align 8
  %_18 = alloca [48 x i8], align 8
  %res = alloca [24 x i8], align 8
  %new2 = alloca [24 x i8], align 8
  %_15 = alloca [4 x i8], align 4
  %_14 = alloca [1 x i8], align 1
  %_13 = alloca [4 x i8], align 4
  %_12 = alloca [16 x i8], align 8
  %_11 = alloca [16 x i8], align 8
  %MAIN_PROGRAM_STRING = alloca [24 x i8], align 8
  %_7 = alloca [16 x i8], align 8
  %_6 = alloca [16 x i8], align 8
  %_4 = alloca [48 x i8], align 8
  %_2 = alloca [4 x i8], align 4
  %kk = alloca [12 x i8], align 4
  call void @llvm.lifetime.start.p0(i64 12, ptr %kk)
  %1 = getelementptr inbounds [3 x i32], ptr %kk, i64 0, i64 0
  store i32 8, ptr %1, align 4
  %2 = getelementptr inbounds [3 x i32], ptr %kk, i64 0, i64 1
  store i32 8, ptr %2, align 4
  %3 = getelementptr inbounds [3 x i32], ptr %kk, i64 0, i64 2
  store i32 9283, ptr %3, align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr %_2)
  store i32 1000, ptr %_2, align 4
  call void @llvm.lifetime.start.p0(i64 48, ptr %_4)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_6)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_7)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_38)
  store ptr %_2, ptr %_38, align 8
  %4 = getelementptr inbounds i8, ptr %_38, i64 8
  store ptr @"_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17h5ebe2532ddb9230eE", ptr %4, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_7, ptr align 8 %_38, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_38)
  %5 = getelementptr inbounds [1 x %"core::fmt::rt::Argument<'_>"], ptr %_6, i64 0, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %5, ptr align 8 %_7, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_7)
  store ptr @alloc_3cf8a28b1a0b9f6efeedeb779c4e30d8, ptr %_4, align 8
  %6 = getelementptr inbounds i8, ptr %_4, i64 8
  store i64 2, ptr %6, align 8
  %7 = load ptr, ptr @0, align 8, !align !6, !noundef !4
  %8 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %9 = getelementptr inbounds i8, ptr %_4, i64 32
  store ptr %7, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %9, i64 8
  store i64 %8, ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %_4, i64 16
  store ptr %_6, ptr %11, align 8
  %12 = getelementptr inbounds i8, ptr %11, i64 8
  store i64 1, ptr %12, align 8
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr noalias nocapture noundef align 8 dereferenceable(48) %_4)
  call void @llvm.lifetime.end.p0(i64 48, ptr %_4)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_6)
  call void @llvm.lifetime.start.p0(i64 24, ptr %MAIN_PROGRAM_STRING)
  call void @llvm.lifetime.start.p0(i64 24, ptr %_43)
  store i64 0, ptr %_43, align 8
  %13 = getelementptr inbounds i8, ptr %_43, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %13, align 8
  %14 = getelementptr inbounds i8, ptr %_43, i64 16
  store i64 0, ptr %14, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %MAIN_PROGRAM_STRING, ptr align 8 %_43, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr %_43)
; invoke alloc::vec::Vec<T,A>::extend_from_slice
  invoke void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17hb14ca464f4c24098E"(ptr noalias noundef align 8 dereferenceable(24) %MAIN_PROGRAM_STRING, ptr noalias noundef nonnull readonly align 1 @alloc_c9c83b5dacde997ee6686b983822ba47, i64 noundef 33)
          to label %bb9 unwind label %cleanup

bb7:                                              ; preds = %bb6, %cleanup
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h6e310eb1b38c7843E"(ptr noalias noundef align 8 dereferenceable(24) %MAIN_PROGRAM_STRING) #24
          to label %bb8 unwind label %terminate

cleanup:                                          ; preds = %bb3, %bb9, %start
  %15 = landingpad { ptr, i32 }
          cleanup
  %16 = extractvalue { ptr, i32 } %15, 0
  %17 = extractvalue { ptr, i32 } %15, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %0)
  store ptr %16, ptr %0, align 8
  %18 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %17, ptr %18, align 8
  br label %bb7

bb9:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr %_11)
  store ptr @alloc_3edef0b68cfa9c8c95e6d4fe1a68842b, ptr %_11, align 8
  %19 = getelementptr inbounds i8, ptr %_11, i64 8
  store i64 5, ptr %19, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_12)
  store ptr @alloc_788cd4e9a7412aa082bf4e248b638dbd, ptr %_12, align 8
  %20 = getelementptr inbounds i8, ptr %_12, i64 8
  store i64 6, ptr %20, align 8
  call void @llvm.lifetime.start.p0(i64 4, ptr %_13)
  store i32 88888, ptr %_13, align 4
  call void @llvm.lifetime.start.p0(i64 1, ptr %_14)
  store i8 1, ptr %_14, align 1
  call void @llvm.lifetime.start.p0(i64 4, ptr %_15)
  store i32 99999, ptr %_15, align 4
  call void @llvm.lifetime.start.p0(i64 24, ptr %new2)
  call void @llvm.lifetime.start.p0(i64 24, ptr %res)
  call void @llvm.lifetime.start.p0(i64 48, ptr %_18)
  call void @llvm.lifetime.start.p0(i64 32, ptr %_20)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_21)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_48)
  store ptr %_11, ptr %_48, align 8
  %21 = getelementptr inbounds i8, ptr %_48, i64 8
  store ptr @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd63a69400007f2aeE", ptr %21, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_21, ptr align 8 %_48, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_48)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_23)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_52)
  store ptr %_12, ptr %_52, align 8
  %22 = getelementptr inbounds i8, ptr %_52, i64 8
  store ptr @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd63a69400007f2aeE", ptr %22, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_23, ptr align 8 %_52, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_52)
  %23 = getelementptr inbounds [2 x %"core::fmt::rt::Argument<'_>"], ptr %_20, i64 0, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %23, ptr align 8 %_21, i64 16, i1 false)
  %24 = getelementptr inbounds [2 x %"core::fmt::rt::Argument<'_>"], ptr %_20, i64 0, i64 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %24, ptr align 8 %_23, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_23)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_21)
  store ptr @alloc_4e0023beeca5f8e5d06a41f60e7c1e6e, ptr %_18, align 8
  %25 = getelementptr inbounds i8, ptr %_18, i64 8
  store i64 2, ptr %25, align 8
  %26 = load ptr, ptr @0, align 8, !align !6, !noundef !4
  %27 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %28 = getelementptr inbounds i8, ptr %_18, i64 32
  store ptr %26, ptr %28, align 8
  %29 = getelementptr inbounds i8, ptr %28, i64 8
  store i64 %27, ptr %29, align 8
  %30 = getelementptr inbounds i8, ptr %_18, i64 16
  store ptr %_20, ptr %30, align 8
  %31 = getelementptr inbounds i8, ptr %30, i64 8
  store i64 2, ptr %31, align 8
; invoke alloc::fmt::format
  invoke void @_ZN5alloc3fmt6format17h58dbea2d6d469c4dE(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %res, ptr noalias nocapture noundef readonly align 8 dereferenceable(48) %_18)
          to label %bb2 unwind label %cleanup

bb2:                                              ; preds = %bb9
  call void @llvm.lifetime.end.p0(i64 48, ptr %_18)
  call void @llvm.lifetime.end.p0(i64 32, ptr %_20)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %new2, ptr align 8 %res, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr %res)
  call void @llvm.lifetime.start.p0(i64 48, ptr %_26)
  call void @llvm.lifetime.start.p0(i64 64, ptr %_28)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_29)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_58)
  store ptr %new2, ptr %_58, align 8
  %32 = getelementptr inbounds i8, ptr %_58, i64 8
  store ptr @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h3a517c0b8c90cbfeE", ptr %32, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_29, ptr align 8 %_58, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_58)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_31)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_62)
  store ptr %_13, ptr %_62, align 8
  %33 = getelementptr inbounds i8, ptr %_62, i64 8
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E", ptr %33, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_31, ptr align 8 %_62, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_62)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_33)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_66)
  store ptr %_15, ptr %_66, align 8
  %34 = getelementptr inbounds i8, ptr %_66, i64 8
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E", ptr %34, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_33, ptr align 8 %_66, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_66)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_35)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_70)
  store ptr %_14, ptr %_70, align 8
  %35 = getelementptr inbounds i8, ptr %_70, i64 8
  store ptr @"_ZN45_$LT$main..Op$u20$as$u20$core..fmt..Debug$GT$3fmt17h25842c25dc8b05cbE", ptr %35, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_35, ptr align 8 %_70, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_70)
  %36 = getelementptr inbounds [4 x %"core::fmt::rt::Argument<'_>"], ptr %_28, i64 0, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %36, ptr align 8 %_29, i64 16, i1 false)
  %37 = getelementptr inbounds [4 x %"core::fmt::rt::Argument<'_>"], ptr %_28, i64 0, i64 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %37, ptr align 8 %_31, i64 16, i1 false)
  %38 = getelementptr inbounds [4 x %"core::fmt::rt::Argument<'_>"], ptr %_28, i64 0, i64 2
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %38, ptr align 8 %_33, i64 16, i1 false)
  %39 = getelementptr inbounds [4 x %"core::fmt::rt::Argument<'_>"], ptr %_28, i64 0, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %39, ptr align 8 %_35, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_35)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_33)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_31)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_29)
  store ptr @alloc_d44aaeff7ed57f2fd407cc45b24d7fa9, ptr %_26, align 8
  %40 = getelementptr inbounds i8, ptr %_26, i64 8
  store i64 5, ptr %40, align 8
  %41 = load ptr, ptr @0, align 8, !align !6, !noundef !4
  %42 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %43 = getelementptr inbounds i8, ptr %_26, i64 32
  store ptr %41, ptr %43, align 8
  %44 = getelementptr inbounds i8, ptr %43, i64 8
  store i64 %42, ptr %44, align 8
  %45 = getelementptr inbounds i8, ptr %_26, i64 16
  store ptr %_28, ptr %45, align 8
  %46 = getelementptr inbounds i8, ptr %45, i64 8
  store i64 4, ptr %46, align 8
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr noalias nocapture noundef align 8 dereferenceable(48) %_26)
          to label %bb3 unwind label %cleanup1

bb6:                                              ; preds = %cleanup1
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h6e310eb1b38c7843E"(ptr noalias noundef align 8 dereferenceable(24) %new2) #24
          to label %bb7 unwind label %terminate

cleanup1:                                         ; preds = %bb2
  %47 = landingpad { ptr, i32 }
          cleanup
  %48 = extractvalue { ptr, i32 } %47, 0
  %49 = extractvalue { ptr, i32 } %47, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %0)
  store ptr %48, ptr %0, align 8
  %50 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %49, ptr %50, align 8
  br label %bb6

bb3:                                              ; preds = %bb2
  call void @llvm.lifetime.end.p0(i64 48, ptr %_26)
  call void @llvm.lifetime.end.p0(i64 64, ptr %_28)
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h6e310eb1b38c7843E"(ptr noalias noundef align 8 dereferenceable(24) %new2)
          to label %bb4 unwind label %cleanup

bb4:                                              ; preds = %bb3
  call void @llvm.lifetime.end.p0(i64 24, ptr %new2)
  call void @llvm.lifetime.end.p0(i64 4, ptr %_15)
  call void @llvm.lifetime.end.p0(i64 1, ptr %_14)
  call void @llvm.lifetime.end.p0(i64 4, ptr %_13)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_12)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_11)
; call core::ptr::drop_in_place<alloc::string::String>
  call void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h6e310eb1b38c7843E"(ptr noalias noundef align 8 dereferenceable(24) %MAIN_PROGRAM_STRING)
  call void @llvm.lifetime.end.p0(i64 24, ptr %MAIN_PROGRAM_STRING)
  call void @llvm.lifetime.end.p0(i64 4, ptr %_2)
  call void @llvm.lifetime.end.p0(i64 12, ptr %kk)
  ret void

terminate:                                        ; preds = %bb7, %bb6
  %51 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %52 = extractvalue { ptr, i32 } %51, 0
  %53 = extractvalue { ptr, i32 } %51, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() #23
  unreachable

bb8:                                              ; preds = %bb7
  %54 = load ptr, ptr %0, align 8, !noundef !4
  %55 = getelementptr inbounds i8, ptr %0, i64 8
  %56 = load i32, ptr %55, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %0)
  %57 = insertvalue { ptr, i32 } poison, ptr %54, 0
  %58 = insertvalue { ptr, i32 } %57, i32 %56, 1
  resume { ptr, i32 } %58
}

; <main::Op as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @"_ZN45_$LT$main..Op$u20$as$u20$core..fmt..Debug$GT$3fmt17h25842c25dc8b05cbE"(ptr noalias noundef readonly align 1 dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #2 {
start:
  %_3 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_3)
  %0 = load i8, ptr %self, align 1, !range !14, !noundef !4
  %_4 = zext i8 %0 to i64
  switch i64 %_4, label %bb1 [
    i64 0, label %bb3
    i64 1, label %bb4
    i64 2, label %bb2
  ]

bb1:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  store ptr @alloc_e5af32aee833d2e1c4d09a8b77fd9715, ptr %_3, align 8
  %1 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 4, ptr %1, align 8
  br label %bb5

bb4:                                              ; preds = %start
  store ptr @alloc_5fbefac694924822db8fd0f7c11bfc6e, ptr %_3, align 8
  %2 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 3, ptr %2, align 8
  br label %bb5

bb2:                                              ; preds = %start
  store ptr @alloc_f855d6e9fed35f8dfc3fb4617c12f82f, ptr %_3, align 8
  %3 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 3, ptr %3, align 8
  br label %bb5

bb5:                                              ; preds = %bb2, %bb4, %bb3
  %4 = load ptr, ptr %_3, align 8, !nonnull !4, !align !5, !noundef !4
  %5 = getelementptr inbounds i8, ptr %_3, i64 8
  %6 = load i64, ptr %5, align 8, !noundef !4
; call core::fmt::Formatter::write_str
  %_0 = call noundef zeroext i1 @_ZN4core3fmt9Formatter9write_str17hc300e87735b333caE(ptr noalias noundef align 8 dereferenceable(64) %f, ptr noalias noundef nonnull readonly align 1 %4, i64 noundef %6)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_3)
  ret i1 %_0
}

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare noundef i64 @_ZN3std2rt19lang_start_internal17hd3ad173a069f2ae5E(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd248aa3f582d4052E"(ptr noalias noundef nonnull readonly align 1, i64 noundef, ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #0

; Function Attrs: uwtable
declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctpop.i64(i64) #7

; core::panicking::panic_nounwind
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking14panic_nounwind17h0ca77474896e89daE(ptr noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #8

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17hf3031f82c202a80dE(ptr noalias nocapture noundef readonly align 8 dereferenceable(48), ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #9

; core::panicking::panic_cannot_unwind
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking19panic_cannot_unwind17he766a33bd515ac66E() unnamed_addr #8

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E"(ptr noalias noundef readonly align 4 dereferenceable(4), ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::UpperHex for i32>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17hc15a4a0cd502e3bcE"(ptr noalias noundef readonly align 4 dereferenceable(4), ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::LowerHex for i32>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17hdb5970dd58d96e1dE"(ptr noalias noundef readonly align 4 dereferenceable(4), ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #0

; core::panicking::panic_in_cleanup
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() unnamed_addr #8

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17hfb97fcfe2d432218E(ptr noalias noundef nonnull readonly align 1, i64 noundef, ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #9

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #10

; core::panicking::panic_const::panic_const_div_by_zero
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h6a572ca1aae6312bE(ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #9

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #11

; alloc::raw_vec::handle_error
; Function Attrs: cold noreturn uwtable
declare void @_ZN5alloc7raw_vec12handle_error17h7f9cd8199ddef69cE(i64 noundef, i64) unnamed_addr #12

; alloc::fmt::format::format_inner
; Function Attrs: uwtable
declare void @_ZN5alloc3fmt6format12format_inner17h457d6f4cc84042e9E(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24), ptr noalias nocapture noundef readonly align 8 dereferenceable(48)) unnamed_addr #0

; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @__rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #13

; Function Attrs: nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias noundef ptr @__rust_alloc_zeroed(i64 noundef, i64 allocalign noundef) unnamed_addr #14

; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @__rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #15

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #16

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #7

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #17

; Function Attrs: nounwind allockind("free") uwtable
declare void @__rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #18

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr noalias nocapture noundef align 8 dereferenceable(48)) unnamed_addr #0

; core::fmt::Formatter::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_ZN4core3fmt9Formatter9write_str17hc300e87735b333caE(ptr noalias noundef align 8 dereferenceable(64), ptr noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #0

define i32 @main(i32 %0, ptr %1) unnamed_addr #19 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17hfc21bb30c4fbffc2E(ptr @_ZN4main4main17h67bdd25297d83862E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #20

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #20

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { inlinehint nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { alwaysinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #8 = { cold noinline noreturn nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #11 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #12 = { cold noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #17 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #18 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #19 = { "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #20 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #21 = { noreturn nounwind }
attributes #22 = { noreturn }
attributes #23 = { cold noreturn nounwind }
attributes #24 = { cold }
attributes #25 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.80.0 (051478957 2024-07-21)"}
!3 = !{i32 3094383}
!4 = !{}
!5 = !{i64 1}
!6 = !{i64 8}
!7 = !{i8 -1, i8 2}
!8 = !{i8 0, i8 2}
!9 = !{i64 1, i64 -9223372036854775807}
!10 = !{i64 0, i64 -9223372036854775807}
!11 = !{i64 0, i64 2}
!12 = !{i64 0, i64 -9223372036854775808}
!13 = !{i64 0, i64 -9223372036854775806}
!14 = !{i8 0, i8 3}
