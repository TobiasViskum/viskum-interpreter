; ModuleID = 'main.205e55082b145553-cgu.0'
source_filename = "main.205e55082b145553-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::fmt::rt::Argument<'_>" = type { %"core::fmt::rt::ArgumentType<'_>" }
%"core::fmt::rt::ArgumentType<'_>" = type { [1 x i64], ptr }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h2da35ab277f4fe50E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h10256255c7a161d1E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h10256255c7a161d1E" }>, align 8
@alloc_1f65c820ca68ae2f599734dcc2c915bc = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/alloc/layout.rs" }>, align 1
@alloc_312fbec039d269105e1e1b37557eaa16 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_1f65c820ca68ae2f599734dcc2c915bc, [16 x i8] c"P\00\00\00\00\00\00\00\C3\01\00\00)\00\00\00" }>, align 8
@0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@__rust_no_alloc_shim_is_unstable = external global i8
@alloc_3edef0b68cfa9c8c95e6d4fe1a68842b = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"Hello" }>, align 1
@alloc_788cd4e9a7412aa082bf4e248b638dbd = private unnamed_addr constant <{ [6 x i8] }> <{ [6 x i8] c"World!" }>, align 1
@alloc_0242e8ee118de705af76c627590b82cc = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c" " }>, align 1
@alloc_4e0023beeca5f8e5d06a41f60e7c1e6e = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr inttoptr (i64 1 to ptr), [8 x i8] zeroinitializer, ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_fa49c7126bbf089d50625d8296ac7a1f = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"Result: " }>, align 1
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc_d44aaeff7ed57f2fd407cc45b24d7fa9 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_fa49c7126bbf089d50625d8296ac7a1f, [8 x i8] c"\08\00\00\00\00\00\00\00", ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00", ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00", ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_e5af32aee833d2e1c4d09a8b77fd9715 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"NoOp" }>, align 1
@alloc_5fbefac694924822db8fd0f7c11bfc6e = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c"Add" }>, align 1
@alloc_f855d6e9fed35f8dfc3fb4617c12f82f = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c"Sub" }>, align 1

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hba6d22205787d239E(ptr noundef nonnull %f) unnamed_addr #0 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h1d2788fa1499e7ddE(ptr noundef nonnull %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !3
  ret void
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17hfc21bb30c4fbffc2E(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #1 {
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
define internal noundef zeroext i1 @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd63a69400007f2aeE"(ptr noalias noundef readonly align 8 dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #1 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !4, !align !5, !noundef !4
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !4
; call <str as core::fmt::Display>::fmt
  %_0 = call noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd248aa3f582d4052E"(ptr noalias noundef nonnull readonly align 1 %_3.0, i64 noundef %_3.1, ptr noalias noundef align 8 dereferenceable(64) %f)
  ret i1 %_0
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
define internal void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h6e310eb1b38c7843E"(ptr noalias noundef align 8 dereferenceable(24) %_1) unnamed_addr #1 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb6b2f82722c8c530E"(ptr noalias noundef align 8 dereferenceable(24) %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb6b2f82722c8c530E"(ptr noalias noundef align 8 dereferenceable(24) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h770afccffcf33d3eE"(ptr noalias noundef align 8 dereferenceable(24) %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  invoke void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc6571d6d0c59ea66E"(ptr noalias noundef align 8 dereferenceable(16) %_1) #13
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
  call void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() #14
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
define internal void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17hc6571d6d0c59ea66E"(ptr noalias noundef align 8 dereferenceable(16) %_1) unnamed_addr #1 {
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
  %_14 = load i64, ptr %_13, align 8, !range !6, !noundef !4
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
  call void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h6a572ca1aae6312bE(ptr noalias noundef readonly align 8 dereferenceable(24) @alloc_312fbec039d269105e1e1b37557eaa16) #15
  unreachable

bb4:                                              ; preds = %bb2
  br label %bb5

bb3:                                              ; preds = %bb2
  %1 = load i64, ptr @0, align 8, !range !7, !noundef !4
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  store i64 %1, ptr %_0, align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 %2, ptr %3, align 8
  br label %bb6

bb8:                                              ; preds = %bb5
  %array_size = mul nuw i64 %element_size, %n
  call void @llvm.lifetime.start.p0(i64 8, ptr %_20)
  store i64 %align, ptr %_20, align 8
  %_21 = load i64, ptr %_20, align 8, !range !6, !noundef !4
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
  %7 = load i64, ptr %_0, align 8, !range !7, !noundef !4
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
  %self1 = load i64, ptr %self, align 8, !range !6, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_5)
  store i64 %self1, ptr %_5, align 8
  %_6 = load i64, ptr %_5, align 8, !range !6, !noundef !4
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
  %_13 = load i64, ptr %_12, align 8, !range !9, !noundef !4
  switch i64 %_13, label %bb2 [
    i64 0, label %bb4
    i64 1, label %bb3
  ]

bb2:                                              ; preds = %start
  unreachable

bb4:                                              ; preds = %start
  %0 = getelementptr inbounds i8, ptr %_12, i64 8
  %res.0 = load i64, ptr %0, align 8, !range !10, !noundef !4
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
  %err.0 = load i64, ptr %5, align 8, !range !7, !noundef !4
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  %err.1 = load i64, ptr %6, align 8
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h7f9cd8199ddef69cE(i64 noundef %err.0, i64 %err.1) #15
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
  %_6.0 = load ptr, ptr %args, align 8, !nonnull !4, !align !11, !noundef !4
  %0 = getelementptr inbounds i8, ptr %args, i64 8
  %_6.1 = load i64, ptr %0, align 8, !noundef !4
  %1 = getelementptr inbounds i8, ptr %args, i64 16
  %_7.0 = load ptr, ptr %1, align 8, !nonnull !4, !align !11, !noundef !4
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
  %self = load i64, ptr %layout, align 8, !range !6, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_11)
  store i64 %self, ptr %_11, align 8
  %_12 = load i64, ptr %_11, align 8, !range !6, !noundef !4
  %_13 = icmp uge i64 %_12, 1
  %_14 = icmp ule i64 %_12, -9223372036854775808
  %_15 = and i1 %_13, %_14
  call void @llvm.assume(i1 %_15)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_11)
  %_0 = call noundef ptr @__rust_alloc(i64 noundef %_3, i64 noundef %_12) #16
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
  %11 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %12 = getelementptr inbounds i8, ptr %layout, i64 8
  %13 = load i64, ptr %12, align 8, !noundef !4
; call alloc::alloc::alloc
  %14 = call noundef ptr @_ZN5alloc5alloc5alloc17h1ece6c5426488636E(i64 noundef %11, i64 noundef %13)
  store ptr %14, ptr %raw_ptr, align 8
  br label %bb6

bb4:                                              ; preds = %bb1
  call void @llvm.lifetime.start.p0(i64 16, ptr %layout1)
  %15 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %16 = getelementptr inbounds i8, ptr %layout, i64 8
  %17 = load i64, ptr %16, align 8, !noundef !4
  store i64 %15, ptr %layout1, align 8
  %18 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %17, ptr %18, align 8
  %self4 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_29)
  store i64 %self4, ptr %_29, align 8
  %_30 = load i64, ptr %_29, align 8, !range !6, !noundef !4
  %_31 = icmp uge i64 %_30, 1
  %_32 = icmp ule i64 %_30, -9223372036854775808
  %_33 = and i1 %_31, %_32
  call void @llvm.assume(i1 %_33)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_29)
  %19 = call noundef ptr @__rust_alloc_zeroed(i64 noundef %size, i64 noundef %_30) #16
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

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h769bf7a93d043036E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 8 dereferenceable(16) %self) unnamed_addr #1 {
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

; alloc::raw_vec::RawVec<T,A>::try_allocate_in
; Function Attrs: uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17hfa11d2430c197263E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, i64 noundef %capacity, i1 noundef zeroext %0) unnamed_addr #1 personality ptr @rust_eh_personality {
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
  %15 = load i64, ptr %_8, align 8, !range !7, !noundef !4
  %16 = icmp eq i64 %15, 0
  %_9 = select i1 %16, i64 1, i64 0
  switch i64 %_9, label %bb5 [
    i64 0, label %bb7
    i64 1, label %bb6
  ]

bb5:                                              ; preds = %bb12, %bb7, %bb21
  unreachable

bb7:                                              ; preds = %bb21
  %layout.0 = load i64, ptr %_8, align 8, !range !6, !noundef !4
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
  %21 = load i64, ptr @0, align 8, !range !7, !noundef !4
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
  %_19.0 = load i64, ptr %self, align 8, !range !7, !noundef !4
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
define internal { i64, ptr } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17haac1913c85d88fc7E"() unnamed_addr #1 {
start:
  br label %bb3

bb3:                                              ; preds = %start
  ret { i64, ptr } { i64 0, ptr getelementptr (i8, ptr null, i64 1) }

bb1:                                              ; No predecessors!
  unreachable

bb2:                                              ; No predecessors!
  unreachable
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
  %5 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  %6 = getelementptr inbounds i8, ptr %layout, i64 8
  %7 = load i64, ptr %6, align 8, !noundef !4
  store i64 %5, ptr %layout1, align 8
  %8 = getelementptr inbounds i8, ptr %layout1, i64 8
  store i64 %7, ptr %8, align 8
  %self2 = load i64, ptr %layout, align 8, !range !6, !noundef !4
  call void @llvm.lifetime.start.p0(i64 8, ptr %_13)
  store i64 %self2, ptr %_13, align 8
  %_14 = load i64, ptr %_13, align 8, !range !6, !noundef !4
  %_15 = icmp uge i64 %_14, 1
  %_16 = icmp ule i64 %_14, -9223372036854775808
  %_17 = and i1 %_15, %_16
  call void @llvm.assume(i1 %_17)
  call void @llvm.lifetime.end.p0(i64 8, ptr %_13)
  call void @__rust_dealloc(ptr noundef %ptr, i64 noundef %_4, i64 noundef %_14) #16
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
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h770afccffcf33d3eE"(ptr noalias noundef align 8 dereferenceable(24) %self) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %self1 = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8, !noundef !4
  ret void
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h8043150a46d436d6E"(ptr noalias noundef align 8 dereferenceable(16) %self) unnamed_addr #1 {
start:
  %_2 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr %_2)
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h769bf7a93d043036E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_2, ptr noalias noundef readonly align 8 dereferenceable(16) %self)
  %0 = getelementptr inbounds i8, ptr %_2, i64 8
  %1 = load i64, ptr %0, align 8, !range !7, !noundef !4
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
  %layout.0 = load i64, ptr %3, align 8, !range !6, !noundef !4
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
define internal void @_ZN4main4main17h67bdd25297d83862E() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_50 = alloca [16 x i8], align 8
  %_46 = alloca [16 x i8], align 8
  %_42 = alloca [16 x i8], align 8
  %_38 = alloca [16 x i8], align 8
  %_32 = alloca [16 x i8], align 8
  %_28 = alloca [16 x i8], align 8
  %_25 = alloca [16 x i8], align 8
  %_23 = alloca [16 x i8], align 8
  %_21 = alloca [16 x i8], align 8
  %_19 = alloca [16 x i8], align 8
  %_18 = alloca [64 x i8], align 8
  %_16 = alloca [48 x i8], align 8
  %_13 = alloca [16 x i8], align 8
  %_11 = alloca [16 x i8], align 8
  %_10 = alloca [32 x i8], align 8
  %_8 = alloca [48 x i8], align 8
  %res = alloca [24 x i8], align 8
  %new2 = alloca [24 x i8], align 8
  %_5 = alloca [4 x i8], align 4
  %_4 = alloca [1 x i8], align 1
  %_3 = alloca [4 x i8], align 4
  %_2 = alloca [16 x i8], align 8
  %_1 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_1)
  store ptr @alloc_3edef0b68cfa9c8c95e6d4fe1a68842b, ptr %_1, align 8
  %1 = getelementptr inbounds i8, ptr %_1, i64 8
  store i64 5, ptr %1, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_2)
  store ptr @alloc_788cd4e9a7412aa082bf4e248b638dbd, ptr %_2, align 8
  %2 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 6, ptr %2, align 8
  call void @llvm.lifetime.start.p0(i64 4, ptr %_3)
  store i32 88888, ptr %_3, align 4
  call void @llvm.lifetime.start.p0(i64 1, ptr %_4)
  store i8 1, ptr %_4, align 1
  call void @llvm.lifetime.start.p0(i64 4, ptr %_5)
  store i32 99999, ptr %_5, align 4
  call void @llvm.lifetime.start.p0(i64 24, ptr %new2)
  call void @llvm.lifetime.start.p0(i64 24, ptr %res)
  call void @llvm.lifetime.start.p0(i64 48, ptr %_8)
  call void @llvm.lifetime.start.p0(i64 32, ptr %_10)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_11)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_28)
  store ptr %_1, ptr %_28, align 8
  %3 = getelementptr inbounds i8, ptr %_28, i64 8
  store ptr @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd63a69400007f2aeE", ptr %3, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_11, ptr align 8 %_28, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_28)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_13)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_32)
  store ptr %_2, ptr %_32, align 8
  %4 = getelementptr inbounds i8, ptr %_32, i64 8
  store ptr @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17hd63a69400007f2aeE", ptr %4, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_13, ptr align 8 %_32, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_32)
  %5 = getelementptr inbounds [2 x %"core::fmt::rt::Argument<'_>"], ptr %_10, i64 0, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %5, ptr align 8 %_11, i64 16, i1 false)
  %6 = getelementptr inbounds [2 x %"core::fmt::rt::Argument<'_>"], ptr %_10, i64 0, i64 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %6, ptr align 8 %_13, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_13)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_11)
  store ptr @alloc_4e0023beeca5f8e5d06a41f60e7c1e6e, ptr %_8, align 8
  %7 = getelementptr inbounds i8, ptr %_8, i64 8
  store i64 2, ptr %7, align 8
  %8 = load ptr, ptr @0, align 8, !align !11, !noundef !4
  %9 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %10 = getelementptr inbounds i8, ptr %_8, i64 32
  store ptr %8, ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %10, i64 8
  store i64 %9, ptr %11, align 8
  %12 = getelementptr inbounds i8, ptr %_8, i64 16
  store ptr %_10, ptr %12, align 8
  %13 = getelementptr inbounds i8, ptr %12, i64 8
  store i64 2, ptr %13, align 8
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17h58dbea2d6d469c4dE(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %res, ptr noalias nocapture noundef readonly align 8 dereferenceable(48) %_8)
  call void @llvm.lifetime.end.p0(i64 48, ptr %_8)
  call void @llvm.lifetime.end.p0(i64 32, ptr %_10)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %new2, ptr align 8 %res, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr %res)
  call void @llvm.lifetime.start.p0(i64 48, ptr %_16)
  call void @llvm.lifetime.start.p0(i64 64, ptr %_18)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_19)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_38)
  store ptr %new2, ptr %_38, align 8
  %14 = getelementptr inbounds i8, ptr %_38, i64 8
  store ptr @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h3a517c0b8c90cbfeE", ptr %14, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_19, ptr align 8 %_38, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_38)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_21)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_42)
  store ptr %_3, ptr %_42, align 8
  %15 = getelementptr inbounds i8, ptr %_42, i64 8
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E", ptr %15, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_21, ptr align 8 %_42, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_42)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_23)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_46)
  store ptr %_5, ptr %_46, align 8
  %16 = getelementptr inbounds i8, ptr %_46, i64 8
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E", ptr %16, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_23, ptr align 8 %_46, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_46)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_25)
  call void @llvm.lifetime.start.p0(i64 16, ptr %_50)
  store ptr %_4, ptr %_50, align 8
  %17 = getelementptr inbounds i8, ptr %_50, i64 8
  store ptr @"_ZN45_$LT$main..Op$u20$as$u20$core..fmt..Debug$GT$3fmt17h25842c25dc8b05cbE", ptr %17, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_25, ptr align 8 %_50, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_50)
  %18 = getelementptr inbounds [4 x %"core::fmt::rt::Argument<'_>"], ptr %_18, i64 0, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %18, ptr align 8 %_19, i64 16, i1 false)
  %19 = getelementptr inbounds [4 x %"core::fmt::rt::Argument<'_>"], ptr %_18, i64 0, i64 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %19, ptr align 8 %_21, i64 16, i1 false)
  %20 = getelementptr inbounds [4 x %"core::fmt::rt::Argument<'_>"], ptr %_18, i64 0, i64 2
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %20, ptr align 8 %_23, i64 16, i1 false)
  %21 = getelementptr inbounds [4 x %"core::fmt::rt::Argument<'_>"], ptr %_18, i64 0, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %21, ptr align 8 %_25, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_25)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_23)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_21)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_19)
  store ptr @alloc_d44aaeff7ed57f2fd407cc45b24d7fa9, ptr %_16, align 8
  %22 = getelementptr inbounds i8, ptr %_16, i64 8
  store i64 5, ptr %22, align 8
  %23 = load ptr, ptr @0, align 8, !align !11, !noundef !4
  %24 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8
  %25 = getelementptr inbounds i8, ptr %_16, i64 32
  store ptr %23, ptr %25, align 8
  %26 = getelementptr inbounds i8, ptr %25, i64 8
  store i64 %24, ptr %26, align 8
  %27 = getelementptr inbounds i8, ptr %_16, i64 16
  store ptr %_18, ptr %27, align 8
  %28 = getelementptr inbounds i8, ptr %27, i64 8
  store i64 4, ptr %28, align 8
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr noalias nocapture noundef align 8 dereferenceable(48) %_16)
          to label %bb2 unwind label %cleanup

bb4:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h6e310eb1b38c7843E"(ptr noalias noundef align 8 dereferenceable(24) %new2) #13
          to label %bb5 unwind label %terminate

cleanup:                                          ; preds = %start
  %29 = landingpad { ptr, i32 }
          cleanup
  %30 = extractvalue { ptr, i32 } %29, 0
  %31 = extractvalue { ptr, i32 } %29, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %0)
  store ptr %30, ptr %0, align 8
  %32 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %31, ptr %32, align 8
  br label %bb4

bb2:                                              ; preds = %start
  call void @llvm.lifetime.end.p0(i64 48, ptr %_16)
  call void @llvm.lifetime.end.p0(i64 64, ptr %_18)
; call core::ptr::drop_in_place<alloc::string::String>
  call void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h6e310eb1b38c7843E"(ptr noalias noundef align 8 dereferenceable(24) %new2)
  call void @llvm.lifetime.end.p0(i64 24, ptr %new2)
  call void @llvm.lifetime.end.p0(i64 4, ptr %_5)
  call void @llvm.lifetime.end.p0(i64 1, ptr %_4)
  call void @llvm.lifetime.end.p0(i64 4, ptr %_3)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_2)
  call void @llvm.lifetime.end.p0(i64 16, ptr %_1)
  ret void

terminate:                                        ; preds = %bb4
  %33 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %34 = extractvalue { ptr, i32 } %33, 0
  %35 = extractvalue { ptr, i32 } %33, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() #14
  unreachable

bb5:                                              ; preds = %bb4
  %36 = load ptr, ptr %0, align 8, !noundef !4
  %37 = getelementptr inbounds i8, ptr %0, i64 8
  %38 = load i32, ptr %37, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 16, ptr %0)
  %39 = insertvalue { ptr, i32 } poison, ptr %36, 0
  %40 = insertvalue { ptr, i32 } %39, i32 %38, 1
  resume { ptr, i32 } %40
}

; <main::Op as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @"_ZN45_$LT$main..Op$u20$as$u20$core..fmt..Debug$GT$3fmt17h25842c25dc8b05cbE"(ptr noalias noundef readonly align 1 dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #2 {
start:
  %_3 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr %_3)
  %0 = load i8, ptr %self, align 1, !range !12, !noundef !4
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
declare noundef i64 @_ZN3std2rt19lang_start_internal17hd3ad173a069f2ae5E(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #1

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd248aa3f582d4052E"(ptr noalias noundef nonnull readonly align 1, i64 noundef, ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #1

; Function Attrs: uwtable
declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; core::panicking::panic_in_cleanup
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() unnamed_addr #3

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #4

; core::panicking::panic_const::panic_const_div_by_zero
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h6a572ca1aae6312bE(ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #5

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #6

; alloc::raw_vec::handle_error
; Function Attrs: cold noreturn uwtable
declare void @_ZN5alloc7raw_vec12handle_error17h7f9cd8199ddef69cE(i64 noundef, i64) unnamed_addr #7

; alloc::fmt::format::format_inner
; Function Attrs: uwtable
declare void @_ZN5alloc3fmt6format12format_inner17h457d6f4cc84042e9E(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24), ptr noalias nocapture noundef readonly align 8 dereferenceable(48)) unnamed_addr #1

; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @__rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #8

; Function Attrs: nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias noundef ptr @__rust_alloc_zeroed(i64 noundef, i64 allocalign noundef) unnamed_addr #9

; Function Attrs: nounwind allockind("free") uwtable
declare void @__rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #10

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h5e7a4face9b21216E"(ptr noalias noundef readonly align 4 dereferenceable(4), ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #1

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr noalias nocapture noundef align 8 dereferenceable(48)) unnamed_addr #1

; core::fmt::Formatter::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_ZN4core3fmt9Formatter9write_str17hc300e87735b333caE(ptr noalias noundef align 8 dereferenceable(64), ptr noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #1

define i32 @main(i32 %0, ptr %1) unnamed_addr #11 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17hfc21bb30c4fbffc2E(ptr @_ZN4main4main17h67bdd25297d83862E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #12

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #12

attributes #0 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold noinline noreturn nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #5 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #7 = { cold noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #12 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #13 = { cold }
attributes #14 = { cold noreturn nounwind }
attributes #15 = { noreturn }
attributes #16 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.80.0 (051478957 2024-07-21)"}
!3 = !{i32 3065311}
!4 = !{}
!5 = !{i64 1}
!6 = !{i64 1, i64 -9223372036854775807}
!7 = !{i64 0, i64 -9223372036854775807}
!8 = !{i8 0, i8 2}
!9 = !{i64 0, i64 2}
!10 = !{i64 0, i64 -9223372036854775808}
!11 = !{i64 8}
!12 = !{i8 0, i8 3}
