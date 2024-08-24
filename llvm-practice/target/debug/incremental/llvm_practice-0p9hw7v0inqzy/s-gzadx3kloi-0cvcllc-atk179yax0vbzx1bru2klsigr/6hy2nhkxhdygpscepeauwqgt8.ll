; ModuleID = '6hy2nhkxhdygpscepeauwqgt8'
source_filename = "6hy2nhkxhdygpscepeauwqgt8"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::fmt::rt::Argument<'_>" = type { %"core::fmt::rt::ArgumentType<'_>" }
%"core::fmt::rt::ArgumentType<'_>" = type { [1 x i64], ptr }

@vtable.0 = private constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h76b38b0eafd0646bE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h14c250ebdcfcdfffE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h14c250ebdcfcdfffE" }>, align 8, !dbg !0
@0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_1f65c820ca68ae2f599734dcc2c915bc = private unnamed_addr constant <{ [80 x i8] }> <{ [80 x i8] c"/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/alloc/layout.rs" }>, align 1
@alloc_312fbec039d269105e1e1b37557eaa16 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_1f65c820ca68ae2f599734dcc2c915bc, [16 x i8] c"P\00\00\00\00\00\00\00\C3\01\00\00)\00\00\00" }>, align 8
@__rust_no_alloc_shim_is_unstable = external global i8
@1 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] c"\01\00\00\00\00\00\00\80", [8 x i8] undef }>, align 8
@alloc_3edef0b68cfa9c8c95e6d4fe1a68842b = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"Hello" }>, align 1
@alloc_788cd4e9a7412aa082bf4e248b638dbd = private unnamed_addr constant <{ [6 x i8] }> <{ [6 x i8] c"World!" }>, align 1
@alloc_0242e8ee118de705af76c627590b82cc = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c" " }>, align 1
@alloc_4e0023beeca5f8e5d06a41f60e7c1e6e = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr inttoptr (i64 1 to ptr), [8 x i8] zeroinitializer, ptr @alloc_0242e8ee118de705af76c627590b82cc, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_fa49c7126bbf089d50625d8296ac7a1f = private unnamed_addr constant <{ [8 x i8] }> <{ [8 x i8] c"Result: " }>, align 1
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc_9bce62b4958e9ae9fca1ec2ed4203a0b = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_fa49c7126bbf089d50625d8296ac7a1f, [8 x i8] c"\08\00\00\00\00\00\00\00", ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8

; std::sys_common::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hbb678c5755af6f60E(ptr noundef nonnull %f) unnamed_addr #0 !dbg !118 {
start:
  %f.dbg.spill = alloca [8 x i8], align 8
  %result.dbg.spill = alloca [0 x i8], align 1
  %dummy.dbg.spill = alloca [0 x i8], align 1
  call void @llvm.dbg.declare(metadata ptr %dummy.dbg.spill, metadata !131, metadata !DIExpression()), !dbg !140
  call void @llvm.dbg.declare(metadata ptr %result.dbg.spill, metadata !126, metadata !DIExpression()), !dbg !142
  store ptr %f, ptr %f.dbg.spill, align 8, !dbg !140
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill, metadata !125, metadata !DIExpression()), !dbg !143
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17hd5bf480b695223baE(ptr noundef nonnull %f), !dbg !144
  call void asm sideeffect "", "~{memory}"(), !dbg !145, !srcloc !146
  ret void, !dbg !147
}

; std::rt::lang_start
; Function Attrs: uwtable
define hidden noundef i64 @_ZN3std2rt10lang_start17he351fb1c915132d9E(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #1 !dbg !148 {
start:
  %v.dbg.spill = alloca [8 x i8], align 8
  %sigpipe.dbg.spill = alloca [1 x i8], align 1
  %argv.dbg.spill = alloca [8 x i8], align 8
  %argc.dbg.spill = alloca [8 x i8], align 8
  %main.dbg.spill = alloca [8 x i8], align 8
  %_8 = alloca [8 x i8], align 8
  %_5 = alloca [8 x i8], align 8
  store ptr %main, ptr %main.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %main.dbg.spill, metadata !156, metadata !DIExpression()), !dbg !162
  store i64 %argc, ptr %argc.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %argc.dbg.spill, metadata !157, metadata !DIExpression()), !dbg !163
  store ptr %argv, ptr %argv.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %argv.dbg.spill, metadata !158, metadata !DIExpression()), !dbg !164
  store i8 %sigpipe, ptr %sigpipe.dbg.spill, align 1
  call void @llvm.dbg.declare(metadata ptr %sigpipe.dbg.spill, metadata !159, metadata !DIExpression()), !dbg !165
  call void @llvm.lifetime.start.p0(i64 8, ptr %_5), !dbg !166
  call void @llvm.lifetime.start.p0(i64 8, ptr %_8), !dbg !167
  store ptr %main, ptr %_8, align 8, !dbg !167
; call std::rt::lang_start_internal
  %0 = call noundef i64 @_ZN3std2rt19lang_start_internal17hd3ad173a069f2ae5E(ptr noundef nonnull align 1 %_8, ptr noalias noundef readonly align 8 dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe), !dbg !166
  store i64 %0, ptr %_5, align 8, !dbg !166
  %v = load i64, ptr %_5, align 8, !dbg !168, !noundef !23
  store i64 %v, ptr %v.dbg.spill, align 8, !dbg !168
  call void @llvm.dbg.declare(metadata ptr %v.dbg.spill, metadata !160, metadata !DIExpression()), !dbg !169
  call void @llvm.lifetime.end.p0(i64 8, ptr %_8), !dbg !170
  call void @llvm.lifetime.end.p0(i64 8, ptr %_5), !dbg !170
  ret i64 %v, !dbg !171
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h14c250ebdcfcdfffE"(ptr noalias noundef readonly align 8 dereferenceable(8) %_1) unnamed_addr #2 !dbg !172 {
start:
  %self.dbg.spill = alloca [8 x i8], align 8
  %_1.dbg.spill = alloca [8 x i8], align 8
  %self = alloca [1 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !178, metadata !DIExpression(DW_OP_deref)), !dbg !179
  call void @llvm.dbg.declare(metadata ptr %self, metadata !180, metadata !DIExpression()), !dbg !200
  call void @llvm.lifetime.start.p0(i64 1, ptr %self), !dbg !202
  %_4 = load ptr, ptr %_1, align 8, !dbg !203, !nonnull !23, !noundef !23
; call std::sys_common::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hbb678c5755af6f60E(ptr noundef nonnull %_4), !dbg !202
; call <() as std::process::Termination>::report
  %0 = call noundef i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hfeb601ef834eda5dE"(), !dbg !202
  store i8 %0, ptr %self, align 1, !dbg !202
  store ptr %self, ptr %self.dbg.spill, align 8, !dbg !204
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !205, metadata !DIExpression()), !dbg !214
  %_6 = load i8, ptr %self, align 1, !dbg !216, !noundef !23
  %_0 = zext i8 %_6 to i32, !dbg !216
  call void @llvm.lifetime.end.p0(i64 1, ptr %self), !dbg !217
  ret i32 %_0, !dbg !218
}

; <&T as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h34d24e463382a7ceE"(ptr noalias noundef readonly align 8 dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #1 !dbg !219 {
start:
  %f.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !283, metadata !DIExpression()), !dbg !287
  store ptr %f, ptr %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill, metadata !284, metadata !DIExpression()), !dbg !288
  %_3.0 = load ptr, ptr %self, align 8, !dbg !289, !nonnull !23, !align !290, !noundef !23
  %0 = getelementptr inbounds i8, ptr %self, i64 8, !dbg !289
  %_3.1 = load i64, ptr %0, align 8, !dbg !289, !noundef !23
; call <str as core::fmt::Display>::fmt
  %_0 = call noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd248aa3f582d4052E"(ptr noalias noundef nonnull readonly align 1 %_3.0, i64 noundef %_3.1, ptr noalias noundef align 8 dereferenceable(64) %f), !dbg !291
  ret i1 %_0, !dbg !292
}

; core::fmt::rt::Argument::new_display
; Function Attrs: alwaysinline uwtable
define internal void @_ZN4core3fmt2rt8Argument11new_display17h419d8407df184383E(ptr dead_on_unwind noalias nocapture noundef writable sret([16 x i8]) align 8 dereferenceable(16) %_0, ptr noalias noundef readonly align 8 dereferenceable(16) %x) unnamed_addr #3 !dbg !293 {
start:
  %f.dbg.spill = alloca [8 x i8], align 8
  %x.dbg.spill = alloca [8 x i8], align 8
  %_3 = alloca [16 x i8], align 8
  store ptr %x, ptr %x.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill, metadata !324, metadata !DIExpression()), !dbg !325
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill, metadata !326, metadata !DIExpression()), !dbg !335
  store ptr @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h34d24e463382a7ceE", ptr %f.dbg.spill, align 8, !dbg !337
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill, metadata !334, metadata !DIExpression()), !dbg !338
  call void @llvm.lifetime.start.p0(i64 16, ptr %_3), !dbg !339
  store ptr %x, ptr %_3, align 8, !dbg !339
  %0 = getelementptr inbounds i8, ptr %_3, i64 8, !dbg !339
  store ptr @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h34d24e463382a7ceE", ptr %0, align 8, !dbg !339
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_3, i64 16, i1 false), !dbg !340
  call void @llvm.lifetime.end.p0(i64 16, ptr %_3), !dbg !341
  ret void, !dbg !342
}

; core::fmt::rt::Argument::new_display
; Function Attrs: alwaysinline uwtable
define internal void @_ZN4core3fmt2rt8Argument11new_display17hfecb26d178e99591E(ptr dead_on_unwind noalias nocapture noundef writable sret([16 x i8]) align 8 dereferenceable(16) %_0, ptr noalias noundef readonly align 8 dereferenceable(24) %x) unnamed_addr #3 !dbg !343 {
start:
  %f.dbg.spill = alloca [8 x i8], align 8
  %x.dbg.spill = alloca [8 x i8], align 8
  %_3 = alloca [16 x i8], align 8
  store ptr %x, ptr %x.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill, metadata !383, metadata !DIExpression()), !dbg !384
  call void @llvm.dbg.declare(metadata ptr %x.dbg.spill, metadata !385, metadata !DIExpression()), !dbg !396
  store ptr @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h3a517c0b8c90cbfeE", ptr %f.dbg.spill, align 8, !dbg !398
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill, metadata !395, metadata !DIExpression()), !dbg !399
  call void @llvm.lifetime.start.p0(i64 16, ptr %_3), !dbg !400
  store ptr %x, ptr %_3, align 8, !dbg !400
  %0 = getelementptr inbounds i8, ptr %_3, i64 8, !dbg !400
  store ptr @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h3a517c0b8c90cbfeE", ptr %0, align 8, !dbg !400
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_3, i64 16, i1 false), !dbg !401
  call void @llvm.lifetime.end.p0(i64 16, ptr %_3), !dbg !402
  ret void, !dbg !403
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117h208dee809765621aE(ptr dead_on_unwind noalias nocapture noundef writable sret([48 x i8]) align 8 dereferenceable(48) %_0, ptr noalias noundef readonly align 8 dereferenceable(32) %pieces, ptr noalias noundef readonly align 8 dereferenceable(32) %args) unnamed_addr #2 !dbg !404 {
start:
  %args.dbg.spill = alloca [8 x i8], align 8
  %pieces.dbg.spill = alloca [8 x i8], align 8
  store ptr %pieces, ptr %pieces.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %pieces.dbg.spill, metadata !471, metadata !DIExpression()), !dbg !473
  store ptr %args, ptr %args.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %args.dbg.spill, metadata !472, metadata !DIExpression()), !dbg !474
  store ptr %pieces, ptr %_0, align 8, !dbg !475
  %0 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !475
  store i64 2, ptr %0, align 8, !dbg !475
  %1 = load ptr, ptr @0, align 8, !dbg !475, !align !476, !noundef !23
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !475
  %3 = getelementptr inbounds i8, ptr %_0, i64 32, !dbg !475
  store ptr %1, ptr %3, align 8, !dbg !475
  %4 = getelementptr inbounds i8, ptr %3, i64 8, !dbg !475
  store i64 %2, ptr %4, align 8, !dbg !475
  %5 = getelementptr inbounds i8, ptr %_0, i64 16, !dbg !475
  store ptr %args, ptr %5, align 8, !dbg !475
  %6 = getelementptr inbounds i8, ptr %5, i64 8, !dbg !475
  store i64 2, ptr %6, align 8, !dbg !475
  ret void, !dbg !477
}

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hcfdb94a12fe950e6E(ptr dead_on_unwind noalias nocapture noundef writable sret([48 x i8]) align 8 dereferenceable(48) %_0, ptr noalias noundef readonly align 8 dereferenceable(32) %pieces, ptr noalias noundef readonly align 8 dereferenceable(16) %args) unnamed_addr #2 !dbg !478 {
start:
  %args.dbg.spill = alloca [8 x i8], align 8
  %pieces.dbg.spill = alloca [8 x i8], align 8
  store ptr %pieces, ptr %pieces.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %pieces.dbg.spill, metadata !487, metadata !DIExpression()), !dbg !489
  store ptr %args, ptr %args.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %args.dbg.spill, metadata !488, metadata !DIExpression()), !dbg !490
  store ptr %pieces, ptr %_0, align 8, !dbg !491
  %0 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !491
  store i64 2, ptr %0, align 8, !dbg !491
  %1 = load ptr, ptr @0, align 8, !dbg !491, !align !476, !noundef !23
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !491
  %3 = getelementptr inbounds i8, ptr %_0, i64 32, !dbg !491
  store ptr %1, ptr %3, align 8, !dbg !491
  %4 = getelementptr inbounds i8, ptr %3, i64 8, !dbg !491
  store i64 %2, ptr %4, align 8, !dbg !491
  %5 = getelementptr inbounds i8, ptr %_0, i64 16, !dbg !491
  store ptr %args, ptr %5, align 8, !dbg !491
  %6 = getelementptr inbounds i8, ptr %5, i64 8, !dbg !491
  store i64 1, ptr %6, align 8, !dbg !491
  ret void, !dbg !492
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h76b38b0eafd0646bE"(ptr noundef %_1) unnamed_addr #2 !dbg !493 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !502, metadata !DIExpression()), !dbg !507
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !503, metadata !DIExpression()), !dbg !507
  %0 = load ptr, ptr %_1, align 8, !dbg !507, !nonnull !23, !noundef !23
; call core::ops::function::FnOnce::call_once
  %_0 = call noundef i32 @_ZN4core3ops8function6FnOnce9call_once17haba073e74399bc57E(ptr noundef nonnull %0), !dbg !507
  ret i32 %_0, !dbg !507
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h0ff3fcf9ce826b3aE(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %0, i64 noundef %1) unnamed_addr #2 !dbg !508 {
start:
  %_1.dbg.spill = alloca [0 x i8], align 1
  %_2 = alloca [16 x i8], align 8
  store ptr %0, ptr %_2, align 8
  %2 = getelementptr inbounds i8, ptr %_2, i64 8
  store i64 %1, ptr %2, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !515, metadata !DIExpression()), !dbg !523
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !516, metadata !DIExpression()), !dbg !523
  %3 = load ptr, ptr %_2, align 8, !dbg !523, !nonnull !23, !align !290, !noundef !23
  %4 = getelementptr inbounds i8, ptr %_2, i64 8, !dbg !523
  %5 = load i64, ptr %4, align 8, !dbg !523, !noundef !23
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  call void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h12890283f9e53a37E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %3, i64 noundef %5), !dbg !523
  ret void, !dbg !523
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_ZN4core3ops8function6FnOnce9call_once17haba073e74399bc57E(ptr noundef nonnull %0) unnamed_addr #2 personality ptr @rust_eh_personality !dbg !524 {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
  call void @llvm.dbg.declare(metadata ptr %_1, metadata !528, metadata !DIExpression()), !dbg !530
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !529, metadata !DIExpression()), !dbg !530
; invoke std::rt::lang_start::{{closure}}
  %_0 = invoke noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h14c250ebdcfcdfffE"(ptr noalias noundef readonly align 8 dereferenceable(8) %_1)
          to label %bb1 unwind label %cleanup, !dbg !530

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8, !dbg !530, !noundef !23
  %3 = getelementptr inbounds i8, ptr %1, i64 8, !dbg !530
  %4 = load i32, ptr %3, align 8, !dbg !530, !noundef !23
  call void @llvm.lifetime.end.p0(i64 16, ptr %1), !dbg !530
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0, !dbg !530
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1, !dbg !530
  resume { ptr, i32 } %6, !dbg !530

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
  ret i32 %_0, !dbg !530
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17hd5bf480b695223baE(ptr noundef nonnull %_1) unnamed_addr #2 !dbg !531 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !533, metadata !DIExpression()), !dbg !537
  call void @llvm.dbg.declare(metadata ptr %_2, metadata !534, metadata !DIExpression()), !dbg !537
  call void %_1(), !dbg !537
  ret void, !dbg !537
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4d7861b68adcd32bE"(ptr noalias noundef align 8 dereferenceable(24) %_1) unnamed_addr #1 !dbg !538 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !544, metadata !DIExpression()), !dbg !545
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h3161afec24d1e10cE"(ptr noalias noundef align 8 dereferenceable(24) %_1), !dbg !545
  ret void, !dbg !545
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h3161afec24d1e10cE"(ptr noalias noundef align 8 dereferenceable(24) %_1) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !546 {
start:
  %0 = alloca [16 x i8], align 8
  %_1.dbg.spill = alloca [8 x i8], align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !551, metadata !DIExpression()), !dbg !554
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h29810ea43683eadfE"(ptr noalias noundef align 8 dereferenceable(24) %_1)
          to label %bb4 unwind label %cleanup, !dbg !554

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  invoke void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h957fa1f1ac57f0f4E"(ptr noalias noundef align 8 dereferenceable(16) %_1) #15
          to label %bb1 unwind label %terminate, !dbg !554

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
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h957fa1f1ac57f0f4E"(ptr noalias noundef align 8 dereferenceable(16) %_1), !dbg !554
  ret void, !dbg !554

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %6 = extractvalue { ptr, i32 } %5, 0
  %7 = extractvalue { ptr, i32 } %5, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() #16, !dbg !554
  unreachable, !dbg !554

bb1:                                              ; preds = %bb3
  %8 = load ptr, ptr %0, align 8, !dbg !554, !noundef !23
  %9 = getelementptr inbounds i8, ptr %0, i64 8, !dbg !554
  %10 = load i32, ptr %9, align 8, !dbg !554, !noundef !23
  call void @llvm.lifetime.end.p0(i64 16, ptr %0), !dbg !554
  %11 = insertvalue { ptr, i32 } poison, ptr %8, 0, !dbg !554
  %12 = insertvalue { ptr, i32 } %11, i32 %10, 1, !dbg !554
  resume { ptr, i32 } %12, !dbg !554
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h957fa1f1ac57f0f4E"(ptr noalias noundef align 8 dereferenceable(16) %_1) unnamed_addr #1 !dbg !555 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !560, metadata !DIExpression()), !dbg !563
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h78f429c21ee2e9e9E"(ptr noalias noundef align 8 dereferenceable(16) %_1), !dbg !563
  ret void, !dbg !563
}

; core::ptr::drop_in_place<std::rt::lang_start<()>::{{closure}}>
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hc293cf27ba73e22fE"(ptr noalias noundef align 8 dereferenceable(8) %_1) unnamed_addr #2 !dbg !564 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !568, metadata !DIExpression()), !dbg !571
  ret void, !dbg !571
}

; core::alloc::layout::Layout::array::inner
; Function Attrs: inlinehint uwtable
define internal { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h1e2e6b6a7c48ef5dE(i64 noundef %element_size, i64 noundef %align, i64 noundef %n) unnamed_addr #2 !dbg !572 {
start:
  %align.dbg.spill1 = alloca [8 x i8], align 8
  %array_size.dbg.spill = alloca [8 x i8], align 8
  %n.dbg.spill = alloca [8 x i8], align 8
  %align.dbg.spill = alloca [8 x i8], align 8
  %element_size.dbg.spill = alloca [8 x i8], align 8
  %_20 = alloca [8 x i8], align 8
  %_13 = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  store i64 %element_size, ptr %element_size.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %element_size.dbg.spill, metadata !605, metadata !DIExpression()), !dbg !610
  call void @llvm.dbg.declare(metadata ptr %element_size.dbg.spill, metadata !611, metadata !DIExpression()), !dbg !621
  store i64 %align, ptr %align.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill, metadata !606, metadata !DIExpression()), !dbg !623
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill, metadata !624, metadata !DIExpression()), !dbg !631
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill, metadata !633, metadata !DIExpression()), !dbg !641
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill, metadata !639, metadata !DIExpression()), !dbg !643
  store i64 %n, ptr %n.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %n.dbg.spill, metadata !607, metadata !DIExpression()), !dbg !645
  call void @llvm.dbg.declare(metadata ptr %n.dbg.spill, metadata !620, metadata !DIExpression()), !dbg !646
  %0 = icmp eq i64 %element_size, 0, !dbg !647
  br i1 %0, label %bb5, label %bb1, !dbg !647

bb5:                                              ; preds = %bb4, %start
  br label %bb8, !dbg !648

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr %_13), !dbg !651
  store i64 %align, ptr %_13, align 8, !dbg !651
  %_14 = load i64, ptr %_13, align 8, !dbg !651, !range !652, !noundef !23
  %_15 = icmp uge i64 %_14, 1, !dbg !651
  %_16 = icmp ule i64 %_14, -9223372036854775808, !dbg !651
  %_17 = and i1 %_15, %_16, !dbg !651
  call void @llvm.assume(i1 %_17), !dbg !651
  call void @llvm.lifetime.end.p0(i64 8, ptr %_13), !dbg !653
  %_11 = sub i64 %_14, 1, !dbg !654
  %_6 = sub i64 9223372036854775807, %_11, !dbg !655
  %_7 = icmp eq i64 %element_size, 0, !dbg !632
  br i1 %_7, label %panic, label %bb2, !dbg !632

bb2:                                              ; preds = %bb1
  %_5 = udiv i64 %_6, %element_size, !dbg !632
  %_4 = icmp ugt i64 %n, %_5, !dbg !656
  br i1 %_4, label %bb3, label %bb4, !dbg !656

panic:                                            ; preds = %bb1
; call core::panicking::panic_const::panic_const_div_by_zero
  call void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h6a572ca1aae6312bE(ptr noalias noundef readonly align 8 dereferenceable(24) @alloc_312fbec039d269105e1e1b37557eaa16) #17, !dbg !632
  unreachable, !dbg !632

bb4:                                              ; preds = %bb2
  br label %bb5, !dbg !657

bb3:                                              ; preds = %bb2
  %1 = load i64, ptr @0, align 8, !dbg !660, !range !661, !noundef !23
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !660
  store i64 %1, ptr %_0, align 8, !dbg !660
  %3 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !660
  store i64 %2, ptr %3, align 8, !dbg !660
  br label %bb6, !dbg !662

bb8:                                              ; preds = %bb5
  %array_size = mul nuw i64 %element_size, %n, !dbg !663
  store i64 %array_size, ptr %array_size.dbg.spill, align 8, !dbg !663
  call void @llvm.dbg.declare(metadata ptr %array_size.dbg.spill, metadata !608, metadata !DIExpression()), !dbg !664
  call void @llvm.dbg.declare(metadata ptr %array_size.dbg.spill, metadata !665, metadata !DIExpression()), !dbg !673
  call void @llvm.lifetime.start.p0(i64 8, ptr %_20), !dbg !675
  store i64 %align, ptr %_20, align 8, !dbg !675
  %_21 = load i64, ptr %_20, align 8, !dbg !675, !range !652, !noundef !23
  %_22 = icmp uge i64 %_21, 1, !dbg !675
  %_23 = icmp ule i64 %_21, -9223372036854775808, !dbg !675
  %_24 = and i1 %_22, %_23, !dbg !675
  call void @llvm.assume(i1 %_24), !dbg !675
  store i64 %_21, ptr %align.dbg.spill1, align 8, !dbg !675
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill1, metadata !672, metadata !DIExpression()), !dbg !676
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill1, metadata !677, metadata !DIExpression()), !dbg !684
  call void @llvm.lifetime.end.p0(i64 8, ptr %_20), !dbg !686
  %4 = icmp uge i64 %_21, 1, !dbg !687
  call void @llvm.assume(i1 %4), !dbg !687
  %5 = icmp ule i64 %_21, -9223372036854775808, !dbg !687
  call void @llvm.assume(i1 %5), !dbg !687
  store i64 %_21, ptr %_0, align 8, !dbg !688
  %6 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !688
  store i64 %array_size, ptr %6, align 8, !dbg !688
  br label %bb6, !dbg !662

bb7:                                              ; No predecessors!
  unreachable

bb6:                                              ; preds = %bb3, %bb8
  %7 = load i64, ptr %_0, align 8, !dbg !662, !range !661, !noundef !23
  %8 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !662
  %9 = load i64, ptr %8, align 8, !dbg !662
  %10 = insertvalue { i64, i64 } poison, i64 %7, 0, !dbg !662
  %11 = insertvalue { i64, i64 } %10, i64 %9, 1, !dbg !662
  ret { i64, i64 } %11, !dbg !662
}

; core::alloc::layout::Layout::dangling
; Function Attrs: inlinehint uwtable
define internal noundef nonnull ptr @_ZN4core5alloc6layout6Layout8dangling17h837658e2318fa047E(ptr noalias noundef readonly align 8 dereferenceable(16) %self) unnamed_addr #2 !dbg !689 {
start:
  %ptr.dbg.spill = alloca [8 x i8], align 8
  %addr.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill2 = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %_5 = alloca [8 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !695, metadata !DIExpression()), !dbg !696
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !697, metadata !DIExpression()), !dbg !704
  %self1 = load i64, ptr %self, align 8, !dbg !706, !range !652, !noundef !23
  store i64 %self1, ptr %self.dbg.spill2, align 8, !dbg !706
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !707, metadata !DIExpression()), !dbg !711
  call void @llvm.lifetime.start.p0(i64 8, ptr %_5), !dbg !713
  store i64 %self1, ptr %_5, align 8, !dbg !713
  %_6 = load i64, ptr %_5, align 8, !dbg !713, !range !652, !noundef !23
  %_7 = icmp uge i64 %_6, 1, !dbg !713
  %_8 = icmp ule i64 %_6, -9223372036854775808, !dbg !713
  %_9 = and i1 %_7, %_8, !dbg !713
  call void @llvm.assume(i1 %_9), !dbg !713
  store i64 %_6, ptr %addr.dbg.spill, align 8, !dbg !713
  call void @llvm.dbg.declare(metadata ptr %addr.dbg.spill, metadata !714, metadata !DIExpression()), !dbg !721
  call void @llvm.lifetime.end.p0(i64 8, ptr %_5), !dbg !723
  %ptr = getelementptr i8, ptr null, i64 %_6, !dbg !724
  store ptr %ptr, ptr %ptr.dbg.spill, align 8, !dbg !724
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !725, metadata !DIExpression()), !dbg !733
  br label %bb3, !dbg !735

bb3:                                              ; preds = %start
  ret ptr %ptr, !dbg !737

bb1:                                              ; No predecessors!
  unreachable

bb2:                                              ; No predecessors!
  unreachable
}

; core::option::Option<T>::map_or_else
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17ha6e74c3354ce4cf9E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 1 %0, i64 %1, ptr noalias noundef readonly align 8 dereferenceable(48) %default) unnamed_addr #2 personality ptr @rust_eh_personality !dbg !738 {
start:
  %t.dbg.spill = alloca [16 x i8], align 8
  %2 = alloca [16 x i8], align 8
  %f.dbg.spill = alloca [0 x i8], align 1
  %default.dbg.spill = alloca [8 x i8], align 8
  %_10 = alloca [1 x i8], align 1
  %_9 = alloca [1 x i8], align 1
  %self = alloca [16 x i8], align 8
  store ptr %0, ptr %self, align 8
  %3 = getelementptr inbounds i8, ptr %self, i64 8
  store i64 %1, ptr %3, align 8
  call void @llvm.dbg.declare(metadata ptr %self, metadata !765, metadata !DIExpression()), !dbg !770
  store ptr %default, ptr %default.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %default.dbg.spill, metadata !766, metadata !DIExpression()), !dbg !771
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill, metadata !767, metadata !DIExpression()), !dbg !772
  store i8 1, ptr %_10, align 1, !dbg !773
  store i8 1, ptr %_9, align 1, !dbg !773
  %4 = load ptr, ptr %self, align 8, !dbg !773, !noundef !23
  %5 = ptrtoint ptr %4 to i64, !dbg !773
  %6 = icmp eq i64 %5, 0, !dbg !773
  %_4 = select i1 %6, i64 0, i64 1, !dbg !773
  switch i64 %_4, label %bb1 [
    i64 0, label %bb2
    i64 1, label %bb3
  ], !dbg !774

bb1:                                              ; preds = %start
  unreachable, !dbg !773

bb2:                                              ; preds = %start
  store i8 0, ptr %_10, align 1, !dbg !775
; invoke alloc::fmt::format::{{closure}}
  invoke void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h2dc8367f5c84c60dE"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 8 dereferenceable(48) %default)
          to label %bb5 unwind label %cleanup, !dbg !775

bb3:                                              ; preds = %start
  %t.0 = load ptr, ptr %self, align 8, !dbg !776, !nonnull !23, !align !290, !noundef !23
  %7 = getelementptr inbounds i8, ptr %self, i64 8, !dbg !776
  %t.1 = load i64, ptr %7, align 8, !dbg !776, !noundef !23
  store ptr %t.0, ptr %t.dbg.spill, align 8, !dbg !776
  %8 = getelementptr inbounds i8, ptr %t.dbg.spill, i64 8, !dbg !776
  store i64 %t.1, ptr %8, align 8, !dbg !776
  call void @llvm.dbg.declare(metadata ptr %t.dbg.spill, metadata !768, metadata !DIExpression()), !dbg !777
  store i8 0, ptr %_9, align 1, !dbg !778
; invoke core::ops::function::FnOnce::call_once
  invoke void @_ZN4core3ops8function6FnOnce9call_once17h0ff3fcf9ce826b3aE(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %t.0, i64 noundef %t.1)
          to label %bb4 unwind label %cleanup, !dbg !778

bb11:                                             ; preds = %cleanup
  %9 = load i8, ptr %_9, align 1, !dbg !779, !range !780, !noundef !23
  %10 = trunc i8 %9 to i1, !dbg !779
  br i1 %10, label %bb10, label %bb7, !dbg !779

cleanup:                                          ; preds = %bb3, %bb2
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = extractvalue { ptr, i32 } %11, 0
  %13 = extractvalue { ptr, i32 } %11, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %2)
  store ptr %12, ptr %2, align 8
  %14 = getelementptr inbounds i8, ptr %2, i64 8
  store i32 %13, ptr %14, align 8
  br label %bb11

bb5:                                              ; preds = %bb2
  br label %bb6, !dbg !779

bb6:                                              ; preds = %bb9, %bb4, %bb5
  ret void, !dbg !781

bb4:                                              ; preds = %bb3
  %15 = load i8, ptr %_10, align 1, !dbg !779, !range !780, !noundef !23
  %16 = trunc i8 %15 to i1, !dbg !779
  br i1 %16, label %bb9, label %bb6, !dbg !779

bb9:                                              ; preds = %bb4
  br label %bb6, !dbg !779

bb7:                                              ; preds = %bb10, %bb11
  %17 = load i8, ptr %_10, align 1, !dbg !779, !range !780, !noundef !23
  %18 = trunc i8 %17 to i1, !dbg !779
  br i1 %18, label %bb12, label %bb8, !dbg !779

bb10:                                             ; preds = %bb11
  br label %bb7, !dbg !779

bb8:                                              ; preds = %bb12, %bb7
  %19 = load ptr, ptr %2, align 8, !dbg !782, !noundef !23
  %20 = getelementptr inbounds i8, ptr %2, i64 8, !dbg !782
  %21 = load i32, ptr %20, align 8, !dbg !782, !noundef !23
  call void @llvm.lifetime.end.p0(i64 16, ptr %2), !dbg !782
  %22 = insertvalue { ptr, i32 } poison, ptr %19, 0, !dbg !782
  %23 = insertvalue { ptr, i32 } %22, i32 %21, 1, !dbg !782
  resume { ptr, i32 } %23, !dbg !782

bb12:                                             ; preds = %bb7
  br label %bb8, !dbg !779
}

; <T as alloc::slice::hack::ConvertVec>::to_vec
; Function Attrs: inlinehint uwtable
define internal void @"_ZN52_$LT$T$u20$as$u20$alloc..slice..hack..ConvertVec$GT$6to_vec17hd776f6a506b6fc27E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %s.0, i64 noundef %s.1) unnamed_addr #2 !dbg !783 {
start:
  %err.dbg.spill = alloca [16 x i8], align 8
  %new_len.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill4 = alloca [8 x i8], align 8
  %count.dbg.spill = alloca [8 x i8], align 8
  %dest.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill3 = alloca [8 x i8], align 8
  %self.dbg.spill2 = alloca [8 x i8], align 8
  %self.dbg.spill1 = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %res.dbg.spill = alloca [16 x i8], align 8
  %capacity.dbg.spill = alloca [8 x i8], align 8
  %alloc.dbg.spill = alloca [0 x i8], align 1
  %s.dbg.spill = alloca [16 x i8], align 8
  %_12 = alloca [24 x i8], align 8
  %v = alloca [24 x i8], align 8
  store ptr %s.0, ptr %s.dbg.spill, align 8
  %0 = getelementptr inbounds i8, ptr %s.dbg.spill, i64 8
  store i64 %s.1, ptr %0, align 8
  call void @llvm.dbg.declare(metadata ptr %s.dbg.spill, metadata !795, metadata !DIExpression()), !dbg !799
  call void @llvm.dbg.declare(metadata ptr %s.dbg.spill, metadata !800, metadata !DIExpression()), !dbg !809
  call void @llvm.dbg.declare(metadata ptr %alloc.dbg.spill, metadata !796, metadata !DIExpression()), !dbg !811
  call void @llvm.dbg.declare(metadata ptr %alloc.dbg.spill, metadata !812, metadata !DIExpression()), !dbg !821
  call void @llvm.dbg.declare(metadata ptr %alloc.dbg.spill, metadata !823, metadata !DIExpression()), !dbg !852
  call void @llvm.dbg.declare(metadata ptr %v, metadata !797, metadata !DIExpression()), !dbg !854
  call void @llvm.lifetime.start.p0(i64 24, ptr %v), !dbg !855
  store i64 %s.1, ptr %capacity.dbg.spill, align 8, !dbg !856
  call void @llvm.dbg.declare(metadata ptr %capacity.dbg.spill, metadata !820, metadata !DIExpression()), !dbg !857
  call void @llvm.dbg.declare(metadata ptr %capacity.dbg.spill, metadata !831, metadata !DIExpression()), !dbg !858
  call void @llvm.lifetime.start.p0(i64 24, ptr %_12), !dbg !859
; call alloc::raw_vec::RawVec<T,A>::try_allocate_in
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h7f310d67781440f8E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_12, i64 noundef %s.1, i1 noundef zeroext false), !dbg !859
  %_13 = load i64, ptr %_12, align 8, !dbg !859, !range !860, !noundef !23
  switch i64 %_13, label %bb2 [
    i64 0, label %bb4
    i64 1, label %bb3
  ], !dbg !861

bb2:                                              ; preds = %start
  unreachable, !dbg !859

bb4:                                              ; preds = %start
  %1 = getelementptr inbounds i8, ptr %_12, i64 8, !dbg !862
  %res.0 = load i64, ptr %1, align 8, !dbg !862, !range !863, !noundef !23
  %2 = getelementptr inbounds i8, ptr %1, i64 8, !dbg !862
  %res.1 = load ptr, ptr %2, align 8, !dbg !862, !nonnull !23, !noundef !23
  store i64 %res.0, ptr %res.dbg.spill, align 8, !dbg !862
  %3 = getelementptr inbounds i8, ptr %res.dbg.spill, i64 8, !dbg !862
  store ptr %res.1, ptr %3, align 8, !dbg !862
  call void @llvm.dbg.declare(metadata ptr %res.dbg.spill, metadata !832, metadata !DIExpression()), !dbg !864
  call void @llvm.lifetime.end.p0(i64 24, ptr %_12), !dbg !865
  store i64 %res.0, ptr %v, align 8, !dbg !866
  %4 = getelementptr inbounds i8, ptr %v, i64 8, !dbg !866
  store ptr %res.1, ptr %4, align 8, !dbg !866
  %5 = getelementptr inbounds i8, ptr %v, i64 16, !dbg !866
  store i64 0, ptr %5, align 8, !dbg !866
  store ptr %s.0, ptr %self.dbg.spill, align 8, !dbg !867
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !868, metadata !DIExpression()), !dbg !879
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !881, metadata !DIExpression()), !dbg !889
  store ptr %v, ptr %self.dbg.spill1, align 8, !dbg !891
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !892, metadata !DIExpression()), !dbg !900
  store ptr %v, ptr %self.dbg.spill2, align 8, !dbg !902
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !903, metadata !DIExpression()), !dbg !911
  %6 = getelementptr inbounds i8, ptr %v, i64 8, !dbg !913
  %self = load ptr, ptr %6, align 8, !dbg !913, !nonnull !23, !noundef !23
  store ptr %self, ptr %self.dbg.spill3, align 8, !dbg !913
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill3, metadata !914, metadata !DIExpression()), !dbg !922
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill3, metadata !924, metadata !DIExpression()), !dbg !931
  store ptr %self, ptr %dest.dbg.spill, align 8, !dbg !933
  call void @llvm.dbg.declare(metadata ptr %dest.dbg.spill, metadata !877, metadata !DIExpression()), !dbg !934
  call void @llvm.dbg.declare(metadata ptr %dest.dbg.spill, metadata !887, metadata !DIExpression()), !dbg !935
  store i64 %s.1, ptr %count.dbg.spill, align 8, !dbg !936
  call void @llvm.dbg.declare(metadata ptr %count.dbg.spill, metadata !878, metadata !DIExpression()), !dbg !937
  call void @llvm.dbg.declare(metadata ptr %count.dbg.spill, metadata !888, metadata !DIExpression()), !dbg !938
  br label %bb7, !dbg !939

bb3:                                              ; preds = %start
  %7 = getelementptr inbounds i8, ptr %_12, i64 8, !dbg !941
  %err.0 = load i64, ptr %7, align 8, !dbg !941, !range !661, !noundef !23
  %8 = getelementptr inbounds i8, ptr %7, i64 8, !dbg !941
  %err.1 = load i64, ptr %8, align 8, !dbg !941
  store i64 %err.0, ptr %err.dbg.spill, align 8, !dbg !941
  %9 = getelementptr inbounds i8, ptr %err.dbg.spill, i64 8, !dbg !941
  store i64 %err.1, ptr %9, align 8, !dbg !941
  call void @llvm.dbg.declare(metadata ptr %err.dbg.spill, metadata !834, metadata !DIExpression()), !dbg !942
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h7f9cd8199ddef69cE(i64 noundef %err.0, i64 %err.1) #17, !dbg !943
  unreachable, !dbg !943

bb7:                                              ; preds = %bb4
  %10 = mul i64 %s.1, 1, !dbg !944
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %self, ptr align 1 %s.0, i64 %10, i1 false), !dbg !944
  store ptr %v, ptr %self.dbg.spill4, align 8, !dbg !945
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !946, metadata !DIExpression()), !dbg !954
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !956, metadata !DIExpression()), !dbg !964
  store i64 %s.1, ptr %new_len.dbg.spill, align 8, !dbg !966
  call void @llvm.dbg.declare(metadata ptr %new_len.dbg.spill, metadata !953, metadata !DIExpression()), !dbg !967
  %11 = getelementptr inbounds i8, ptr %v, i64 16, !dbg !968
  store i64 %s.1, ptr %11, align 8, !dbg !968
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %v, i64 24, i1 false), !dbg !969
  call void @llvm.lifetime.end.p0(i64 24, ptr %v), !dbg !970
  ret void, !dbg !971

bb5:                                              ; No predecessors!
  unreachable

bb6:                                              ; No predecessors!
  unreachable
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal noundef i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hfeb601ef834eda5dE"() unnamed_addr #2 !dbg !972 {
start:
  %_1.dbg.spill = alloca [0 x i8], align 1
  %self.dbg.spill = alloca [0 x i8], align 1
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !977, metadata !DIExpression()), !dbg !979
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !978, metadata !DIExpression()), !dbg !979
  ret i8 0, !dbg !980
}

; alloc::fmt::format
; Function Attrs: inlinehint uwtable
define internal void @_ZN5alloc3fmt6format17h58dbea2d6d469c4dE(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias nocapture noundef align 8 dereferenceable(48) %args) unnamed_addr #2 !dbg !981 {
start:
  %s.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [16 x i8], align 8
  call void @llvm.dbg.declare(metadata ptr %args, metadata !986, metadata !DIExpression()), !dbg !987
  call void @llvm.lifetime.start.p0(i64 16, ptr %_2), !dbg !988
  store ptr %args, ptr %self.dbg.spill, align 8, !dbg !988
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !989, metadata !DIExpression()), !dbg !998
  %_6.0 = load ptr, ptr %args, align 8, !dbg !1000, !nonnull !23, !align !476, !noundef !23
  %0 = getelementptr inbounds i8, ptr %args, i64 8, !dbg !1000
  %_6.1 = load i64, ptr %0, align 8, !dbg !1000, !noundef !23
  %1 = getelementptr inbounds i8, ptr %args, i64 16, !dbg !1001
  %_7.0 = load ptr, ptr %1, align 8, !dbg !1001, !nonnull !23, !align !476, !noundef !23
  %2 = getelementptr inbounds i8, ptr %1, i64 8, !dbg !1001
  %_7.1 = load i64, ptr %2, align 8, !dbg !1001, !noundef !23
  %3 = icmp eq i64 %_6.1, 0, !dbg !1002
  br i1 %3, label %bb4, label %bb5, !dbg !1002

bb4:                                              ; preds = %start
  %4 = icmp eq i64 %_7.1, 0, !dbg !1003
  br i1 %4, label %bb7, label %bb3, !dbg !1003

bb5:                                              ; preds = %start
  %5 = icmp eq i64 %_6.1, 1, !dbg !1004
  br i1 %5, label %bb6, label %bb3, !dbg !1004

bb7:                                              ; preds = %bb4
  store ptr inttoptr (i64 1 to ptr), ptr %_2, align 8, !dbg !1005
  %6 = getelementptr inbounds i8, ptr %_2, i64 8, !dbg !1005
  store i64 0, ptr %6, align 8, !dbg !1005
  br label %bb2, !dbg !1006

bb3:                                              ; preds = %bb6, %bb5, %bb4
  %7 = load ptr, ptr @0, align 8, !dbg !1007, !align !290, !noundef !23
  %8 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !1007
  store ptr %7, ptr %_2, align 8, !dbg !1007
  %9 = getelementptr inbounds i8, ptr %_2, i64 8, !dbg !1007
  store i64 %8, ptr %9, align 8, !dbg !1007
  br label %bb2, !dbg !1007

bb2:                                              ; preds = %bb3, %bb8, %bb7
  %10 = load ptr, ptr %_2, align 8, !dbg !988, !align !290, !noundef !23
  %11 = getelementptr inbounds i8, ptr %_2, i64 8, !dbg !988
  %12 = load i64, ptr %11, align 8, !dbg !988
; call core::option::Option<T>::map_or_else
  call void @"_ZN4core6option15Option$LT$T$GT$11map_or_else17ha6e74c3354ce4cf9E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 1 %10, i64 %12, ptr noalias noundef readonly align 8 dereferenceable(48) %args), !dbg !988
  call void @llvm.lifetime.end.p0(i64 16, ptr %_2), !dbg !1008
  ret void, !dbg !1009

bb6:                                              ; preds = %bb5
  %13 = icmp eq i64 %_7.1, 0, !dbg !1010
  br i1 %13, label %bb8, label %bb3, !dbg !1010

bb8:                                              ; preds = %bb6
  %s = getelementptr inbounds [0 x { ptr, i64 }], ptr %_6.0, i64 0, i64 0, !dbg !1011
  store ptr %s, ptr %s.dbg.spill, align 8, !dbg !1011
  call void @llvm.dbg.declare(metadata ptr %s.dbg.spill, metadata !996, metadata !DIExpression()), !dbg !1012
  %14 = getelementptr inbounds [0 x { ptr, i64 }], ptr %_6.0, i64 0, i64 0, !dbg !1013
  %_13.0 = load ptr, ptr %14, align 8, !dbg !1013, !nonnull !23, !align !290, !noundef !23
  %15 = getelementptr inbounds i8, ptr %14, i64 8, !dbg !1013
  %_13.1 = load i64, ptr %15, align 8, !dbg !1013, !noundef !23
  store ptr %_13.0, ptr %_2, align 8, !dbg !1014
  %16 = getelementptr inbounds i8, ptr %_2, i64 8, !dbg !1014
  store i64 %_13.1, ptr %16, align 8, !dbg !1014
  br label %bb2, !dbg !1015
}

; alloc::fmt::format::{{closure}}
; Function Attrs: inlinehint uwtable
define internal void @"_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h2dc8367f5c84c60dE"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 8 dereferenceable(48) %_1) unnamed_addr #2 !dbg !1016 {
start:
  %_1.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [48 x i8], align 8
  store ptr %_1, ptr %_1.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %_1.dbg.spill, metadata !1020, metadata !DIExpression(DW_OP_deref)), !dbg !1021
  call void @llvm.lifetime.start.p0(i64 48, ptr %_2), !dbg !1022
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_2, ptr align 8 %_1, i64 48, i1 false), !dbg !1022
; call alloc::fmt::format::format_inner
  call void @_ZN5alloc3fmt6format12format_inner17h457d6f4cc84042e9E(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias nocapture noundef align 8 dereferenceable(48) %_2), !dbg !1023
  call void @llvm.lifetime.end.p0(i64 48, ptr %_2), !dbg !1024
  ret void, !dbg !1025
}

; alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
; Function Attrs: inlinehint uwtable
define internal void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h12890283f9e53a37E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 %self.0, i64 noundef %self.1) unnamed_addr #2 !dbg !1026 {
start:
  %self.dbg.spill2 = alloca [16 x i8], align 8
  %self.dbg.spill = alloca [16 x i8], align 8
  %bytes = alloca [24 x i8], align 8
  %alloc.dbg.spill1 = alloca [0 x i8], align 1
  %alloc.dbg.spill = alloca [0 x i8], align 1
  call void @llvm.dbg.declare(metadata ptr %alloc.dbg.spill, metadata !1032, metadata !DIExpression()), !dbg !1038
  call void @llvm.dbg.declare(metadata ptr %alloc.dbg.spill1, metadata !1053, metadata !DIExpression()), !dbg !1058
  store ptr %self.0, ptr %self.dbg.spill, align 8, !dbg !1058
  %0 = getelementptr inbounds i8, ptr %self.dbg.spill, i64 8, !dbg !1058
  store i64 %self.1, ptr %0, align 8, !dbg !1058
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !1031, metadata !DIExpression()), !dbg !1060
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !1061, metadata !DIExpression()), !dbg !1070
  call void @llvm.dbg.declare(metadata ptr %bytes, metadata !1072, metadata !DIExpression()), !dbg !1080
  call void @llvm.lifetime.start.p0(i64 24, ptr %bytes), !dbg !1082
  store ptr %self.0, ptr %self.dbg.spill2, align 8, !dbg !1083
  %1 = getelementptr inbounds i8, ptr %self.dbg.spill2, i64 8, !dbg !1083
  store i64 %self.1, ptr %1, align 8, !dbg !1083
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !1051, metadata !DIExpression()), !dbg !1084
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !1045, metadata !DIExpression()), !dbg !1085
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !1037, metadata !DIExpression()), !dbg !1086
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !1057, metadata !DIExpression()), !dbg !1087
; call <T as alloc::slice::hack::ConvertVec>::to_vec
  call void @"_ZN52_$LT$T$u20$as$u20$alloc..slice..hack..ConvertVec$GT$6to_vec17hd776f6a506b6fc27E"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %bytes, ptr noalias noundef nonnull readonly align 1 %self.0, i64 noundef %self.1), !dbg !1088
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %bytes, i64 24, i1 false), !dbg !1089
  call void @llvm.lifetime.end.p0(i64 24, ptr %bytes), !dbg !1090
  ret void, !dbg !1091
}

; alloc::alloc::alloc
; Function Attrs: inlinehint uwtable
define internal noundef ptr @_ZN5alloc5alloc5alloc17h1ece6c5426488636E(i64 noundef %0, i64 noundef %1) unnamed_addr #2 !dbg !1092 {
start:
  %self.dbg.spill2 = alloca [8 x i8], align 8
  %self.dbg.spill1 = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %2 = alloca [1 x i8], align 1
  %_11 = alloca [8 x i8], align 8
  %layout = alloca [16 x i8], align 8
  %src.dbg.spill = alloca [8 x i8], align 8
  store ptr @__rust_no_alloc_shim_is_unstable, ptr %src.dbg.spill, align 8, !dbg !1098
  call void @llvm.dbg.declare(metadata ptr %src.dbg.spill, metadata !1104, metadata !DIExpression()), !dbg !1098
  store i64 %0, ptr %layout, align 8, !dbg !1098
  %3 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !1098
  store i64 %1, ptr %3, align 8, !dbg !1098
  call void @llvm.dbg.declare(metadata ptr %layout, metadata !1097, metadata !DIExpression()), !dbg !1106
  br label %bb5, !dbg !1107

bb5:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 1, ptr %2), !dbg !1109
  %4 = load volatile i8, ptr @__rust_no_alloc_shim_is_unstable, align 1, !dbg !1109
  store i8 %4, ptr %2, align 1, !dbg !1109
  %_2 = load i8, ptr %2, align 1, !dbg !1109, !noundef !23
  call void @llvm.lifetime.end.p0(i64 1, ptr %2), !dbg !1109
  store ptr %layout, ptr %self.dbg.spill, align 8, !dbg !1110
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !1111, metadata !DIExpression()), !dbg !1116
  %5 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !1118
  %_3 = load i64, ptr %5, align 8, !dbg !1118, !noundef !23
  store ptr %layout, ptr %self.dbg.spill1, align 8, !dbg !1119
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !1120, metadata !DIExpression()), !dbg !1124
  %self = load i64, ptr %layout, align 8, !dbg !1126, !range !652, !noundef !23
  store i64 %self, ptr %self.dbg.spill2, align 8, !dbg !1126
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !1127, metadata !DIExpression()), !dbg !1131
  call void @llvm.lifetime.start.p0(i64 8, ptr %_11), !dbg !1133
  store i64 %self, ptr %_11, align 8, !dbg !1133
  %_12 = load i64, ptr %_11, align 8, !dbg !1133, !range !652, !noundef !23
  %_13 = icmp uge i64 %_12, 1, !dbg !1133
  %_14 = icmp ule i64 %_12, -9223372036854775808, !dbg !1133
  %_15 = and i1 %_13, %_14, !dbg !1133
  call void @llvm.assume(i1 %_15), !dbg !1133
  call void @llvm.lifetime.end.p0(i64 8, ptr %_11), !dbg !1134
  %_0 = call noundef ptr @__rust_alloc(i64 noundef %_3, i64 noundef %_12) #18, !dbg !1135
  ret ptr %_0, !dbg !1136

bb3:                                              ; No predecessors!
  unreachable

bb4:                                              ; No predecessors!
  unreachable
}

; alloc::alloc::Global::alloc_impl
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %0, i64 noundef %1, i1 noundef zeroext %zeroed) unnamed_addr #2 !dbg !1137 {
start:
  %ptr.dbg.spill19 = alloca [16 x i8], align 8
  %data.dbg.spill18 = alloca [8 x i8], align 8
  %ptr.dbg.spill17 = alloca [8 x i8], align 8
  %v.dbg.spill15 = alloca [8 x i8], align 8
  %v.dbg.spill = alloca [8 x i8], align 8
  %ptr.dbg.spill13 = alloca [8 x i8], align 8
  %self.dbg.spill12 = alloca [8 x i8], align 8
  %self.dbg.spill10 = alloca [8 x i8], align 8
  %self.dbg.spill9 = alloca [8 x i8], align 8
  %ptr.dbg.spill = alloca [16 x i8], align 8
  %data.dbg.spill8 = alloca [8 x i8], align 8
  %data.dbg.spill = alloca [8 x i8], align 8
  %size.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill7 = alloca [8 x i8], align 8
  %zeroed.dbg.spill = alloca [1 x i8], align 1
  %self.dbg.spill = alloca [8 x i8], align 8
  %_29 = alloca [8 x i8], align 8
  %self6 = alloca [8 x i8], align 8
  %self5 = alloca [8 x i8], align 8
  %_11 = alloca [8 x i8], align 8
  %layout4 = alloca [16 x i8], align 8
  %raw_ptr = alloca [8 x i8], align 8
  %_0 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  %t.dbg.spill = alloca [0 x i8], align 1
  %e.dbg.spill3 = alloca [0 x i8], align 1
  %residual.dbg.spill2 = alloca [0 x i8], align 1
  %e.dbg.spill = alloca [0 x i8], align 1
  %err.dbg.spill = alloca [0 x i8], align 1
  %metadata.dbg.spill = alloca [8 x i8], align 8
  %len.dbg.spill1 = alloca [8 x i8], align 8
  %len.dbg.spill = alloca [8 x i8], align 8
  %residual.dbg.spill = alloca [0 x i8], align 1
  call void @llvm.dbg.declare(metadata ptr %residual.dbg.spill, metadata !1177, metadata !DIExpression()), !dbg !1199
  store i64 0, ptr %len.dbg.spill, align 8, !dbg !1200
  call void @llvm.dbg.declare(metadata ptr %len.dbg.spill, metadata !1208, metadata !DIExpression()), !dbg !1200
  store i64 0, ptr %len.dbg.spill1, align 8, !dbg !1213
  call void @llvm.dbg.declare(metadata ptr %len.dbg.spill1, metadata !1224, metadata !DIExpression()), !dbg !1213
  store i64 0, ptr %metadata.dbg.spill, align 8, !dbg !1229
  call void @llvm.dbg.declare(metadata ptr %metadata.dbg.spill, metadata !1236, metadata !DIExpression()), !dbg !1229
  call void @llvm.dbg.declare(metadata ptr %err.dbg.spill, metadata !1243, metadata !DIExpression()), !dbg !1280
  call void @llvm.dbg.declare(metadata ptr %e.dbg.spill, metadata !1282, metadata !DIExpression()), !dbg !1311
  call void @llvm.dbg.declare(metadata ptr %residual.dbg.spill2, metadata !1313, metadata !DIExpression()), !dbg !1332
  call void @llvm.dbg.declare(metadata ptr %e.dbg.spill3, metadata !1328, metadata !DIExpression()), !dbg !1334
  call void @llvm.dbg.declare(metadata ptr %t.dbg.spill, metadata !1335, metadata !DIExpression()), !dbg !1345
  store i64 %0, ptr %layout, align 8, !dbg !1345
  %2 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !1345
  store i64 %1, ptr %2, align 8, !dbg !1345
  store ptr %self, ptr %self.dbg.spill, align 8, !dbg !1345
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !1168, metadata !DIExpression()), !dbg !1347
  call void @llvm.dbg.declare(metadata ptr %layout, metadata !1169, metadata !DIExpression()), !dbg !1348
  %3 = zext i1 %zeroed to i8, !dbg !1345
  store i8 %3, ptr %zeroed.dbg.spill, align 1, !dbg !1345
  call void @llvm.dbg.declare(metadata ptr %zeroed.dbg.spill, metadata !1170, metadata !DIExpression()), !dbg !1349
  call void @llvm.dbg.declare(metadata ptr %raw_ptr, metadata !1173, metadata !DIExpression()), !dbg !1350
  call void @llvm.dbg.declare(metadata ptr %layout4, metadata !1351, metadata !DIExpression()), !dbg !1355
  call void @llvm.dbg.declare(metadata ptr %self5, metadata !1308, metadata !DIExpression()), !dbg !1357
  call void @llvm.dbg.declare(metadata ptr %self6, metadata !1277, metadata !DIExpression()), !dbg !1358
  store ptr %layout, ptr %self.dbg.spill7, align 8, !dbg !1359
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill7, metadata !1360, metadata !DIExpression()), !dbg !1366
  %4 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !1368
  %size = load i64, ptr %4, align 8, !dbg !1368, !noundef !23
  store i64 %size, ptr %size.dbg.spill, align 8, !dbg !1368
  call void @llvm.dbg.declare(metadata ptr %size.dbg.spill, metadata !1171, metadata !DIExpression()), !dbg !1369
  call void @llvm.dbg.declare(metadata ptr %size.dbg.spill, metadata !1211, metadata !DIExpression()), !dbg !1370
  call void @llvm.dbg.declare(metadata ptr %size.dbg.spill, metadata !1227, metadata !DIExpression()), !dbg !1372
  call void @llvm.dbg.declare(metadata ptr %size.dbg.spill, metadata !1239, metadata !DIExpression()), !dbg !1374
  %5 = icmp eq i64 %size, 0, !dbg !1376
  br i1 %5, label %bb2, label %bb1, !dbg !1376

bb2:                                              ; preds = %start
; call core::alloc::layout::Layout::dangling
  %data = call noundef nonnull ptr @_ZN4core5alloc6layout6Layout8dangling17h837658e2318fa047E(ptr noalias noundef readonly align 8 dereferenceable(16) %layout), !dbg !1377
  store ptr %data, ptr %data.dbg.spill, align 8, !dbg !1377
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill, metadata !1207, metadata !DIExpression()), !dbg !1378
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill, metadata !1379, metadata !DIExpression()), !dbg !1385
  store ptr %data, ptr %data.dbg.spill8, align 8, !dbg !1387
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill8, metadata !1223, metadata !DIExpression()), !dbg !1388
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill8, metadata !1235, metadata !DIExpression()), !dbg !1389
  store ptr %data, ptr %ptr.dbg.spill, align 8, !dbg !1390
  %6 = getelementptr inbounds i8, ptr %ptr.dbg.spill, i64 8, !dbg !1390
  store i64 0, ptr %6, align 8, !dbg !1390
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !1391, metadata !DIExpression()), !dbg !1400
  br label %bb10, !dbg !1402

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr %raw_ptr), !dbg !1404
  br i1 %zeroed, label %bb4, label %bb5, !dbg !1405

bb10:                                             ; preds = %bb2
  store ptr %data, ptr %_0, align 8, !dbg !1406
  %7 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1406
  store i64 0, ptr %7, align 8, !dbg !1406
  br label %bb7, !dbg !1407

bb8:                                              ; No predecessors!
  unreachable

bb9:                                              ; No predecessors!
  unreachable

bb7:                                              ; preds = %bb18, %bb12, %bb10
  %8 = load ptr, ptr %_0, align 8, !dbg !1408, !noundef !23
  %9 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1408
  %10 = load i64, ptr %9, align 8, !dbg !1408
  %11 = insertvalue { ptr, i64 } poison, ptr %8, 0, !dbg !1408
  %12 = insertvalue { ptr, i64 } %11, i64 %10, 1, !dbg !1408
  ret { ptr, i64 } %12, !dbg !1408

bb5:                                              ; preds = %bb1
  %13 = load i64, ptr %layout, align 8, !dbg !1409, !range !652, !noundef !23
  %14 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !1409
  %15 = load i64, ptr %14, align 8, !dbg !1409, !noundef !23
; call alloc::alloc::alloc
  %16 = call noundef ptr @_ZN5alloc5alloc5alloc17h1ece6c5426488636E(i64 noundef %13, i64 noundef %15), !dbg !1409
  store ptr %16, ptr %raw_ptr, align 8, !dbg !1409
  br label %bb6, !dbg !1409

bb4:                                              ; preds = %bb1
  call void @llvm.lifetime.start.p0(i64 16, ptr %layout4), !dbg !1410
  %17 = load i64, ptr %layout, align 8, !dbg !1410, !range !652, !noundef !23
  %18 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !1410
  %19 = load i64, ptr %18, align 8, !dbg !1410, !noundef !23
  store i64 %17, ptr %layout4, align 8, !dbg !1410
  %20 = getelementptr inbounds i8, ptr %layout4, i64 8, !dbg !1410
  store i64 %19, ptr %20, align 8, !dbg !1410
  store ptr %layout4, ptr %self.dbg.spill9, align 8, !dbg !1411
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill9, metadata !1364, metadata !DIExpression()), !dbg !1412
  store ptr %layout4, ptr %self.dbg.spill10, align 8, !dbg !1414
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill10, metadata !1415, metadata !DIExpression()), !dbg !1419
  %self11 = load i64, ptr %layout, align 8, !dbg !1421, !range !652, !noundef !23
  store i64 %self11, ptr %self.dbg.spill12, align 8, !dbg !1421
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill12, metadata !1422, metadata !DIExpression()), !dbg !1426
  call void @llvm.lifetime.start.p0(i64 8, ptr %_29), !dbg !1428
  store i64 %self11, ptr %_29, align 8, !dbg !1428
  %_30 = load i64, ptr %_29, align 8, !dbg !1428, !range !652, !noundef !23
  %_31 = icmp uge i64 %_30, 1, !dbg !1428
  %_32 = icmp ule i64 %_30, -9223372036854775808, !dbg !1428
  %_33 = and i1 %_31, %_32, !dbg !1428
  call void @llvm.assume(i1 %_33), !dbg !1428
  call void @llvm.lifetime.end.p0(i64 8, ptr %_29), !dbg !1429
  %21 = call noundef ptr @__rust_alloc_zeroed(i64 noundef %size, i64 noundef %_30) #18, !dbg !1430
  store ptr %21, ptr %raw_ptr, align 8, !dbg !1430
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout4), !dbg !1431
  br label %bb6, !dbg !1432

bb6:                                              ; preds = %bb4, %bb5
  call void @llvm.lifetime.start.p0(i64 8, ptr %_11), !dbg !1312
  call void @llvm.lifetime.start.p0(i64 8, ptr %self5), !dbg !1312
  call void @llvm.lifetime.start.p0(i64 8, ptr %self6), !dbg !1312
  %ptr = load ptr, ptr %raw_ptr, align 8, !dbg !1433, !noundef !23
  store ptr %ptr, ptr %ptr.dbg.spill13, align 8, !dbg !1433
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill13, metadata !1434, metadata !DIExpression()), !dbg !1441
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill13, metadata !1442, metadata !DIExpression()), !dbg !1451
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill13, metadata !1453, metadata !DIExpression()), !dbg !1458
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill13, metadata !1460, metadata !DIExpression()), !dbg !1466
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill13, metadata !1468, metadata !DIExpression()), !dbg !1477
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill13, metadata !1479, metadata !DIExpression()), !dbg !1483
  %_35 = ptrtoint ptr %ptr to i64, !dbg !1485
  %22 = icmp eq i64 %_35, 0, !dbg !1486
  br i1 %22, label %bb12, label %bb13, !dbg !1486

bb12:                                             ; preds = %bb6
  store ptr null, ptr %self6, align 8, !dbg !1487
  store ptr null, ptr %self5, align 8, !dbg !1488
  call void @llvm.lifetime.end.p0(i64 8, ptr %self6), !dbg !1489
  call void @llvm.lifetime.end.p0(i64 8, ptr %self5), !dbg !1490
  %23 = load ptr, ptr @0, align 8, !dbg !1491, !noundef !23
  %24 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !1491
  store ptr %23, ptr %_0, align 8, !dbg !1491
  %25 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1491
  store i64 %24, ptr %25, align 8, !dbg !1491
  call void @llvm.lifetime.end.p0(i64 8, ptr %_11), !dbg !1492
  call void @llvm.lifetime.end.p0(i64 8, ptr %raw_ptr), !dbg !1493
  br label %bb7, !dbg !1408

bb13:                                             ; preds = %bb6
  br label %bb15, !dbg !1494

bb15:                                             ; preds = %bb13
  store ptr %ptr, ptr %self6, align 8, !dbg !1496
  %v = load ptr, ptr %self6, align 8, !dbg !1497, !nonnull !23, !noundef !23
  store ptr %v, ptr %v.dbg.spill, align 8, !dbg !1497
  call void @llvm.dbg.declare(metadata ptr %v.dbg.spill, metadata !1278, metadata !DIExpression()), !dbg !1498
  store ptr %v, ptr %self5, align 8, !dbg !1499
  call void @llvm.lifetime.end.p0(i64 8, ptr %self6), !dbg !1489
  %v14 = load ptr, ptr %self5, align 8, !dbg !1500, !nonnull !23, !noundef !23
  store ptr %v14, ptr %v.dbg.spill15, align 8, !dbg !1500
  call void @llvm.dbg.declare(metadata ptr %v.dbg.spill15, metadata !1309, metadata !DIExpression()), !dbg !1501
  store ptr %v14, ptr %_11, align 8, !dbg !1502
  call void @llvm.lifetime.end.p0(i64 8, ptr %self5), !dbg !1490
  %ptr16 = load ptr, ptr %_11, align 8, !dbg !1312, !nonnull !23, !noundef !23
  store ptr %ptr16, ptr %ptr.dbg.spill17, align 8, !dbg !1312
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill17, metadata !1175, metadata !DIExpression()), !dbg !1503
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill17, metadata !1197, metadata !DIExpression()), !dbg !1504
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill17, metadata !1209, metadata !DIExpression()), !dbg !1505
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill17, metadata !1383, metadata !DIExpression()), !dbg !1506
  call void @llvm.lifetime.end.p0(i64 8, ptr %_11), !dbg !1492
  store ptr %ptr16, ptr %data.dbg.spill18, align 8, !dbg !1508
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill18, metadata !1225, metadata !DIExpression()), !dbg !1509
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill18, metadata !1237, metadata !DIExpression()), !dbg !1510
  store ptr %ptr16, ptr %ptr.dbg.spill19, align 8, !dbg !1511
  %26 = getelementptr inbounds i8, ptr %ptr.dbg.spill19, i64 8, !dbg !1511
  store i64 %size, ptr %26, align 8, !dbg !1511
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill19, metadata !1398, metadata !DIExpression()), !dbg !1512
  br label %bb18, !dbg !1514

bb14:                                             ; No predecessors!
  unreachable

bb18:                                             ; preds = %bb15
  store ptr %ptr16, ptr %_0, align 8, !dbg !1516
  %27 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1516
  store i64 %size, ptr %27, align 8, !dbg !1516
  call void @llvm.lifetime.end.p0(i64 8, ptr %raw_ptr), !dbg !1493
  br label %bb7, !dbg !1517

bb16:                                             ; No predecessors!
  unreachable

bb17:                                             ; No predecessors!
  unreachable
}

; alloc::raw_vec::RawVec<T,A>::current_memory
; Function Attrs: uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h18f91f87e43f8decE"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, ptr noalias noundef readonly align 8 dereferenceable(16) %self) unnamed_addr #1 !dbg !1518 {
start:
  %self.dbg.spill4 = alloca [8 x i8], align 8
  %self.dbg.spill3 = alloca [8 x i8], align 8
  %layout.dbg.spill = alloca [16 x i8], align 8
  %size.dbg.spill = alloca [8 x i8], align 8
  %rhs.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill1 = alloca [8 x i8], align 8
  %align.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %_9 = alloca [24 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !1540, metadata !DIExpression()), !dbg !1547
  br label %bb1, !dbg !1548

bb1:                                              ; preds = %start
  %_3 = load i64, ptr %self, align 8, !dbg !1549, !noundef !23
  %0 = icmp eq i64 %_3, 0, !dbg !1549
  br i1 %0, label %bb2, label %bb4, !dbg !1549

bb2:                                              ; preds = %bb1
  br label %bb3, !dbg !1550

bb4:                                              ; preds = %bb1
  store i64 1, ptr %align.dbg.spill, align 8, !dbg !1551
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill, metadata !1541, metadata !DIExpression()), !dbg !1559
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill, metadata !1560, metadata !DIExpression()), !dbg !1565
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill, metadata !1567, metadata !DIExpression()), !dbg !1571
  store i64 1, ptr %self.dbg.spill1, align 8, !dbg !1573
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !1577, metadata !DIExpression()), !dbg !1582
  %rhs = load i64, ptr %self, align 8, !dbg !1584, !noundef !23
  store i64 %rhs, ptr %rhs.dbg.spill, align 8, !dbg !1584
  call void @llvm.dbg.declare(metadata ptr %rhs.dbg.spill, metadata !1581, metadata !DIExpression()), !dbg !1585
  br label %bb7, !dbg !1586

bb3:                                              ; preds = %bb2
  %1 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1588
  store i64 0, ptr %1, align 8, !dbg !1588
  br label %bb5, !dbg !1589

bb7:                                              ; preds = %bb4
  %size = mul nuw i64 1, %rhs, !dbg !1590
  store i64 %size, ptr %size.dbg.spill, align 8, !dbg !1590
  call void @llvm.dbg.declare(metadata ptr %size.dbg.spill, metadata !1543, metadata !DIExpression()), !dbg !1591
  call void @llvm.dbg.declare(metadata ptr %size.dbg.spill, metadata !1564, metadata !DIExpression()), !dbg !1592
  call void @llvm.assume(i1 true), !dbg !1593
  call void @llvm.assume(i1 true), !dbg !1593
  store i64 1, ptr %layout.dbg.spill, align 8, !dbg !1594
  %2 = getelementptr inbounds i8, ptr %layout.dbg.spill, i64 8, !dbg !1594
  store i64 %size, ptr %2, align 8, !dbg !1594
  call void @llvm.dbg.declare(metadata ptr %layout.dbg.spill, metadata !1545, metadata !DIExpression()), !dbg !1595
  call void @llvm.lifetime.start.p0(i64 24, ptr %_9), !dbg !1596
  %3 = getelementptr inbounds i8, ptr %self, i64 8, !dbg !1597
  %self2 = load ptr, ptr %3, align 8, !dbg !1597, !nonnull !23, !noundef !23
  store ptr %self2, ptr %self.dbg.spill3, align 8, !dbg !1597
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill3, metadata !1598, metadata !DIExpression()), !dbg !1607
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill3, metadata !1609, metadata !DIExpression()), !dbg !1616
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill3, metadata !1618, metadata !DIExpression()), !dbg !1622
  store ptr %self2, ptr %self.dbg.spill4, align 8, !dbg !1624
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !1625, metadata !DIExpression()), !dbg !1635
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !1637, metadata !DIExpression()), !dbg !1644
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !1646, metadata !DIExpression()), !dbg !1651
  store ptr %self2, ptr %_9, align 8, !dbg !1596
  %4 = getelementptr inbounds i8, ptr %_9, i64 8, !dbg !1596
  store i64 1, ptr %4, align 8, !dbg !1596
  %5 = getelementptr inbounds i8, ptr %4, i64 8, !dbg !1596
  store i64 %size, ptr %5, align 8, !dbg !1596
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_9, i64 24, i1 false), !dbg !1653
  call void @llvm.lifetime.end.p0(i64 24, ptr %_9), !dbg !1654
  br label %bb5, !dbg !1589

bb6:                                              ; No predecessors!
  unreachable

bb5:                                              ; preds = %bb3, %bb7
  ret void, !dbg !1655
}

; alloc::raw_vec::RawVec<T,A>::try_allocate_in
; Function Attrs: uwtable
define internal void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h7f310d67781440f8E"(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24) %_0, i64 noundef %capacity, i1 noundef zeroext %0) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !1656 {
start:
  %pointer.dbg.spill = alloca [8 x i8], align 8
  %ptr.dbg.spill = alloca [16 x i8], align 8
  %alloc_size.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill3 = alloca [8 x i8], align 8
  %layout.dbg.spill = alloca [16 x i8], align 8
  %1 = alloca [16 x i8], align 8
  %align.dbg.spill = alloca [8 x i8], align 8
  %capacity.dbg.spill = alloca [8 x i8], align 8
  %_26 = alloca [1 x i8], align 1
  %self = alloca [16 x i8], align 8
  %result = alloca [16 x i8], align 8
  %_8 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  %alloc = alloca [0 x i8], align 1
  %init = alloca [1 x i8], align 1
  %kind.dbg.spill2 = alloca [16 x i8], align 8
  %self.dbg.spill1 = alloca [16 x i8], align 8
  %kind.dbg.spill = alloca [16 x i8], align 8
  %self.dbg.spill = alloca [16 x i8], align 8
  %err.dbg.spill = alloca [16 x i8], align 8
  %2 = load i64, ptr @1, align 8, !range !661, !noundef !23
  %3 = load i64, ptr getelementptr inbounds (i8, ptr @1, i64 8), align 8
  store i64 %2, ptr %err.dbg.spill, align 8, !dbg !1691
  %4 = getelementptr inbounds i8, ptr %err.dbg.spill, i64 8, !dbg !1691
  store i64 %3, ptr %4, align 8, !dbg !1691
  call void @llvm.dbg.declare(metadata ptr %err.dbg.spill, metadata !1683, metadata !DIExpression()), !dbg !1691
  %5 = load i64, ptr @0, align 8, !dbg !1691, !range !661, !noundef !23
  %6 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !1691
  store i64 %5, ptr %self.dbg.spill, align 8, !dbg !1692
  %7 = getelementptr inbounds i8, ptr %self.dbg.spill, i64 8, !dbg !1692
  store i64 %6, ptr %7, align 8, !dbg !1692
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !1698, metadata !DIExpression()), !dbg !1692
  %8 = load i64, ptr @0, align 8, !dbg !1692, !range !661, !noundef !23
  %9 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !1692
  store i64 %8, ptr %kind.dbg.spill, align 8, !dbg !1707
  %10 = getelementptr inbounds i8, ptr %kind.dbg.spill, i64 8, !dbg !1707
  store i64 %9, ptr %10, align 8, !dbg !1707
  call void @llvm.dbg.declare(metadata ptr %kind.dbg.spill, metadata !1715, metadata !DIExpression()), !dbg !1707
  %11 = load i64, ptr @0, align 8, !dbg !1707, !range !661, !noundef !23
  %12 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !1707
  store i64 %11, ptr %self.dbg.spill1, align 8, !dbg !1721
  %13 = getelementptr inbounds i8, ptr %self.dbg.spill1, i64 8, !dbg !1721
  store i64 %12, ptr %13, align 8, !dbg !1721
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !1699, metadata !DIExpression()), !dbg !1721
  %14 = load i64, ptr @0, align 8, !dbg !1721, !range !661, !noundef !23
  %15 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !1721
  store i64 %14, ptr %kind.dbg.spill2, align 8, !dbg !1744
  %16 = getelementptr inbounds i8, ptr %kind.dbg.spill2, i64 8, !dbg !1744
  store i64 %15, ptr %16, align 8, !dbg !1744
  call void @llvm.dbg.declare(metadata ptr %kind.dbg.spill2, metadata !1716, metadata !DIExpression()), !dbg !1744
  %17 = zext i1 %0 to i8, !dbg !1744
  store i8 %17, ptr %init, align 1, !dbg !1744
  store i64 %capacity, ptr %capacity.dbg.spill, align 8, !dbg !1744
  call void @llvm.dbg.declare(metadata ptr %capacity.dbg.spill, metadata !1676, metadata !DIExpression()), !dbg !1746
  call void @llvm.dbg.declare(metadata ptr %capacity.dbg.spill, metadata !1747, metadata !DIExpression()), !dbg !1754
  call void @llvm.dbg.declare(metadata ptr %init, metadata !1677, metadata !DIExpression()), !dbg !1756
  call void @llvm.dbg.declare(metadata ptr %alloc, metadata !1678, metadata !DIExpression()), !dbg !1757
  call void @llvm.dbg.declare(metadata ptr %layout, metadata !1679, metadata !DIExpression()), !dbg !1758
  call void @llvm.dbg.declare(metadata ptr %result, metadata !1685, metadata !DIExpression()), !dbg !1759
  call void @llvm.dbg.declare(metadata ptr %self, metadata !1701, metadata !DIExpression()), !dbg !1760
  call void @llvm.dbg.declare(metadata ptr %self, metadata !1718, metadata !DIExpression()), !dbg !1762
  store i8 1, ptr %_26, align 1, !dbg !1764
  br label %bb1, !dbg !1764

bb1:                                              ; preds = %start
  %18 = icmp eq i64 %capacity, 0, !dbg !1765
  br i1 %18, label %bb2, label %bb4, !dbg !1765

bb2:                                              ; preds = %bb1
  store i8 0, ptr %_26, align 1, !dbg !1766
; invoke alloc::raw_vec::RawVec<T,A>::new_in
  %19 = invoke { i64, ptr } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h42ecedc66878b095E"()
          to label %bb3 unwind label %cleanup, !dbg !1767

bb4:                                              ; preds = %bb1
  call void @llvm.lifetime.start.p0(i64 16, ptr %layout), !dbg !1768
  call void @llvm.lifetime.start.p0(i64 16, ptr %_8), !dbg !1755
  store i64 1, ptr %align.dbg.spill, align 8, !dbg !1769
  call void @llvm.dbg.declare(metadata ptr %align.dbg.spill, metadata !1779, metadata !DIExpression()), !dbg !1783
  call void @llvm.assume(i1 true), !dbg !1785
  call void @llvm.assume(i1 true), !dbg !1785
; invoke core::alloc::layout::Layout::array::inner
  %20 = invoke { i64, i64 } @_ZN4core5alloc6layout6Layout5array5inner17h1e2e6b6a7c48ef5dE(i64 noundef 1, i64 noundef 1, i64 noundef %capacity)
          to label %bb21 unwind label %cleanup, !dbg !1786

bb20:                                             ; preds = %cleanup
  %21 = load i8, ptr %_26, align 1, !dbg !1787, !range !780, !noundef !23
  %22 = trunc i8 %21 to i1, !dbg !1787
  br i1 %22, label %bb19, label %bb18, !dbg !1787

cleanup:                                          ; preds = %bb2, %bb8, %bb9, %bb4
  %23 = landingpad { ptr, i32 }
          cleanup
  %24 = extractvalue { ptr, i32 } %23, 0
  %25 = extractvalue { ptr, i32 } %23, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %1)
  store ptr %24, ptr %1, align 8
  %26 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %25, ptr %26, align 8
  br label %bb20

bb21:                                             ; preds = %bb4
  %27 = extractvalue { i64, i64 } %20, 0, !dbg !1786
  %28 = extractvalue { i64, i64 } %20, 1, !dbg !1786
  store i64 %27, ptr %_8, align 8, !dbg !1786
  %29 = getelementptr inbounds i8, ptr %_8, i64 8, !dbg !1786
  store i64 %28, ptr %29, align 8, !dbg !1786
  %30 = load i64, ptr %_8, align 8, !dbg !1755, !range !661, !noundef !23
  %31 = icmp eq i64 %30, 0, !dbg !1755
  %_9 = select i1 %31, i64 1, i64 0, !dbg !1755
  switch i64 %_9, label %bb5 [
    i64 0, label %bb7
    i64 1, label %bb6
  ], !dbg !1788

bb5:                                              ; preds = %bb12, %bb7, %bb21
  unreachable, !dbg !1755

bb7:                                              ; preds = %bb21
  %layout.0 = load i64, ptr %_8, align 8, !dbg !1789, !range !652, !noundef !23
  %32 = getelementptr inbounds i8, ptr %_8, i64 8, !dbg !1789
  %layout.1 = load i64, ptr %32, align 8, !dbg !1789, !noundef !23
  store i64 %layout.0, ptr %layout.dbg.spill, align 8, !dbg !1789
  %33 = getelementptr inbounds i8, ptr %layout.dbg.spill, i64 8, !dbg !1789
  store i64 %layout.1, ptr %33, align 8, !dbg !1789
  call void @llvm.dbg.declare(metadata ptr %layout.dbg.spill, metadata !1681, metadata !DIExpression()), !dbg !1790
  store i64 %layout.0, ptr %layout, align 8, !dbg !1791
  %34 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !1791
  store i64 %layout.1, ptr %34, align 8, !dbg !1791
  call void @llvm.lifetime.end.p0(i64 16, ptr %_8), !dbg !1792
  store ptr %layout, ptr %self.dbg.spill3, align 8, !dbg !1793
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill3, metadata !1794, metadata !DIExpression()), !dbg !1798
  store i64 %layout.1, ptr %alloc_size.dbg.spill, align 8, !dbg !1800
  call void @llvm.dbg.declare(metadata ptr %alloc_size.dbg.spill, metadata !1742, metadata !DIExpression()), !dbg !1801
  call void @llvm.lifetime.start.p0(i64 16, ptr %result), !dbg !1802
  %35 = load i8, ptr %init, align 1, !dbg !1803, !range !780, !noundef !23
  %36 = trunc i8 %35 to i1, !dbg !1803
  %_14 = zext i1 %36 to i64, !dbg !1803
  switch i64 %_14, label %bb5 [
    i64 0, label %bb9
    i64 1, label %bb8
  ], !dbg !1804

bb6:                                              ; preds = %bb21
  %37 = load i64, ptr @0, align 8, !dbg !1805, !range !661, !noundef !23
  %38 = load i64, ptr getelementptr inbounds (i8, ptr @0, i64 8), align 8, !dbg !1805
  %39 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1805
  store i64 %37, ptr %39, align 8, !dbg !1805
  %40 = getelementptr inbounds i8, ptr %39, i64 8, !dbg !1805
  store i64 %38, ptr %40, align 8, !dbg !1805
  store i64 1, ptr %_0, align 8, !dbg !1805
  call void @llvm.lifetime.end.p0(i64 16, ptr %_8), !dbg !1792
  br label %bb16, !dbg !1806

bb9:                                              ; preds = %bb7
; invoke <alloc::alloc::Global as core::alloc::Allocator>::allocate
  %41 = invoke { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h6ca1b7a3462ca17fE"(ptr noalias noundef nonnull readonly align 1 %alloc, i64 noundef %layout.0, i64 noundef %layout.1)
          to label %bb10 unwind label %cleanup, !dbg !1808

bb8:                                              ; preds = %bb7
; invoke <alloc::alloc::Global as core::alloc::Allocator>::allocate_zeroed
  %42 = invoke { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h51d97e6b86c1750aE"(ptr noalias noundef nonnull readonly align 1 %alloc, i64 noundef %layout.0, i64 noundef %layout.1)
          to label %bb11 unwind label %cleanup, !dbg !1809

bb10:                                             ; preds = %bb9
  %43 = extractvalue { ptr, i64 } %41, 0, !dbg !1808
  %44 = extractvalue { ptr, i64 } %41, 1, !dbg !1808
  store ptr %43, ptr %result, align 8, !dbg !1808
  %45 = getelementptr inbounds i8, ptr %result, i64 8, !dbg !1808
  store i64 %44, ptr %45, align 8, !dbg !1808
  br label %bb12, !dbg !1810

bb12:                                             ; preds = %bb11, %bb10
  %46 = load ptr, ptr %result, align 8, !dbg !1811, !noundef !23
  %47 = ptrtoint ptr %46 to i64, !dbg !1811
  %48 = icmp eq i64 %47, 0, !dbg !1811
  %_17 = select i1 %48, i64 1, i64 0, !dbg !1811
  switch i64 %_17, label %bb5 [
    i64 0, label %bb14
    i64 1, label %bb13
  ], !dbg !1812

bb11:                                             ; preds = %bb8
  %49 = extractvalue { ptr, i64 } %42, 0, !dbg !1809
  %50 = extractvalue { ptr, i64 } %42, 1, !dbg !1809
  store ptr %49, ptr %result, align 8, !dbg !1809
  %51 = getelementptr inbounds i8, ptr %result, i64 8, !dbg !1809
  store i64 %50, ptr %51, align 8, !dbg !1809
  br label %bb12, !dbg !1813

bb14:                                             ; preds = %bb12
  %ptr.0 = load ptr, ptr %result, align 8, !dbg !1814, !nonnull !23, !noundef !23
  %52 = getelementptr inbounds i8, ptr %result, i64 8, !dbg !1814
  %ptr.1 = load i64, ptr %52, align 8, !dbg !1814, !noundef !23
  store ptr %ptr.0, ptr %ptr.dbg.spill, align 8, !dbg !1814
  %53 = getelementptr inbounds i8, ptr %ptr.dbg.spill, i64 8, !dbg !1814
  store i64 %ptr.1, ptr %53, align 8, !dbg !1814
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !1687, metadata !DIExpression()), !dbg !1815
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !1689, metadata !DIExpression()), !dbg !1816
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !1817, metadata !DIExpression()), !dbg !1824
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !1826, metadata !DIExpression()), !dbg !1833
  store ptr %ptr.0, ptr %pointer.dbg.spill, align 8, !dbg !1835
  call void @llvm.dbg.declare(metadata ptr %pointer.dbg.spill, metadata !1836, metadata !DIExpression()), !dbg !1843
  %54 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1845
  store i64 %capacity, ptr %54, align 8, !dbg !1845
  %55 = getelementptr inbounds i8, ptr %54, i64 8, !dbg !1845
  store ptr %ptr.0, ptr %55, align 8, !dbg !1845
  store i64 0, ptr %_0, align 8, !dbg !1845
  call void @llvm.lifetime.end.p0(i64 16, ptr %result), !dbg !1846
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout), !dbg !1847
  br label %bb15, !dbg !1848

bb13:                                             ; preds = %bb12
  call void @llvm.lifetime.start.p0(i64 16, ptr %self), !dbg !1849
  store i64 %layout.0, ptr %self, align 8, !dbg !1849
  %56 = getelementptr inbounds i8, ptr %self, i64 8, !dbg !1849
  store i64 %layout.1, ptr %56, align 8, !dbg !1849
  %_19.0 = load i64, ptr %self, align 8, !dbg !1850, !range !661, !noundef !23
  %57 = getelementptr inbounds i8, ptr %self, i64 8, !dbg !1850
  %_19.1 = load i64, ptr %57, align 8, !dbg !1850
  call void @llvm.lifetime.end.p0(i64 16, ptr %self), !dbg !1851
  %58 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1852
  store i64 %_19.0, ptr %58, align 8, !dbg !1852
  %59 = getelementptr inbounds i8, ptr %58, i64 8, !dbg !1852
  store i64 %_19.1, ptr %59, align 8, !dbg !1852
  store i64 1, ptr %_0, align 8, !dbg !1852
  call void @llvm.lifetime.end.p0(i64 16, ptr %result), !dbg !1846
  br label %bb16, !dbg !1853

bb15:                                             ; preds = %bb3, %bb14
  br label %bb17, !dbg !1787

bb16:                                             ; preds = %bb6, %bb13
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout), !dbg !1847
  br label %bb17, !dbg !1787

bb17:                                             ; preds = %bb15, %bb16
  ret void, !dbg !1855

bb3:                                              ; preds = %bb2
  %_5.0 = extractvalue { i64, ptr } %19, 0, !dbg !1767
  %_5.1 = extractvalue { i64, ptr } %19, 1, !dbg !1767
  %60 = getelementptr inbounds i8, ptr %_0, i64 8, !dbg !1856
  store i64 %_5.0, ptr %60, align 8, !dbg !1856
  %61 = getelementptr inbounds i8, ptr %60, i64 8, !dbg !1856
  store ptr %_5.1, ptr %61, align 8, !dbg !1856
  store i64 0, ptr %_0, align 8, !dbg !1856
  br label %bb15, !dbg !1848

bb18:                                             ; preds = %bb19, %bb20
  %62 = load ptr, ptr %1, align 8, !dbg !1857, !noundef !23
  %63 = getelementptr inbounds i8, ptr %1, i64 8, !dbg !1857
  %64 = load i32, ptr %63, align 8, !dbg !1857, !noundef !23
  call void @llvm.lifetime.end.p0(i64 16, ptr %1), !dbg !1857
  %65 = insertvalue { ptr, i32 } poison, ptr %62, 0, !dbg !1857
  %66 = insertvalue { ptr, i32 } %65, i32 %64, 1, !dbg !1857
  resume { ptr, i32 } %66, !dbg !1857

bb19:                                             ; preds = %bb20
  br label %bb18, !dbg !1787
}

; alloc::raw_vec::RawVec<T,A>::new_in
; Function Attrs: uwtable
define internal { i64, ptr } @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h42ecedc66878b095E"() unnamed_addr #1 !dbg !1858 {
start:
  %ptr.dbg.spill = alloca [8 x i8], align 8
  %addr.dbg.spill = alloca [8 x i8], align 8
  %alloc.dbg.spill = alloca [0 x i8], align 1
  call void @llvm.dbg.declare(metadata ptr %alloc.dbg.spill, metadata !1863, metadata !DIExpression()), !dbg !1864
  store i64 1, ptr %addr.dbg.spill, align 8, !dbg !1865
  call void @llvm.dbg.declare(metadata ptr %addr.dbg.spill, metadata !1889, metadata !DIExpression()), !dbg !1893
  store ptr getelementptr (i8, ptr null, i64 1), ptr %ptr.dbg.spill, align 8, !dbg !1895
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !1880, metadata !DIExpression()), !dbg !1896
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !1897, metadata !DIExpression()), !dbg !1901
  br label %bb3, !dbg !1903

bb3:                                              ; preds = %start
  ret { i64, ptr } { i64 0, ptr getelementptr (i8, ptr null, i64 1) }, !dbg !1905

bb1:                                              ; No predecessors!
  unreachable

bb2:                                              ; No predecessors!
  unreachable
}

; <alloc::string::String as core::fmt::Display>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h3a517c0b8c90cbfeE"(ptr noalias noundef readonly align 8 dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(64) %f) unnamed_addr #2 !dbg !1906 {
start:
  %v.dbg.spill = alloca [16 x i8], align 8
  %len.dbg.spill = alloca [8 x i8], align 8
  %data.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill4 = alloca [8 x i8], align 8
  %self.dbg.spill2 = alloca [8 x i8], align 8
  %self.dbg.spill1 = alloca [8 x i8], align 8
  %f.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !1909, metadata !DIExpression()), !dbg !1911
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !1912, metadata !DIExpression()), !dbg !1919
  store ptr %f, ptr %f.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %f.dbg.spill, metadata !1910, metadata !DIExpression()), !dbg !1921
  store ptr %self, ptr %self.dbg.spill1, align 8, !dbg !1922
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !1923, metadata !DIExpression()), !dbg !1930
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !1931, metadata !DIExpression()), !dbg !1938
  store ptr %self, ptr %self.dbg.spill2, align 8, !dbg !1940
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !1941, metadata !DIExpression()), !dbg !1945
  %0 = getelementptr inbounds i8, ptr %self, i64 8, !dbg !1947
  %self3 = load ptr, ptr %0, align 8, !dbg !1947, !nonnull !23, !noundef !23
  store ptr %self3, ptr %self.dbg.spill4, align 8, !dbg !1947
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !1948, metadata !DIExpression()), !dbg !1952
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !1954, metadata !DIExpression()), !dbg !1958
  store ptr %self3, ptr %data.dbg.spill, align 8, !dbg !1960
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill, metadata !1961, metadata !DIExpression()), !dbg !1970
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill, metadata !1972, metadata !DIExpression()), !dbg !1979
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill, metadata !1981, metadata !DIExpression()), !dbg !1986
  %1 = getelementptr inbounds i8, ptr %self, i64 16, !dbg !1988
  %len = load i64, ptr %1, align 8, !dbg !1988, !noundef !23
  store i64 %len, ptr %len.dbg.spill, align 8, !dbg !1988
  call void @llvm.dbg.declare(metadata ptr %len.dbg.spill, metadata !1969, metadata !DIExpression()), !dbg !1989
  call void @llvm.dbg.declare(metadata ptr %len.dbg.spill, metadata !1978, metadata !DIExpression()), !dbg !1990
  call void @llvm.dbg.declare(metadata ptr %len.dbg.spill, metadata !1985, metadata !DIExpression()), !dbg !1991
  br label %bb4, !dbg !1992

bb4:                                              ; preds = %start
  store ptr %self3, ptr %v.dbg.spill, align 8, !dbg !1994
  %2 = getelementptr inbounds i8, ptr %v.dbg.spill, i64 8, !dbg !1994
  store i64 %len, ptr %2, align 8, !dbg !1994
  call void @llvm.dbg.declare(metadata ptr %v.dbg.spill, metadata !1995, metadata !DIExpression()), !dbg !2003
; call <str as core::fmt::Display>::fmt
  %_0 = call noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd248aa3f582d4052E"(ptr noalias noundef nonnull readonly align 1 %self3, i64 noundef %len, ptr noalias noundef align 8 dereferenceable(64) %f), !dbg !2005
  ret i1 %_0, !dbg !2006

bb2:                                              ; No predecessors!
  unreachable

bb3:                                              ; No predecessors!
  unreachable
}

; <alloc::alloc::Global as core::alloc::Allocator>::deallocate
; Function Attrs: inlinehint uwtable
define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h37134dd5886182ffE"(ptr noalias noundef nonnull readonly align 1 %self, ptr noundef nonnull %ptr, i64 noundef %0, i64 noundef %1) unnamed_addr #2 !dbg !2007 {
start:
  %self.dbg.spill7 = alloca [8 x i8], align 8
  %self.dbg.spill5 = alloca [8 x i8], align 8
  %self.dbg.spill4 = alloca [8 x i8], align 8
  %ptr.dbg.spill3 = alloca [8 x i8], align 8
  %self.dbg.spill2 = alloca [8 x i8], align 8
  %ptr.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %_13 = alloca [8 x i8], align 8
  %layout1 = alloca [16 x i8], align 8
  %layout = alloca [16 x i8], align 8
  store i64 %0, ptr %layout, align 8
  %2 = getelementptr inbounds i8, ptr %layout, i64 8
  store i64 %1, ptr %2, align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !2012, metadata !DIExpression()), !dbg !2015
  store ptr %ptr, ptr %ptr.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !2013, metadata !DIExpression()), !dbg !2016
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !2017, metadata !DIExpression()), !dbg !2021
  call void @llvm.dbg.declare(metadata ptr %layout, metadata !2014, metadata !DIExpression()), !dbg !2023
  call void @llvm.dbg.declare(metadata ptr %layout1, metadata !2024, metadata !DIExpression()), !dbg !2031
  store ptr %layout, ptr %self.dbg.spill2, align 8, !dbg !2033
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill2, metadata !2034, metadata !DIExpression()), !dbg !2040
  %3 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !2042
  %_4 = load i64, ptr %3, align 8, !dbg !2042, !noundef !23
  %4 = icmp eq i64 %_4, 0, !dbg !2033
  br i1 %4, label %bb2, label %bb1, !dbg !2033

bb2:                                              ; preds = %bb1, %start
  ret void, !dbg !2043

bb1:                                              ; preds = %start
  store ptr %ptr, ptr %ptr.dbg.spill3, align 8, !dbg !2044
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill3, metadata !2030, metadata !DIExpression()), !dbg !2045
  call void @llvm.lifetime.start.p0(i64 16, ptr %layout1), !dbg !2046
  %5 = load i64, ptr %layout, align 8, !dbg !2046, !range !652, !noundef !23
  %6 = getelementptr inbounds i8, ptr %layout, i64 8, !dbg !2046
  %7 = load i64, ptr %6, align 8, !dbg !2046, !noundef !23
  store i64 %5, ptr %layout1, align 8, !dbg !2046
  %8 = getelementptr inbounds i8, ptr %layout1, i64 8, !dbg !2046
  store i64 %7, ptr %8, align 8, !dbg !2046
  store ptr %layout1, ptr %self.dbg.spill4, align 8, !dbg !2047
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill4, metadata !2038, metadata !DIExpression()), !dbg !2048
  store ptr %layout1, ptr %self.dbg.spill5, align 8, !dbg !2050
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill5, metadata !2051, metadata !DIExpression()), !dbg !2055
  %self6 = load i64, ptr %layout, align 8, !dbg !2057, !range !652, !noundef !23
  store i64 %self6, ptr %self.dbg.spill7, align 8, !dbg !2057
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill7, metadata !2058, metadata !DIExpression()), !dbg !2062
  call void @llvm.lifetime.start.p0(i64 8, ptr %_13), !dbg !2064
  store i64 %self6, ptr %_13, align 8, !dbg !2064
  %_14 = load i64, ptr %_13, align 8, !dbg !2064, !range !652, !noundef !23
  %_15 = icmp uge i64 %_14, 1, !dbg !2064
  %_16 = icmp ule i64 %_14, -9223372036854775808, !dbg !2064
  %_17 = and i1 %_15, %_16, !dbg !2064
  call void @llvm.assume(i1 %_17), !dbg !2064
  call void @llvm.lifetime.end.p0(i64 8, ptr %_13), !dbg !2065
  call void @__rust_dealloc(ptr noundef %ptr, i64 noundef %_4, i64 noundef %_14) #18, !dbg !2066
  call void @llvm.lifetime.end.p0(i64 16, ptr %layout1), !dbg !2067
  br label %bb2, !dbg !2068
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate_zeroed
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h51d97e6b86c1750aE"(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %layout.0, i64 noundef %layout.1) unnamed_addr #2 !dbg !2069 {
start:
  %layout.dbg.spill = alloca [16 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !2073, metadata !DIExpression()), !dbg !2075
  store i64 %layout.0, ptr %layout.dbg.spill, align 8
  %0 = getelementptr inbounds i8, ptr %layout.dbg.spill, i64 8
  store i64 %layout.1, ptr %0, align 8
  call void @llvm.dbg.declare(metadata ptr %layout.dbg.spill, metadata !2074, metadata !DIExpression()), !dbg !2076
; call alloc::alloc::Global::alloc_impl
  %1 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %layout.0, i64 noundef %layout.1, i1 noundef zeroext true), !dbg !2077
  %_0.0 = extractvalue { ptr, i64 } %1, 0, !dbg !2077
  %_0.1 = extractvalue { ptr, i64 } %1, 1, !dbg !2077
  %2 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0, !dbg !2078
  %3 = insertvalue { ptr, i64 } %2, i64 %_0.1, 1, !dbg !2078
  ret { ptr, i64 } %3, !dbg !2078
}

; <alloc::alloc::Global as core::alloc::Allocator>::allocate
; Function Attrs: inlinehint uwtable
define internal { ptr, i64 } @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h6ca1b7a3462ca17fE"(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %layout.0, i64 noundef %layout.1) unnamed_addr #2 !dbg !2079 {
start:
  %layout.dbg.spill = alloca [16 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !2081, metadata !DIExpression()), !dbg !2083
  store i64 %layout.0, ptr %layout.dbg.spill, align 8
  %0 = getelementptr inbounds i8, ptr %layout.dbg.spill, i64 8
  store i64 %layout.1, ptr %0, align 8
  call void @llvm.dbg.declare(metadata ptr %layout.dbg.spill, metadata !2082, metadata !DIExpression()), !dbg !2084
; call alloc::alloc::Global::alloc_impl
  %1 = call { ptr, i64 } @_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E(ptr noalias noundef nonnull readonly align 1 %self, i64 noundef %layout.0, i64 noundef %layout.1, i1 noundef zeroext false), !dbg !2085
  %_0.0 = extractvalue { ptr, i64 } %1, 0, !dbg !2085
  %_0.1 = extractvalue { ptr, i64 } %1, 1, !dbg !2085
  %2 = insertvalue { ptr, i64 } poison, ptr %_0.0, 0, !dbg !2086
  %3 = insertvalue { ptr, i64 } %2, i64 %_0.1, 1, !dbg !2086
  ret { ptr, i64 } %3, !dbg !2086
}

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h29810ea43683eadfE"(ptr noalias noundef align 8 dereferenceable(24) %self) unnamed_addr #1 !dbg !2087 {
start:
  %len.dbg.spill = alloca [8 x i8], align 8
  %data.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill3 = alloca [8 x i8], align 8
  %self.dbg.spill1 = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !2092, metadata !DIExpression()), !dbg !2093
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !2094, metadata !DIExpression()), !dbg !2098
  store ptr %self, ptr %self.dbg.spill1, align 8, !dbg !2100
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !2101, metadata !DIExpression()), !dbg !2105
  %0 = getelementptr inbounds i8, ptr %self, i64 8, !dbg !2107
  %self2 = load ptr, ptr %0, align 8, !dbg !2107, !nonnull !23, !noundef !23
  store ptr %self2, ptr %self.dbg.spill3, align 8, !dbg !2107
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill3, metadata !2108, metadata !DIExpression()), !dbg !2112
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill3, metadata !2114, metadata !DIExpression()), !dbg !2118
  store ptr %self2, ptr %data.dbg.spill, align 8, !dbg !2120
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill, metadata !2121, metadata !DIExpression()), !dbg !2126
  call void @llvm.dbg.declare(metadata ptr %data.dbg.spill, metadata !2128, metadata !DIExpression()), !dbg !2133
  %1 = getelementptr inbounds i8, ptr %self, i64 16, !dbg !2135
  %len = load i64, ptr %1, align 8, !dbg !2135, !noundef !23
  store i64 %len, ptr %len.dbg.spill, align 8, !dbg !2135
  call void @llvm.dbg.declare(metadata ptr %len.dbg.spill, metadata !2125, metadata !DIExpression()), !dbg !2136
  call void @llvm.dbg.declare(metadata ptr %len.dbg.spill, metadata !2132, metadata !DIExpression()), !dbg !2137
  ret void, !dbg !2138
}

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define internal void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h78f429c21ee2e9e9E"(ptr noalias noundef align 8 dereferenceable(16) %self) unnamed_addr #1 !dbg !2139 {
start:
  %layout.dbg.spill = alloca [16 x i8], align 8
  %ptr.dbg.spill = alloca [8 x i8], align 8
  %self.dbg.spill = alloca [8 x i8], align 8
  %_2 = alloca [24 x i8], align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !2145, metadata !DIExpression()), !dbg !2149
  call void @llvm.lifetime.start.p0(i64 24, ptr %_2), !dbg !2150
; call alloc::raw_vec::RawVec<T,A>::current_memory
  call void @"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h18f91f87e43f8decE"(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %_2, ptr noalias noundef readonly align 8 dereferenceable(16) %self), !dbg !2150
  %0 = getelementptr inbounds i8, ptr %_2, i64 8, !dbg !2151
  %1 = load i64, ptr %0, align 8, !dbg !2151, !range !661, !noundef !23
  %2 = icmp eq i64 %1, 0, !dbg !2151
  %_4 = select i1 %2, i64 0, i64 1, !dbg !2151
  switch i64 %_4, label %bb5 [
    i64 1, label %bb2
    i64 0, label %bb4
  ], !dbg !2151

bb5:                                              ; preds = %start
  unreachable, !dbg !2152

bb2:                                              ; preds = %start
  %ptr = load ptr, ptr %_2, align 8, !dbg !2153, !nonnull !23, !noundef !23
  store ptr %ptr, ptr %ptr.dbg.spill, align 8, !dbg !2153
  call void @llvm.dbg.declare(metadata ptr %ptr.dbg.spill, metadata !2146, metadata !DIExpression()), !dbg !2153
  %3 = getelementptr inbounds i8, ptr %_2, i64 8, !dbg !2154
  %layout.0 = load i64, ptr %3, align 8, !dbg !2154, !range !652, !noundef !23
  %4 = getelementptr inbounds i8, ptr %3, i64 8, !dbg !2154
  %layout.1 = load i64, ptr %4, align 8, !dbg !2154, !noundef !23
  store i64 %layout.0, ptr %layout.dbg.spill, align 8, !dbg !2154
  %5 = getelementptr inbounds i8, ptr %layout.dbg.spill, i64 8, !dbg !2154
  store i64 %layout.1, ptr %5, align 8, !dbg !2154
  call void @llvm.dbg.declare(metadata ptr %layout.dbg.spill, metadata !2148, metadata !DIExpression()), !dbg !2154
  %_7 = getelementptr inbounds i8, ptr %self, i64 16, !dbg !2155
; call <alloc::alloc::Global as core::alloc::Allocator>::deallocate
  call void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h37134dd5886182ffE"(ptr noalias noundef nonnull readonly align 1 %_7, ptr noundef nonnull %ptr, i64 noundef %layout.0, i64 noundef %layout.1), !dbg !2155
  br label %bb4, !dbg !2156

bb4:                                              ; preds = %bb2, %start
  call void @llvm.lifetime.end.p0(i64 24, ptr %_2), !dbg !2157
  ret void, !dbg !2158
}

; llvm_practice::main
; Function Attrs: uwtable
define internal void @_ZN13llvm_practice4main17h1c5c652c4b5d6518E() unnamed_addr #1 personality ptr @rust_eh_personality !dbg !2159 {
start:
  %0 = alloca [16 x i8], align 8
  %_16 = alloca [16 x i8], align 8
  %_15 = alloca [16 x i8], align 8
  %_13 = alloca [48 x i8], align 8
  %_10 = alloca [16 x i8], align 8
  %_8 = alloca [16 x i8], align 8
  %_7 = alloca [32 x i8], align 8
  %_5 = alloca [48 x i8], align 8
  %res = alloca [24 x i8], align 8
  %new = alloca [24 x i8], align 8
  %_2 = alloca [16 x i8], align 8
  %_1 = alloca [16 x i8], align 8
  %world.dbg.spill = alloca [16 x i8], align 8
  %hello.dbg.spill = alloca [16 x i8], align 8
  store ptr @alloc_3edef0b68cfa9c8c95e6d4fe1a68842b, ptr %hello.dbg.spill, align 8, !dbg !2172
  %1 = getelementptr inbounds i8, ptr %hello.dbg.spill, i64 8, !dbg !2172
  store i64 5, ptr %1, align 8, !dbg !2172
  call void @llvm.dbg.declare(metadata ptr %hello.dbg.spill, metadata !2162, metadata !DIExpression()), !dbg !2172
  store ptr @alloc_788cd4e9a7412aa082bf4e248b638dbd, ptr %world.dbg.spill, align 8, !dbg !2173
  %2 = getelementptr inbounds i8, ptr %world.dbg.spill, i64 8, !dbg !2173
  store i64 6, ptr %2, align 8, !dbg !2173
  call void @llvm.dbg.declare(metadata ptr %world.dbg.spill, metadata !2164, metadata !DIExpression()), !dbg !2173
  call void @llvm.dbg.declare(metadata ptr %new, metadata !2166, metadata !DIExpression()), !dbg !2174
  call void @llvm.dbg.declare(metadata ptr %res, metadata !2168, metadata !DIExpression()), !dbg !2175
  call void @llvm.lifetime.start.p0(i64 16, ptr %_1), !dbg !2176
  store ptr @alloc_3edef0b68cfa9c8c95e6d4fe1a68842b, ptr %_1, align 8, !dbg !2177
  %3 = getelementptr inbounds i8, ptr %_1, i64 8, !dbg !2177
  store i64 5, ptr %3, align 8, !dbg !2177
  call void @llvm.lifetime.start.p0(i64 16, ptr %_2), !dbg !2178
  store ptr @alloc_788cd4e9a7412aa082bf4e248b638dbd, ptr %_2, align 8, !dbg !2179
  %4 = getelementptr inbounds i8, ptr %_2, i64 8, !dbg !2179
  store i64 6, ptr %4, align 8, !dbg !2179
  call void @llvm.lifetime.start.p0(i64 24, ptr %new), !dbg !2180
  call void @llvm.lifetime.start.p0(i64 24, ptr %res), !dbg !2181
  call void @llvm.lifetime.start.p0(i64 48, ptr %_5), !dbg !2181
  call void @llvm.lifetime.start.p0(i64 32, ptr %_7), !dbg !2181
  call void @llvm.lifetime.start.p0(i64 16, ptr %_8), !dbg !2181
; call core::fmt::rt::Argument::new_display
  call void @_ZN4core3fmt2rt8Argument11new_display17h419d8407df184383E(ptr noalias nocapture noundef sret([16 x i8]) align 8 dereferenceable(16) %_8, ptr noalias noundef readonly align 8 dereferenceable(16) %_1), !dbg !2181
  call void @llvm.lifetime.start.p0(i64 16, ptr %_10), !dbg !2181
; call core::fmt::rt::Argument::new_display
  call void @_ZN4core3fmt2rt8Argument11new_display17h419d8407df184383E(ptr noalias nocapture noundef sret([16 x i8]) align 8 dereferenceable(16) %_10, ptr noalias noundef readonly align 8 dereferenceable(16) %_2), !dbg !2181
  %5 = getelementptr inbounds [2 x %"core::fmt::rt::Argument<'_>"], ptr %_7, i64 0, i64 0, !dbg !2181
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %5, ptr align 8 %_8, i64 16, i1 false), !dbg !2181
  %6 = getelementptr inbounds [2 x %"core::fmt::rt::Argument<'_>"], ptr %_7, i64 0, i64 1, !dbg !2181
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %6, ptr align 8 %_10, i64 16, i1 false), !dbg !2181
  call void @llvm.lifetime.end.p0(i64 16, ptr %_10), !dbg !2181
  call void @llvm.lifetime.end.p0(i64 16, ptr %_8), !dbg !2181
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117h208dee809765621aE(ptr noalias nocapture noundef sret([48 x i8]) align 8 dereferenceable(48) %_5, ptr noalias noundef readonly align 8 dereferenceable(32) @alloc_4e0023beeca5f8e5d06a41f60e7c1e6e, ptr noalias noundef readonly align 8 dereferenceable(32) %_7), !dbg !2181
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17h58dbea2d6d469c4dE(ptr noalias nocapture noundef sret([24 x i8]) align 8 dereferenceable(24) %res, ptr noalias nocapture noundef align 8 dereferenceable(48) %_5), !dbg !2181
  call void @llvm.lifetime.end.p0(i64 48, ptr %_5), !dbg !2181
  call void @llvm.lifetime.end.p0(i64 32, ptr %_7), !dbg !2181
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %new, ptr align 8 %res, i64 24, i1 false), !dbg !2175
  call void @llvm.lifetime.end.p0(i64 24, ptr %res), !dbg !2181
  call void @llvm.lifetime.start.p0(i64 48, ptr %_13), !dbg !2182
  call void @llvm.lifetime.start.p0(i64 16, ptr %_15), !dbg !2182
  call void @llvm.lifetime.start.p0(i64 16, ptr %_16), !dbg !2182
; invoke core::fmt::rt::Argument::new_display
  invoke void @_ZN4core3fmt2rt8Argument11new_display17hfecb26d178e99591E(ptr noalias nocapture noundef sret([16 x i8]) align 8 dereferenceable(16) %_16, ptr noalias noundef readonly align 8 dereferenceable(24) %new)
          to label %bb5 unwind label %cleanup, !dbg !2182

bb9:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4d7861b68adcd32bE"(ptr noalias noundef align 8 dereferenceable(24) %new) #15
          to label %bb10 unwind label %terminate, !dbg !2183

cleanup:                                          ; preds = %bb6, %bb5, %start
  %7 = landingpad { ptr, i32 }
          cleanup
  %8 = extractvalue { ptr, i32 } %7, 0
  %9 = extractvalue { ptr, i32 } %7, 1
  call void @llvm.lifetime.start.p0(i64 16, ptr %0)
  store ptr %8, ptr %0, align 8
  %10 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %9, ptr %10, align 8
  br label %bb9

bb5:                                              ; preds = %start
  %11 = getelementptr inbounds [1 x %"core::fmt::rt::Argument<'_>"], ptr %_15, i64 0, i64 0, !dbg !2182
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %11, ptr align 8 %_16, i64 16, i1 false), !dbg !2182
  call void @llvm.lifetime.end.p0(i64 16, ptr %_16), !dbg !2182
; invoke core::fmt::Arguments::new_v1
  invoke void @_ZN4core3fmt9Arguments6new_v117hcfdb94a12fe950e6E(ptr noalias nocapture noundef sret([48 x i8]) align 8 dereferenceable(48) %_13, ptr noalias noundef readonly align 8 dereferenceable(32) @alloc_9bce62b4958e9ae9fca1ec2ed4203a0b, ptr noalias noundef readonly align 8 dereferenceable(16) %_15)
          to label %bb6 unwind label %cleanup, !dbg !2182

bb6:                                              ; preds = %bb5
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr noalias nocapture noundef align 8 dereferenceable(48) %_13)
          to label %bb7 unwind label %cleanup, !dbg !2182

bb7:                                              ; preds = %bb6
  call void @llvm.lifetime.end.p0(i64 48, ptr %_13), !dbg !2182
  call void @llvm.lifetime.end.p0(i64 16, ptr %_15), !dbg !2182
; call core::ptr::drop_in_place<alloc::string::String>
  call void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4d7861b68adcd32bE"(ptr noalias noundef align 8 dereferenceable(24) %new), !dbg !2183
  call void @llvm.lifetime.end.p0(i64 24, ptr %new), !dbg !2183
  call void @llvm.lifetime.end.p0(i64 16, ptr %_2), !dbg !2184
  call void @llvm.lifetime.end.p0(i64 16, ptr %_1), !dbg !2185
  ret void, !dbg !2186

terminate:                                        ; preds = %bb9
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
  %13 = extractvalue { ptr, i32 } %12, 0
  %14 = extractvalue { ptr, i32 } %12, 1
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() #16, !dbg !2187
  unreachable, !dbg !2187

bb10:                                             ; preds = %bb9
  %15 = load ptr, ptr %0, align 8, !dbg !2187, !noundef !23
  %16 = getelementptr inbounds i8, ptr %0, i64 8, !dbg !2187
  %17 = load i32, ptr %16, align 8, !dbg !2187, !noundef !23
  call void @llvm.lifetime.end.p0(i64 16, ptr %0), !dbg !2187
  %18 = insertvalue { ptr, i32 } poison, ptr %15, 0, !dbg !2187
  %19 = insertvalue { ptr, i32 } %18, i32 %17, 1, !dbg !2187
  resume { ptr, i32 } %19, !dbg !2187
}

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare void @llvm.dbg.declare(metadata, metadata, metadata) #4

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare noundef i64 @_ZN3std2rt19lang_start_internal17hd3ad173a069f2ae5E(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #1

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17hd248aa3f582d4052E"(ptr noalias noundef nonnull readonly align 1, i64 noundef, ptr noalias noundef align 8 dereferenceable(64)) unnamed_addr #1

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #5

; Function Attrs: uwtable
declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; core::panicking::panic_in_cleanup
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17h1ce87a78bfb0c4c7E() unnamed_addr #6

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #7

; core::panicking::panic_const::panic_const_div_by_zero
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h6a572ca1aae6312bE(ptr noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #8

; alloc::raw_vec::handle_error
; Function Attrs: cold noreturn uwtable
declare void @_ZN5alloc7raw_vec12handle_error17h7f9cd8199ddef69cE(i64 noundef, i64) unnamed_addr #9

; alloc::fmt::format::format_inner
; Function Attrs: uwtable
declare void @_ZN5alloc3fmt6format12format_inner17h457d6f4cc84042e9E(ptr dead_on_unwind noalias nocapture noundef writable sret([24 x i8]) align 8 dereferenceable(24), ptr noalias nocapture noundef align 8 dereferenceable(48)) unnamed_addr #1

; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @__rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #10

; Function Attrs: nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias noundef ptr @__rust_alloc_zeroed(i64 noundef, i64 allocalign noundef) unnamed_addr #11

; Function Attrs: nounwind allockind("free") uwtable
declare void @__rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #12

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17h27a0dd1f46c92d75E(ptr noalias nocapture noundef align 8 dereferenceable(48)) unnamed_addr #1

define i32 @main(i32 %0, ptr %1) unnamed_addr #13 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17he351fb1c915132d9E(ptr @_ZN13llvm_practice4main17h1c5c652c4b5d6518E, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #14

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #14

attributes #0 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { alwaysinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #5 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { cold noinline noreturn nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #8 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { cold noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #14 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #15 = { cold }
attributes #16 = { cold noreturn nounwind }
attributes #17 = { noreturn }
attributes #18 = { nounwind }

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
!29 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !30, producer: "clang LLVM (rustc version 1.80.0 (051478957 2024-07-21))", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !31, globals: !117, splitDebugInlining: false, nameTableKind: None)
!30 = !DIFile(filename: "src/main.rs/@/6hy2nhkxhdygpscepeauwqgt8", directory: "/Users/tobiasviskum/Programming/Rust/viskum-language/llvm-practice")
!31 = !{!32, !42, !111}
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
!42 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "AlignmentEnum", scope: !43, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagEnumClass, elements: !46)
!43 = !DINamespace(name: "alignment", scope: !44)
!44 = !DINamespace(name: "ptr", scope: !35)
!45 = !DIBasicType(name: "u64", size: 64, encoding: DW_ATE_unsigned)
!46 = !{!47, !48, !49, !50, !51, !52, !53, !54, !55, !56, !57, !58, !59, !60, !61, !62, !63, !64, !65, !66, !67, !68, !69, !70, !71, !72, !73, !74, !75, !76, !77, !78, !79, !80, !81, !82, !83, !84, !85, !86, !87, !88, !89, !90, !91, !92, !93, !94, !95, !96, !97, !98, !99, !100, !101, !102, !103, !104, !105, !106, !107, !108, !109, !110}
!47 = !DIEnumerator(name: "_Align1Shl0", value: 1, isUnsigned: true)
!48 = !DIEnumerator(name: "_Align1Shl1", value: 2, isUnsigned: true)
!49 = !DIEnumerator(name: "_Align1Shl2", value: 4, isUnsigned: true)
!50 = !DIEnumerator(name: "_Align1Shl3", value: 8, isUnsigned: true)
!51 = !DIEnumerator(name: "_Align1Shl4", value: 16, isUnsigned: true)
!52 = !DIEnumerator(name: "_Align1Shl5", value: 32, isUnsigned: true)
!53 = !DIEnumerator(name: "_Align1Shl6", value: 64, isUnsigned: true)
!54 = !DIEnumerator(name: "_Align1Shl7", value: 128, isUnsigned: true)
!55 = !DIEnumerator(name: "_Align1Shl8", value: 256, isUnsigned: true)
!56 = !DIEnumerator(name: "_Align1Shl9", value: 512, isUnsigned: true)
!57 = !DIEnumerator(name: "_Align1Shl10", value: 1024, isUnsigned: true)
!58 = !DIEnumerator(name: "_Align1Shl11", value: 2048, isUnsigned: true)
!59 = !DIEnumerator(name: "_Align1Shl12", value: 4096, isUnsigned: true)
!60 = !DIEnumerator(name: "_Align1Shl13", value: 8192, isUnsigned: true)
!61 = !DIEnumerator(name: "_Align1Shl14", value: 16384, isUnsigned: true)
!62 = !DIEnumerator(name: "_Align1Shl15", value: 32768, isUnsigned: true)
!63 = !DIEnumerator(name: "_Align1Shl16", value: 65536, isUnsigned: true)
!64 = !DIEnumerator(name: "_Align1Shl17", value: 131072, isUnsigned: true)
!65 = !DIEnumerator(name: "_Align1Shl18", value: 262144, isUnsigned: true)
!66 = !DIEnumerator(name: "_Align1Shl19", value: 524288, isUnsigned: true)
!67 = !DIEnumerator(name: "_Align1Shl20", value: 1048576, isUnsigned: true)
!68 = !DIEnumerator(name: "_Align1Shl21", value: 2097152, isUnsigned: true)
!69 = !DIEnumerator(name: "_Align1Shl22", value: 4194304, isUnsigned: true)
!70 = !DIEnumerator(name: "_Align1Shl23", value: 8388608, isUnsigned: true)
!71 = !DIEnumerator(name: "_Align1Shl24", value: 16777216, isUnsigned: true)
!72 = !DIEnumerator(name: "_Align1Shl25", value: 33554432, isUnsigned: true)
!73 = !DIEnumerator(name: "_Align1Shl26", value: 67108864, isUnsigned: true)
!74 = !DIEnumerator(name: "_Align1Shl27", value: 134217728, isUnsigned: true)
!75 = !DIEnumerator(name: "_Align1Shl28", value: 268435456, isUnsigned: true)
!76 = !DIEnumerator(name: "_Align1Shl29", value: 536870912, isUnsigned: true)
!77 = !DIEnumerator(name: "_Align1Shl30", value: 1073741824, isUnsigned: true)
!78 = !DIEnumerator(name: "_Align1Shl31", value: 2147483648, isUnsigned: true)
!79 = !DIEnumerator(name: "_Align1Shl32", value: 4294967296, isUnsigned: true)
!80 = !DIEnumerator(name: "_Align1Shl33", value: 8589934592, isUnsigned: true)
!81 = !DIEnumerator(name: "_Align1Shl34", value: 17179869184, isUnsigned: true)
!82 = !DIEnumerator(name: "_Align1Shl35", value: 34359738368, isUnsigned: true)
!83 = !DIEnumerator(name: "_Align1Shl36", value: 68719476736, isUnsigned: true)
!84 = !DIEnumerator(name: "_Align1Shl37", value: 137438953472, isUnsigned: true)
!85 = !DIEnumerator(name: "_Align1Shl38", value: 274877906944, isUnsigned: true)
!86 = !DIEnumerator(name: "_Align1Shl39", value: 549755813888, isUnsigned: true)
!87 = !DIEnumerator(name: "_Align1Shl40", value: 1099511627776, isUnsigned: true)
!88 = !DIEnumerator(name: "_Align1Shl41", value: 2199023255552, isUnsigned: true)
!89 = !DIEnumerator(name: "_Align1Shl42", value: 4398046511104, isUnsigned: true)
!90 = !DIEnumerator(name: "_Align1Shl43", value: 8796093022208, isUnsigned: true)
!91 = !DIEnumerator(name: "_Align1Shl44", value: 17592186044416, isUnsigned: true)
!92 = !DIEnumerator(name: "_Align1Shl45", value: 35184372088832, isUnsigned: true)
!93 = !DIEnumerator(name: "_Align1Shl46", value: 70368744177664, isUnsigned: true)
!94 = !DIEnumerator(name: "_Align1Shl47", value: 140737488355328, isUnsigned: true)
!95 = !DIEnumerator(name: "_Align1Shl48", value: 281474976710656, isUnsigned: true)
!96 = !DIEnumerator(name: "_Align1Shl49", value: 562949953421312, isUnsigned: true)
!97 = !DIEnumerator(name: "_Align1Shl50", value: 1125899906842624, isUnsigned: true)
!98 = !DIEnumerator(name: "_Align1Shl51", value: 2251799813685248, isUnsigned: true)
!99 = !DIEnumerator(name: "_Align1Shl52", value: 4503599627370496, isUnsigned: true)
!100 = !DIEnumerator(name: "_Align1Shl53", value: 9007199254740992, isUnsigned: true)
!101 = !DIEnumerator(name: "_Align1Shl54", value: 18014398509481984, isUnsigned: true)
!102 = !DIEnumerator(name: "_Align1Shl55", value: 36028797018963968, isUnsigned: true)
!103 = !DIEnumerator(name: "_Align1Shl56", value: 72057594037927936, isUnsigned: true)
!104 = !DIEnumerator(name: "_Align1Shl57", value: 144115188075855872, isUnsigned: true)
!105 = !DIEnumerator(name: "_Align1Shl58", value: 288230376151711744, isUnsigned: true)
!106 = !DIEnumerator(name: "_Align1Shl59", value: 576460752303423488, isUnsigned: true)
!107 = !DIEnumerator(name: "_Align1Shl60", value: 1152921504606846976, isUnsigned: true)
!108 = !DIEnumerator(name: "_Align1Shl61", value: 2305843009213693952, isUnsigned: true)
!109 = !DIEnumerator(name: "_Align1Shl62", value: 4611686018427387904, isUnsigned: true)
!110 = !DIEnumerator(name: "_Align1Shl63", value: 9223372036854775808, isUnsigned: true)
!111 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "AllocInit", scope: !112, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagEnumClass, elements: !114)
!112 = !DINamespace(name: "raw_vec", scope: !113)
!113 = !DINamespace(name: "alloc", scope: null)
!114 = !{!115, !116}
!115 = !DIEnumerator(name: "Uninitialized", value: 0, isUnsigned: true)
!116 = !DIEnumerator(name: "Zeroed", value: 1, isUnsigned: true)
!117 = !{!0}
!118 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hbb678c5755af6f60E", scope: !120, file: !119, line: 151, type: !122, scopeLine: 151, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !128, retainedNodes: !124)
!119 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/sys_common/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "9a938a0945aa66d12453850743d3bf49")
!120 = !DINamespace(name: "backtrace", scope: !121)
!121 = !DINamespace(name: "sys_common", scope: !17)
!122 = !DISubroutineType(types: !123)
!123 = !{null, !20}
!124 = !{!125, !126}
!125 = !DILocalVariable(name: "f", arg: 1, scope: !118, file: !119, line: 151, type: !20)
!126 = !DILocalVariable(name: "result", scope: !127, file: !119, line: 155, type: !7, align: 1)
!127 = distinct !DILexicalBlock(scope: !118, file: !119, line: 155, column: 5)
!128 = !{!129, !130}
!129 = !DITemplateTypeParameter(name: "F", type: !20)
!130 = !DITemplateTypeParameter(name: "T", type: !7)
!131 = !DILocalVariable(name: "dummy", scope: !132, file: !133, line: 337, type: !7, align: 1)
!132 = distinct !DILexicalBlock(scope: !134, file: !133, line: 337, column: 1)
!133 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/hint.rs", directory: "", checksumkind: CSK_MD5, checksum: "36624a7f44e0e372094a9874489ad080")
!134 = distinct !DISubprogram(name: "black_box<()>", linkageName: "_ZN4core4hint9black_box17hc6747fa09023e917E", scope: !135, file: !133, line: 337, type: !136, scopeLine: 337, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !139, retainedNodes: !138)
!135 = !DINamespace(name: "hint", scope: !35)
!136 = !DISubroutineType(types: !137)
!137 = !{null, !7}
!138 = !{!131}
!139 = !{!130}
!140 = !DILocation(line: 337, column: 27, scope: !132, inlinedAt: !141)
!141 = !DILocation(line: 158, column: 5, scope: !127)
!142 = !DILocation(line: 155, column: 9, scope: !127)
!143 = !DILocation(line: 151, column: 43, scope: !118)
!144 = !DILocation(line: 155, column: 18, scope: !118)
!145 = !DILocation(line: 338, column: 5, scope: !132, inlinedAt: !141)
!146 = !{i32 2975779}
!147 = !DILocation(line: 161, column: 2, scope: !118)
!148 = distinct !DISubprogram(name: "lang_start<()>", linkageName: "_ZN3std2rt10lang_start17he351fb1c915132d9E", scope: !16, file: !149, line: 152, type: !150, scopeLine: 152, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !139, retainedNodes: !155)
!149 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "d023918fb5f452acdbb300902bf5fc59")
!150 = !DISubroutineType(types: !151)
!151 = !{!152, !20, !152, !153, !36}
!152 = !DIBasicType(name: "isize", size: 64, encoding: DW_ATE_signed)
!153 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const *const u8", baseType: !154, size: 64, align: 64, dwarfAddressSpace: 0)
!154 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !36, size: 64, align: 64, dwarfAddressSpace: 0)
!155 = !{!156, !157, !158, !159, !160}
!156 = !DILocalVariable(name: "main", arg: 1, scope: !148, file: !149, line: 153, type: !20)
!157 = !DILocalVariable(name: "argc", arg: 2, scope: !148, file: !149, line: 154, type: !152)
!158 = !DILocalVariable(name: "argv", arg: 3, scope: !148, file: !149, line: 155, type: !153)
!159 = !DILocalVariable(name: "sigpipe", arg: 4, scope: !148, file: !149, line: 156, type: !36)
!160 = !DILocalVariable(name: "v", scope: !161, file: !149, line: 158, type: !152, align: 8)
!161 = distinct !DILexicalBlock(scope: !148, file: !149, line: 158, column: 5)
!162 = !DILocation(line: 153, column: 5, scope: !148)
!163 = !DILocation(line: 154, column: 5, scope: !148)
!164 = !DILocation(line: 155, column: 5, scope: !148)
!165 = !DILocation(line: 156, column: 5, scope: !148)
!166 = !DILocation(line: 158, column: 17, scope: !148)
!167 = !DILocation(line: 159, column: 10, scope: !148)
!168 = !DILocation(line: 158, column: 12, scope: !148)
!169 = !DILocation(line: 158, column: 12, scope: !161)
!170 = !DILocation(line: 163, column: 6, scope: !148)
!171 = !DILocation(line: 165, column: 2, scope: !148)
!172 = distinct !DISubprogram(name: "{closure#0}<()>", linkageName: "_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h14c250ebdcfcdfffE", scope: !15, file: !149, line: 159, type: !173, scopeLine: 159, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !139, retainedNodes: !177)
!173 = !DISubroutineType(types: !174)
!174 = !{!175, !176}
!175 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!176 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!177 = !{!178}
!178 = !DILocalVariable(name: "main", scope: !172, file: !149, line: 153, type: !20, align: 8)
!179 = !DILocation(line: 153, column: 5, scope: !172)
!180 = !DILocalVariable(name: "self", arg: 1, scope: !181, file: !182, line: 2048, type: !184)
!181 = distinct !DILexicalBlock(scope: !183, file: !182, line: 2048, column: 5)
!182 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/process.rs", directory: "", checksumkind: CSK_MD5, checksum: "9e51e22eb3333ae012d05fdfbdaeaf7a")
!183 = distinct !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217ha22b56a3d3c7c6bbE", scope: !184, file: !182, line: 2048, type: !196, scopeLine: 2048, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !198, retainedNodes: !199)
!184 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !185, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !186, templateParams: !23, identifier: "26ed9badc2529fac4b11f00e1f76ebd2")
!185 = !DINamespace(name: "process", scope: !17)
!186 = !{!187}
!187 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !184, file: !2, baseType: !188, size: 8, align: 8, flags: DIFlagPrivate)
!188 = !DICompositeType(tag: DW_TAG_structure_type, name: "ExitCode", scope: !189, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !194, templateParams: !23, identifier: "e8a72f2f2ac5c333219035ecc5a8a7e1")
!189 = !DINamespace(name: "process_common", scope: !190)
!190 = !DINamespace(name: "process", scope: !191)
!191 = !DINamespace(name: "unix", scope: !192)
!192 = !DINamespace(name: "pal", scope: !193)
!193 = !DINamespace(name: "sys", scope: !17)
!194 = !{!195}
!195 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !188, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagPrivate)
!196 = !DISubroutineType(types: !197)
!197 = !{!175, !184}
!198 = !DISubprogram(name: "to_i32", linkageName: "_ZN3std7process8ExitCode6to_i3217ha22b56a3d3c7c6bbE", scope: !184, file: !182, line: 2048, type: !196, scopeLine: 2048, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!199 = !{!180}
!200 = !DILocation(line: 2048, column: 19, scope: !181, inlinedAt: !201)
!201 = !DILocation(line: 159, column: 92, scope: !172)
!202 = !DILocation(line: 159, column: 18, scope: !172)
!203 = !DILocation(line: 159, column: 77, scope: !172)
!204 = !DILocation(line: 2049, column: 9, scope: !181, inlinedAt: !201)
!205 = !DILocalVariable(name: "self", arg: 1, scope: !206, file: !207, line: 638, type: !211)
!206 = distinct !DILexicalBlock(scope: !208, file: !207, line: 638, column: 5)
!207 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/std/src/sys/pal/unix/process/process_common.rs", directory: "", checksumkind: CSK_MD5, checksum: "f12d6cc5fbe6e47291b02b1d467e8da3")
!208 = distinct !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys3pal4unix7process14process_common8ExitCode6as_i3217h8853ec1a996fefafE", scope: !188, file: !207, line: 638, type: !209, scopeLine: 638, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !212, retainedNodes: !213)
!209 = !DISubroutineType(types: !210)
!210 = !{!175, !211}
!211 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&std::sys::pal::unix::process::process_common::ExitCode", baseType: !188, size: 64, align: 64, dwarfAddressSpace: 0)
!212 = !DISubprogram(name: "as_i32", linkageName: "_ZN3std3sys3pal4unix7process14process_common8ExitCode6as_i3217h8853ec1a996fefafE", scope: !188, file: !207, line: 638, type: !209, scopeLine: 638, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!213 = !{!205}
!214 = !DILocation(line: 638, column: 19, scope: !206, inlinedAt: !215)
!215 = !DILocation(line: 2049, column: 16, scope: !181, inlinedAt: !201)
!216 = !DILocation(line: 639, column: 9, scope: !206, inlinedAt: !215)
!217 = !DILocation(line: 159, column: 99, scope: !172)
!218 = !DILocation(line: 159, column: 100, scope: !172)
!219 = distinct !DISubprogram(name: "fmt<str>", linkageName: "_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h34d24e463382a7ceE", scope: !221, file: !220, line: 2354, type: !222, scopeLine: 2354, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !282)
!220 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "d0edb1b569d2fa74945fd472d62c28dc")
!221 = !DINamespace(name: "{impl#53}", scope: !34)
!222 = !DISubroutineType(types: !223)
!223 = !{!224, !241, !247}
!224 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), core::fmt::Error>", scope: !225, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !226, templateParams: !23, identifier: "6c840619c30f4ae36491fb7dced63bc")
!225 = !DINamespace(name: "result", scope: !35)
!226 = !{!227}
!227 = !DICompositeType(tag: DW_TAG_variant_part, scope: !224, file: !2, size: 8, align: 8, elements: !228, templateParams: !23, identifier: "288eb4c9a506330b8b455aa98e665788", discriminator: !240)
!228 = !{!229, !236}
!229 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !227, file: !2, baseType: !230, size: 8, align: 8, extraData: i128 0)
!230 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !224, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !231, templateParams: !233, identifier: "6613f4563358f09920d070f9e159e1d5")
!231 = !{!232}
!232 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !230, file: !2, baseType: !7, align: 8, offset: 8, flags: DIFlagPublic)
!233 = !{!130, !234}
!234 = !DITemplateTypeParameter(name: "E", type: !235)
!235 = !DICompositeType(tag: DW_TAG_structure_type, name: "Error", scope: !34, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "3a48e56d0aef43fb722d6937f8f560dd")
!236 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !227, file: !2, baseType: !237, size: 8, align: 8, extraData: i128 1)
!237 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !224, file: !2, size: 8, align: 8, flags: DIFlagPublic, elements: !238, templateParams: !233, identifier: "cd0715ab4b3caa165e90f0874d4b294b")
!238 = !{!239}
!239 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !237, file: !2, baseType: !235, align: 8, offset: 8, flags: DIFlagPublic)
!240 = !DIDerivedType(tag: DW_TAG_member, scope: !224, file: !2, baseType: !36, size: 8, align: 8, flags: DIFlagArtificial)
!241 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&str", baseType: !242, size: 64, align: 64, dwarfAddressSpace: 0)
!242 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !243, templateParams: !23, identifier: "9277eecd40495f85161460476aacc992")
!243 = !{!244, !246}
!244 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !242, file: !2, baseType: !245, size: 64, align: 64)
!245 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !36, size: 64, align: 64, dwarfAddressSpace: 0)
!246 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !242, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!247 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !248, size: 64, align: 64, dwarfAddressSpace: 0)
!248 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !34, file: !2, size: 512, align: 64, flags: DIFlagPublic, elements: !249, templateParams: !23, identifier: "e64159529f90e6c93501e1c8b44635e1")
!249 = !{!250, !252, !254, !255, !270, !271}
!250 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !248, file: !2, baseType: !251, size: 32, align: 32, offset: 416, flags: DIFlagPrivate)
!251 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!252 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !248, file: !2, baseType: !253, size: 32, align: 32, offset: 384, flags: DIFlagPrivate)
!253 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_UTF)
!254 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !248, file: !2, baseType: !32, size: 8, align: 8, offset: 448, flags: DIFlagPrivate)
!255 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !248, file: !2, baseType: !256, size: 128, align: 64, flags: DIFlagPrivate)
!256 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !257, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !258, templateParams: !23, identifier: "562a25f644cfbb3ec8203ae3d455f2e")
!257 = !DINamespace(name: "option", scope: !35)
!258 = !{!259}
!259 = !DICompositeType(tag: DW_TAG_variant_part, scope: !256, file: !2, size: 128, align: 64, elements: !260, templateParams: !23, identifier: "8ed3b56007451f33e768ffbb6948a6c4", discriminator: !269)
!260 = !{!261, !265}
!261 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !259, file: !2, baseType: !262, size: 128, align: 64, extraData: i128 0)
!262 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !256, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !263, identifier: "3040cd18654cf144208f719c3a1aa714")
!263 = !{!264}
!264 = !DITemplateTypeParameter(name: "T", type: !9)
!265 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !259, file: !2, baseType: !266, size: 128, align: 64, extraData: i128 1)
!266 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !256, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !267, templateParams: !263, identifier: "485854eed0ea9812f1a7f394ec88ec1d")
!267 = !{!268}
!268 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !266, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!269 = !DIDerivedType(tag: DW_TAG_member, scope: !256, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!270 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !248, file: !2, baseType: !256, size: 128, align: 64, offset: 128, flags: DIFlagPrivate)
!271 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !248, file: !2, baseType: !272, size: 128, align: 64, offset: 256, flags: DIFlagPrivate)
!272 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut dyn core::fmt::Write", file: !2, size: 128, align: 64, elements: !273, templateParams: !23, identifier: "7314e09b4ef07e046a853e7dcce5165f")
!273 = !{!274, !277}
!274 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !272, file: !2, baseType: !275, size: 64, align: 64)
!275 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !276, size: 64, align: 64, dwarfAddressSpace: 0)
!276 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn core::fmt::Write", file: !2, align: 8, elements: !23, identifier: "8d7cf6ee3dd70fbcd6a3056bca62fea8")
!277 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !272, file: !2, baseType: !278, size: 64, align: 64, offset: 64)
!278 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 6]", baseType: !279, size: 64, align: 64, dwarfAddressSpace: 0)
!279 = !DICompositeType(tag: DW_TAG_array_type, baseType: !9, size: 384, align: 64, elements: !280)
!280 = !{!281}
!281 = !DISubrange(count: 6, lowerBound: 0)
!282 = !{!283, !284}
!283 = !DILocalVariable(name: "self", arg: 1, scope: !219, file: !220, line: 2354, type: !241)
!284 = !DILocalVariable(name: "f", arg: 2, scope: !219, file: !220, line: 2354, type: !247)
!285 = !{!286}
!286 = !DITemplateTypeParameter(name: "T", type: !36)
!287 = !DILocation(line: 2354, column: 20, scope: !219)
!288 = !DILocation(line: 2354, column: 27, scope: !219)
!289 = !DILocation(line: 2354, column: 71, scope: !219)
!290 = !{i64 1}
!291 = !DILocation(line: 2354, column: 62, scope: !219)
!292 = !DILocation(line: 2354, column: 84, scope: !219)
!293 = distinct !DISubprogram(name: "new_display<&str>", linkageName: "_ZN4core3fmt2rt8Argument11new_display17h419d8407df184383E", scope: !295, file: !294, line: 112, type: !318, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !321, declaration: !320, retainedNodes: !323)
!294 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/fmt/rt.rs", directory: "", checksumkind: CSK_MD5, checksum: "97b307aeb2cfde091afce8909100596c")
!295 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !33, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !296, templateParams: !23, identifier: "ecd976d121f35b1f1bf62ffde9d03f36")
!296 = !{!297}
!297 = !DIDerivedType(tag: DW_TAG_member, name: "ty", scope: !295, file: !2, baseType: !298, size: 128, align: 64, flags: DIFlagPrivate)
!298 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentType", scope: !33, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !299, templateParams: !23, identifier: "ff3f2e7a1fa7ea1eec8f058c6c7ec6b4")
!299 = !{!300}
!300 = !DICompositeType(tag: DW_TAG_variant_part, scope: !298, file: !2, size: 128, align: 64, elements: !301, templateParams: !23, identifier: "6ca585353d9c5045547be772bdd68196", discriminator: !317)
!301 = !{!302, !313}
!302 = !DIDerivedType(tag: DW_TAG_member, name: "Placeholder", scope: !300, file: !2, baseType: !303, size: 128, align: 64)
!303 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !298, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !304, templateParams: !23, identifier: "21d35b48df9bcc19905f29e285e9d5a1")
!304 = !{!305, !309}
!305 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !303, file: !2, baseType: !306, size: 64, align: 64, flags: DIFlagPrivate)
!306 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::rt::{extern#0}::Opaque", baseType: !307, size: 64, align: 64, dwarfAddressSpace: 0)
!307 = !DICompositeType(tag: DW_TAG_structure_type, name: "Opaque", scope: !308, file: !2, align: 8, elements: !23, identifier: "87190b3403781efddea9ff07bab29af3")
!308 = !DINamespace(name: "{extern#0}", scope: !33)
!309 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !303, file: !2, baseType: !310, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!310 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&core::fmt::rt::{extern#0}::Opaque, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !311, size: 64, align: 64, dwarfAddressSpace: 0)
!311 = !DISubroutineType(types: !312)
!312 = !{!224, !306, !247}
!313 = !DIDerivedType(tag: DW_TAG_member, name: "Count", scope: !300, file: !2, baseType: !314, size: 128, align: 64, extraData: i128 0)
!314 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !298, file: !2, size: 128, align: 64, flags: DIFlagPrivate, elements: !315, templateParams: !23, identifier: "578cfa2846b688ed99eab6949d37fe79")
!315 = !{!316}
!316 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !314, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPrivate)
!317 = !DIDerivedType(tag: DW_TAG_member, scope: !298, file: !2, baseType: !45, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)
!318 = !DISubroutineType(types: !319)
!319 = !{!295, !241}
!320 = !DISubprogram(name: "new_display<&str>", linkageName: "_ZN4core3fmt2rt8Argument11new_display17h419d8407df184383E", scope: !295, file: !294, line: 112, type: !318, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !321)
!321 = !{!322}
!322 = !DITemplateTypeParameter(name: "T", type: !242)
!323 = !{!324}
!324 = !DILocalVariable(name: "x", arg: 1, scope: !293, file: !294, line: 112, type: !241)
!325 = !DILocation(line: 112, column: 40, scope: !293)
!326 = !DILocalVariable(name: "x", arg: 1, scope: !327, file: !294, line: 92, type: !241)
!327 = distinct !DILexicalBlock(scope: !328, file: !294, line: 92, column: 5)
!328 = distinct !DISubprogram(name: "new<&str>", linkageName: "_ZN4core3fmt2rt8Argument3new17ha2aa9659eacd8584E", scope: !295, file: !294, line: 92, type: !329, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !321, declaration: !332, retainedNodes: !333)
!329 = !DISubroutineType(types: !330)
!330 = !{!295, !241, !331}
!331 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&&str, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !222, size: 64, align: 64, dwarfAddressSpace: 0)
!332 = !DISubprogram(name: "new<&str>", linkageName: "_ZN4core3fmt2rt8Argument3new17ha2aa9659eacd8584E", scope: !295, file: !294, line: 92, type: !329, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !321)
!333 = !{!326, !334}
!334 = !DILocalVariable(name: "f", arg: 2, scope: !327, file: !294, line: 92, type: !331)
!335 = !DILocation(line: 92, column: 19, scope: !327, inlinedAt: !336)
!336 = !DILocation(line: 113, column: 9, scope: !293)
!337 = !DILocation(line: 113, column: 22, scope: !293)
!338 = !DILocation(line: 92, column: 29, scope: !327, inlinedAt: !336)
!339 = !DILocation(line: 103, column: 21, scope: !327, inlinedAt: !336)
!340 = !DILocation(line: 102, column: 13, scope: !327, inlinedAt: !336)
!341 = !DILocation(line: 107, column: 13, scope: !327, inlinedAt: !336)
!342 = !DILocation(line: 114, column: 6, scope: !293)
!343 = distinct !DISubprogram(name: "new_display<alloc::string::String>", linkageName: "_ZN4core3fmt2rt8Argument11new_display17hfecb26d178e99591E", scope: !295, file: !294, line: 112, type: !344, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !380, declaration: !379, retainedNodes: !382)
!344 = !DISubroutineType(types: !345)
!345 = !{!295, !346}
!346 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::string::String", baseType: !347, size: 64, align: 64, dwarfAddressSpace: 0)
!347 = !DICompositeType(tag: DW_TAG_structure_type, name: "String", scope: !348, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !349, templateParams: !23, identifier: "c85e023002fe0099cad4dd85960e9be")
!348 = !DINamespace(name: "string", scope: !113)
!349 = !{!350}
!350 = !DIDerivedType(tag: DW_TAG_member, name: "vec", scope: !347, file: !2, baseType: !351, size: 192, align: 64, flags: DIFlagPrivate)
!351 = !DICompositeType(tag: DW_TAG_structure_type, name: "Vec<u8, alloc::alloc::Global>", scope: !352, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !353, templateParams: !376, identifier: "ba0736225007dd5b2bf8e32390c8e8c0")
!352 = !DINamespace(name: "vec", scope: !113)
!353 = !{!354, !378}
!354 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !351, file: !2, baseType: !355, size: 128, align: 64, flags: DIFlagPrivate)
!355 = !DICompositeType(tag: DW_TAG_structure_type, name: "RawVec<u8, alloc::alloc::Global>", scope: !112, file: !2, size: 128, align: 64, flags: DIFlagProtected, elements: !356, templateParams: !376, identifier: "ec372bd4404c67f02818fa92cf01ac4e")
!356 = !{!357, !369, !373}
!357 = !DIDerivedType(tag: DW_TAG_member, name: "ptr", scope: !355, file: !2, baseType: !358, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!358 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unique<u8>", scope: !359, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !360, templateParams: !285, identifier: "d778c8db5eb1f8d9f80930294480b137")
!359 = !DINamespace(name: "unique", scope: !44)
!360 = !{!361, !366}
!361 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !358, file: !2, baseType: !362, size: 64, align: 64, flags: DIFlagPrivate)
!362 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<u8>", scope: !363, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !364, templateParams: !285, identifier: "e76ed63236c3b89729dfafe6ada93374")
!363 = !DINamespace(name: "non_null", scope: !44)
!364 = !{!365}
!365 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !362, file: !2, baseType: !154, size: 64, align: 64, flags: DIFlagPrivate)
!366 = !DIDerivedType(tag: DW_TAG_member, name: "_marker", scope: !358, file: !2, baseType: !367, align: 8, offset: 64, flags: DIFlagPrivate)
!367 = !DICompositeType(tag: DW_TAG_structure_type, name: "PhantomData<u8>", scope: !368, file: !2, align: 8, flags: DIFlagPublic, elements: !23, templateParams: !285, identifier: "4342a95b36d31af995f45cf6dbb57ffc")
!368 = !DINamespace(name: "marker", scope: !35)
!369 = !DIDerivedType(tag: DW_TAG_member, name: "cap", scope: !355, file: !2, baseType: !370, size: 64, align: 64, flags: DIFlagPrivate)
!370 = !DICompositeType(tag: DW_TAG_structure_type, name: "Cap", scope: !112, file: !2, size: 64, align: 64, flags: DIFlagPrivate, elements: !371, templateParams: !23, identifier: "40219fc18b58ecfcd792d09619af4c11")
!371 = !{!372}
!372 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !370, file: !2, baseType: !9, size: 64, align: 64, flags: DIFlagPrivate)
!373 = !DIDerivedType(tag: DW_TAG_member, name: "alloc", scope: !355, file: !2, baseType: !374, align: 8, offset: 128, flags: DIFlagPrivate)
!374 = !DICompositeType(tag: DW_TAG_structure_type, name: "Global", scope: !375, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "7bea9ced9e807157e908035c36ce6f5d")
!375 = !DINamespace(name: "alloc", scope: !113)
!376 = !{!286, !377}
!377 = !DITemplateTypeParameter(name: "A", type: !374)
!378 = !DIDerivedType(tag: DW_TAG_member, name: "len", scope: !351, file: !2, baseType: !9, size: 64, align: 64, offset: 128, flags: DIFlagPrivate)
!379 = !DISubprogram(name: "new_display<alloc::string::String>", linkageName: "_ZN4core3fmt2rt8Argument11new_display17hfecb26d178e99591E", scope: !295, file: !294, line: 112, type: !344, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !380)
!380 = !{!381}
!381 = !DITemplateTypeParameter(name: "T", type: !347)
!382 = !{!383}
!383 = !DILocalVariable(name: "x", arg: 1, scope: !343, file: !294, line: 112, type: !346)
!384 = !DILocation(line: 112, column: 40, scope: !343)
!385 = !DILocalVariable(name: "x", arg: 1, scope: !386, file: !294, line: 92, type: !346)
!386 = distinct !DILexicalBlock(scope: !387, file: !294, line: 92, column: 5)
!387 = distinct !DISubprogram(name: "new<alloc::string::String>", linkageName: "_ZN4core3fmt2rt8Argument3new17h08c721151aeeb680E", scope: !295, file: !294, line: 92, type: !388, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !380, declaration: !393, retainedNodes: !394)
!388 = !DISubroutineType(types: !389)
!389 = !{!295, !346, !390}
!390 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&alloc::string::String, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !391, size: 64, align: 64, dwarfAddressSpace: 0)
!391 = !DISubroutineType(types: !392)
!392 = !{!224, !346, !247}
!393 = !DISubprogram(name: "new<alloc::string::String>", linkageName: "_ZN4core3fmt2rt8Argument3new17h08c721151aeeb680E", scope: !295, file: !294, line: 92, type: !388, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !380)
!394 = !{!385, !395}
!395 = !DILocalVariable(name: "f", arg: 2, scope: !386, file: !294, line: 92, type: !390)
!396 = !DILocation(line: 92, column: 19, scope: !386, inlinedAt: !397)
!397 = !DILocation(line: 113, column: 9, scope: !343)
!398 = !DILocation(line: 113, column: 22, scope: !343)
!399 = !DILocation(line: 92, column: 29, scope: !386, inlinedAt: !397)
!400 = !DILocation(line: 103, column: 21, scope: !386, inlinedAt: !397)
!401 = !DILocation(line: 102, column: 13, scope: !386, inlinedAt: !397)
!402 = !DILocation(line: 107, column: 13, scope: !386, inlinedAt: !397)
!403 = !DILocation(line: 114, column: 6, scope: !343)
!404 = distinct !DISubprogram(name: "new_v1<2, 2>", linkageName: "_ZN4core3fmt9Arguments6new_v117h208dee809765621aE", scope: !405, file: !220, line: 349, type: !461, scopeLine: 349, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !469, retainedNodes: !470)
!405 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !34, file: !2, size: 384, align: 64, flags: DIFlagPublic, elements: !406, templateParams: !23, identifier: "b46d88cc0bf2e6afccf1ddcc24dfbeb3")
!406 = !{!407, !413, !455}
!407 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !405, file: !2, baseType: !408, size: 128, align: 64, flags: DIFlagPrivate)
!408 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 128, align: 64, elements: !409, templateParams: !23, identifier: "4e66b00a376d6af5b8765440fb2839f")
!409 = !{!410, !412}
!410 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !408, file: !2, baseType: !411, size: 64, align: 64)
!411 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !242, size: 64, align: 64, dwarfAddressSpace: 0)
!412 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !408, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!413 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !405, file: !2, baseType: !414, size: 128, align: 64, offset: 256, flags: DIFlagPrivate)
!414 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::Placeholder]>", scope: !257, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !415, templateParams: !23, identifier: "d774420e8e3096a8e0063cd51ebf0c01")
!415 = !{!416}
!416 = !DICompositeType(tag: DW_TAG_variant_part, scope: !414, file: !2, size: 128, align: 64, elements: !417, templateParams: !23, identifier: "42af0665edf854dcb9ea52e84c3cefe8", discriminator: !454)
!417 = !{!418, !450}
!418 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !416, file: !2, baseType: !419, size: 128, align: 64, extraData: i128 0)
!419 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !414, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !420, identifier: "fc691c729a9e3e4d57fa2678a9cd73d")
!420 = !{!421}
!421 = !DITemplateTypeParameter(name: "T", type: !422)
!422 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Placeholder]", file: !2, size: 128, align: 64, elements: !423, templateParams: !23, identifier: "18b2c68086ffbcbd7f221423f493c07f")
!423 = !{!424, !449}
!424 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !422, file: !2, baseType: !425, size: 64, align: 64)
!425 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !426, size: 64, align: 64, dwarfAddressSpace: 0)
!426 = !DICompositeType(tag: DW_TAG_structure_type, name: "Placeholder", scope: !33, file: !2, size: 448, align: 64, flags: DIFlagPublic, elements: !427, templateParams: !23, identifier: "3c94cc8264d3d8b055b5acbf4afe9590")
!427 = !{!428, !429, !430, !431, !432, !448}
!428 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !426, file: !2, baseType: !9, size: 64, align: 64, offset: 256, flags: DIFlagPublic)
!429 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !426, file: !2, baseType: !253, size: 32, align: 32, offset: 320, flags: DIFlagPublic)
!430 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !426, file: !2, baseType: !32, size: 8, align: 8, offset: 384, flags: DIFlagPublic)
!431 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !426, file: !2, baseType: !251, size: 32, align: 32, offset: 352, flags: DIFlagPublic)
!432 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !426, file: !2, baseType: !433, size: 128, align: 64, flags: DIFlagPublic)
!433 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !33, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !434, templateParams: !23, identifier: "7bbb02f6e78a3559b539405c91777483")
!434 = !{!435}
!435 = !DICompositeType(tag: DW_TAG_variant_part, scope: !433, file: !2, size: 128, align: 64, elements: !436, templateParams: !23, identifier: "5d0823612efc111239df02572d876ea", discriminator: !447)
!436 = !{!437, !441, !445}
!437 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !435, file: !2, baseType: !438, size: 128, align: 64, extraData: i128 0)
!438 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !433, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !439, templateParams: !23, identifier: "544eb65eb435c61322c3979318601571")
!439 = !{!440}
!440 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !438, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!441 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !435, file: !2, baseType: !442, size: 128, align: 64, extraData: i128 1)
!442 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !433, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !443, templateParams: !23, identifier: "a926f0d235e4de79f584934dbdbca4fc")
!443 = !{!444}
!444 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !442, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPublic)
!445 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !435, file: !2, baseType: !446, size: 128, align: 64, extraData: i128 2)
!446 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !433, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, identifier: "4f14f535fef5d63a8c4b0fca5a7e7c0")
!447 = !DIDerivedType(tag: DW_TAG_member, scope: !433, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!448 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !426, file: !2, baseType: !433, size: 128, align: 64, offset: 128, flags: DIFlagPublic)
!449 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !422, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!450 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !416, file: !2, baseType: !451, size: 128, align: 64)
!451 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !414, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !452, templateParams: !420, identifier: "989f9335e9b530569cebf2d4c10723d8")
!452 = !{!453}
!453 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !451, file: !2, baseType: !422, size: 128, align: 64, flags: DIFlagPublic)
!454 = !DIDerivedType(tag: DW_TAG_member, scope: !414, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!455 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !405, file: !2, baseType: !456, size: 128, align: 64, offset: 128, flags: DIFlagPrivate)
!456 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::Argument]", file: !2, size: 128, align: 64, elements: !457, templateParams: !23, identifier: "6ec08f8a7448e7974bb363f956a6ce2")
!457 = !{!458, !460}
!458 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !456, file: !2, baseType: !459, size: 64, align: 64)
!459 = !DIDerivedType(tag: DW_TAG_pointer_type, baseType: !295, size: 64, align: 64, dwarfAddressSpace: 0)
!460 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !456, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!461 = !DISubroutineType(types: !462)
!462 = !{!405, !463, !467}
!463 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[&str; 2]", baseType: !464, size: 64, align: 64, dwarfAddressSpace: 0)
!464 = !DICompositeType(tag: DW_TAG_array_type, baseType: !242, size: 256, align: 64, elements: !465)
!465 = !{!466}
!466 = !DISubrange(count: 2, lowerBound: 0)
!467 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[core::fmt::rt::Argument; 2]", baseType: !468, size: 64, align: 64, dwarfAddressSpace: 0)
!468 = !DICompositeType(tag: DW_TAG_array_type, baseType: !295, size: 256, align: 64, elements: !465)
!469 = !DISubprogram(name: "new_v1<2, 2>", linkageName: "_ZN4core3fmt9Arguments6new_v117h208dee809765621aE", scope: !405, file: !220, line: 349, type: !461, scopeLine: 349, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!470 = !{!471, !472}
!471 = !DILocalVariable(name: "pieces", arg: 1, scope: !404, file: !220, line: 350, type: !463)
!472 = !DILocalVariable(name: "args", arg: 2, scope: !404, file: !220, line: 351, type: !467)
!473 = !DILocation(line: 350, column: 9, scope: !404)
!474 = !DILocation(line: 351, column: 9, scope: !404)
!475 = !DILocation(line: 354, column: 9, scope: !404)
!476 = !{i64 8}
!477 = !DILocation(line: 355, column: 6, scope: !404)
!478 = distinct !DISubprogram(name: "new_v1<2, 1>", linkageName: "_ZN4core3fmt9Arguments6new_v117hcfdb94a12fe950e6E", scope: !405, file: !220, line: 349, type: !479, scopeLine: 349, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !485, retainedNodes: !486)
!479 = !DISubroutineType(types: !480)
!480 = !{!405, !463, !481}
!481 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[core::fmt::rt::Argument; 1]", baseType: !482, size: 64, align: 64, dwarfAddressSpace: 0)
!482 = !DICompositeType(tag: DW_TAG_array_type, baseType: !295, size: 128, align: 64, elements: !483)
!483 = !{!484}
!484 = !DISubrange(count: 1, lowerBound: 0)
!485 = !DISubprogram(name: "new_v1<2, 1>", linkageName: "_ZN4core3fmt9Arguments6new_v117hcfdb94a12fe950e6E", scope: !405, file: !220, line: 349, type: !479, scopeLine: 349, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!486 = !{!487, !488}
!487 = !DILocalVariable(name: "pieces", arg: 1, scope: !478, file: !220, line: 350, type: !463)
!488 = !DILocalVariable(name: "args", arg: 2, scope: !478, file: !220, line: 351, type: !481)
!489 = !DILocation(line: 350, column: 9, scope: !478)
!490 = !DILocation(line: 351, column: 9, scope: !478)
!491 = !DILocation(line: 354, column: 9, scope: !478)
!492 = !DILocation(line: 355, column: 6, scope: !478)
!493 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h76b38b0eafd0646bE", scope: !495, file: !494, line: 250, type: !498, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !504, retainedNodes: !501)
!494 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ops/function.rs", directory: "", checksumkind: CSK_MD5, checksum: "abc772494ea8033dad5cae2e40e54b10")
!495 = !DINamespace(name: "FnOnce", scope: !496)
!496 = !DINamespace(name: "function", scope: !497)
!497 = !DINamespace(name: "ops", scope: !35)
!498 = !DISubroutineType(types: !499)
!499 = !{!175, !500}
!500 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut std::rt::lang_start::{closure_env#0}<()>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!501 = !{!502, !503}
!502 = !DILocalVariable(arg: 1, scope: !493, file: !494, line: 250, type: !500)
!503 = !DILocalVariable(arg: 2, scope: !493, file: !494, line: 250, type: !7)
!504 = !{!505, !506}
!505 = !DITemplateTypeParameter(name: "Self", type: !14)
!506 = !DITemplateTypeParameter(name: "Args", type: !7)
!507 = !DILocation(line: 250, column: 5, scope: !493)
!508 = distinct !DISubprogram(name: "call_once<fn(&str) -> alloc::string::String, (&str)>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17h0ff3fcf9ce826b3aE", scope: !495, file: !494, line: 250, type: !509, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !520, retainedNodes: !514)
!509 = !DISubroutineType(types: !510)
!510 = !{!347, !511, !242}
!511 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&str) -> alloc::string::String", baseType: !512, align: 1, dwarfAddressSpace: 0)
!512 = !DISubroutineType(types: !513)
!513 = !{!347, !242}
!514 = !{!515, !516}
!515 = !DILocalVariable(arg: 1, scope: !508, file: !494, line: 250, type: !511)
!516 = !DILocalVariable(arg: 2, scope: !508, file: !494, line: 250, type: !517)
!517 = !DICompositeType(tag: DW_TAG_structure_type, name: "(&str)", file: !2, size: 128, align: 64, elements: !518, templateParams: !23, identifier: "b67f40c07c754af7164d3770ea629c0b")
!518 = !{!519}
!519 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !517, file: !2, baseType: !242, size: 128, align: 64)
!520 = !{!521, !522}
!521 = !DITemplateTypeParameter(name: "Self", type: !511)
!522 = !DITemplateTypeParameter(name: "Args", type: !517)
!523 = !DILocation(line: 250, column: 5, scope: !508)
!524 = distinct !DISubprogram(name: "call_once<std::rt::lang_start::{closure_env#0}<()>, ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17haba073e74399bc57E", scope: !495, file: !494, line: 250, type: !525, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !504, retainedNodes: !527)
!525 = !DISubroutineType(types: !526)
!526 = !{!175, !14}
!527 = !{!528, !529}
!528 = !DILocalVariable(arg: 1, scope: !524, file: !494, line: 250, type: !14)
!529 = !DILocalVariable(arg: 2, scope: !524, file: !494, line: 250, type: !7)
!530 = !DILocation(line: 250, column: 5, scope: !524)
!531 = distinct !DISubprogram(name: "call_once<fn(), ()>", linkageName: "_ZN4core3ops8function6FnOnce9call_once17hd5bf480b695223baE", scope: !495, file: !494, line: 250, type: !122, scopeLine: 250, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !535, retainedNodes: !532)
!532 = !{!533, !534}
!533 = !DILocalVariable(arg: 1, scope: !531, file: !494, line: 250, type: !20)
!534 = !DILocalVariable(arg: 2, scope: !531, file: !494, line: 250, type: !7)
!535 = !{!536, !506}
!536 = !DITemplateTypeParameter(name: "Self", type: !20)
!537 = !DILocation(line: 250, column: 5, scope: !531)
!538 = distinct !DISubprogram(name: "drop_in_place<alloc::string::String>", linkageName: "_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h4d7861b68adcd32bE", scope: !44, file: !539, line: 542, type: !540, scopeLine: 542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !380, retainedNodes: !543)
!539 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "a85d519116fde26821981e2ebfa0ecba")
!540 = !DISubroutineType(types: !541)
!541 = !{null, !542}
!542 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut alloc::string::String", baseType: !347, size: 64, align: 64, dwarfAddressSpace: 0)
!543 = !{!544}
!544 = !DILocalVariable(arg: 1, scope: !538, file: !539, line: 542, type: !542)
!545 = !DILocation(line: 542, column: 1, scope: !538)
!546 = distinct !DISubprogram(name: "drop_in_place<alloc::vec::Vec<u8, alloc::alloc::Global>>", linkageName: "_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h3161afec24d1e10cE", scope: !44, file: !539, line: 542, type: !547, scopeLine: 542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !552, retainedNodes: !550)
!547 = !DISubroutineType(types: !548)
!548 = !{null, !549}
!549 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut alloc::vec::Vec<u8, alloc::alloc::Global>", baseType: !351, size: 64, align: 64, dwarfAddressSpace: 0)
!550 = !{!551}
!551 = !DILocalVariable(arg: 1, scope: !546, file: !539, line: 542, type: !549)
!552 = !{!553}
!553 = !DITemplateTypeParameter(name: "T", type: !351)
!554 = !DILocation(line: 542, column: 1, scope: !546)
!555 = distinct !DISubprogram(name: "drop_in_place<alloc::raw_vec::RawVec<u8, alloc::alloc::Global>>", linkageName: "_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h957fa1f1ac57f0f4E", scope: !44, file: !539, line: 542, type: !556, scopeLine: 542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !561, retainedNodes: !559)
!556 = !DISubroutineType(types: !557)
!557 = !{null, !558}
!558 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut alloc::raw_vec::RawVec<u8, alloc::alloc::Global>", baseType: !355, size: 64, align: 64, dwarfAddressSpace: 0)
!559 = !{!560}
!560 = !DILocalVariable(arg: 1, scope: !555, file: !539, line: 542, type: !558)
!561 = !{!562}
!562 = !DITemplateTypeParameter(name: "T", type: !355)
!563 = !DILocation(line: 542, column: 1, scope: !555)
!564 = distinct !DISubprogram(name: "drop_in_place<std::rt::lang_start::{closure_env#0}<()>>", linkageName: "_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hc293cf27ba73e22fE", scope: !44, file: !539, line: 542, type: !565, scopeLine: 542, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !569, retainedNodes: !567)
!565 = !DISubroutineType(types: !566)
!566 = !{null, !500}
!567 = !{!568}
!568 = !DILocalVariable(arg: 1, scope: !564, file: !539, line: 542, type: !500)
!569 = !{!570}
!570 = !DITemplateTypeParameter(name: "T", type: !14)
!571 = !DILocation(line: 542, column: 1, scope: !564)
!572 = distinct !DISubprogram(name: "inner", linkageName: "_ZN4core5alloc6layout6Layout5array5inner17h1e2e6b6a7c48ef5dE", scope: !574, file: !573, line: 440, type: !578, scopeLine: 440, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !604)
!573 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/alloc/layout.rs", directory: "", checksumkind: CSK_MD5, checksum: "df5528cae3613abf75e076bcff2e62b8")
!574 = !DINamespace(name: "array", scope: !575)
!575 = !DINamespace(name: "{impl#0}", scope: !576)
!576 = !DINamespace(name: "layout", scope: !577)
!577 = !DINamespace(name: "alloc", scope: !35)
!578 = !DISubroutineType(types: !579)
!579 = !{!580, !9, !592, !9}
!580 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::alloc::layout::Layout, core::alloc::layout::LayoutError>", scope: !225, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !581, templateParams: !23, identifier: "a5503b0996807de1dff7eeaaecf1eb26")
!581 = !{!582}
!582 = !DICompositeType(tag: DW_TAG_variant_part, scope: !580, file: !2, size: 128, align: 64, elements: !583, templateParams: !23, identifier: "849ee722c33fadc5548bd00034b11fd8", discriminator: !603)
!583 = !{!584, !599}
!584 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !582, file: !2, baseType: !585, size: 128, align: 64)
!585 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !580, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !586, templateParams: !595, identifier: "2fa47e4880aa2c94bffaf17f6ab34a8")
!586 = !{!587}
!587 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !585, file: !2, baseType: !588, size: 128, align: 64, flags: DIFlagPublic)
!588 = !DICompositeType(tag: DW_TAG_structure_type, name: "Layout", scope: !576, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !589, templateParams: !23, identifier: "d3070cc9600d1ec836ff13fd8560911")
!589 = !{!590, !591}
!590 = !DIDerivedType(tag: DW_TAG_member, name: "size", scope: !588, file: !2, baseType: !9, size: 64, align: 64, offset: 64, flags: DIFlagPrivate)
!591 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !588, file: !2, baseType: !592, size: 64, align: 64, flags: DIFlagPrivate)
!592 = !DICompositeType(tag: DW_TAG_structure_type, name: "Alignment", scope: !43, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !593, templateParams: !23, identifier: "6aae6ba4cb54950ac047b91a4a957721")
!593 = !{!594}
!594 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !592, file: !2, baseType: !42, size: 64, align: 64, flags: DIFlagPrivate)
!595 = !{!596, !597}
!596 = !DITemplateTypeParameter(name: "T", type: !588)
!597 = !DITemplateTypeParameter(name: "E", type: !598)
!598 = !DICompositeType(tag: DW_TAG_structure_type, name: "LayoutError", scope: !576, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "358eb9c6f95b67e19243fb0174d5871b")
!599 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !582, file: !2, baseType: !600, size: 128, align: 64, extraData: i128 0)
!600 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !580, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !601, templateParams: !595, identifier: "504bf457706c904cbf5bc979d182276b")
!601 = !{!602}
!602 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !600, file: !2, baseType: !598, align: 8, flags: DIFlagPublic)
!603 = !DIDerivedType(tag: DW_TAG_member, scope: !580, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!604 = !{!605, !606, !607, !608}
!605 = !DILocalVariable(name: "element_size", arg: 1, scope: !572, file: !573, line: 441, type: !9)
!606 = !DILocalVariable(name: "align", arg: 2, scope: !572, file: !573, line: 442, type: !592)
!607 = !DILocalVariable(name: "n", arg: 3, scope: !572, file: !573, line: 443, type: !9)
!608 = !DILocalVariable(name: "array_size", scope: !609, file: !573, line: 459, type: !9, align: 8)
!609 = distinct !DILexicalBlock(scope: !572, file: !573, line: 459, column: 13)
!610 = !DILocation(line: 441, column: 13, scope: !572)
!611 = !DILocalVariable(name: "self", arg: 1, scope: !612, file: !613, line: 809, type: !9)
!612 = distinct !DILexicalBlock(scope: !614, file: !613, line: 809, column: 9)
!613 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/num/uint_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "e4dc3d289178c3555c7388acbb7ca29b")
!614 = distinct !DISubprogram(name: "unchecked_mul", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$13unchecked_mul17h35d3b0db80b4cb81E", scope: !615, file: !613, line: 809, type: !617, scopeLine: 809, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !619)
!615 = !DINamespace(name: "{impl#11}", scope: !616)
!616 = !DINamespace(name: "num", scope: !35)
!617 = !DISubroutineType(types: !618)
!618 = !{!9, !9, !9}
!619 = !{!611, !620}
!620 = !DILocalVariable(name: "rhs", arg: 2, scope: !612, file: !613, line: 809, type: !9)
!621 = !DILocation(line: 809, column: 43, scope: !612, inlinedAt: !622)
!622 = !DILocation(line: 459, column: 52, scope: !572)
!623 = !DILocation(line: 442, column: 13, scope: !572)
!624 = !DILocalVariable(name: "align", arg: 1, scope: !625, file: !573, line: 80, type: !592)
!625 = distinct !DILexicalBlock(scope: !626, file: !573, line: 80, column: 5)
!626 = distinct !DISubprogram(name: "max_size_for_align", linkageName: "_ZN4core5alloc6layout6Layout18max_size_for_align17hbbdcfeb020a0a92fE", scope: !588, file: !573, line: 80, type: !627, scopeLine: 80, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !629, retainedNodes: !630)
!627 = !DISubroutineType(types: !628)
!628 = !{!9, !592}
!629 = !DISubprogram(name: "max_size_for_align", linkageName: "_ZN4core5alloc6layout6Layout18max_size_for_align17hbbdcfeb020a0a92fE", scope: !588, file: !573, line: 80, type: !627, scopeLine: 80, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!630 = !{!624}
!631 = !DILocation(line: 80, column: 33, scope: !625, inlinedAt: !632)
!632 = !DILocation(line: 451, column: 41, scope: !572)
!633 = !DILocalVariable(name: "self", arg: 1, scope: !634, file: !635, line: 96, type: !592)
!634 = distinct !DILexicalBlock(scope: !636, file: !635, line: 96, column: 5)
!635 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/alignment.rs", directory: "", checksumkind: CSK_MD5, checksum: "7cb3a809f490e73cc9f019990f7d048c")
!636 = distinct !DISubprogram(name: "as_usize", linkageName: "_ZN4core3ptr9alignment9Alignment8as_usize17h0458ba04f1c34212E", scope: !592, file: !635, line: 96, type: !627, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !637, retainedNodes: !638)
!637 = !DISubprogram(name: "as_usize", linkageName: "_ZN4core3ptr9alignment9Alignment8as_usize17h0458ba04f1c34212E", scope: !592, file: !635, line: 96, type: !627, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!638 = !{!633, !639}
!639 = !DILocalVariable(name: "self", arg: 1, scope: !640, file: !635, line: 96, type: !592)
!640 = distinct !DILexicalBlock(scope: !636, file: !635, line: 96, column: 5)
!641 = !DILocation(line: 96, column: 27, scope: !634, inlinedAt: !642)
!642 = !DILocation(line: 95, column: 38, scope: !625, inlinedAt: !632)
!643 = !DILocation(line: 96, column: 27, scope: !640, inlinedAt: !644)
!644 = !DILocation(line: 464, column: 77, scope: !609)
!645 = !DILocation(line: 443, column: 13, scope: !572)
!646 = !DILocation(line: 809, column: 49, scope: !612, inlinedAt: !622)
!647 = !DILocation(line: 451, column: 16, scope: !572)
!648 = !DILocation(line: 74, column: 35, scope: !649, inlinedAt: !622)
!649 = !DILexicalBlockFile(scope: !612, file: !650, discriminator: 0)
!650 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ub_checks.rs", directory: "", checksumkind: CSK_MD5, checksum: "c591e051246a94356b6c95b694f07a19")
!651 = !DILocation(line: 97, column: 9, scope: !634, inlinedAt: !642)
!652 = !{i64 1, i64 -9223372036854775807}
!653 = !DILocation(line: 97, column: 23, scope: !634, inlinedAt: !642)
!654 = !DILocation(line: 95, column: 31, scope: !625, inlinedAt: !632)
!655 = !DILocation(line: 95, column: 9, scope: !625, inlinedAt: !632)
!656 = !DILocation(line: 451, column: 37, scope: !572)
!657 = !DILocation(line: 1, column: 1, scope: !658)
!658 = !DILexicalBlockFile(scope: !572, file: !659, discriminator: 0)
!659 = !DIFile(filename: "src/main.rs", directory: "/Users/tobiasviskum/Programming/Rust/viskum-language/llvm-practice", checksumkind: CSK_MD5, checksum: "e9669c841609aacbbdcb922eb036dcd5")
!660 = !DILocation(line: 452, column: 24, scope: !572)
!661 = !{i64 0, i64 -9223372036854775807}
!662 = !DILocation(line: 465, column: 10, scope: !572)
!663 = !DILocation(line: 821, column: 17, scope: !612, inlinedAt: !622)
!664 = !DILocation(line: 459, column: 17, scope: !609)
!665 = !DILocalVariable(name: "size", arg: 1, scope: !666, file: !573, line: 120, type: !9)
!666 = distinct !DILexicalBlock(scope: !667, file: !573, line: 120, column: 5)
!667 = distinct !DISubprogram(name: "from_size_align_unchecked", linkageName: "_ZN4core5alloc6layout6Layout25from_size_align_unchecked17ha924e1e1478b15b8E", scope: !588, file: !573, line: 120, type: !668, scopeLine: 120, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !670, retainedNodes: !671)
!668 = !DISubroutineType(types: !669)
!669 = !{!588, !9, !9}
!670 = !DISubprogram(name: "from_size_align_unchecked", linkageName: "_ZN4core5alloc6layout6Layout25from_size_align_unchecked17ha924e1e1478b15b8E", scope: !588, file: !573, line: 120, type: !668, scopeLine: 120, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!671 = !{!665, !672}
!672 = !DILocalVariable(name: "align", arg: 2, scope: !666, file: !573, line: 120, type: !9)
!673 = !DILocation(line: 120, column: 51, scope: !666, inlinedAt: !674)
!674 = !DILocation(line: 464, column: 25, scope: !609)
!675 = !DILocation(line: 97, column: 9, scope: !640, inlinedAt: !644)
!676 = !DILocation(line: 120, column: 64, scope: !666, inlinedAt: !674)
!677 = !DILocalVariable(name: "align", arg: 1, scope: !678, file: !635, line: 79, type: !9)
!678 = distinct !DILexicalBlock(scope: !679, file: !635, line: 79, column: 5)
!679 = distinct !DISubprogram(name: "new_unchecked", linkageName: "_ZN4core3ptr9alignment9Alignment13new_unchecked17he62723cff9e75a02E", scope: !592, file: !635, line: 79, type: !680, scopeLine: 79, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !682, retainedNodes: !683)
!680 = !DISubroutineType(types: !681)
!681 = !{!592, !9}
!682 = !DISubprogram(name: "new_unchecked", linkageName: "_ZN4core3ptr9alignment9Alignment13new_unchecked17he62723cff9e75a02E", scope: !592, file: !635, line: 79, type: !680, scopeLine: 79, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!683 = !{!677}
!684 = !DILocation(line: 79, column: 39, scope: !678, inlinedAt: !685)
!685 = !DILocation(line: 122, column: 40, scope: !666, inlinedAt: !674)
!686 = !DILocation(line: 97, column: 23, scope: !640, inlinedAt: !644)
!687 = !DILocation(line: 89, column: 18, scope: !678, inlinedAt: !685)
!688 = !DILocation(line: 464, column: 22, scope: !609)
!689 = distinct !DISubprogram(name: "dangling", linkageName: "_ZN4core5alloc6layout6Layout8dangling17h837658e2318fa047E", scope: !588, file: !573, line: 218, type: !690, scopeLine: 218, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !693, retainedNodes: !694)
!690 = !DISubroutineType(types: !691)
!691 = !{!362, !692}
!692 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::alloc::layout::Layout", baseType: !588, size: 64, align: 64, dwarfAddressSpace: 0)
!693 = !DISubprogram(name: "dangling", linkageName: "_ZN4core5alloc6layout6Layout8dangling17h837658e2318fa047E", scope: !588, file: !573, line: 218, type: !690, scopeLine: 218, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!694 = !{!695}
!695 = !DILocalVariable(name: "self", arg: 1, scope: !689, file: !573, line: 218, type: !692)
!696 = !DILocation(line: 218, column: 27, scope: !689)
!697 = !DILocalVariable(name: "self", arg: 1, scope: !698, file: !573, line: 143, type: !692)
!698 = distinct !DILexicalBlock(scope: !699, file: !573, line: 143, column: 5)
!699 = distinct !DISubprogram(name: "align", linkageName: "_ZN4core5alloc6layout6Layout5align17hd70febe67b65a137E", scope: !588, file: !573, line: 143, type: !700, scopeLine: 143, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !702, retainedNodes: !703)
!700 = !DISubroutineType(types: !701)
!701 = !{!9, !692}
!702 = !DISubprogram(name: "align", linkageName: "_ZN4core5alloc6layout6Layout5align17hd70febe67b65a137E", scope: !588, file: !573, line: 143, type: !700, scopeLine: 143, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!703 = !{!697}
!704 = !DILocation(line: 143, column: 24, scope: !698, inlinedAt: !705)
!705 = !DILocation(line: 220, column: 87, scope: !689)
!706 = !DILocation(line: 144, column: 9, scope: !698, inlinedAt: !705)
!707 = !DILocalVariable(name: "self", arg: 1, scope: !708, file: !635, line: 96, type: !592)
!708 = distinct !DILexicalBlock(scope: !709, file: !635, line: 96, column: 5)
!709 = distinct !DISubprogram(name: "as_usize", linkageName: "_ZN4core3ptr9alignment9Alignment8as_usize17h0458ba04f1c34212E", scope: !592, file: !635, line: 96, type: !627, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !637, retainedNodes: !710)
!710 = !{!707}
!711 = !DILocation(line: 96, column: 27, scope: !708, inlinedAt: !712)
!712 = !DILocation(line: 144, column: 20, scope: !698, inlinedAt: !705)
!713 = !DILocation(line: 97, column: 9, scope: !708, inlinedAt: !712)
!714 = !DILocalVariable(name: "addr", arg: 1, scope: !715, file: !539, line: 664, type: !9)
!715 = distinct !DILexicalBlock(scope: !716, file: !539, line: 664, column: 1)
!716 = distinct !DISubprogram(name: "without_provenance_mut<u8>", linkageName: "_ZN4core3ptr22without_provenance_mut17hcad31b4679884145E", scope: !44, file: !539, line: 664, type: !717, scopeLine: 664, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !720)
!717 = !DISubroutineType(types: !718)
!718 = !{!719, !9}
!719 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !36, size: 64, align: 64, dwarfAddressSpace: 0)
!720 = !{!714}
!721 = !DILocation(line: 664, column: 40, scope: !715, inlinedAt: !722)
!722 = !DILocation(line: 220, column: 41, scope: !689)
!723 = !DILocation(line: 97, column: 23, scope: !708, inlinedAt: !712)
!724 = !DILocation(line: 670, column: 14, scope: !715, inlinedAt: !722)
!725 = !DILocalVariable(name: "ptr", arg: 1, scope: !726, file: !727, line: 217, type: !719)
!726 = distinct !DILexicalBlock(scope: !728, file: !727, line: 217, column: 5)
!727 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/non_null.rs", directory: "", checksumkind: CSK_MD5, checksum: "19d9838ed489cc493bac2e425215a13e")
!728 = distinct !DISubprogram(name: "new_unchecked<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h9c3f888b72d32029E", scope: !362, file: !727, line: 217, type: !729, scopeLine: 217, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !731, retainedNodes: !732)
!729 = !DISubroutineType(types: !730)
!730 = !{!362, !719}
!731 = !DISubprogram(name: "new_unchecked<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h9c3f888b72d32029E", scope: !362, file: !727, line: 217, type: !729, scopeLine: 217, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!732 = !{!725}
!733 = !DILocation(line: 217, column: 39, scope: !726, inlinedAt: !734)
!734 = !DILocation(line: 220, column: 18, scope: !689)
!735 = !DILocation(line: 74, column: 35, scope: !736, inlinedAt: !734)
!736 = !DILexicalBlockFile(scope: !726, file: !650, discriminator: 0)
!737 = !DILocation(line: 221, column: 6, scope: !689)
!738 = distinct !DISubprogram(name: "map_or_else<&str, alloc::string::String, alloc::fmt::format::{closure_env#0}, fn(&str) -> alloc::string::String>", linkageName: "_ZN4core6option15Option$LT$T$GT$11map_or_else17ha6e74c3354ce4cf9E", scope: !740, file: !739, line: 1174, type: !751, scopeLine: 1174, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !760, declaration: !759, retainedNodes: !764)
!739 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/option.rs", directory: "", checksumkind: CSK_MD5, checksum: "1839402b3e3d27abc7bbff44f5b669ff")
!740 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&str>", scope: !257, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !741, templateParams: !23, identifier: "a694897fd8eae33cc88e4964bcbf564a")
!741 = !{!742}
!742 = !DICompositeType(tag: DW_TAG_variant_part, scope: !740, file: !2, size: 128, align: 64, elements: !743, templateParams: !23, identifier: "e779c75a35e33f76954d58f57c238832", discriminator: !750)
!743 = !{!744, !746}
!744 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !742, file: !2, baseType: !745, size: 128, align: 64, extraData: i128 0)
!745 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !740, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !321, identifier: "427cbda7985c3dc6122ee7b3c44fdf6")
!746 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !742, file: !2, baseType: !747, size: 128, align: 64)
!747 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !740, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !748, templateParams: !321, identifier: "7fd63a4db9dc9a7cc7fc39a60add5b3b")
!748 = !{!749}
!749 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !747, file: !2, baseType: !242, size: 128, align: 64, flags: DIFlagPublic)
!750 = !DIDerivedType(tag: DW_TAG_member, scope: !740, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!751 = !DISubroutineType(types: !752)
!752 = !{!347, !740, !753, !511}
!753 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}", scope: !754, file: !2, size: 64, align: 64, elements: !756, templateParams: !23, identifier: "9a82f91fabbe5f1a29e99d2f4c8ce2cb")
!754 = !DINamespace(name: "format", scope: !755)
!755 = !DINamespace(name: "fmt", scope: !113)
!756 = !{!757}
!757 = !DIDerivedType(tag: DW_TAG_member, name: "_ref__args", scope: !753, file: !2, baseType: !758, size: 64, align: 64)
!758 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::Arguments", baseType: !405, size: 64, align: 64, dwarfAddressSpace: 0)
!759 = !DISubprogram(name: "map_or_else<&str, alloc::string::String, alloc::fmt::format::{closure_env#0}, fn(&str) -> alloc::string::String>", linkageName: "_ZN4core6option15Option$LT$T$GT$11map_or_else17ha6e74c3354ce4cf9E", scope: !740, file: !739, line: 1174, type: !751, scopeLine: 1174, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !760)
!760 = !{!322, !761, !762, !763}
!761 = !DITemplateTypeParameter(name: "U", type: !347)
!762 = !DITemplateTypeParameter(name: "D", type: !753)
!763 = !DITemplateTypeParameter(name: "F", type: !511)
!764 = !{!765, !766, !767, !768}
!765 = !DILocalVariable(name: "self", arg: 1, scope: !738, file: !739, line: 1174, type: !740)
!766 = !DILocalVariable(name: "default", arg: 2, scope: !738, file: !739, line: 1174, type: !753)
!767 = !DILocalVariable(name: "f", arg: 3, scope: !738, file: !739, line: 1174, type: !511)
!768 = !DILocalVariable(name: "t", scope: !769, file: !739, line: 1180, type: !242, align: 8)
!769 = distinct !DILexicalBlock(scope: !738, file: !739, line: 1180, column: 13)
!770 = !DILocation(line: 1174, column: 33, scope: !738)
!771 = !DILocation(line: 1174, column: 39, scope: !738)
!772 = !DILocation(line: 1174, column: 51, scope: !738)
!773 = !DILocation(line: 1179, column: 15, scope: !738)
!774 = !DILocation(line: 1179, column: 9, scope: !738)
!775 = !DILocation(line: 1181, column: 21, scope: !738)
!776 = !DILocation(line: 1180, column: 18, scope: !738)
!777 = !DILocation(line: 1180, column: 18, scope: !769)
!778 = !DILocation(line: 1180, column: 24, scope: !769)
!779 = !DILocation(line: 1183, column: 5, scope: !738)
!780 = !{i8 0, i8 2}
!781 = !DILocation(line: 1183, column: 6, scope: !738)
!782 = !DILocation(line: 1174, column: 5, scope: !738)
!783 = distinct !DISubprogram(name: "to_vec<u8, alloc::alloc::Global>", linkageName: "_ZN52_$LT$T$u20$as$u20$alloc..slice..hack..ConvertVec$GT$6to_vec17hd776f6a506b6fc27E", scope: !785, file: !784, line: 161, type: !788, scopeLine: 161, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, retainedNodes: !794)
!784 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/slice.rs", directory: "", checksumkind: CSK_MD5, checksum: "61ba2fcd5957ff17c5931914ca2c6e0b")
!785 = !DINamespace(name: "{impl#1}", scope: !786)
!786 = !DINamespace(name: "hack", scope: !787)
!787 = !DINamespace(name: "slice", scope: !113)
!788 = !DISubroutineType(types: !789)
!789 = !{!351, !790, !374}
!790 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[u8]", file: !2, size: 128, align: 64, elements: !791, templateParams: !23, identifier: "31681e0c10b314f1f33e38b2779acbb4")
!791 = !{!792, !793}
!792 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !790, file: !2, baseType: !245, size: 64, align: 64)
!793 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !790, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!794 = !{!795, !796, !797}
!795 = !DILocalVariable(name: "s", arg: 1, scope: !783, file: !784, line: 161, type: !790)
!796 = !DILocalVariable(name: "alloc", arg: 2, scope: !783, file: !784, line: 161, type: !374)
!797 = !DILocalVariable(name: "v", scope: !798, file: !784, line: 162, type: !351, align: 8)
!798 = distinct !DILexicalBlock(scope: !783, file: !784, line: 162, column: 13)
!799 = !DILocation(line: 161, column: 33, scope: !783)
!800 = !DILocalVariable(name: "self", arg: 1, scope: !801, file: !802, line: 762, type: !790)
!801 = distinct !DILexicalBlock(scope: !803, file: !802, line: 762, column: 5)
!802 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/slice/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "b9c99b41e5a028756ff4b0fa65d11506")
!803 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6as_ptr17hb2e0056b6e1f8c27E", scope: !804, file: !802, line: 762, type: !806, scopeLine: 762, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !808)
!804 = !DINamespace(name: "{impl#0}", scope: !805)
!805 = !DINamespace(name: "slice", scope: !35)
!806 = !DISubroutineType(types: !807)
!807 = !{!154, !790}
!808 = !{!800}
!809 = !DILocation(line: 762, column: 25, scope: !801, inlinedAt: !810)
!810 = !DILocation(line: 167, column: 19, scope: !798)
!811 = !DILocation(line: 161, column: 45, scope: !783)
!812 = !DILocalVariable(name: "alloc", arg: 2, scope: !813, file: !814, line: 698, type: !374)
!813 = distinct !DILexicalBlock(scope: !815, file: !814, line: 698, column: 5)
!814 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/vec/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "69d957016438ff2dba09b6c7fc66594f")
!815 = distinct !DISubprogram(name: "with_capacity_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$16with_capacity_in17hbc040a7ed8e4f8b1E", scope: !351, file: !814, line: 698, type: !816, scopeLine: 698, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !818, retainedNodes: !819)
!816 = !DISubroutineType(types: !817)
!817 = !{!351, !9, !374}
!818 = !DISubprogram(name: "with_capacity_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$16with_capacity_in17hbc040a7ed8e4f8b1E", scope: !351, file: !814, line: 698, type: !816, scopeLine: 698, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!819 = !{!820, !812}
!820 = !DILocalVariable(name: "capacity", arg: 1, scope: !813, file: !814, line: 698, type: !9)
!821 = !DILocation(line: 698, column: 46, scope: !813, inlinedAt: !822)
!822 = !DILocation(line: 162, column: 25, scope: !783)
!823 = !DILocalVariable(name: "alloc", arg: 2, scope: !824, file: !825, line: 157, type: !374)
!824 = distinct !DILexicalBlock(scope: !826, file: !825, line: 157, column: 5)
!825 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/raw_vec.rs", directory: "", checksumkind: CSK_MD5, checksum: "99d32400863317b069599ba961cfe288")
!826 = distinct !DISubprogram(name: "with_capacity_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16with_capacity_in17h59791bb17281e7fbE", scope: !355, file: !825, line: 157, type: !827, scopeLine: 157, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !829, retainedNodes: !830)
!827 = !DISubroutineType(types: !828)
!828 = !{!355, !9, !374}
!829 = !DISubprogram(name: "with_capacity_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16with_capacity_in17h59791bb17281e7fbE", scope: !355, file: !825, line: 157, type: !827, scopeLine: 157, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!830 = !{!831, !823, !832, !834}
!831 = !DILocalVariable(name: "capacity", arg: 1, scope: !824, file: !825, line: 157, type: !9)
!832 = !DILocalVariable(name: "res", scope: !833, file: !825, line: 159, type: !355, align: 8)
!833 = distinct !DILexicalBlock(scope: !824, file: !825, line: 159, column: 13)
!834 = !DILocalVariable(name: "err", scope: !835, file: !825, line: 160, type: !836, align: 8)
!835 = distinct !DILexicalBlock(scope: !824, file: !825, line: 160, column: 13)
!836 = !DICompositeType(tag: DW_TAG_structure_type, name: "TryReserveError", scope: !837, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !838, templateParams: !23, identifier: "eba8bc6e8666e1a650422427eaf34b1b")
!837 = !DINamespace(name: "collections", scope: !113)
!838 = !{!839}
!839 = !DIDerivedType(tag: DW_TAG_member, name: "kind", scope: !836, file: !2, baseType: !840, size: 128, align: 64, flags: DIFlagPrivate)
!840 = !DICompositeType(tag: DW_TAG_structure_type, name: "TryReserveErrorKind", scope: !837, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !841, templateParams: !23, identifier: "c110128c7be5158110818042a7b0bbaa")
!841 = !{!842}
!842 = !DICompositeType(tag: DW_TAG_variant_part, scope: !840, file: !2, size: 128, align: 64, elements: !843, templateParams: !23, identifier: "80286da70c6379f716ba791be1dab040", discriminator: !851)
!843 = !{!844, !846}
!844 = !DIDerivedType(tag: DW_TAG_member, name: "CapacityOverflow", scope: !842, file: !2, baseType: !845, size: 128, align: 64, extraData: i128 0)
!845 = !DICompositeType(tag: DW_TAG_structure_type, name: "CapacityOverflow", scope: !840, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !23, identifier: "99daf11aa03037472cad0c5dc75a31f0")
!846 = !DIDerivedType(tag: DW_TAG_member, name: "AllocError", scope: !842, file: !2, baseType: !847, size: 128, align: 64)
!847 = !DICompositeType(tag: DW_TAG_structure_type, name: "AllocError", scope: !840, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !848, templateParams: !23, identifier: "2cafa62c7a86f44be1a9ffaa6a7cce7f")
!848 = !{!849, !850}
!849 = !DIDerivedType(tag: DW_TAG_member, name: "layout", scope: !847, file: !2, baseType: !588, size: 128, align: 64, flags: DIFlagPublic)
!850 = !DIDerivedType(tag: DW_TAG_member, name: "non_exhaustive", scope: !847, file: !2, baseType: !7, align: 8, offset: 128, flags: DIFlagPublic)
!851 = !DIDerivedType(tag: DW_TAG_member, scope: !840, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!852 = !DILocation(line: 157, column: 46, scope: !824, inlinedAt: !853)
!853 = !DILocation(line: 699, column: 20, scope: !813, inlinedAt: !822)
!854 = !DILocation(line: 162, column: 17, scope: !798)
!855 = !DILocation(line: 162, column: 17, scope: !783)
!856 = !DILocation(line: 162, column: 47, scope: !783)
!857 = !DILocation(line: 698, column: 29, scope: !813, inlinedAt: !822)
!858 = !DILocation(line: 157, column: 29, scope: !824, inlinedAt: !853)
!859 = !DILocation(line: 158, column: 15, scope: !824, inlinedAt: !853)
!860 = !{i64 0, i64 2}
!861 = !DILocation(line: 158, column: 9, scope: !824, inlinedAt: !853)
!862 = !DILocation(line: 159, column: 16, scope: !824, inlinedAt: !853)
!863 = !{i64 0, i64 -9223372036854775808}
!864 = !DILocation(line: 159, column: 16, scope: !833, inlinedAt: !853)
!865 = !DILocation(line: 162, column: 5, scope: !824, inlinedAt: !853)
!866 = !DILocation(line: 699, column: 9, scope: !813, inlinedAt: !822)
!867 = !DILocation(line: 763, column: 9, scope: !801, inlinedAt: !810)
!868 = !DILocalVariable(name: "self", arg: 1, scope: !869, file: !870, line: 1372, type: !154)
!869 = distinct !DILexicalBlock(scope: !871, file: !870, line: 1372, column: 5)
!870 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/const_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "b925067ec6cc15f01151b5ae643852d1")
!871 = distinct !DISubprogram(name: "copy_to_nonoverlapping<u8>", linkageName: "_ZN4core3ptr9const_ptr33_$LT$impl$u20$$BP$const$u20$T$GT$22copy_to_nonoverlapping17hd0dfa0fadba359f4E", scope: !872, file: !870, line: 1372, type: !874, scopeLine: 1372, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !876)
!872 = !DINamespace(name: "{impl#0}", scope: !873)
!873 = !DINamespace(name: "const_ptr", scope: !44)
!874 = !DISubroutineType(types: !875)
!875 = !{null, !154, !719, !9}
!876 = !{!868, !877, !878}
!877 = !DILocalVariable(name: "dest", arg: 2, scope: !869, file: !870, line: 1372, type: !719)
!878 = !DILocalVariable(name: "count", arg: 3, scope: !869, file: !870, line: 1372, type: !9)
!879 = !DILocation(line: 1372, column: 48, scope: !869, inlinedAt: !880)
!880 = !DILocation(line: 167, column: 28, scope: !798)
!881 = !DILocalVariable(name: "src", arg: 1, scope: !882, file: !883, line: 2934, type: !154)
!882 = distinct !DILexicalBlock(scope: !884, file: !883, line: 2934, column: 1)
!883 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/intrinsics.rs", directory: "", checksumkind: CSK_MD5, checksum: "07c886c0e74c4d03adac48db772adcc3")
!884 = distinct !DISubprogram(name: "copy_nonoverlapping<u8>", linkageName: "_ZN4core10intrinsics19copy_nonoverlapping17hedc01625c1f67f96E", scope: !885, file: !883, line: 2934, type: !874, scopeLine: 2934, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !886)
!885 = !DINamespace(name: "intrinsics", scope: !35)
!886 = !{!881, !887, !888}
!887 = !DILocalVariable(name: "dst", arg: 2, scope: !882, file: !883, line: 2934, type: !719)
!888 = !DILocalVariable(name: "count", arg: 3, scope: !882, file: !883, line: 2934, type: !9)
!889 = !DILocation(line: 2934, column: 44, scope: !882, inlinedAt: !890)
!890 = !DILocation(line: 1377, column: 18, scope: !869, inlinedAt: !880)
!891 = !DILocation(line: 167, column: 51, scope: !798)
!892 = !DILocalVariable(name: "self", arg: 1, scope: !893, file: !814, line: 1389, type: !897)
!893 = distinct !DILexicalBlock(scope: !894, file: !814, line: 1389, column: 5)
!894 = distinct !DISubprogram(name: "as_mut_ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17h477d382ab6ac9780E", scope: !351, file: !814, line: 1389, type: !895, scopeLine: 1389, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !898, retainedNodes: !899)
!895 = !DISubroutineType(types: !896)
!896 = !{!719, !897}
!897 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut alloc::vec::Vec<u8, alloc::alloc::Global>", baseType: !351, size: 64, align: 64, dwarfAddressSpace: 0)
!898 = !DISubprogram(name: "as_mut_ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17h477d382ab6ac9780E", scope: !351, file: !814, line: 1389, type: !895, scopeLine: 1389, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!899 = !{!892}
!900 = !DILocation(line: 1389, column: 23, scope: !893, inlinedAt: !901)
!901 = !DILocation(line: 167, column: 53, scope: !798)
!902 = !DILocation(line: 1392, column: 9, scope: !893, inlinedAt: !901)
!903 = !DILocalVariable(name: "self", arg: 1, scope: !904, file: !825, line: 277, type: !908)
!904 = distinct !DILexicalBlock(scope: !905, file: !825, line: 277, column: 5)
!905 = distinct !DISubprogram(name: "ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17he44bf70be75db6eaE", scope: !355, file: !825, line: 277, type: !906, scopeLine: 277, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !909, retainedNodes: !910)
!906 = !DISubroutineType(types: !907)
!907 = !{!719, !908}
!908 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::raw_vec::RawVec<u8, alloc::alloc::Global>", baseType: !355, size: 64, align: 64, dwarfAddressSpace: 0)
!909 = !DISubprogram(name: "ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17he44bf70be75db6eaE", scope: !355, file: !825, line: 277, type: !906, scopeLine: 277, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!910 = !{!903}
!911 = !DILocation(line: 277, column: 16, scope: !904, inlinedAt: !912)
!912 = !DILocation(line: 1392, column: 18, scope: !893, inlinedAt: !901)
!913 = !DILocation(line: 278, column: 9, scope: !904, inlinedAt: !912)
!914 = !DILocalVariable(name: "self", scope: !915, file: !916, line: 105, type: !358, align: 8)
!915 = distinct !DILexicalBlock(scope: !917, file: !916, line: 105, column: 5)
!916 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/unique.rs", directory: "", checksumkind: CSK_MD5, checksum: "732da2a5cbcfe11170beb774dbba4d93")
!917 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h53c445a80d45c1caE", scope: !358, file: !916, line: 105, type: !918, scopeLine: 105, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !920, retainedNodes: !921)
!918 = !DISubroutineType(types: !919)
!919 = !{!719, !358}
!920 = !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h53c445a80d45c1caE", scope: !358, file: !916, line: 105, type: !918, scopeLine: 105, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!921 = !{!914, !914}
!922 = !DILocation(line: 105, column: 25, scope: !915, inlinedAt: !923)
!923 = !DILocation(line: 278, column: 18, scope: !904, inlinedAt: !912)
!924 = !DILocalVariable(name: "self", arg: 1, scope: !925, file: !727, line: 350, type: !362)
!925 = distinct !DILexicalBlock(scope: !926, file: !727, line: 350, column: 5)
!926 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hb4ed4f907fea3a81E", scope: !362, file: !727, line: 350, type: !927, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !929, retainedNodes: !930)
!927 = !DISubroutineType(types: !928)
!928 = !{!719, !362}
!929 = !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hb4ed4f907fea3a81E", scope: !362, file: !727, line: 350, type: !927, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!930 = !{!924}
!931 = !DILocation(line: 350, column: 25, scope: !925, inlinedAt: !932)
!932 = !DILocation(line: 106, column: 22, scope: !915, inlinedAt: !923)
!933 = !DILocation(line: 351, column: 9, scope: !925, inlinedAt: !932)
!934 = !DILocation(line: 1372, column: 54, scope: !869, inlinedAt: !880)
!935 = !DILocation(line: 2934, column: 59, scope: !882, inlinedAt: !890)
!936 = !DILocation(line: 167, column: 67, scope: !798)
!937 = !DILocation(line: 1372, column: 68, scope: !869, inlinedAt: !880)
!938 = !DILocation(line: 2934, column: 72, scope: !882, inlinedAt: !890)
!939 = !DILocation(line: 74, column: 35, scope: !940, inlinedAt: !890)
!940 = !DILexicalBlockFile(scope: !882, file: !650, discriminator: 0)
!941 = !DILocation(line: 160, column: 17, scope: !824, inlinedAt: !853)
!942 = !DILocation(line: 160, column: 17, scope: !835, inlinedAt: !853)
!943 = !DILocation(line: 160, column: 25, scope: !835, inlinedAt: !853)
!944 = !DILocation(line: 2959, column: 14, scope: !882, inlinedAt: !890)
!945 = !DILocation(line: 168, column: 17, scope: !798)
!946 = !DILocalVariable(name: "self", arg: 1, scope: !947, file: !814, line: 1482, type: !897)
!947 = distinct !DILexicalBlock(scope: !948, file: !814, line: 1482, column: 5)
!948 = distinct !DISubprogram(name: "set_len<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len17h76240d2a8cbbd6d9E", scope: !351, file: !814, line: 1482, type: !949, scopeLine: 1482, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !951, retainedNodes: !952)
!949 = !DISubroutineType(types: !950)
!950 = !{null, !897, !9}
!951 = !DISubprogram(name: "set_len<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len17h76240d2a8cbbd6d9E", scope: !351, file: !814, line: 1482, type: !949, scopeLine: 1482, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!952 = !{!946, !953}
!953 = !DILocalVariable(name: "new_len", arg: 2, scope: !947, file: !814, line: 1482, type: !9)
!954 = !DILocation(line: 1482, column: 27, scope: !947, inlinedAt: !955)
!955 = !DILocation(line: 168, column: 19, scope: !798)
!956 = !DILocalVariable(name: "self", arg: 1, scope: !957, file: !814, line: 948, type: !897)
!957 = distinct !DILexicalBlock(scope: !958, file: !814, line: 948, column: 5)
!958 = distinct !DISubprogram(name: "capacity<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$8capacity17h75d119ec2a921e8dE", scope: !351, file: !814, line: 948, type: !959, scopeLine: 948, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !962, retainedNodes: !963)
!959 = !DISubroutineType(types: !960)
!960 = !{!9, !961}
!961 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::vec::Vec<u8, alloc::alloc::Global>", baseType: !351, size: 64, align: 64, dwarfAddressSpace: 0)
!962 = !DISubprogram(name: "capacity<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$8capacity17h75d119ec2a921e8dE", scope: !351, file: !814, line: 948, type: !959, scopeLine: 948, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!963 = !{!956}
!964 = !DILocation(line: 948, column: 21, scope: !957, inlinedAt: !965)
!965 = !DILocation(line: 1483, column: 39, scope: !947, inlinedAt: !955)
!966 = !DILocation(line: 168, column: 27, scope: !798)
!967 = !DILocation(line: 1482, column: 38, scope: !947, inlinedAt: !955)
!968 = !DILocation(line: 1485, column: 9, scope: !947, inlinedAt: !955)
!969 = !DILocation(line: 170, column: 13, scope: !798)
!970 = !DILocation(line: 171, column: 9, scope: !783)
!971 = !DILocation(line: 171, column: 10, scope: !783)
!972 = distinct !DISubprogram(name: "report", linkageName: "_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17hfeb601ef834eda5dE", scope: !973, file: !182, line: 2421, type: !974, scopeLine: 2421, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !976)
!973 = !DINamespace(name: "{impl#57}", scope: !185)
!974 = !DISubroutineType(types: !975)
!975 = !{!184, !7}
!976 = !{!977, !978}
!977 = !DILocalVariable(name: "self", scope: !972, file: !182, line: 2421, type: !7, align: 1)
!978 = !DILocalVariable(arg: 1, scope: !972, file: !182, line: 2421, type: !7)
!979 = !DILocation(line: 2421, column: 15, scope: !972)
!980 = !DILocation(line: 2423, column: 6, scope: !972)
!981 = distinct !DISubprogram(name: "format", linkageName: "_ZN5alloc3fmt6format17h58dbea2d6d469c4dE", scope: !755, file: !982, line: 629, type: !983, scopeLine: 629, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !985)
!982 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/fmt.rs", directory: "", checksumkind: CSK_MD5, checksum: "f6ac2dba32fa39c01538f6ab9aa1fdc8")
!983 = !DISubroutineType(types: !984)
!984 = !{!347, !405}
!985 = !{!986}
!986 = !DILocalVariable(name: "args", arg: 1, scope: !981, file: !982, line: 629, type: !405)
!987 = !DILocation(line: 629, column: 15, scope: !981)
!988 = !DILocation(line: 639, column: 5, scope: !981)
!989 = !DILocalVariable(name: "self", arg: 1, scope: !990, file: !220, line: 445, type: !758)
!990 = distinct !DILexicalBlock(scope: !991, file: !220, line: 445, column: 5)
!991 = distinct !DISubprogram(name: "as_str", linkageName: "_ZN4core3fmt9Arguments6as_str17h6b5ff2c39c01a730E", scope: !405, file: !220, line: 445, type: !992, scopeLine: 445, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !994, retainedNodes: !995)
!992 = !DISubroutineType(types: !993)
!993 = !{!740, !758}
!994 = !DISubprogram(name: "as_str", linkageName: "_ZN4core3fmt9Arguments6as_str17h6b5ff2c39c01a730E", scope: !405, file: !220, line: 445, type: !992, scopeLine: 445, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!995 = !{!989, !996}
!996 = !DILocalVariable(name: "s", scope: !997, file: !220, line: 448, type: !241, align: 8)
!997 = distinct !DILexicalBlock(scope: !990, file: !220, line: 448, column: 13)
!998 = !DILocation(line: 445, column: 25, scope: !990, inlinedAt: !999)
!999 = !DILocation(line: 639, column: 10, scope: !981)
!1000 = !DILocation(line: 446, column: 16, scope: !990, inlinedAt: !999)
!1001 = !DILocation(line: 446, column: 29, scope: !990, inlinedAt: !999)
!1002 = !DILocation(line: 447, column: 14, scope: !990, inlinedAt: !999)
!1003 = !DILocation(line: 447, column: 18, scope: !990, inlinedAt: !999)
!1004 = !DILocation(line: 448, column: 14, scope: !990, inlinedAt: !999)
!1005 = !DILocation(line: 447, column: 25, scope: !990, inlinedAt: !999)
!1006 = !DILocation(line: 447, column: 32, scope: !990, inlinedAt: !999)
!1007 = !DILocation(line: 449, column: 18, scope: !990, inlinedAt: !999)
!1008 = !DILocation(line: 639, column: 86, scope: !981)
!1009 = !DILocation(line: 640, column: 2, scope: !981)
!1010 = !DILocation(line: 448, column: 19, scope: !990, inlinedAt: !999)
!1011 = !DILocation(line: 448, column: 15, scope: !990, inlinedAt: !999)
!1012 = !DILocation(line: 448, column: 15, scope: !997, inlinedAt: !999)
!1013 = !DILocation(line: 448, column: 31, scope: !997, inlinedAt: !999)
!1014 = !DILocation(line: 448, column: 26, scope: !997, inlinedAt: !999)
!1015 = !DILocation(line: 448, column: 32, scope: !990, inlinedAt: !999)
!1016 = distinct !DISubprogram(name: "{closure#0}", linkageName: "_ZN5alloc3fmt6format28_$u7b$$u7b$closure$u7d$$u7d$17h2dc8367f5c84c60dE", scope: !754, file: !982, line: 639, type: !1017, scopeLine: 639, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1019)
!1017 = !DISubroutineType(types: !1018)
!1018 = !{!347, !753}
!1019 = !{!1020}
!1020 = !DILocalVariable(name: "args", scope: !1016, file: !982, line: 629, type: !405, align: 8)
!1021 = !DILocation(line: 629, column: 15, scope: !1016)
!1022 = !DILocation(line: 639, column: 47, scope: !1016)
!1023 = !DILocation(line: 639, column: 34, scope: !1016)
!1024 = !DILocation(line: 639, column: 51, scope: !1016)
!1025 = !DILocation(line: 639, column: 52, scope: !1016)
!1026 = distinct !DISubprogram(name: "to_owned", linkageName: "_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17h12890283f9e53a37E", scope: !1028, file: !1027, line: 210, type: !512, scopeLine: 210, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1030)
!1027 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/str.rs", directory: "", checksumkind: CSK_MD5, checksum: "8aee8cc10963fc5755a5ce533b85b83c")
!1028 = !DINamespace(name: "{impl#4}", scope: !1029)
!1029 = !DINamespace(name: "str", scope: !113)
!1030 = !{!1031}
!1031 = !DILocalVariable(name: "self", arg: 1, scope: !1026, file: !1027, line: 210, type: !242)
!1032 = !DILocalVariable(name: "alloc", scope: !1033, file: !784, line: 436, type: !374, align: 1)
!1033 = distinct !DILexicalBlock(scope: !1034, file: !784, line: 436, column: 5)
!1034 = distinct !DISubprogram(name: "to_vec_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9to_vec_in17h1cd3e0f46d7e49d2E", scope: !1035, file: !784, line: 436, type: !788, scopeLine: 436, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, retainedNodes: !1036)
!1035 = !DINamespace(name: "{impl#0}", scope: !787)
!1036 = !{!1037, !1032}
!1037 = !DILocalVariable(name: "self", arg: 1, scope: !1033, file: !784, line: 436, type: !790)
!1038 = !DILocation(line: 436, column: 43, scope: !1033, inlinedAt: !1039)
!1039 = !DILocation(line: 416, column: 14, scope: !1040, inlinedAt: !1046)
!1040 = distinct !DILexicalBlock(scope: !1041, file: !784, line: 412, column: 5)
!1041 = distinct !DISubprogram(name: "to_vec<u8>", linkageName: "_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17h26b74980b8ed562fE", scope: !1035, file: !784, line: 412, type: !1042, scopeLine: 412, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1044)
!1042 = !DISubroutineType(types: !1043)
!1043 = !{!351, !790}
!1044 = !{!1045}
!1045 = !DILocalVariable(name: "self", arg: 1, scope: !1040, file: !784, line: 412, type: !790)
!1046 = !DILocation(line: 823, column: 14, scope: !1047, inlinedAt: !1052)
!1047 = distinct !DILexicalBlock(scope: !1048, file: !784, line: 822, column: 5)
!1048 = distinct !DISubprogram(name: "to_owned<u8>", linkageName: "_ZN5alloc5slice64_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$8to_owned17h36e68c25f03febcaE", scope: !1049, file: !784, line: 822, type: !1042, scopeLine: 822, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1050)
!1049 = !DINamespace(name: "{impl#9}", scope: !787)
!1050 = !{!1051}
!1051 = !DILocalVariable(name: "self", arg: 1, scope: !1047, file: !784, line: 822, type: !790)
!1052 = !DILocation(line: 211, column: 62, scope: !1026)
!1053 = !DILocalVariable(name: "alloc", scope: !1054, file: !784, line: 110, type: !374, align: 1)
!1054 = distinct !DILexicalBlock(scope: !1055, file: !784, line: 110, column: 5)
!1055 = distinct !DISubprogram(name: "to_vec<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc5slice4hack6to_vec17heafceac16bdcac94E", scope: !786, file: !784, line: 110, type: !788, scopeLine: 110, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, retainedNodes: !1056)
!1056 = !{!1057, !1053}
!1057 = !DILocalVariable(name: "s", arg: 1, scope: !1054, file: !784, line: 110, type: !790)
!1058 = !DILocation(line: 110, column: 57, scope: !1054, inlinedAt: !1059)
!1059 = !DILocation(line: 441, column: 9, scope: !1033, inlinedAt: !1039)
!1060 = !DILocation(line: 210, column: 17, scope: !1026)
!1061 = !DILocalVariable(name: "self", arg: 1, scope: !1062, file: !1063, line: 320, type: !242)
!1062 = distinct !DILexicalBlock(scope: !1064, file: !1063, line: 320, column: 5)
!1063 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/str/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "8e1c84e0855cb642390047f356befa9d")
!1064 = distinct !DISubprogram(name: "as_bytes", linkageName: "_ZN4core3str21_$LT$impl$u20$str$GT$8as_bytes17hcb0fdd62aef58c8aE", scope: !1065, file: !1063, line: 320, type: !1067, scopeLine: 320, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1069)
!1065 = !DINamespace(name: "{impl#0}", scope: !1066)
!1066 = !DINamespace(name: "str", scope: !35)
!1067 = !DISubroutineType(types: !1068)
!1068 = !{!790, !242}
!1069 = !{!1061}
!1070 = !DILocation(line: 320, column: 27, scope: !1062, inlinedAt: !1071)
!1071 = !DILocation(line: 211, column: 51, scope: !1026)
!1072 = !DILocalVariable(name: "bytes", arg: 1, scope: !1073, file: !1074, line: 993, type: !351)
!1073 = distinct !DILexicalBlock(scope: !1075, file: !1074, line: 993, column: 5)
!1074 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/string.rs", directory: "", checksumkind: CSK_MD5, checksum: "6cd8d9ab0cb0bda6e9c0ce18eb9cca99")
!1075 = distinct !DISubprogram(name: "from_utf8_unchecked", linkageName: "_ZN5alloc6string6String19from_utf8_unchecked17hbdbd6ced53b6f262E", scope: !347, file: !1074, line: 993, type: !1076, scopeLine: 993, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !1078, retainedNodes: !1079)
!1076 = !DISubroutineType(types: !1077)
!1077 = !{!347, !351}
!1078 = !DISubprogram(name: "from_utf8_unchecked", linkageName: "_ZN5alloc6string6String19from_utf8_unchecked17hbdbd6ced53b6f262E", scope: !347, file: !1074, line: 993, type: !1076, scopeLine: 993, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!1079 = !{!1072}
!1080 = !DILocation(line: 993, column: 39, scope: !1073, inlinedAt: !1081)
!1081 = !DILocation(line: 211, column: 18, scope: !1026)
!1082 = !DILocation(line: 211, column: 46, scope: !1026)
!1083 = !DILocation(line: 322, column: 18, scope: !1062, inlinedAt: !1071)
!1084 = !DILocation(line: 822, column: 17, scope: !1047, inlinedAt: !1052)
!1085 = !DILocation(line: 412, column: 19, scope: !1040, inlinedAt: !1046)
!1086 = !DILocation(line: 436, column: 36, scope: !1033, inlinedAt: !1039)
!1087 = !DILocation(line: 110, column: 48, scope: !1054, inlinedAt: !1059)
!1088 = !DILocation(line: 111, column: 9, scope: !1054, inlinedAt: !1059)
!1089 = !DILocation(line: 994, column: 9, scope: !1073, inlinedAt: !1081)
!1090 = !DILocation(line: 211, column: 72, scope: !1026)
!1091 = !DILocation(line: 212, column: 6, scope: !1026)
!1092 = distinct !DISubprogram(name: "alloc", linkageName: "_ZN5alloc5alloc5alloc17h1ece6c5426488636E", scope: !375, file: !1093, line: 94, type: !1094, scopeLine: 94, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1096)
!1093 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/alloc.rs", directory: "", checksumkind: CSK_MD5, checksum: "6687e3e140ed9b8c51f77000a3d3a272")
!1094 = !DISubroutineType(types: !1095)
!1095 = !{!719, !588}
!1096 = !{!1097}
!1097 = !DILocalVariable(name: "layout", arg: 1, scope: !1092, file: !1093, line: 94, type: !588)
!1098 = !DILocation(line: 1695, column: 32, scope: !1099, inlinedAt: !1105)
!1099 = distinct !DILexicalBlock(scope: !1100, file: !539, line: 1695, column: 1)
!1100 = distinct !DISubprogram(name: "read_volatile<u8>", linkageName: "_ZN4core3ptr13read_volatile17h365b7112c0ea65f8E", scope: !44, file: !539, line: 1695, type: !1101, scopeLine: 1695, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1103)
!1101 = !DISubroutineType(types: !1102)
!1102 = !{!36, !154}
!1103 = !{!1104}
!1104 = !DILocalVariable(name: "src", scope: !1099, file: !539, line: 1695, type: !154, align: 8)
!1105 = !DILocation(line: 98, column: 9, scope: !1092)
!1106 = !DILocation(line: 94, column: 21, scope: !1092)
!1107 = !DILocation(line: 74, column: 35, scope: !1108, inlinedAt: !1105)
!1108 = !DILexicalBlockFile(scope: !1099, file: !650, discriminator: 0)
!1109 = !DILocation(line: 1706, column: 9, scope: !1099, inlinedAt: !1105)
!1110 = !DILocation(line: 100, column: 22, scope: !1092)
!1111 = !DILocalVariable(name: "self", arg: 1, scope: !1112, file: !573, line: 130, type: !692)
!1112 = distinct !DILexicalBlock(scope: !1113, file: !573, line: 130, column: 5)
!1113 = distinct !DISubprogram(name: "size", linkageName: "_ZN4core5alloc6layout6Layout4size17h94d72322f31a62e0E", scope: !588, file: !573, line: 130, type: !700, scopeLine: 130, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !1114, retainedNodes: !1115)
!1114 = !DISubprogram(name: "size", linkageName: "_ZN4core5alloc6layout6Layout4size17h94d72322f31a62e0E", scope: !588, file: !573, line: 130, type: !700, scopeLine: 130, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!1115 = !{!1111}
!1116 = !DILocation(line: 130, column: 23, scope: !1112, inlinedAt: !1117)
!1117 = !DILocation(line: 100, column: 29, scope: !1092)
!1118 = !DILocation(line: 131, column: 9, scope: !1112, inlinedAt: !1117)
!1119 = !DILocation(line: 100, column: 37, scope: !1092)
!1120 = !DILocalVariable(name: "self", arg: 1, scope: !1121, file: !573, line: 143, type: !692)
!1121 = distinct !DILexicalBlock(scope: !1122, file: !573, line: 143, column: 5)
!1122 = distinct !DISubprogram(name: "align", linkageName: "_ZN4core5alloc6layout6Layout5align17hd70febe67b65a137E", scope: !588, file: !573, line: 143, type: !700, scopeLine: 143, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !702, retainedNodes: !1123)
!1123 = !{!1120}
!1124 = !DILocation(line: 143, column: 24, scope: !1121, inlinedAt: !1125)
!1125 = !DILocation(line: 100, column: 44, scope: !1092)
!1126 = !DILocation(line: 144, column: 9, scope: !1121, inlinedAt: !1125)
!1127 = !DILocalVariable(name: "self", arg: 1, scope: !1128, file: !635, line: 96, type: !592)
!1128 = distinct !DILexicalBlock(scope: !1129, file: !635, line: 96, column: 5)
!1129 = distinct !DISubprogram(name: "as_usize", linkageName: "_ZN4core3ptr9alignment9Alignment8as_usize17h0458ba04f1c34212E", scope: !592, file: !635, line: 96, type: !627, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !637, retainedNodes: !1130)
!1130 = !{!1127}
!1131 = !DILocation(line: 96, column: 27, scope: !1128, inlinedAt: !1132)
!1132 = !DILocation(line: 144, column: 20, scope: !1121, inlinedAt: !1125)
!1133 = !DILocation(line: 97, column: 9, scope: !1128, inlinedAt: !1132)
!1134 = !DILocation(line: 97, column: 23, scope: !1128, inlinedAt: !1132)
!1135 = !DILocation(line: 100, column: 9, scope: !1092)
!1136 = !DILocation(line: 102, column: 2, scope: !1092)
!1137 = distinct !DISubprogram(name: "alloc_impl", linkageName: "_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E", scope: !374, file: !1093, line: 178, type: !1138, scopeLine: 178, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !1166, retainedNodes: !1167)
!1138 = !DISubroutineType(types: !1139)
!1139 = !{!1140, !1164, !588, !1165}
!1140 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError>", scope: !225, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1141, templateParams: !23, identifier: "80606fecc880ae2928960cee4ba81d5c")
!1141 = !{!1142}
!1142 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1140, file: !2, size: 128, align: 64, elements: !1143, templateParams: !23, identifier: "211b50244dcaf7231a014a7c17b18685", discriminator: !1163)
!1143 = !{!1144, !1159}
!1144 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1142, file: !2, baseType: !1145, size: 128, align: 64)
!1145 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1140, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1146, templateParams: !1155, identifier: "c31cc39fb15ea61d8e955957a965dfd5")
!1146 = !{!1147}
!1147 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1145, file: !2, baseType: !1148, size: 128, align: 64, flags: DIFlagPublic)
!1148 = !DICompositeType(tag: DW_TAG_structure_type, name: "NonNull<[u8]>", scope: !363, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1149, templateParams: !285, identifier: "28785a76644c70d515f628b2fe557ae1")
!1149 = !{!1150}
!1150 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !1148, file: !2, baseType: !1151, size: 128, align: 64, flags: DIFlagPrivate)
!1151 = !DICompositeType(tag: DW_TAG_structure_type, name: "*const [u8]", file: !2, size: 128, align: 64, elements: !1152, templateParams: !23, identifier: "a10360edaf335c418dbc95bccd0cb05d")
!1152 = !{!1153, !1154}
!1153 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !1151, file: !2, baseType: !245, size: 64, align: 64)
!1154 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !1151, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!1155 = !{!1156, !1157}
!1156 = !DITemplateTypeParameter(name: "T", type: !1148)
!1157 = !DITemplateTypeParameter(name: "E", type: !1158)
!1158 = !DICompositeType(tag: DW_TAG_structure_type, name: "AllocError", scope: !577, file: !2, align: 8, flags: DIFlagPublic, elements: !23, identifier: "47d6361a123597d4d924834ce3743b83")
!1159 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1142, file: !2, baseType: !1160, size: 128, align: 64, extraData: i128 0)
!1160 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1140, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1161, templateParams: !1155, identifier: "403bc91d5e98745925c0cb3256e996ef")
!1161 = !{!1162}
!1162 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1160, file: !2, baseType: !1158, align: 8, flags: DIFlagPublic)
!1163 = !DIDerivedType(tag: DW_TAG_member, scope: !1140, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!1164 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&alloc::alloc::Global", baseType: !374, size: 64, align: 64, dwarfAddressSpace: 0)
!1165 = !DIBasicType(name: "bool", size: 8, encoding: DW_ATE_boolean)
!1166 = !DISubprogram(name: "alloc_impl", linkageName: "_ZN5alloc5alloc6Global10alloc_impl17h19546100811e9e02E", scope: !374, file: !1093, line: 178, type: !1138, scopeLine: 178, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !23)
!1167 = !{!1168, !1169, !1170, !1171, !1173, !1175, !1177, !1197}
!1168 = !DILocalVariable(name: "self", arg: 1, scope: !1137, file: !1093, line: 178, type: !1164)
!1169 = !DILocalVariable(name: "layout", arg: 2, scope: !1137, file: !1093, line: 178, type: !588)
!1170 = !DILocalVariable(name: "zeroed", arg: 3, scope: !1137, file: !1093, line: 178, type: !1165)
!1171 = !DILocalVariable(name: "size", scope: !1172, file: !1093, line: 182, type: !9, align: 8)
!1172 = distinct !DILexicalBlock(scope: !1137, file: !1093, line: 182, column: 13)
!1173 = !DILocalVariable(name: "raw_ptr", scope: !1174, file: !1093, line: 183, type: !719, align: 8)
!1174 = distinct !DILexicalBlock(scope: !1172, file: !1093, line: 183, column: 17)
!1175 = !DILocalVariable(name: "ptr", scope: !1176, file: !1093, line: 184, type: !362, align: 8)
!1176 = distinct !DILexicalBlock(scope: !1174, file: !1093, line: 184, column: 17)
!1177 = !DILocalVariable(name: "residual", scope: !1178, file: !1093, line: 184, type: !1179, align: 1)
!1178 = distinct !DILexicalBlock(scope: !1174, file: !1093, line: 184, column: 66)
!1179 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::convert::Infallible, core::alloc::AllocError>", scope: !225, file: !2, align: 8, flags: DIFlagPublic, elements: !1180, templateParams: !23, identifier: "7b61b925021e42117f486d5aecf2a734")
!1180 = !{!1181}
!1181 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1179, file: !2, align: 8, elements: !1182, templateParams: !23, identifier: "382e2df3210abdc3aae61b4ce80fc154")
!1182 = !{!1183, !1193}
!1183 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1181, file: !2, baseType: !1184, align: 8)
!1184 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1179, file: !2, align: 8, flags: DIFlagPublic, elements: !1185, templateParams: !1191, identifier: "f4d848b432587ab1c3176fb87ca61aff")
!1185 = !{!1186}
!1186 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1184, file: !2, baseType: !1187, align: 8, flags: DIFlagPublic)
!1187 = !DICompositeType(tag: DW_TAG_structure_type, name: "Infallible", scope: !1188, file: !2, align: 8, flags: DIFlagPublic, elements: !1189, templateParams: !23, identifier: "9483f8862e5675d21ba246c8b418738d")
!1188 = !DINamespace(name: "convert", scope: !35)
!1189 = !{!1190}
!1190 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1187, file: !2, align: 8, elements: !23, identifier: "ca7d250b4a4e3bbea15182fd92628f71")
!1191 = !{!1192, !1157}
!1192 = !DITemplateTypeParameter(name: "T", type: !1187)
!1193 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1181, file: !2, baseType: !1194, align: 8)
!1194 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1179, file: !2, align: 8, flags: DIFlagPublic, elements: !1195, templateParams: !1191, identifier: "dac906ddfd75e03fa58e055de73e029c")
!1195 = !{!1196}
!1196 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1194, file: !2, baseType: !1158, align: 8, flags: DIFlagPublic)
!1197 = !DILocalVariable(name: "val", scope: !1198, file: !1093, line: 184, type: !362, align: 8)
!1198 = distinct !DILexicalBlock(scope: !1174, file: !1093, line: 184, column: 27)
!1199 = !DILocation(line: 184, column: 66, scope: !1178)
!1200 = !DILocation(line: 1530, column: 57, scope: !1201, inlinedAt: !1212)
!1201 = distinct !DILexicalBlock(scope: !1202, file: !727, line: 1530, column: 5)
!1202 = distinct !DISubprogram(name: "slice_from_raw_parts<u8>", linkageName: "_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h9367e5b81c5a542cE", scope: !1148, file: !727, line: 1530, type: !1203, scopeLine: 1530, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1205, retainedNodes: !1206)
!1203 = !DISubroutineType(types: !1204)
!1204 = !{!1148, !362, !9}
!1205 = !DISubprogram(name: "slice_from_raw_parts<u8>", linkageName: "_ZN4core3ptr8non_null26NonNull$LT$$u5b$T$u5d$$GT$20slice_from_raw_parts17h9367e5b81c5a542cE", scope: !1148, file: !727, line: 1530, type: !1203, scopeLine: 1530, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1206 = !{!1207, !1208, !1209, !1211}
!1207 = !DILocalVariable(name: "data", arg: 1, scope: !1201, file: !727, line: 1530, type: !362)
!1208 = !DILocalVariable(name: "len", scope: !1201, file: !727, line: 1530, type: !9, align: 8)
!1209 = !DILocalVariable(name: "data", arg: 1, scope: !1210, file: !727, line: 1530, type: !362)
!1210 = distinct !DILexicalBlock(scope: !1202, file: !727, line: 1530, column: 5)
!1211 = !DILocalVariable(name: "len", arg: 2, scope: !1210, file: !727, line: 1530, type: !9)
!1212 = !DILocation(line: 180, column: 21, scope: !1137)
!1213 = !DILocation(line: 888, column: 56, scope: !1214, inlinedAt: !1228)
!1214 = distinct !DILexicalBlock(scope: !1215, file: !539, line: 888, column: 1)
!1215 = distinct !DISubprogram(name: "slice_from_raw_parts_mut<u8>", linkageName: "_ZN4core3ptr24slice_from_raw_parts_mut17h67507d7975176d6fE", scope: !44, file: !539, line: 888, type: !1216, scopeLine: 888, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1222)
!1216 = !DISubroutineType(types: !1217)
!1217 = !{!1218, !719, !9}
!1218 = !DICompositeType(tag: DW_TAG_structure_type, name: "*mut [u8]", file: !2, size: 128, align: 64, elements: !1219, templateParams: !23, identifier: "eb80752d8dc9079cf56e9f0de61d8d5f")
!1219 = !{!1220, !1221}
!1220 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !1218, file: !2, baseType: !245, size: 64, align: 64)
!1221 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !1218, file: !2, baseType: !9, size: 64, align: 64, offset: 64)
!1222 = !{!1223, !1224, !1225, !1227}
!1223 = !DILocalVariable(name: "data", arg: 1, scope: !1214, file: !539, line: 888, type: !719)
!1224 = !DILocalVariable(name: "len", scope: !1214, file: !539, line: 888, type: !9, align: 8)
!1225 = !DILocalVariable(name: "data", arg: 1, scope: !1226, file: !539, line: 888, type: !719)
!1226 = distinct !DILexicalBlock(scope: !1215, file: !539, line: 888, column: 1)
!1227 = !DILocalVariable(name: "len", arg: 2, scope: !1226, file: !539, line: 888, type: !9)
!1228 = !DILocation(line: 1532, column: 38, scope: !1201, inlinedAt: !1212)
!1229 = !DILocation(line: 138, column: 5, scope: !1230, inlinedAt: !1242)
!1230 = distinct !DILexicalBlock(scope: !1232, file: !1231, line: 136, column: 1)
!1231 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/metadata.rs", directory: "", checksumkind: CSK_MD5, checksum: "2b0cc3fdaa3f6ef53f463e56dd523dc3")
!1232 = distinct !DISubprogram(name: "from_raw_parts_mut<[u8], u8>", linkageName: "_ZN4core3ptr8metadata18from_raw_parts_mut17ha8d71fcd9909e534E", scope: !1233, file: !1231, line: 136, type: !1216, scopeLine: 136, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1240, retainedNodes: !1234)
!1233 = !DINamespace(name: "metadata", scope: !44)
!1234 = !{!1235, !1236, !1237, !1239}
!1235 = !DILocalVariable(name: "data_pointer", arg: 1, scope: !1230, file: !1231, line: 137, type: !719)
!1236 = !DILocalVariable(name: "metadata", scope: !1230, file: !1231, line: 138, type: !9, align: 8)
!1237 = !DILocalVariable(name: "data_pointer", arg: 1, scope: !1238, file: !1231, line: 137, type: !719)
!1238 = distinct !DILexicalBlock(scope: !1232, file: !1231, line: 136, column: 1)
!1239 = !DILocalVariable(name: "metadata", arg: 2, scope: !1238, file: !1231, line: 138, type: !9)
!1240 = !{!286, !1241}
!1241 = !DITemplateTypeParameter(name: "impl Thin", type: !36)
!1242 = !DILocation(line: 889, column: 5, scope: !1214, inlinedAt: !1228)
!1243 = !DILocalVariable(name: "err", scope: !1244, file: !739, line: 1208, type: !1158, align: 1)
!1244 = distinct !DILexicalBlock(scope: !1245, file: !739, line: 1208, column: 5)
!1245 = distinct !DISubprogram(name: "ok_or<core::ptr::non_null::NonNull<u8>, core::alloc::AllocError>", linkageName: "_ZN4core6option15Option$LT$T$GT$5ok_or17h891be1b59b0d8ff2E", scope: !1246, file: !739, line: 1208, type: !1259, scopeLine: 1208, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1269, declaration: !1275, retainedNodes: !1276)
!1246 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<core::ptr::non_null::NonNull<u8>>", scope: !257, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1247, templateParams: !23, identifier: "8dbd318d3aaa4f3db918317c88bd1d45")
!1247 = !{!1248}
!1248 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1246, file: !2, size: 64, align: 64, elements: !1249, templateParams: !23, identifier: "f8fc177c2ce9b0a1a915ffd6a3969244", discriminator: !1258)
!1249 = !{!1250, !1254}
!1250 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1248, file: !2, baseType: !1251, size: 64, align: 64, extraData: i128 0)
!1251 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1246, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !1252, identifier: "66c7f7a23607a9405a536b271229538b")
!1252 = !{!1253}
!1253 = !DITemplateTypeParameter(name: "T", type: !362)
!1254 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1248, file: !2, baseType: !1255, size: 64, align: 64)
!1255 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1246, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1256, templateParams: !1252, identifier: "66fe66ae712be336864cd73a9d3b6b5f")
!1256 = !{!1257}
!1257 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1255, file: !2, baseType: !362, size: 64, align: 64, flags: DIFlagPublic)
!1258 = !DIDerivedType(tag: DW_TAG_member, scope: !1246, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!1259 = !DISubroutineType(types: !1260)
!1260 = !{!1261, !1246, !1158}
!1261 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<core::ptr::non_null::NonNull<u8>, core::alloc::AllocError>", scope: !225, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1262, templateParams: !23, identifier: "a93b514be24cf40fd89e6f3e973c312e")
!1262 = !{!1263}
!1263 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1261, file: !2, size: 64, align: 64, elements: !1264, templateParams: !23, identifier: "854c1bb7919a4566bd06501ad8b0c4bd", discriminator: !1274)
!1264 = !{!1265, !1270}
!1265 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1263, file: !2, baseType: !1266, size: 64, align: 64)
!1266 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1261, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1267, templateParams: !1269, identifier: "4bfac77697326918b1898ad2d2633134")
!1267 = !{!1268}
!1268 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1266, file: !2, baseType: !362, size: 64, align: 64, flags: DIFlagPublic)
!1269 = !{!1253, !1157}
!1270 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1263, file: !2, baseType: !1271, size: 64, align: 64, extraData: i128 0)
!1271 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1261, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1272, templateParams: !1269, identifier: "807dbf2e270662bb1462ee89afb95a80")
!1272 = !{!1273}
!1273 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1271, file: !2, baseType: !1158, align: 8, flags: DIFlagPublic)
!1274 = !DIDerivedType(tag: DW_TAG_member, scope: !1261, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!1275 = !DISubprogram(name: "ok_or<core::ptr::non_null::NonNull<u8>, core::alloc::AllocError>", linkageName: "_ZN4core6option15Option$LT$T$GT$5ok_or17h891be1b59b0d8ff2E", scope: !1246, file: !739, line: 1208, type: !1259, scopeLine: 1208, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1269)
!1276 = !{!1277, !1243, !1278}
!1277 = !DILocalVariable(name: "self", arg: 1, scope: !1244, file: !739, line: 1208, type: !1246)
!1278 = !DILocalVariable(name: "v", scope: !1279, file: !739, line: 1210, type: !362, align: 8)
!1279 = distinct !DILexicalBlock(scope: !1244, file: !739, line: 1210, column: 13)
!1280 = !DILocation(line: 1208, column: 27, scope: !1244, inlinedAt: !1281)
!1281 = !DILocation(line: 184, column: 49, scope: !1174)
!1282 = !DILocalVariable(name: "e", scope: !1283, file: !1284, line: 1978, type: !1158, align: 1)
!1283 = distinct !DILexicalBlock(scope: !1285, file: !1284, line: 1978, column: 13)
!1284 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/result.rs", directory: "", checksumkind: CSK_MD5, checksum: "3803467de0bb49bd3e784a9a6b155e26")
!1285 = distinct !DILexicalBlock(scope: !1286, file: !1284, line: 1975, column: 5)
!1286 = distinct !DISubprogram(name: "branch<core::ptr::non_null::NonNull<u8>, core::alloc::AllocError>", linkageName: "_ZN79_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..ops..try_trait..Try$GT$6branch17h831ce08e6f7e4b80E", scope: !1287, file: !1284, line: 1975, type: !1288, scopeLine: 1975, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1269, retainedNodes: !1307)
!1287 = !DINamespace(name: "{impl#26}", scope: !225)
!1288 = !DISubroutineType(types: !1289)
!1289 = !{!1290, !1261}
!1290 = !DICompositeType(tag: DW_TAG_structure_type, name: "ControlFlow<core::result::Result<core::convert::Infallible, core::alloc::AllocError>, core::ptr::non_null::NonNull<u8>>", scope: !1291, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1292, templateParams: !23, identifier: "a0142340207968fda7c75b416df4e32e")
!1291 = !DINamespace(name: "control_flow", scope: !497)
!1292 = !{!1293}
!1293 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1290, file: !2, size: 64, align: 64, elements: !1294, templateParams: !23, identifier: "e39492dddcb842e7867369ed5bb402a8", discriminator: !1306)
!1294 = !{!1295, !1302}
!1295 = !DIDerivedType(tag: DW_TAG_member, name: "Continue", scope: !1293, file: !2, baseType: !1296, size: 64, align: 64)
!1296 = !DICompositeType(tag: DW_TAG_structure_type, name: "Continue", scope: !1290, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1297, templateParams: !1299, identifier: "215d0615a836ba6618bfb562db26210c")
!1297 = !{!1298}
!1298 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1296, file: !2, baseType: !362, size: 64, align: 64, flags: DIFlagPublic)
!1299 = !{!1300, !1301}
!1300 = !DITemplateTypeParameter(name: "B", type: !1179)
!1301 = !DITemplateTypeParameter(name: "C", type: !362)
!1302 = !DIDerivedType(tag: DW_TAG_member, name: "Break", scope: !1293, file: !2, baseType: !1303, size: 64, align: 64, extraData: i128 0)
!1303 = !DICompositeType(tag: DW_TAG_structure_type, name: "Break", scope: !1290, file: !2, size: 64, align: 64, flags: DIFlagPublic, elements: !1304, templateParams: !1299, identifier: "80cca19fa5a7a7171578f43cc9c87122")
!1304 = !{!1305}
!1305 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1303, file: !2, baseType: !1179, align: 8, flags: DIFlagPublic)
!1306 = !DIDerivedType(tag: DW_TAG_member, scope: !1290, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!1307 = !{!1308, !1309, !1282}
!1308 = !DILocalVariable(name: "self", arg: 1, scope: !1285, file: !1284, line: 1975, type: !1261)
!1309 = !DILocalVariable(name: "v", scope: !1310, file: !1284, line: 1977, type: !362, align: 8)
!1310 = distinct !DILexicalBlock(scope: !1285, file: !1284, line: 1977, column: 13)
!1311 = !DILocation(line: 1978, column: 17, scope: !1283, inlinedAt: !1312)
!1312 = !DILocation(line: 184, column: 27, scope: !1174)
!1313 = !DILocalVariable(name: "residual", scope: !1314, file: !1284, line: 1987, type: !1179, align: 1)
!1314 = distinct !DILexicalBlock(scope: !1315, file: !1284, line: 1987, column: 5)
!1315 = distinct !DISubprogram(name: "from_residual<core::ptr::non_null::NonNull<[u8]>, core::alloc::AllocError, core::alloc::AllocError>", linkageName: "_ZN153_$LT$core..result..Result$LT$T$C$F$GT$$u20$as$u20$core..ops..try_trait..FromResidual$LT$core..result..Result$LT$core..convert..Infallible$C$E$GT$$GT$$GT$13from_residual17h86e7c948a7c433d9E", scope: !1316, file: !1284, line: 1987, type: !1317, scopeLine: 1987, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1330, retainedNodes: !1327)
!1316 = !DINamespace(name: "{impl#27}", scope: !225)
!1317 = !DISubroutineType(types: !1318)
!1318 = !{!1140, !1179, !1319}
!1319 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::location::Location", baseType: !1320, size: 64, align: 64, dwarfAddressSpace: 0)
!1320 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !1321, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !1323, templateParams: !23, identifier: "93b71791e758e84bfa222d86a69b93")
!1321 = !DINamespace(name: "location", scope: !1322)
!1322 = !DINamespace(name: "panic", scope: !35)
!1323 = !{!1324, !1325, !1326}
!1324 = !DIDerivedType(tag: DW_TAG_member, name: "file", scope: !1320, file: !2, baseType: !242, size: 128, align: 64, flags: DIFlagPrivate)
!1325 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !1320, file: !2, baseType: !251, size: 32, align: 32, offset: 128, flags: DIFlagPrivate)
!1326 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !1320, file: !2, baseType: !251, size: 32, align: 32, offset: 160, flags: DIFlagPrivate)
!1327 = !{!1313, !1328}
!1328 = !DILocalVariable(name: "e", scope: !1329, file: !1284, line: 1989, type: !1158, align: 1)
!1329 = distinct !DILexicalBlock(scope: !1314, file: !1284, line: 1989, column: 13)
!1330 = !{!1156, !1157, !1331}
!1331 = !DITemplateTypeParameter(name: "F", type: !1158)
!1332 = !DILocation(line: 1987, column: 22, scope: !1314, inlinedAt: !1333)
!1333 = !DILocation(line: 184, column: 27, scope: !1178)
!1334 = !DILocation(line: 1989, column: 17, scope: !1329, inlinedAt: !1333)
!1335 = !DILocalVariable(name: "t", scope: !1336, file: !1337, line: 768, type: !1158, align: 1)
!1336 = distinct !DILexicalBlock(scope: !1338, file: !1337, line: 768, column: 5)
!1337 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/convert/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "da159ddadc0729d78bae61e73fd78caf")
!1338 = distinct !DISubprogram(name: "from<core::alloc::AllocError>", linkageName: "_ZN50_$LT$T$u20$as$u20$core..convert..From$LT$T$GT$$GT$4from17h3dc49fd9cd508140E", scope: !1339, file: !1337, line: 768, type: !1340, scopeLine: 768, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1343, retainedNodes: !1342)
!1339 = !DINamespace(name: "{impl#4}", scope: !1188)
!1340 = !DISubroutineType(types: !1341)
!1341 = !{null, !1158}
!1342 = !{!1335}
!1343 = !{!1344}
!1344 = !DITemplateTypeParameter(name: "T", type: !1158)
!1345 = !DILocation(line: 768, column: 13, scope: !1336, inlinedAt: !1346)
!1346 = !DILocation(line: 1989, column: 27, scope: !1329, inlinedAt: !1333)
!1347 = !DILocation(line: 178, column: 19, scope: !1137)
!1348 = !DILocation(line: 178, column: 26, scope: !1137)
!1349 = !DILocation(line: 178, column: 42, scope: !1137)
!1350 = !DILocation(line: 183, column: 21, scope: !1174)
!1351 = !DILocalVariable(name: "layout", arg: 1, scope: !1352, file: !1093, line: 171, type: !588)
!1352 = distinct !DILexicalBlock(scope: !1353, file: !1093, line: 171, column: 1)
!1353 = distinct !DISubprogram(name: "alloc_zeroed", linkageName: "_ZN5alloc5alloc12alloc_zeroed17h108aa653d6d3d405E", scope: !375, file: !1093, line: 171, type: !1094, scopeLine: 171, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1354)
!1354 = !{!1351}
!1355 = !DILocation(line: 171, column: 28, scope: !1352, inlinedAt: !1356)
!1356 = !DILocation(line: 183, column: 43, scope: !1172)
!1357 = !DILocation(line: 1975, column: 15, scope: !1285, inlinedAt: !1312)
!1358 = !DILocation(line: 1208, column: 21, scope: !1244, inlinedAt: !1281)
!1359 = !DILocation(line: 179, column: 15, scope: !1137)
!1360 = !DILocalVariable(name: "self", arg: 1, scope: !1361, file: !573, line: 130, type: !692)
!1361 = distinct !DILexicalBlock(scope: !1362, file: !573, line: 130, column: 5)
!1362 = distinct !DISubprogram(name: "size", linkageName: "_ZN4core5alloc6layout6Layout4size17h94d72322f31a62e0E", scope: !588, file: !573, line: 130, type: !700, scopeLine: 130, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !1114, retainedNodes: !1363)
!1363 = !{!1360, !1364}
!1364 = !DILocalVariable(name: "self", arg: 1, scope: !1365, file: !573, line: 130, type: !692)
!1365 = distinct !DILexicalBlock(scope: !1362, file: !573, line: 130, column: 5)
!1366 = !DILocation(line: 130, column: 23, scope: !1361, inlinedAt: !1367)
!1367 = !DILocation(line: 179, column: 22, scope: !1137)
!1368 = !DILocation(line: 131, column: 9, scope: !1361, inlinedAt: !1367)
!1369 = !DILocation(line: 182, column: 13, scope: !1172)
!1370 = !DILocation(line: 1530, column: 57, scope: !1210, inlinedAt: !1371)
!1371 = !DILocation(line: 185, column: 20, scope: !1176)
!1372 = !DILocation(line: 888, column: 56, scope: !1226, inlinedAt: !1373)
!1373 = !DILocation(line: 1532, column: 38, scope: !1210, inlinedAt: !1371)
!1374 = !DILocation(line: 138, column: 5, scope: !1238, inlinedAt: !1375)
!1375 = !DILocation(line: 889, column: 5, scope: !1226, inlinedAt: !1373)
!1376 = !DILocation(line: 179, column: 9, scope: !1137)
!1377 = !DILocation(line: 180, column: 51, scope: !1137)
!1378 = !DILocation(line: 1530, column: 39, scope: !1201, inlinedAt: !1212)
!1379 = !DILocalVariable(name: "self", arg: 1, scope: !1380, file: !727, line: 350, type: !362)
!1380 = distinct !DILexicalBlock(scope: !1381, file: !727, line: 350, column: 5)
!1381 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hb4ed4f907fea3a81E", scope: !362, file: !727, line: 350, type: !927, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !929, retainedNodes: !1382)
!1382 = !{!1379, !1383}
!1383 = !DILocalVariable(name: "self", arg: 1, scope: !1384, file: !727, line: 350, type: !362)
!1384 = distinct !DILexicalBlock(scope: !1381, file: !727, line: 350, column: 5)
!1385 = !DILocation(line: 350, column: 25, scope: !1380, inlinedAt: !1386)
!1386 = !DILocation(line: 1532, column: 75, scope: !1201, inlinedAt: !1212)
!1387 = !DILocation(line: 351, column: 9, scope: !1380, inlinedAt: !1386)
!1388 = !DILocation(line: 888, column: 42, scope: !1214, inlinedAt: !1228)
!1389 = !DILocation(line: 137, column: 5, scope: !1230, inlinedAt: !1242)
!1390 = !DILocation(line: 140, column: 5, scope: !1230, inlinedAt: !1242)
!1391 = !DILocalVariable(name: "ptr", arg: 1, scope: !1392, file: !727, line: 217, type: !1218)
!1392 = distinct !DILexicalBlock(scope: !1393, file: !727, line: 217, column: 5)
!1393 = distinct !DISubprogram(name: "new_unchecked<[u8]>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h334493148410fa38E", scope: !1148, file: !727, line: 217, type: !1394, scopeLine: 217, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1396, retainedNodes: !1397)
!1394 = !DISubroutineType(types: !1395)
!1395 = !{!1148, !1218}
!1396 = !DISubprogram(name: "new_unchecked<[u8]>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h334493148410fa38E", scope: !1148, file: !727, line: 217, type: !1394, scopeLine: 217, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1397 = !{!1391, !1398}
!1398 = !DILocalVariable(name: "ptr", arg: 1, scope: !1399, file: !727, line: 217, type: !1218)
!1399 = distinct !DILexicalBlock(scope: !1393, file: !727, line: 217, column: 5)
!1400 = !DILocation(line: 217, column: 39, scope: !1392, inlinedAt: !1401)
!1401 = !DILocation(line: 1532, column: 18, scope: !1201, inlinedAt: !1212)
!1402 = !DILocation(line: 74, column: 35, scope: !1403, inlinedAt: !1401)
!1403 = !DILexicalBlockFile(scope: !1392, file: !650, discriminator: 0)
!1404 = !DILocation(line: 183, column: 21, scope: !1172)
!1405 = !DILocation(line: 183, column: 34, scope: !1172)
!1406 = !DILocation(line: 180, column: 18, scope: !1137)
!1407 = !DILocation(line: 180, column: 72, scope: !1137)
!1408 = !DILocation(line: 188, column: 6, scope: !1137)
!1409 = !DILocation(line: 183, column: 73, scope: !1172)
!1410 = !DILocation(line: 183, column: 56, scope: !1172)
!1411 = !DILocation(line: 172, column: 34, scope: !1352, inlinedAt: !1356)
!1412 = !DILocation(line: 130, column: 23, scope: !1365, inlinedAt: !1413)
!1413 = !DILocation(line: 172, column: 41, scope: !1352, inlinedAt: !1356)
!1414 = !DILocation(line: 172, column: 49, scope: !1352, inlinedAt: !1356)
!1415 = !DILocalVariable(name: "self", arg: 1, scope: !1416, file: !573, line: 143, type: !692)
!1416 = distinct !DILexicalBlock(scope: !1417, file: !573, line: 143, column: 5)
!1417 = distinct !DISubprogram(name: "align", linkageName: "_ZN4core5alloc6layout6Layout5align17hd70febe67b65a137E", scope: !588, file: !573, line: 143, type: !700, scopeLine: 143, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !702, retainedNodes: !1418)
!1418 = !{!1415}
!1419 = !DILocation(line: 143, column: 24, scope: !1416, inlinedAt: !1420)
!1420 = !DILocation(line: 172, column: 56, scope: !1352, inlinedAt: !1356)
!1421 = !DILocation(line: 144, column: 9, scope: !1416, inlinedAt: !1420)
!1422 = !DILocalVariable(name: "self", arg: 1, scope: !1423, file: !635, line: 96, type: !592)
!1423 = distinct !DILexicalBlock(scope: !1424, file: !635, line: 96, column: 5)
!1424 = distinct !DISubprogram(name: "as_usize", linkageName: "_ZN4core3ptr9alignment9Alignment8as_usize17h0458ba04f1c34212E", scope: !592, file: !635, line: 96, type: !627, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !637, retainedNodes: !1425)
!1425 = !{!1422}
!1426 = !DILocation(line: 96, column: 27, scope: !1423, inlinedAt: !1427)
!1427 = !DILocation(line: 144, column: 20, scope: !1416, inlinedAt: !1420)
!1428 = !DILocation(line: 97, column: 9, scope: !1423, inlinedAt: !1427)
!1429 = !DILocation(line: 97, column: 23, scope: !1423, inlinedAt: !1427)
!1430 = !DILocation(line: 172, column: 14, scope: !1352, inlinedAt: !1356)
!1431 = !DILocation(line: 183, column: 62, scope: !1172)
!1432 = !DILocation(line: 183, column: 31, scope: !1172)
!1433 = !DILocation(line: 184, column: 40, scope: !1174)
!1434 = !DILocalVariable(name: "ptr", arg: 1, scope: !1435, file: !727, line: 246, type: !719)
!1435 = distinct !DILexicalBlock(scope: !1436, file: !727, line: 246, column: 5)
!1436 = distinct !DISubprogram(name: "new<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17ha165b75f9705d2acE", scope: !362, file: !727, line: 246, type: !1437, scopeLine: 246, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1439, retainedNodes: !1440)
!1437 = !DISubroutineType(types: !1438)
!1438 = !{!1246, !719}
!1439 = !DISubprogram(name: "new<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$3new17ha165b75f9705d2acE", scope: !362, file: !727, line: 246, type: !1437, scopeLine: 246, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1440 = !{!1434}
!1441 = !DILocation(line: 246, column: 22, scope: !1435, inlinedAt: !1312)
!1442 = !DILocalVariable(name: "self", arg: 1, scope: !1443, file: !1444, line: 35, type: !719)
!1443 = distinct !DILexicalBlock(scope: !1445, file: !1444, line: 35, column: 5)
!1444 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/ptr/mut_ptr.rs", directory: "", checksumkind: CSK_MD5, checksum: "8cba8f2cafffb0d8862ad9c302ad0cdd")
!1445 = distinct !DISubprogram(name: "is_null<u8>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17h211284e166bc186fE", scope: !1446, file: !1444, line: 35, type: !1448, scopeLine: 35, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1450)
!1446 = !DINamespace(name: "{impl#0}", scope: !1447)
!1447 = !DINamespace(name: "mut_ptr", scope: !44)
!1448 = !DISubroutineType(types: !1449)
!1449 = !{!1165, !719}
!1450 = !{!1442}
!1451 = !DILocation(line: 35, column: 26, scope: !1443, inlinedAt: !1452)
!1452 = !DILocation(line: 247, column: 17, scope: !1435, inlinedAt: !1312)
!1453 = !DILocalVariable(name: "ptr", arg: 1, scope: !1454, file: !1444, line: 37, type: !719)
!1454 = distinct !DILexicalBlock(scope: !1455, file: !1444, line: 37, column: 9)
!1455 = distinct !DISubprogram(name: "runtime_impl", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null12runtime_impl17hf0d8d987850f8c52E", scope: !1456, file: !1444, line: 37, type: !1448, scopeLine: 37, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1457)
!1456 = !DINamespace(name: "is_null", scope: !1446)
!1457 = !{!1453}
!1458 = !DILocation(line: 37, column: 25, scope: !1454, inlinedAt: !1459)
!1459 = !DILocation(line: 51, column: 9, scope: !1443, inlinedAt: !1452)
!1460 = !DILocalVariable(name: "self", arg: 1, scope: !1461, file: !1444, line: 213, type: !719)
!1461 = distinct !DILexicalBlock(scope: !1462, file: !1444, line: 213, column: 5)
!1462 = distinct !DISubprogram(name: "addr<u8>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$4addr17hd2535e1bd9f575a9E", scope: !1446, file: !1444, line: 213, type: !1463, scopeLine: 213, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1465)
!1463 = !DISubroutineType(types: !1464)
!1464 = !{!9, !719}
!1465 = !{!1460}
!1466 = !DILocation(line: 213, column: 17, scope: !1461, inlinedAt: !1467)
!1467 = !DILocation(line: 38, column: 17, scope: !1454, inlinedAt: !1459)
!1468 = !DILocalVariable(name: "self", arg: 1, scope: !1469, file: !1444, line: 59, type: !719)
!1469 = distinct !DILexicalBlock(scope: !1470, file: !1444, line: 59, column: 5)
!1470 = distinct !DISubprogram(name: "cast<u8, ()>", linkageName: "_ZN4core3ptr7mut_ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$4cast17h175b32f6d803d1a2E", scope: !1446, file: !1444, line: 59, type: !1471, scopeLine: 59, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1475, retainedNodes: !1474)
!1471 = !DISubroutineType(types: !1472)
!1472 = !{!1473, !719}
!1473 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut ()", baseType: !7, size: 64, align: 64, dwarfAddressSpace: 0)
!1474 = !{!1468}
!1475 = !{!286, !1476}
!1476 = !DITemplateTypeParameter(name: "U", type: !7)
!1477 = !DILocation(line: 59, column: 26, scope: !1469, inlinedAt: !1478)
!1478 = !DILocation(line: 217, column: 38, scope: !1461, inlinedAt: !1467)
!1479 = !DILocalVariable(name: "ptr", arg: 1, scope: !1480, file: !727, line: 217, type: !719)
!1480 = distinct !DILexicalBlock(scope: !1481, file: !727, line: 217, column: 5)
!1481 = distinct !DISubprogram(name: "new_unchecked<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h9c3f888b72d32029E", scope: !362, file: !727, line: 217, type: !729, scopeLine: 217, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !731, retainedNodes: !1482)
!1482 = !{!1479}
!1483 = !DILocation(line: 217, column: 39, scope: !1480, inlinedAt: !1484)
!1484 = !DILocation(line: 249, column: 27, scope: !1435, inlinedAt: !1312)
!1485 = !DILocation(line: 217, column: 18, scope: !1461, inlinedAt: !1467)
!1486 = !DILocation(line: 247, column: 13, scope: !1435, inlinedAt: !1312)
!1487 = !DILocation(line: 251, column: 13, scope: !1435, inlinedAt: !1312)
!1488 = !DILocation(line: 1211, column: 21, scope: !1244, inlinedAt: !1281)
!1489 = !DILocation(line: 184, column: 65, scope: !1174)
!1490 = !DILocation(line: 184, column: 66, scope: !1174)
!1491 = !DILocation(line: 1989, column: 23, scope: !1329, inlinedAt: !1333)
!1492 = !DILocation(line: 184, column: 67, scope: !1174)
!1493 = !DILocation(line: 186, column: 13, scope: !1172)
!1494 = !DILocation(line: 74, column: 35, scope: !1495, inlinedAt: !1484)
!1495 = !DILexicalBlockFile(scope: !1480, file: !650, discriminator: 0)
!1496 = !DILocation(line: 249, column: 13, scope: !1435, inlinedAt: !1312)
!1497 = !DILocation(line: 1210, column: 18, scope: !1244, inlinedAt: !1281)
!1498 = !DILocation(line: 1210, column: 18, scope: !1279, inlinedAt: !1281)
!1499 = !DILocation(line: 1210, column: 24, scope: !1279, inlinedAt: !1281)
!1500 = !DILocation(line: 1977, column: 16, scope: !1285, inlinedAt: !1312)
!1501 = !DILocation(line: 1977, column: 16, scope: !1310, inlinedAt: !1312)
!1502 = !DILocation(line: 1977, column: 22, scope: !1310, inlinedAt: !1312)
!1503 = !DILocation(line: 184, column: 21, scope: !1176)
!1504 = !DILocation(line: 184, column: 27, scope: !1198)
!1505 = !DILocation(line: 1530, column: 39, scope: !1210, inlinedAt: !1371)
!1506 = !DILocation(line: 350, column: 25, scope: !1384, inlinedAt: !1507)
!1507 = !DILocation(line: 1532, column: 75, scope: !1210, inlinedAt: !1371)
!1508 = !DILocation(line: 351, column: 9, scope: !1384, inlinedAt: !1507)
!1509 = !DILocation(line: 888, column: 42, scope: !1226, inlinedAt: !1373)
!1510 = !DILocation(line: 137, column: 5, scope: !1238, inlinedAt: !1375)
!1511 = !DILocation(line: 140, column: 5, scope: !1238, inlinedAt: !1375)
!1512 = !DILocation(line: 217, column: 39, scope: !1399, inlinedAt: !1513)
!1513 = !DILocation(line: 1532, column: 18, scope: !1210, inlinedAt: !1371)
!1514 = !DILocation(line: 74, column: 35, scope: !1515, inlinedAt: !1513)
!1515 = !DILexicalBlockFile(scope: !1399, file: !650, discriminator: 0)
!1516 = !DILocation(line: 185, column: 17, scope: !1176)
!1517 = !DILocation(line: 186, column: 13, scope: !1137)
!1518 = distinct !DISubprogram(name: "current_memory<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h18f91f87e43f8decE", scope: !355, file: !825, line: 299, type: !1519, scopeLine: 299, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !1538, retainedNodes: !1539)
!1519 = !DISubroutineType(types: !1520)
!1520 = !{!1521, !908}
!1521 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)>", scope: !257, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !1522, templateParams: !23, identifier: "bdcf5293dd90fd37da7a41ea8eb8ae0c")
!1522 = !{!1523}
!1523 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1521, file: !2, size: 192, align: 64, elements: !1524, templateParams: !23, identifier: "c249e48c96cb92fc78e11206f6437250", discriminator: !1537)
!1524 = !{!1525, !1533}
!1525 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !1523, file: !2, baseType: !1526, size: 192, align: 64, extraData: i128 0)
!1526 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !1521, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !23, templateParams: !1527, identifier: "49092a51022d728f9f972167039a815")
!1527 = !{!1528}
!1528 = !DITemplateTypeParameter(name: "T", type: !1529)
!1529 = !DICompositeType(tag: DW_TAG_structure_type, name: "(core::ptr::non_null::NonNull<u8>, core::alloc::layout::Layout)", file: !2, size: 192, align: 64, elements: !1530, templateParams: !23, identifier: "22dd85d8a1f1e38e4c425d63184c7b87")
!1530 = !{!1531, !1532}
!1531 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1529, file: !2, baseType: !362, size: 64, align: 64)
!1532 = !DIDerivedType(tag: DW_TAG_member, name: "__1", scope: !1529, file: !2, baseType: !588, size: 128, align: 64, offset: 64)
!1533 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !1523, file: !2, baseType: !1534, size: 192, align: 64)
!1534 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !1521, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !1535, templateParams: !1527, identifier: "4bd3237001f447f5f5d642bb6b3c627e")
!1535 = !{!1536}
!1536 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1534, file: !2, baseType: !1529, size: 192, align: 64, flags: DIFlagPublic)
!1537 = !DIDerivedType(tag: DW_TAG_member, scope: !1521, file: !2, baseType: !45, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)
!1538 = !DISubprogram(name: "current_memory<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h18f91f87e43f8decE", scope: !355, file: !825, line: 299, type: !1519, scopeLine: 299, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!1539 = !{!1540, !1541, !1543, !1545}
!1540 = !DILocalVariable(name: "self", arg: 1, scope: !1518, file: !825, line: 299, type: !908)
!1541 = !DILocalVariable(name: "align", scope: !1542, file: !825, line: 309, type: !9, align: 8)
!1542 = distinct !DILexicalBlock(scope: !1518, file: !825, line: 309, column: 17)
!1543 = !DILocalVariable(name: "size", scope: !1544, file: !825, line: 310, type: !9, align: 8)
!1544 = distinct !DILexicalBlock(scope: !1542, file: !825, line: 310, column: 17)
!1545 = !DILocalVariable(name: "layout", scope: !1546, file: !825, line: 311, type: !588, align: 8)
!1546 = distinct !DILexicalBlock(scope: !1544, file: !825, line: 311, column: 17)
!1547 = !DILocation(line: 299, column: 23, scope: !1518)
!1548 = !DILocation(line: 300, column: 12, scope: !1518)
!1549 = !DILocation(line: 300, column: 25, scope: !1518)
!1550 = !DILocation(line: 300, column: 41, scope: !1518)
!1551 = !DILocation(line: 466, column: 5, scope: !1552, inlinedAt: !1558)
!1552 = distinct !DILexicalBlock(scope: !1554, file: !1553, line: 465, column: 1)
!1553 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/mem/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "ab6d66e759286ff434b3e279bd7267d7")
!1554 = distinct !DISubprogram(name: "align_of<u8>", linkageName: "_ZN4core3mem8align_of17h9ae4f2aeb6edf7deE", scope: !1555, file: !1553, line: 465, type: !1556, scopeLine: 465, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285)
!1555 = !DINamespace(name: "mem", scope: !35)
!1556 = !DISubroutineType(types: !1557)
!1557 = !{!9}
!1558 = !DILocation(line: 309, column: 29, scope: !1518)
!1559 = !DILocation(line: 309, column: 21, scope: !1542)
!1560 = !DILocalVariable(name: "align", arg: 2, scope: !1561, file: !573, line: 120, type: !9)
!1561 = distinct !DILexicalBlock(scope: !1562, file: !573, line: 120, column: 5)
!1562 = distinct !DISubprogram(name: "from_size_align_unchecked", linkageName: "_ZN4core5alloc6layout6Layout25from_size_align_unchecked17ha924e1e1478b15b8E", scope: !588, file: !573, line: 120, type: !668, scopeLine: 120, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !670, retainedNodes: !1563)
!1563 = !{!1564, !1560}
!1564 = !DILocalVariable(name: "size", arg: 1, scope: !1561, file: !573, line: 120, type: !9)
!1565 = !DILocation(line: 120, column: 64, scope: !1561, inlinedAt: !1566)
!1566 = !DILocation(line: 311, column: 30, scope: !1544)
!1567 = !DILocalVariable(name: "align", arg: 1, scope: !1568, file: !635, line: 79, type: !9)
!1568 = distinct !DILexicalBlock(scope: !1569, file: !635, line: 79, column: 5)
!1569 = distinct !DISubprogram(name: "new_unchecked", linkageName: "_ZN4core3ptr9alignment9Alignment13new_unchecked17he62723cff9e75a02E", scope: !592, file: !635, line: 79, type: !680, scopeLine: 79, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !682, retainedNodes: !1570)
!1570 = !{!1567}
!1571 = !DILocation(line: 79, column: 39, scope: !1568, inlinedAt: !1572)
!1572 = !DILocation(line: 122, column: 40, scope: !1561, inlinedAt: !1566)
!1573 = !DILocation(line: 313, column: 5, scope: !1574, inlinedAt: !1576)
!1574 = distinct !DILexicalBlock(scope: !1575, file: !1553, line: 312, column: 1)
!1575 = distinct !DISubprogram(name: "size_of<u8>", linkageName: "_ZN4core3mem7size_of17hb212f52c3c07fdbeE", scope: !1555, file: !1553, line: 312, type: !1556, scopeLine: 312, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285)
!1576 = !DILocation(line: 310, column: 28, scope: !1542)
!1577 = !DILocalVariable(name: "self", arg: 1, scope: !1578, file: !613, line: 809, type: !9)
!1578 = distinct !DILexicalBlock(scope: !1579, file: !613, line: 809, column: 9)
!1579 = distinct !DISubprogram(name: "unchecked_mul", linkageName: "_ZN4core3num23_$LT$impl$u20$usize$GT$13unchecked_mul17h35d3b0db80b4cb81E", scope: !615, file: !613, line: 809, type: !617, scopeLine: 809, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1580)
!1580 = !{!1577, !1581}
!1581 = !DILocalVariable(name: "rhs", arg: 2, scope: !1578, file: !613, line: 809, type: !9)
!1582 = !DILocation(line: 809, column: 43, scope: !1578, inlinedAt: !1583)
!1583 = !DILocation(line: 310, column: 48, scope: !1542)
!1584 = !DILocation(line: 310, column: 62, scope: !1542)
!1585 = !DILocation(line: 809, column: 49, scope: !1578, inlinedAt: !1583)
!1586 = !DILocation(line: 74, column: 35, scope: !1587, inlinedAt: !1583)
!1587 = !DILexicalBlockFile(scope: !1578, file: !650, discriminator: 0)
!1588 = !DILocation(line: 301, column: 13, scope: !1518)
!1589 = !DILocation(line: 300, column: 9, scope: !1518)
!1590 = !DILocation(line: 821, column: 17, scope: !1578, inlinedAt: !1583)
!1591 = !DILocation(line: 310, column: 21, scope: !1544)
!1592 = !DILocation(line: 120, column: 51, scope: !1561, inlinedAt: !1566)
!1593 = !DILocation(line: 89, column: 18, scope: !1568, inlinedAt: !1572)
!1594 = !DILocation(line: 122, column: 18, scope: !1561, inlinedAt: !1566)
!1595 = !DILocation(line: 311, column: 21, scope: !1546)
!1596 = !DILocation(line: 312, column: 22, scope: !1546)
!1597 = !DILocation(line: 312, column: 23, scope: !1546)
!1598 = !DILocalVariable(name: "self", scope: !1599, file: !916, line: 145, type: !358, align: 8)
!1599 = distinct !DILexicalBlock(scope: !1600, file: !916, line: 145, column: 5)
!1600 = distinct !DISubprogram(name: "cast<u8, u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h0d7783a5573e7dcaE", scope: !358, file: !916, line: 145, type: !1601, scopeLine: 145, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1604, declaration: !1603, retainedNodes: !1606)
!1601 = !DISubroutineType(types: !1602)
!1602 = !{!358, !358}
!1603 = !DISubprogram(name: "cast<u8, u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$4cast17h0d7783a5573e7dcaE", scope: !358, file: !916, line: 145, type: !1601, scopeLine: 145, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1604)
!1604 = !{!286, !1605}
!1605 = !DITemplateTypeParameter(name: "U", type: !36)
!1606 = !{!1598, !1598}
!1607 = !DILocation(line: 145, column: 26, scope: !1599, inlinedAt: !1608)
!1608 = !DILocation(line: 312, column: 32, scope: !1546)
!1609 = !DILocalVariable(name: "self", arg: 1, scope: !1610, file: !727, line: 474, type: !362)
!1610 = distinct !DILexicalBlock(scope: !1611, file: !727, line: 474, column: 5)
!1611 = distinct !DISubprogram(name: "cast<u8, u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h09403a48de658a62E", scope: !362, file: !727, line: 474, type: !1612, scopeLine: 474, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1604, declaration: !1614, retainedNodes: !1615)
!1612 = !DISubroutineType(types: !1613)
!1613 = !{!362, !362}
!1614 = !DISubprogram(name: "cast<u8, u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h09403a48de658a62E", scope: !362, file: !727, line: 474, type: !1612, scopeLine: 474, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1604)
!1615 = !{!1609}
!1616 = !DILocation(line: 474, column: 26, scope: !1610, inlinedAt: !1617)
!1617 = !DILocation(line: 148, column: 40, scope: !1599, inlinedAt: !1608)
!1618 = !DILocalVariable(name: "self", arg: 1, scope: !1619, file: !727, line: 350, type: !362)
!1619 = distinct !DILexicalBlock(scope: !1620, file: !727, line: 350, column: 5)
!1620 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hb4ed4f907fea3a81E", scope: !362, file: !727, line: 350, type: !927, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !929, retainedNodes: !1621)
!1621 = !{!1618}
!1622 = !DILocation(line: 350, column: 25, scope: !1619, inlinedAt: !1623)
!1623 = !DILocation(line: 476, column: 42, scope: !1610, inlinedAt: !1617)
!1624 = !DILocation(line: 476, column: 18, scope: !1610, inlinedAt: !1617)
!1625 = !DILocalVariable(name: "self", scope: !1626, file: !1337, line: 758, type: !358, align: 8)
!1626 = distinct !DILexicalBlock(scope: !1627, file: !1337, line: 758, column: 5)
!1627 = distinct !DISubprogram(name: "into<core::ptr::unique::Unique<u8>, core::ptr::non_null::NonNull<u8>>", linkageName: "_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h5415b031afcd7a4dE", scope: !1628, file: !1337, line: 758, type: !1629, scopeLine: 758, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1632, retainedNodes: !1631)
!1628 = !DINamespace(name: "{impl#3}", scope: !1188)
!1629 = !DISubroutineType(types: !1630)
!1630 = !{!362, !358, !1319}
!1631 = !{!1625, !1625}
!1632 = !{!1633, !1634}
!1633 = !DITemplateTypeParameter(name: "T", type: !358)
!1634 = !DITemplateTypeParameter(name: "U", type: !362)
!1635 = !DILocation(line: 758, column: 13, scope: !1626, inlinedAt: !1636)
!1636 = !DILocation(line: 312, column: 39, scope: !1546)
!1637 = !DILocalVariable(name: "unique", scope: !1638, file: !727, line: 1829, type: !358, align: 8)
!1638 = distinct !DILexicalBlock(scope: !1639, file: !727, line: 1829, column: 5)
!1639 = distinct !DISubprogram(name: "from<u8>", linkageName: "_ZN119_$LT$core..ptr..non_null..NonNull$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..unique..Unique$LT$T$GT$$GT$$GT$4from17hb201a049b4c10aceE", scope: !1640, file: !727, line: 1829, type: !1641, scopeLine: 1829, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1643)
!1640 = !DINamespace(name: "{impl#16}", scope: !363)
!1641 = !DISubroutineType(types: !1642)
!1642 = !{!362, !358}
!1643 = !{!1637, !1637}
!1644 = !DILocation(line: 1829, column: 13, scope: !1638, inlinedAt: !1645)
!1645 = !DILocation(line: 759, column: 9, scope: !1626, inlinedAt: !1636)
!1646 = !DILocalVariable(name: "self", scope: !1647, file: !916, line: 112, type: !358, align: 8)
!1647 = distinct !DILexicalBlock(scope: !1648, file: !916, line: 112, column: 5)
!1648 = distinct !DISubprogram(name: "as_non_null_ptr<u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$15as_non_null_ptr17hd8e7811539129077E", scope: !358, file: !916, line: 112, type: !1641, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1649, retainedNodes: !1650)
!1649 = !DISubprogram(name: "as_non_null_ptr<u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$15as_non_null_ptr17hd8e7811539129077E", scope: !358, file: !916, line: 112, type: !1641, scopeLine: 112, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1650 = !{!1646, !1646}
!1651 = !DILocation(line: 112, column: 34, scope: !1647, inlinedAt: !1652)
!1652 = !DILocation(line: 1830, column: 16, scope: !1638, inlinedAt: !1645)
!1653 = !DILocation(line: 312, column: 17, scope: !1546)
!1654 = !DILocation(line: 312, column: 54, scope: !1546)
!1655 = !DILocation(line: 315, column: 6, scope: !1518)
!1656 = distinct !DISubprogram(name: "try_allocate_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h7f310d67781440f8E", scope: !355, file: !825, line: 208, type: !1657, scopeLine: 208, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !1674, retainedNodes: !1675)
!1657 = !DISubroutineType(types: !1658)
!1658 = !{!1659, !9, !111, !374}
!1659 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<alloc::raw_vec::RawVec<u8, alloc::alloc::Global>, alloc::collections::TryReserveError>", scope: !225, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !1660, templateParams: !23, identifier: "61c6be4b51f5963f8f91d4e5f340f322")
!1660 = !{!1661}
!1661 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1659, file: !2, size: 192, align: 64, elements: !1662, templateParams: !23, identifier: "c491b569586a8cd32888f0ca0c79dc5e", discriminator: !1673)
!1662 = !{!1663, !1669}
!1663 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1661, file: !2, baseType: !1664, size: 192, align: 64, extraData: i128 0)
!1664 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1659, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !1665, templateParams: !1667, identifier: "f205c23d1db365172d515c1620842c06")
!1665 = !{!1666}
!1666 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1664, file: !2, baseType: !355, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
!1667 = !{!562, !1668}
!1668 = !DITemplateTypeParameter(name: "E", type: !836)
!1669 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1661, file: !2, baseType: !1670, size: 192, align: 64, extraData: i128 1)
!1670 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1659, file: !2, size: 192, align: 64, flags: DIFlagPublic, elements: !1671, templateParams: !1667, identifier: "f87cbb52b0d2dd9c6934d67bc8104453")
!1671 = !{!1672}
!1672 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1670, file: !2, baseType: !836, size: 128, align: 64, offset: 64, flags: DIFlagPublic)
!1673 = !DIDerivedType(tag: DW_TAG_member, scope: !1659, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!1674 = !DISubprogram(name: "try_allocate_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$15try_allocate_in17h7f310d67781440f8E", scope: !355, file: !825, line: 208, type: !1657, scopeLine: 208, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!1675 = !{!1676, !1677, !1678, !1679, !1681, !1683, !1685, !1687, !1689}
!1676 = !DILocalVariable(name: "capacity", arg: 1, scope: !1656, file: !825, line: 209, type: !9)
!1677 = !DILocalVariable(name: "init", arg: 2, scope: !1656, file: !825, line: 210, type: !111)
!1678 = !DILocalVariable(name: "alloc", arg: 3, scope: !1656, file: !825, line: 211, type: !374)
!1679 = !DILocalVariable(name: "layout", scope: !1680, file: !825, line: 220, type: !588, align: 8)
!1680 = distinct !DILexicalBlock(scope: !1656, file: !825, line: 220, column: 13)
!1681 = !DILocalVariable(name: "layout", scope: !1682, file: !825, line: 221, type: !588, align: 8)
!1682 = distinct !DILexicalBlock(scope: !1656, file: !825, line: 221, column: 17)
!1683 = !DILocalVariable(name: "err", scope: !1684, file: !825, line: 225, type: !836, align: 8)
!1684 = distinct !DILexicalBlock(scope: !1680, file: !825, line: 225, column: 58)
!1685 = !DILocalVariable(name: "result", scope: !1686, file: !825, line: 229, type: !1140, align: 8)
!1686 = distinct !DILexicalBlock(scope: !1680, file: !825, line: 229, column: 13)
!1687 = !DILocalVariable(name: "ptr", scope: !1688, file: !825, line: 234, type: !1148, align: 8)
!1688 = distinct !DILexicalBlock(scope: !1686, file: !825, line: 234, column: 13)
!1689 = !DILocalVariable(name: "ptr", scope: !1690, file: !825, line: 235, type: !1148, align: 8)
!1690 = distinct !DILexicalBlock(scope: !1686, file: !825, line: 235, column: 17)
!1691 = !DILocation(line: 225, column: 24, scope: !1684)
!1692 = !DILocation(line: 758, column: 13, scope: !1693, inlinedAt: !1706)
!1693 = distinct !DILexicalBlock(scope: !1694, file: !1337, line: 758, column: 5)
!1694 = distinct !DISubprogram(name: "into<alloc::collections::TryReserveErrorKind, alloc::collections::TryReserveError>", linkageName: "_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h83a583a6cdd7ec96E", scope: !1628, file: !1337, line: 758, type: !1695, scopeLine: 758, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1703, retainedNodes: !1697)
!1695 = !DISubroutineType(types: !1696)
!1696 = !{!836, !840, !1319}
!1697 = !{!1698, !1699, !1701}
!1698 = !DILocalVariable(name: "self", scope: !1693, file: !1337, line: 758, type: !840, align: 8)
!1699 = !DILocalVariable(name: "self", scope: !1700, file: !1337, line: 758, type: !840, align: 8)
!1700 = distinct !DILexicalBlock(scope: !1694, file: !1337, line: 758, column: 5)
!1701 = !DILocalVariable(name: "self", arg: 1, scope: !1702, file: !1337, line: 758, type: !840)
!1702 = distinct !DILexicalBlock(scope: !1694, file: !1337, line: 758, column: 5)
!1703 = !{!1704, !1705}
!1704 = !DITemplateTypeParameter(name: "T", type: !840)
!1705 = !DITemplateTypeParameter(name: "U", type: !836)
!1706 = !DILocation(line: 222, column: 55, scope: !1656)
!1707 = !DILocation(line: 116, column: 13, scope: !1708, inlinedAt: !1720)
!1708 = distinct !DILexicalBlock(scope: !1710, file: !1709, line: 116, column: 5)
!1709 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/collections/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "f3287cbb677657477a1b08a55aebae42")
!1710 = distinct !DISubprogram(name: "from", linkageName: "_ZN122_$LT$alloc..collections..TryReserveError$u20$as$u20$core..convert..From$LT$alloc..collections..TryReserveErrorKind$GT$$GT$4from17hadf95da4d8b945eeE", scope: !1711, file: !1709, line: 116, type: !1712, scopeLine: 116, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1714)
!1711 = !DINamespace(name: "{impl#1}", scope: !837)
!1712 = !DISubroutineType(types: !1713)
!1713 = !{!836, !840}
!1714 = !{!1715, !1716, !1718}
!1715 = !DILocalVariable(name: "kind", scope: !1708, file: !1709, line: 116, type: !840, align: 8)
!1716 = !DILocalVariable(name: "kind", scope: !1717, file: !1709, line: 116, type: !840, align: 8)
!1717 = distinct !DILexicalBlock(scope: !1710, file: !1709, line: 116, column: 5)
!1718 = !DILocalVariable(name: "kind", arg: 1, scope: !1719, file: !1709, line: 116, type: !840)
!1719 = distinct !DILexicalBlock(scope: !1710, file: !1709, line: 116, column: 5)
!1720 = !DILocation(line: 759, column: 9, scope: !1693, inlinedAt: !1706)
!1721 = !DILocation(line: 758, column: 13, scope: !1700, inlinedAt: !1722)
!1722 = !DILocation(line: 609, column: 30, scope: !1723, inlinedAt: !1743)
!1723 = distinct !DILexicalBlock(scope: !1724, file: !825, line: 607, column: 1)
!1724 = distinct !DISubprogram(name: "alloc_guard", linkageName: "_ZN5alloc7raw_vec11alloc_guard17haa11e7053b983806E", scope: !112, file: !825, line: 607, type: !1725, scopeLine: 607, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1741)
!1725 = !DISubroutineType(types: !1726)
!1726 = !{!1727, !9}
!1727 = !DICompositeType(tag: DW_TAG_structure_type, name: "Result<(), alloc::collections::TryReserveError>", scope: !225, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1728, templateParams: !23, identifier: "619256fbf92bfce8f7ece0514ed4b8b7")
!1728 = !{!1729}
!1729 = !DICompositeType(tag: DW_TAG_variant_part, scope: !1727, file: !2, size: 128, align: 64, elements: !1730, templateParams: !23, identifier: "68131beb3f982ae718eccfcc90d6830d", discriminator: !1740)
!1730 = !{!1731, !1736}
!1731 = !DIDerivedType(tag: DW_TAG_member, name: "Ok", scope: !1729, file: !2, baseType: !1732, size: 128, align: 64, extraData: i128 9223372036854775809)
!1732 = !DICompositeType(tag: DW_TAG_structure_type, name: "Ok", scope: !1727, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1733, templateParams: !1735, identifier: "e0445ec482e0975275aaa69880e1f78f")
!1733 = !{!1734}
!1734 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1732, file: !2, baseType: !7, align: 8, flags: DIFlagPublic)
!1735 = !{!130, !1668}
!1736 = !DIDerivedType(tag: DW_TAG_member, name: "Err", scope: !1729, file: !2, baseType: !1737, size: 128, align: 64)
!1737 = !DICompositeType(tag: DW_TAG_structure_type, name: "Err", scope: !1727, file: !2, size: 128, align: 64, flags: DIFlagPublic, elements: !1738, templateParams: !1735, identifier: "f8f30e22ee594a3c01848b000a9ffa4")
!1738 = !{!1739}
!1739 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !1737, file: !2, baseType: !836, size: 128, align: 64, flags: DIFlagPublic)
!1740 = !DIDerivedType(tag: DW_TAG_member, scope: !1727, file: !2, baseType: !45, size: 64, align: 64, flags: DIFlagArtificial)
!1741 = !{!1742}
!1742 = !DILocalVariable(name: "alloc_size", arg: 1, scope: !1723, file: !825, line: 607, type: !9)
!1743 = !DILocation(line: 225, column: 31, scope: !1684)
!1744 = !DILocation(line: 116, column: 13, scope: !1717, inlinedAt: !1745)
!1745 = !DILocation(line: 759, column: 9, scope: !1700, inlinedAt: !1722)
!1746 = !DILocation(line: 209, column: 9, scope: !1656)
!1747 = !DILocalVariable(name: "n", arg: 1, scope: !1748, file: !573, line: 435, type: !9)
!1748 = distinct !DILexicalBlock(scope: !1749, file: !573, line: 435, column: 5)
!1749 = distinct !DISubprogram(name: "array<u8>", linkageName: "_ZN4core5alloc6layout6Layout5array17hef0f06cf6bf460f0E", scope: !588, file: !573, line: 435, type: !1750, scopeLine: 435, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1752, retainedNodes: !1753)
!1750 = !DISubroutineType(types: !1751)
!1751 = !{!580, !9}
!1752 = !DISubprogram(name: "array<u8>", linkageName: "_ZN4core5alloc6layout6Layout5array17hef0f06cf6bf460f0E", scope: !588, file: !573, line: 435, type: !1750, scopeLine: 435, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1753 = !{!1747}
!1754 = !DILocation(line: 435, column: 27, scope: !1748, inlinedAt: !1755)
!1755 = !DILocation(line: 220, column: 32, scope: !1656)
!1756 = !DILocation(line: 210, column: 9, scope: !1656)
!1757 = !DILocation(line: 211, column: 9, scope: !1656)
!1758 = !DILocation(line: 220, column: 17, scope: !1680)
!1759 = !DILocation(line: 229, column: 17, scope: !1686)
!1760 = !DILocation(line: 758, column: 13, scope: !1702, inlinedAt: !1761)
!1761 = !DILocation(line: 236, column: 80, scope: !1686)
!1762 = !DILocation(line: 116, column: 13, scope: !1719, inlinedAt: !1763)
!1763 = !DILocation(line: 759, column: 9, scope: !1702, inlinedAt: !1761)
!1764 = !DILocation(line: 215, column: 12, scope: !1656)
!1765 = !DILocation(line: 215, column: 25, scope: !1656)
!1766 = !DILocation(line: 216, column: 29, scope: !1656)
!1767 = !DILocation(line: 216, column: 16, scope: !1656)
!1768 = !DILocation(line: 220, column: 17, scope: !1656)
!1769 = !DILocation(line: 466, column: 5, scope: !1770, inlinedAt: !1772)
!1770 = distinct !DILexicalBlock(scope: !1771, file: !1553, line: 465, column: 1)
!1771 = distinct !DISubprogram(name: "align_of<u8>", linkageName: "_ZN4core3mem8align_of17h9ae4f2aeb6edf7deE", scope: !1555, file: !1553, line: 465, type: !1556, scopeLine: 465, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285)
!1772 = !DILocation(line: 49, column: 43, scope: !1773, inlinedAt: !1778)
!1773 = distinct !DILexicalBlock(scope: !1774, file: !635, line: 47, column: 5)
!1774 = distinct !DISubprogram(name: "of<u8>", linkageName: "_ZN4core3ptr9alignment9Alignment2of17h3af2a49b96f1cee7E", scope: !592, file: !635, line: 47, type: !1775, scopeLine: 47, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1777)
!1775 = !DISubroutineType(types: !1776)
!1776 = !{!592}
!1777 = !DISubprogram(name: "of<u8>", linkageName: "_ZN4core3ptr9alignment9Alignment2of17h3af2a49b96f1cee7E", scope: !592, file: !635, line: 47, type: !1775, scopeLine: 47, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1778 = !DILocation(line: 437, column: 43, scope: !1748, inlinedAt: !1755)
!1779 = !DILocalVariable(name: "align", arg: 1, scope: !1780, file: !635, line: 79, type: !9)
!1780 = distinct !DILexicalBlock(scope: !1781, file: !635, line: 79, column: 5)
!1781 = distinct !DISubprogram(name: "new_unchecked", linkageName: "_ZN4core3ptr9alignment9Alignment13new_unchecked17he62723cff9e75a02E", scope: !592, file: !635, line: 79, type: !680, scopeLine: 79, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !682, retainedNodes: !1782)
!1782 = !{!1779}
!1783 = !DILocation(line: 79, column: 39, scope: !1780, inlinedAt: !1784)
!1784 = !DILocation(line: 49, column: 18, scope: !1773, inlinedAt: !1778)
!1785 = !DILocation(line: 89, column: 18, scope: !1780, inlinedAt: !1784)
!1786 = !DILocation(line: 437, column: 16, scope: !1748, inlinedAt: !1755)
!1787 = !DILocation(line: 244, column: 5, scope: !1656)
!1788 = !DILocation(line: 220, column: 26, scope: !1656)
!1789 = !DILocation(line: 221, column: 20, scope: !1656)
!1790 = !DILocation(line: 221, column: 20, scope: !1682)
!1791 = !DILocation(line: 221, column: 31, scope: !1682)
!1792 = !DILocation(line: 223, column: 14, scope: !1656)
!1793 = !DILocation(line: 225, column: 43, scope: !1684)
!1794 = !DILocalVariable(name: "self", arg: 1, scope: !1795, file: !573, line: 130, type: !692)
!1795 = distinct !DILexicalBlock(scope: !1796, file: !573, line: 130, column: 5)
!1796 = distinct !DISubprogram(name: "size", linkageName: "_ZN4core5alloc6layout6Layout4size17h94d72322f31a62e0E", scope: !588, file: !573, line: 130, type: !700, scopeLine: 130, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !1114, retainedNodes: !1797)
!1797 = !{!1794}
!1798 = !DILocation(line: 130, column: 23, scope: !1795, inlinedAt: !1799)
!1799 = !DILocation(line: 225, column: 50, scope: !1684)
!1800 = !DILocation(line: 131, column: 9, scope: !1795, inlinedAt: !1799)
!1801 = !DILocation(line: 607, column: 16, scope: !1723, inlinedAt: !1743)
!1802 = !DILocation(line: 229, column: 17, scope: !1680)
!1803 = !DILocation(line: 229, column: 32, scope: !1680)
!1804 = !DILocation(line: 229, column: 26, scope: !1680)
!1805 = !DILocation(line: 222, column: 34, scope: !1656)
!1806 = !DILocation(line: 1, column: 1, scope: !1807)
!1807 = !DILexicalBlockFile(scope: !1656, file: !659, discriminator: 0)
!1808 = !DILocation(line: 230, column: 45, scope: !1680)
!1809 = !DILocation(line: 232, column: 38, scope: !1680)
!1810 = !DILocation(line: 230, column: 66, scope: !1680)
!1811 = !DILocation(line: 234, column: 29, scope: !1686)
!1812 = !DILocation(line: 234, column: 23, scope: !1686)
!1813 = !DILocation(line: 232, column: 66, scope: !1680)
!1814 = !DILocation(line: 235, column: 20, scope: !1686)
!1815 = !DILocation(line: 234, column: 17, scope: !1688)
!1816 = !DILocation(line: 235, column: 20, scope: !1690)
!1817 = !DILocalVariable(name: "self", arg: 1, scope: !1818, file: !727, line: 474, type: !1148)
!1818 = distinct !DILexicalBlock(scope: !1819, file: !727, line: 474, column: 5)
!1819 = distinct !DISubprogram(name: "cast<[u8], u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h85db73e84868d5cfE", scope: !1148, file: !727, line: 474, type: !1820, scopeLine: 474, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1604, declaration: !1822, retainedNodes: !1823)
!1820 = !DISubroutineType(types: !1821)
!1821 = !{!362, !1148}
!1822 = !DISubprogram(name: "cast<[u8], u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$4cast17h85db73e84868d5cfE", scope: !1148, file: !727, line: 474, type: !1820, scopeLine: 474, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !1604)
!1823 = !{!1817}
!1824 = !DILocation(line: 474, column: 26, scope: !1818, inlinedAt: !1825)
!1825 = !DILocation(line: 242, column: 45, scope: !1688)
!1826 = !DILocalVariable(name: "self", arg: 1, scope: !1827, file: !727, line: 350, type: !1148)
!1827 = distinct !DILexicalBlock(scope: !1828, file: !727, line: 350, column: 5)
!1828 = distinct !DISubprogram(name: "as_ptr<[u8]>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h4d5bab3d86d220adE", scope: !1148, file: !727, line: 350, type: !1829, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1831, retainedNodes: !1832)
!1829 = !DISubroutineType(types: !1830)
!1830 = !{!1218, !1148}
!1831 = !DISubprogram(name: "as_ptr<[u8]>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17h4d5bab3d86d220adE", scope: !1148, file: !727, line: 350, type: !1829, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1832 = !{!1826}
!1833 = !DILocation(line: 350, column: 25, scope: !1827, inlinedAt: !1834)
!1834 = !DILocation(line: 476, column: 42, scope: !1818, inlinedAt: !1825)
!1835 = !DILocation(line: 476, column: 18, scope: !1818, inlinedAt: !1825)
!1836 = !DILocalVariable(name: "pointer", arg: 1, scope: !1837, file: !916, line: 200, type: !362)
!1837 = distinct !DILexicalBlock(scope: !1838, file: !916, line: 200, column: 5)
!1838 = distinct !DISubprogram(name: "from<u8>", linkageName: "_ZN119_$LT$core..ptr..unique..Unique$LT$T$GT$$u20$as$u20$core..convert..From$LT$core..ptr..non_null..NonNull$LT$T$GT$$GT$$GT$4from17h592ace739987fb0dE", scope: !1839, file: !916, line: 200, type: !1840, scopeLine: 200, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1842)
!1839 = !DINamespace(name: "{impl#11}", scope: !359)
!1840 = !DISubroutineType(types: !1841)
!1841 = !{!358, !362}
!1842 = !{!1836}
!1843 = !DILocation(line: 200, column: 13, scope: !1837, inlinedAt: !1844)
!1844 = !DILocation(line: 242, column: 28, scope: !1688)
!1845 = !DILocation(line: 242, column: 13, scope: !1688)
!1846 = !DILocation(line: 243, column: 9, scope: !1680)
!1847 = !DILocation(line: 243, column: 9, scope: !1656)
!1848 = !DILocation(line: 215, column: 9, scope: !1656)
!1849 = !DILocation(line: 236, column: 38, scope: !1686)
!1850 = !DILocation(line: 117, column: 9, scope: !1719, inlinedAt: !1763)
!1851 = !DILocation(line: 236, column: 85, scope: !1686)
!1852 = !DILocation(line: 236, column: 34, scope: !1686)
!1853 = !DILocation(line: 1, column: 1, scope: !1854)
!1854 = !DILexicalBlockFile(scope: !1680, file: !659, discriminator: 0)
!1855 = !DILocation(line: 244, column: 6, scope: !1656)
!1856 = !DILocation(line: 216, column: 13, scope: !1656)
!1857 = !DILocation(line: 208, column: 5, scope: !1656)
!1858 = distinct !DISubprogram(name: "new_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h42ecedc66878b095E", scope: !355, file: !825, line: 148, type: !1859, scopeLine: 148, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !1861, retainedNodes: !1862)
!1859 = !DISubroutineType(types: !1860)
!1860 = !{!355, !374}
!1861 = !DISubprogram(name: "new_in<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$6new_in17h42ecedc66878b095E", scope: !355, file: !825, line: 148, type: !1859, scopeLine: 148, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!1862 = !{!1863}
!1863 = !DILocalVariable(name: "alloc", arg: 1, scope: !1858, file: !825, line: 148, type: !374)
!1864 = !DILocation(line: 148, column: 25, scope: !1858)
!1865 = !DILocation(line: 466, column: 5, scope: !1866, inlinedAt: !1868)
!1866 = distinct !DILexicalBlock(scope: !1867, file: !1553, line: 465, column: 1)
!1867 = distinct !DISubprogram(name: "align_of<u8>", linkageName: "_ZN4core3mem8align_of17h9ae4f2aeb6edf7deE", scope: !1555, file: !1553, line: 465, type: !1556, scopeLine: 465, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285)
!1868 = !DILocation(line: 687, column: 28, scope: !1869, inlinedAt: !1873)
!1869 = distinct !DILexicalBlock(scope: !1870, file: !539, line: 686, column: 1)
!1870 = distinct !DISubprogram(name: "dangling_mut<u8>", linkageName: "_ZN4core3ptr12dangling_mut17ha01ec2e7227f6fb6E", scope: !44, file: !539, line: 686, type: !1871, scopeLine: 686, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285)
!1871 = !DISubroutineType(types: !1872)
!1872 = !{!719}
!1873 = !DILocation(line: 116, column: 23, scope: !1874, inlinedAt: !1882)
!1874 = distinct !DILexicalBlock(scope: !1875, file: !727, line: 111, column: 5)
!1875 = distinct !DISubprogram(name: "dangling<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$8dangling17ha74b166826e6df90E", scope: !362, file: !727, line: 111, type: !1876, scopeLine: 111, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1878, retainedNodes: !1879)
!1876 = !DISubroutineType(types: !1877)
!1877 = !{!362}
!1878 = !DISubprogram(name: "dangling<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$8dangling17ha74b166826e6df90E", scope: !362, file: !727, line: 111, type: !1876, scopeLine: 111, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1879 = !{!1880}
!1880 = !DILocalVariable(name: "ptr", scope: !1881, file: !727, line: 116, type: !719, align: 8)
!1881 = distinct !DILexicalBlock(scope: !1874, file: !727, line: 116, column: 13)
!1882 = !DILocation(line: 75, column: 27, scope: !1883, inlinedAt: !1888)
!1883 = distinct !DILexicalBlock(scope: !1884, file: !916, line: 73, column: 5)
!1884 = distinct !DISubprogram(name: "dangling<u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$8dangling17h6a0843bd2a3ef31cE", scope: !358, file: !916, line: 73, type: !1885, scopeLine: 73, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !1887)
!1885 = !DISubroutineType(types: !1886)
!1886 = !{!358}
!1887 = !DISubprogram(name: "dangling<u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$8dangling17h6a0843bd2a3ef31cE", scope: !358, file: !916, line: 73, type: !1885, scopeLine: 73, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !285)
!1888 = !DILocation(line: 150, column: 21, scope: !1858)
!1889 = !DILocalVariable(name: "addr", arg: 1, scope: !1890, file: !539, line: 664, type: !9)
!1890 = distinct !DILexicalBlock(scope: !1891, file: !539, line: 664, column: 1)
!1891 = distinct !DISubprogram(name: "without_provenance_mut<u8>", linkageName: "_ZN4core3ptr22without_provenance_mut17hcad31b4679884145E", scope: !44, file: !539, line: 664, type: !717, scopeLine: 664, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1892)
!1892 = !{!1889}
!1893 = !DILocation(line: 664, column: 40, scope: !1890, inlinedAt: !1894)
!1894 = !DILocation(line: 687, column: 5, scope: !1869, inlinedAt: !1873)
!1895 = !DILocation(line: 670, column: 14, scope: !1890, inlinedAt: !1894)
!1896 = !DILocation(line: 116, column: 17, scope: !1881, inlinedAt: !1882)
!1897 = !DILocalVariable(name: "ptr", arg: 1, scope: !1898, file: !727, line: 217, type: !719)
!1898 = distinct !DILexicalBlock(scope: !1899, file: !727, line: 217, column: 5)
!1899 = distinct !DISubprogram(name: "new_unchecked<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked17h9c3f888b72d32029E", scope: !362, file: !727, line: 217, type: !729, scopeLine: 217, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !731, retainedNodes: !1900)
!1900 = !{!1897}
!1901 = !DILocation(line: 217, column: 39, scope: !1898, inlinedAt: !1902)
!1902 = !DILocation(line: 117, column: 13, scope: !1881, inlinedAt: !1882)
!1903 = !DILocation(line: 74, column: 35, scope: !1904, inlinedAt: !1902)
!1904 = !DILexicalBlockFile(scope: !1898, file: !650, discriminator: 0)
!1905 = !DILocation(line: 151, column: 6, scope: !1858)
!1906 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h3a517c0b8c90cbfeE", scope: !1907, file: !1074, line: 2372, type: !391, scopeLine: 2372, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1908)
!1907 = !DINamespace(name: "{impl#21}", scope: !348)
!1908 = !{!1909, !1910}
!1909 = !DILocalVariable(name: "self", arg: 1, scope: !1906, file: !1074, line: 2372, type: !346)
!1910 = !DILocalVariable(name: "f", arg: 2, scope: !1906, file: !1074, line: 2372, type: !247)
!1911 = !DILocation(line: 2372, column: 12, scope: !1906)
!1912 = !DILocalVariable(name: "self", arg: 1, scope: !1913, file: !1074, line: 2483, type: !346)
!1913 = distinct !DILexicalBlock(scope: !1914, file: !1074, line: 2483, column: 5)
!1914 = distinct !DISubprogram(name: "deref", linkageName: "_ZN65_$LT$alloc..string..String$u20$as$u20$core..ops..deref..Deref$GT$5deref17h3def1b3906eaa6d0E", scope: !1915, file: !1074, line: 2483, type: !1916, scopeLine: 2483, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !1918)
!1915 = !DINamespace(name: "{impl#28}", scope: !348)
!1916 = !DISubroutineType(types: !1917)
!1917 = !{!242, !346}
!1918 = !{!1912}
!1919 = !DILocation(line: 2483, column: 14, scope: !1913, inlinedAt: !1920)
!1920 = !DILocation(line: 2373, column: 28, scope: !1906)
!1921 = !DILocation(line: 2372, column: 19, scope: !1906)
!1922 = !DILocation(line: 2484, column: 43, scope: !1913, inlinedAt: !1920)
!1923 = !DILocalVariable(name: "self", arg: 1, scope: !1924, file: !814, line: 2814, type: !961)
!1924 = distinct !DILexicalBlock(scope: !1925, file: !814, line: 2814, column: 5)
!1925 = distinct !DISubprogram(name: "deref<u8, alloc::alloc::Global>", linkageName: "_ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h922ef768a54236a7E", scope: !1926, file: !814, line: 2814, type: !1927, scopeLine: 2814, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, retainedNodes: !1929)
!1926 = !DINamespace(name: "{impl#8}", scope: !352)
!1927 = !DISubroutineType(types: !1928)
!1928 = !{!790, !961}
!1929 = !{!1923}
!1930 = !DILocation(line: 2814, column: 14, scope: !1924, inlinedAt: !1922)
!1931 = !DILocalVariable(name: "self", arg: 1, scope: !1932, file: !814, line: 1329, type: !961)
!1932 = distinct !DILexicalBlock(scope: !1933, file: !814, line: 1329, column: 5)
!1933 = distinct !DISubprogram(name: "as_ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$6as_ptr17hd2c3301bad4068caE", scope: !351, file: !814, line: 1329, type: !1934, scopeLine: 1329, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !1936, retainedNodes: !1937)
!1934 = !DISubroutineType(types: !1935)
!1935 = !{!154, !961}
!1936 = !DISubprogram(name: "as_ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$6as_ptr17hd2c3301bad4068caE", scope: !351, file: !814, line: 1329, type: !1934, scopeLine: 1329, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagOptimized, templateParams: !376)
!1937 = !{!1931}
!1938 = !DILocation(line: 1329, column: 19, scope: !1932, inlinedAt: !1939)
!1939 = !DILocation(line: 2815, column: 45, scope: !1924, inlinedAt: !1922)
!1940 = !DILocation(line: 1332, column: 9, scope: !1932, inlinedAt: !1939)
!1941 = !DILocalVariable(name: "self", arg: 1, scope: !1942, file: !825, line: 277, type: !908)
!1942 = distinct !DILexicalBlock(scope: !1943, file: !825, line: 277, column: 5)
!1943 = distinct !DISubprogram(name: "ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17he44bf70be75db6eaE", scope: !355, file: !825, line: 277, type: !906, scopeLine: 277, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !909, retainedNodes: !1944)
!1944 = !{!1941}
!1945 = !DILocation(line: 277, column: 16, scope: !1942, inlinedAt: !1946)
!1946 = !DILocation(line: 1332, column: 18, scope: !1932, inlinedAt: !1939)
!1947 = !DILocation(line: 278, column: 9, scope: !1942, inlinedAt: !1946)
!1948 = !DILocalVariable(name: "self", scope: !1949, file: !916, line: 105, type: !358, align: 8)
!1949 = distinct !DILexicalBlock(scope: !1950, file: !916, line: 105, column: 5)
!1950 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h53c445a80d45c1caE", scope: !358, file: !916, line: 105, type: !918, scopeLine: 105, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !920, retainedNodes: !1951)
!1951 = !{!1948, !1948}
!1952 = !DILocation(line: 105, column: 25, scope: !1949, inlinedAt: !1953)
!1953 = !DILocation(line: 278, column: 18, scope: !1942, inlinedAt: !1946)
!1954 = !DILocalVariable(name: "self", arg: 1, scope: !1955, file: !727, line: 350, type: !362)
!1955 = distinct !DILexicalBlock(scope: !1956, file: !727, line: 350, column: 5)
!1956 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hb4ed4f907fea3a81E", scope: !362, file: !727, line: 350, type: !927, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !929, retainedNodes: !1957)
!1957 = !{!1954}
!1958 = !DILocation(line: 350, column: 25, scope: !1955, inlinedAt: !1959)
!1959 = !DILocation(line: 106, column: 22, scope: !1949, inlinedAt: !1953)
!1960 = !DILocation(line: 351, column: 9, scope: !1955, inlinedAt: !1959)
!1961 = !DILocalVariable(name: "data", arg: 1, scope: !1962, file: !1963, line: 92, type: !154)
!1962 = distinct !DILexicalBlock(scope: !1964, file: !1963, line: 92, column: 1)
!1963 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/slice/raw.rs", directory: "", checksumkind: CSK_MD5, checksum: "0a25ebd5cafa317a98587b76661f3322")
!1964 = distinct !DISubprogram(name: "from_raw_parts<u8>", linkageName: "_ZN4core5slice3raw14from_raw_parts17h46c42b2042fc701bE", scope: !1965, file: !1963, line: 92, type: !1966, scopeLine: 92, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1968)
!1965 = !DINamespace(name: "raw", scope: !805)
!1966 = !DISubroutineType(types: !1967)
!1967 = !{!790, !154, !9}
!1968 = !{!1961, !1969}
!1969 = !DILocalVariable(name: "len", arg: 2, scope: !1962, file: !1963, line: 92, type: !9)
!1970 = !DILocation(line: 92, column: 43, scope: !1962, inlinedAt: !1971)
!1971 = !DILocation(line: 2815, column: 18, scope: !1924, inlinedAt: !1922)
!1972 = !DILocalVariable(name: "data", arg: 1, scope: !1973, file: !539, line: 842, type: !154)
!1973 = distinct !DILexicalBlock(scope: !1974, file: !539, line: 842, column: 1)
!1974 = distinct !DISubprogram(name: "slice_from_raw_parts<u8>", linkageName: "_ZN4core3ptr20slice_from_raw_parts17hb44cb4f2b2560e9eE", scope: !44, file: !539, line: 842, type: !1975, scopeLine: 842, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !1977)
!1975 = !DISubroutineType(types: !1976)
!1976 = !{!1151, !154, !9}
!1977 = !{!1972, !1978}
!1978 = !DILocalVariable(name: "len", arg: 2, scope: !1973, file: !539, line: 842, type: !9)
!1979 = !DILocation(line: 842, column: 38, scope: !1973, inlinedAt: !1980)
!1980 = !DILocation(line: 107, column: 11, scope: !1962, inlinedAt: !1971)
!1981 = !DILocalVariable(name: "data_pointer", arg: 1, scope: !1982, file: !1231, line: 123, type: !154)
!1982 = distinct !DILexicalBlock(scope: !1983, file: !1231, line: 122, column: 1)
!1983 = distinct !DISubprogram(name: "from_raw_parts<[u8], u8>", linkageName: "_ZN4core3ptr8metadata14from_raw_parts17ha3c4d4089d95277fE", scope: !1233, file: !1231, line: 122, type: !1975, scopeLine: 122, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1240, retainedNodes: !1984)
!1984 = !{!1981, !1985}
!1985 = !DILocalVariable(name: "metadata", arg: 2, scope: !1982, file: !1231, line: 124, type: !9)
!1986 = !DILocation(line: 123, column: 5, scope: !1982, inlinedAt: !1987)
!1987 = !DILocation(line: 843, column: 5, scope: !1973, inlinedAt: !1980)
!1988 = !DILocation(line: 2815, column: 55, scope: !1924, inlinedAt: !1922)
!1989 = !DILocation(line: 92, column: 59, scope: !1962, inlinedAt: !1971)
!1990 = !DILocation(line: 842, column: 54, scope: !1973, inlinedAt: !1980)
!1991 = !DILocation(line: 124, column: 5, scope: !1982, inlinedAt: !1987)
!1992 = !DILocation(line: 74, column: 35, scope: !1993, inlinedAt: !1971)
!1993 = !DILexicalBlockFile(scope: !1962, file: !650, discriminator: 0)
!1994 = !DILocation(line: 126, column: 5, scope: !1982, inlinedAt: !1987)
!1995 = !DILocalVariable(name: "v", arg: 1, scope: !1996, file: !1997, line: 173, type: !1151)
!1996 = distinct !DILexicalBlock(scope: !1998, file: !1997, line: 173, column: 1)
!1997 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/core/src/str/converts.rs", directory: "", checksumkind: CSK_MD5, checksum: "a9ea0fa847a701a1ac223b59ce04fc8d")
!1998 = distinct !DISubprogram(name: "from_utf8_unchecked", linkageName: "_ZN4core3str8converts19from_utf8_unchecked17h75d4893e1656781aE", scope: !1999, file: !1997, line: 173, type: !2000, scopeLine: 173, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !2002)
!1999 = !DINamespace(name: "converts", scope: !1066)
!2000 = !DISubroutineType(types: !2001)
!2001 = !{!242, !790}
!2002 = !{!1995}
!2003 = !DILocation(line: 173, column: 41, scope: !1996, inlinedAt: !2004)
!2004 = !DILocation(line: 2484, column: 18, scope: !1913, inlinedAt: !1920)
!2005 = !DILocation(line: 2373, column: 9, scope: !1906)
!2006 = !DILocation(line: 2374, column: 6, scope: !1906)
!2007 = distinct !DISubprogram(name: "deallocate", linkageName: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h37134dd5886182ffE", scope: !2008, file: !1093, line: 252, type: !2009, scopeLine: 252, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !2011)
!2008 = !DINamespace(name: "{impl#1}", scope: !375)
!2009 = !DISubroutineType(types: !2010)
!2010 = !{null, !1164, !362, !588}
!2011 = !{!2012, !2013, !2014}
!2012 = !DILocalVariable(name: "self", arg: 1, scope: !2007, file: !1093, line: 252, type: !1164)
!2013 = !DILocalVariable(name: "ptr", arg: 2, scope: !2007, file: !1093, line: 252, type: !362)
!2014 = !DILocalVariable(name: "layout", arg: 3, scope: !2007, file: !1093, line: 252, type: !588)
!2015 = !DILocation(line: 252, column: 26, scope: !2007)
!2016 = !DILocation(line: 252, column: 33, scope: !2007)
!2017 = !DILocalVariable(name: "self", arg: 1, scope: !2018, file: !727, line: 350, type: !362)
!2018 = distinct !DILexicalBlock(scope: !2019, file: !727, line: 350, column: 5)
!2019 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hb4ed4f907fea3a81E", scope: !362, file: !727, line: 350, type: !927, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !929, retainedNodes: !2020)
!2020 = !{!2017}
!2021 = !DILocation(line: 350, column: 25, scope: !2018, inlinedAt: !2022)
!2022 = !DILocation(line: 256, column: 34, scope: !2007)
!2023 = !DILocation(line: 252, column: 51, scope: !2007)
!2024 = !DILocalVariable(name: "layout", arg: 2, scope: !2025, file: !1093, line: 118, type: !588)
!2025 = distinct !DILexicalBlock(scope: !2026, file: !1093, line: 118, column: 1)
!2026 = distinct !DISubprogram(name: "dealloc", linkageName: "_ZN5alloc5alloc7dealloc17h6385ad289a3c5e40E", scope: !375, file: !1093, line: 118, type: !2027, scopeLine: 118, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !2029)
!2027 = !DISubroutineType(types: !2028)
!2028 = !{null, !719, !588}
!2029 = !{!2030, !2024}
!2030 = !DILocalVariable(name: "ptr", arg: 1, scope: !2025, file: !1093, line: 118, type: !719)
!2031 = !DILocation(line: 118, column: 37, scope: !2025, inlinedAt: !2032)
!2032 = !DILocation(line: 256, column: 22, scope: !2007)
!2033 = !DILocation(line: 253, column: 12, scope: !2007)
!2034 = !DILocalVariable(name: "self", arg: 1, scope: !2035, file: !573, line: 130, type: !692)
!2035 = distinct !DILexicalBlock(scope: !2036, file: !573, line: 130, column: 5)
!2036 = distinct !DISubprogram(name: "size", linkageName: "_ZN4core5alloc6layout6Layout4size17h94d72322f31a62e0E", scope: !588, file: !573, line: 130, type: !700, scopeLine: 130, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !1114, retainedNodes: !2037)
!2037 = !{!2034, !2038}
!2038 = !DILocalVariable(name: "self", arg: 1, scope: !2039, file: !573, line: 130, type: !692)
!2039 = distinct !DILexicalBlock(scope: !2036, file: !573, line: 130, column: 5)
!2040 = !DILocation(line: 130, column: 23, scope: !2035, inlinedAt: !2041)
!2041 = !DILocation(line: 253, column: 19, scope: !2007)
!2042 = !DILocation(line: 131, column: 9, scope: !2035, inlinedAt: !2041)
!2043 = !DILocation(line: 258, column: 6, scope: !2007)
!2044 = !DILocation(line: 351, column: 9, scope: !2018, inlinedAt: !2022)
!2045 = !DILocation(line: 118, column: 23, scope: !2025, inlinedAt: !2032)
!2046 = !DILocation(line: 256, column: 44, scope: !2007)
!2047 = !DILocation(line: 119, column: 34, scope: !2025, inlinedAt: !2032)
!2048 = !DILocation(line: 130, column: 23, scope: !2039, inlinedAt: !2049)
!2049 = !DILocation(line: 119, column: 41, scope: !2025, inlinedAt: !2032)
!2050 = !DILocation(line: 119, column: 49, scope: !2025, inlinedAt: !2032)
!2051 = !DILocalVariable(name: "self", arg: 1, scope: !2052, file: !573, line: 143, type: !692)
!2052 = distinct !DILexicalBlock(scope: !2053, file: !573, line: 143, column: 5)
!2053 = distinct !DISubprogram(name: "align", linkageName: "_ZN4core5alloc6layout6Layout5align17hd70febe67b65a137E", scope: !588, file: !573, line: 143, type: !700, scopeLine: 143, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !702, retainedNodes: !2054)
!2054 = !{!2051}
!2055 = !DILocation(line: 143, column: 24, scope: !2052, inlinedAt: !2056)
!2056 = !DILocation(line: 119, column: 56, scope: !2025, inlinedAt: !2032)
!2057 = !DILocation(line: 144, column: 9, scope: !2052, inlinedAt: !2056)
!2058 = !DILocalVariable(name: "self", arg: 1, scope: !2059, file: !635, line: 96, type: !592)
!2059 = distinct !DILexicalBlock(scope: !2060, file: !635, line: 96, column: 5)
!2060 = distinct !DISubprogram(name: "as_usize", linkageName: "_ZN4core3ptr9alignment9Alignment8as_usize17h0458ba04f1c34212E", scope: !592, file: !635, line: 96, type: !627, scopeLine: 96, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, declaration: !637, retainedNodes: !2061)
!2061 = !{!2058}
!2062 = !DILocation(line: 96, column: 27, scope: !2059, inlinedAt: !2063)
!2063 = !DILocation(line: 144, column: 20, scope: !2052, inlinedAt: !2056)
!2064 = !DILocation(line: 97, column: 9, scope: !2059, inlinedAt: !2063)
!2065 = !DILocation(line: 97, column: 23, scope: !2059, inlinedAt: !2063)
!2066 = !DILocation(line: 119, column: 14, scope: !2025, inlinedAt: !2032)
!2067 = !DILocation(line: 256, column: 50, scope: !2007)
!2068 = !DILocation(line: 253, column: 9, scope: !2007)
!2069 = distinct !DISubprogram(name: "allocate_zeroed", linkageName: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$15allocate_zeroed17h51d97e6b86c1750aE", scope: !2008, file: !1093, line: 247, type: !2070, scopeLine: 247, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !2072)
!2070 = !DISubroutineType(types: !2071)
!2071 = !{!1140, !1164, !588}
!2072 = !{!2073, !2074}
!2073 = !DILocalVariable(name: "self", arg: 1, scope: !2069, file: !1093, line: 247, type: !1164)
!2074 = !DILocalVariable(name: "layout", arg: 2, scope: !2069, file: !1093, line: 247, type: !588)
!2075 = !DILocation(line: 247, column: 24, scope: !2069)
!2076 = !DILocation(line: 247, column: 31, scope: !2069)
!2077 = !DILocation(line: 248, column: 9, scope: !2069)
!2078 = !DILocation(line: 249, column: 6, scope: !2069)
!2079 = distinct !DISubprogram(name: "allocate", linkageName: "_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h6ca1b7a3462ca17fE", scope: !2008, file: !1093, line: 242, type: !2070, scopeLine: 242, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !23, retainedNodes: !2080)
!2080 = !{!2081, !2082}
!2081 = !DILocalVariable(name: "self", arg: 1, scope: !2079, file: !1093, line: 242, type: !1164)
!2082 = !DILocalVariable(name: "layout", arg: 2, scope: !2079, file: !1093, line: 242, type: !588)
!2083 = !DILocation(line: 242, column: 17, scope: !2079)
!2084 = !DILocation(line: 242, column: 24, scope: !2079)
!2085 = !DILocation(line: 243, column: 9, scope: !2079)
!2086 = !DILocation(line: 244, column: 6, scope: !2079)
!2087 = distinct !DISubprogram(name: "drop<u8, alloc::alloc::Global>", linkageName: "_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h29810ea43683eadfE", scope: !2088, file: !814, line: 3276, type: !2089, scopeLine: 3276, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, retainedNodes: !2091)
!2088 = !DINamespace(name: "{impl#25}", scope: !352)
!2089 = !DISubroutineType(types: !2090)
!2090 = !{null, !897}
!2091 = !{!2092}
!2092 = !DILocalVariable(name: "self", arg: 1, scope: !2087, file: !814, line: 3276, type: !897)
!2093 = !DILocation(line: 3276, column: 13, scope: !2087)
!2094 = !DILocalVariable(name: "self", arg: 1, scope: !2095, file: !814, line: 1389, type: !897)
!2095 = distinct !DILexicalBlock(scope: !2096, file: !814, line: 1389, column: 5)
!2096 = distinct !DISubprogram(name: "as_mut_ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc3vec16Vec$LT$T$C$A$GT$10as_mut_ptr17h477d382ab6ac9780E", scope: !351, file: !814, line: 1389, type: !895, scopeLine: 1389, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !898, retainedNodes: !2097)
!2097 = !{!2094}
!2098 = !DILocation(line: 1389, column: 23, scope: !2095, inlinedAt: !2099)
!2099 = !DILocation(line: 3281, column: 67, scope: !2087)
!2100 = !DILocation(line: 1392, column: 9, scope: !2095, inlinedAt: !2099)
!2101 = !DILocalVariable(name: "self", arg: 1, scope: !2102, file: !825, line: 277, type: !908)
!2102 = distinct !DILexicalBlock(scope: !2103, file: !825, line: 277, column: 5)
!2103 = distinct !DISubprogram(name: "ptr<u8, alloc::alloc::Global>", linkageName: "_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$3ptr17he44bf70be75db6eaE", scope: !355, file: !825, line: 277, type: !906, scopeLine: 277, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, declaration: !909, retainedNodes: !2104)
!2104 = !{!2101}
!2105 = !DILocation(line: 277, column: 16, scope: !2102, inlinedAt: !2106)
!2106 = !DILocation(line: 1392, column: 18, scope: !2095, inlinedAt: !2099)
!2107 = !DILocation(line: 278, column: 9, scope: !2102, inlinedAt: !2106)
!2108 = !DILocalVariable(name: "self", scope: !2109, file: !916, line: 105, type: !358, align: 8)
!2109 = distinct !DILexicalBlock(scope: !2110, file: !916, line: 105, column: 5)
!2110 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr6unique15Unique$LT$T$GT$6as_ptr17h53c445a80d45c1caE", scope: !358, file: !916, line: 105, type: !918, scopeLine: 105, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !920, retainedNodes: !2111)
!2111 = !{!2108, !2108}
!2112 = !DILocation(line: 105, column: 25, scope: !2109, inlinedAt: !2113)
!2113 = !DILocation(line: 278, column: 18, scope: !2102, inlinedAt: !2106)
!2114 = !DILocalVariable(name: "self", arg: 1, scope: !2115, file: !727, line: 350, type: !362)
!2115 = distinct !DILexicalBlock(scope: !2116, file: !727, line: 350, column: 5)
!2116 = distinct !DISubprogram(name: "as_ptr<u8>", linkageName: "_ZN4core3ptr8non_null16NonNull$LT$T$GT$6as_ptr17hb4ed4f907fea3a81E", scope: !362, file: !727, line: 350, type: !927, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, declaration: !929, retainedNodes: !2117)
!2117 = !{!2114}
!2118 = !DILocation(line: 350, column: 25, scope: !2115, inlinedAt: !2119)
!2119 = !DILocation(line: 106, column: 22, scope: !2109, inlinedAt: !2113)
!2120 = !DILocation(line: 351, column: 9, scope: !2115, inlinedAt: !2119)
!2121 = !DILocalVariable(name: "data", arg: 1, scope: !2122, file: !539, line: 888, type: !719)
!2122 = distinct !DILexicalBlock(scope: !2123, file: !539, line: 888, column: 1)
!2123 = distinct !DISubprogram(name: "slice_from_raw_parts_mut<u8>", linkageName: "_ZN4core3ptr24slice_from_raw_parts_mut17h67507d7975176d6fE", scope: !44, file: !539, line: 888, type: !1216, scopeLine: 888, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !285, retainedNodes: !2124)
!2124 = !{!2121, !2125}
!2125 = !DILocalVariable(name: "len", arg: 2, scope: !2122, file: !539, line: 888, type: !9)
!2126 = !DILocation(line: 888, column: 42, scope: !2122, inlinedAt: !2127)
!2127 = !DILocation(line: 3281, column: 32, scope: !2087)
!2128 = !DILocalVariable(name: "data_pointer", arg: 1, scope: !2129, file: !1231, line: 137, type: !719)
!2129 = distinct !DILexicalBlock(scope: !2130, file: !1231, line: 136, column: 1)
!2130 = distinct !DISubprogram(name: "from_raw_parts_mut<[u8], u8>", linkageName: "_ZN4core3ptr8metadata18from_raw_parts_mut17ha8d71fcd9909e534E", scope: !1233, file: !1231, line: 136, type: !1216, scopeLine: 136, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !1240, retainedNodes: !2131)
!2131 = !{!2128, !2132}
!2132 = !DILocalVariable(name: "metadata", arg: 2, scope: !2129, file: !1231, line: 138, type: !9)
!2133 = !DILocation(line: 137, column: 5, scope: !2129, inlinedAt: !2134)
!2134 = !DILocation(line: 889, column: 5, scope: !2122, inlinedAt: !2127)
!2135 = !DILocation(line: 3281, column: 81, scope: !2087)
!2136 = !DILocation(line: 888, column: 56, scope: !2122, inlinedAt: !2127)
!2137 = !DILocation(line: 138, column: 5, scope: !2129, inlinedAt: !2134)
!2138 = !DILocation(line: 3284, column: 6, scope: !2087)
!2139 = distinct !DISubprogram(name: "drop<u8, alloc::alloc::Global>", linkageName: "_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h78f429c21ee2e9e9E", scope: !2140, file: !825, line: 581, type: !2141, scopeLine: 581, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !29, templateParams: !376, retainedNodes: !2144)
!2140 = !DINamespace(name: "{impl#4}", scope: !112)
!2141 = !DISubroutineType(types: !2142)
!2142 = !{null, !2143}
!2143 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut alloc::raw_vec::RawVec<u8, alloc::alloc::Global>", baseType: !355, size: 64, align: 64, dwarfAddressSpace: 0)
!2144 = !{!2145, !2146, !2148}
!2145 = !DILocalVariable(name: "self", arg: 1, scope: !2139, file: !825, line: 581, type: !2143)
!2146 = !DILocalVariable(name: "ptr", scope: !2147, file: !825, line: 582, type: !362, align: 8)
!2147 = distinct !DILexicalBlock(scope: !2139, file: !825, line: 582, column: 60)
!2148 = !DILocalVariable(name: "layout", scope: !2147, file: !825, line: 582, type: !588, align: 8)
!2149 = !DILocation(line: 581, column: 13, scope: !2139)
!2150 = !DILocation(line: 582, column: 38, scope: !2147)
!2151 = !DILocation(line: 582, column: 16, scope: !2147)
!2152 = !DILocation(line: 581, column: 5, scope: !2139)
!2153 = !DILocation(line: 582, column: 22, scope: !2147)
!2154 = !DILocation(line: 582, column: 27, scope: !2147)
!2155 = !DILocation(line: 583, column: 22, scope: !2147)
!2156 = !DILocation(line: 582, column: 9, scope: !2139)
!2157 = !DILocation(line: 585, column: 5, scope: !2139)
!2158 = !DILocation(line: 585, column: 6, scope: !2139)
!2159 = distinct !DISubprogram(name: "main", linkageName: "_ZN13llvm_practice4main17h1c5c652c4b5d6518E", scope: !2160, file: !659, line: 1, type: !21, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized | DISPFlagMainSubprogram, unit: !29, templateParams: !23, retainedNodes: !2161)
!2160 = !DINamespace(name: "llvm_practice", scope: null)
!2161 = !{!2162, !2164, !2166, !2168}
!2162 = !DILocalVariable(name: "hello", scope: !2163, file: !659, line: 2, type: !242, align: 8)
!2163 = distinct !DILexicalBlock(scope: !2159, file: !659, line: 2, column: 5)
!2164 = !DILocalVariable(name: "world", scope: !2165, file: !659, line: 3, type: !242, align: 8)
!2165 = distinct !DILexicalBlock(scope: !2163, file: !659, line: 3, column: 5)
!2166 = !DILocalVariable(name: "new", scope: !2167, file: !659, line: 4, type: !347, align: 8)
!2167 = distinct !DILexicalBlock(scope: !2165, file: !659, line: 4, column: 5)
!2168 = !DILocalVariable(name: "res", scope: !2169, file: !659, line: 4, type: !347, align: 8)
!2169 = !DILexicalBlockFile(scope: !2170, file: !659, discriminator: 0)
!2170 = distinct !DILexicalBlock(scope: !2165, file: !2171, line: 125, column: 9)
!2171 = !DIFile(filename: "/rustc/051478957371ee0084a7c0913941d2a8c4757bb9/library/alloc/src/macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "6db4623b9dda65ff432540e542a62b7e")
!2172 = !DILocation(line: 2, column: 9, scope: !2163)
!2173 = !DILocation(line: 3, column: 9, scope: !2165)
!2174 = !DILocation(line: 4, column: 9, scope: !2167)
!2175 = !DILocation(line: 4, column: 15, scope: !2169)
!2176 = !DILocation(line: 2, column: 9, scope: !2159)
!2177 = !DILocation(line: 2, column: 17, scope: !2159)
!2178 = !DILocation(line: 3, column: 9, scope: !2163)
!2179 = !DILocation(line: 3, column: 17, scope: !2163)
!2180 = !DILocation(line: 4, column: 9, scope: !2165)
!2181 = !DILocation(line: 4, column: 15, scope: !2165)
!2182 = !DILocation(line: 5, column: 5, scope: !2167)
!2183 = !DILocation(line: 6, column: 1, scope: !2165)
!2184 = !DILocation(line: 6, column: 1, scope: !2163)
!2185 = !DILocation(line: 6, column: 1, scope: !2159)
!2186 = !DILocation(line: 6, column: 2, scope: !2159)
!2187 = !DILocation(line: 1, column: 1, scope: !2159)
