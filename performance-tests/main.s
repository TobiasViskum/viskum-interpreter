	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.p2align	2
__ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3d711e262771b209E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	mov	x9, #31681
	movk	x9, #52510, lsl #16
	movk	x9, #51356, lsl #32
	movk	x9, #49570, lsl #48
	mov	x8, #61284
	movk	x8, #177, lsl #16
	movk	x8, #5761, lsl #32
	movk	x8, #64956, lsl #48
	mov.d	v0[0], x9
	mov.d	v0[1], x8
	str	q0, [sp, #32]
	ldr	q0, [sp, #32]
	mov	d1, v0[1]
	str	d1, [sp, #16]
	str	d0, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	ldr	x0, [sp]
	ldr	x1, [sp, #8]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h931e920afff0b6edE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h54d0dd2dfb3d75d7E
	; InlineAsm Start
	; InlineAsm End
	brk	#0x1
	.cfi_endproc

	.p2align	2
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha8560be6525c67edE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4core3ops8function6FnOnce9call_once17h0a913df53902e0f9E
	; InlineAsm Start
	; InlineAsm End
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__ZN3std2rt10lang_start17h3539623d15ccbd4aE
	.globl	__ZN3std2rt10lang_start17h3539623d15ccbd4aE
	.p2align	2
__ZN3std2rt10lang_start17h3539623d15ccbd4aE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	str	x1, [sp]
	mov	x0, x2
	ldr	x2, [sp]
	str	x0, [sp, #8]
	mov	x4, x3
	ldr	x3, [sp, #8]
	sub	x0, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l___unnamed_1@PAGE
	add	x1, x1, l___unnamed_1@PAGEOFF
	bl	__ZN3std2rt19lang_start_internal17h39923ab4c3913741E
	str	x0, [sp, #16]
	ldr	x0, [sp, #16]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h65f6febff2dbe018E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17ha8560be6525c67edE
	bl	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6050c7b93a1c6573E
	sturb	w0, [x29, #-1]
	ldurb	w0, [x29, #-1]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std9panicking11begin_panic17had5db39b47896b4cE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x9, x0
	stur	x2, [x29, #-8]
	ldur	x8, [x29, #-8]
	mov	x0, sp
	str	x9, [sp]
	str	x1, [sp, #8]
	str	x8, [sp, #16]
	bl	__ZN3std10sys_common9backtrace26__rust_end_short_backtrace17h931e920afff0b6edE
	.cfi_endproc

	.p2align	2
__ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h54d0dd2dfb3d75d7E:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	ldr	x10, [x8]
	ldr	x9, [x8, #8]
	str	x10, [sp, #16]
	str	x9, [sp, #24]
	ldr	x10, [sp, #16]
	ldr	x9, [sp, #24]
	mov	x0, sp
	str	x10, [sp]
	str	x9, [sp, #8]
	ldr	x3, [x8, #16]
Ltmp1:
	adrp	x1, l___unnamed_2@PAGE
	add	x1, x1, l___unnamed_2@PAGEOFF
	mov	x2, #0
	mov	w8, #1
	and	w4, w8, #0x1
	mov	w8, #0
	and	w5, w8, #0x1
	bl	__ZN3std9panicking20rust_panic_with_hook17h84760468187ddc85E
Ltmp2:
	b	LBB6_3
LBB6_1:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB6_2:
Ltmp3:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB6_1
LBB6_3:
	brk	#0x1
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table6:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp2-Ltmp1
	.uleb128 Ltmp3-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp2-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp2
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN41_$LT$bool$u20$as$u20$core..fmt..Debug$GT$3fmt17hb620321a489808caE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN43_$LT$bool$u20$as$u20$core..fmt..Display$GT$3fmt17hdd2e4571458cdc6fE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04d0284cf44c688bE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd52e55f810623b49E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h07d103376f470b26E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h7033f97758f45be0E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h1bb02d7b7e96dff5E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hb20366c675940d9bE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN57_$LT$main..vm_util..Value$u20$as$u20$core..fmt..Debug$GT$3fmt17hbf95ed63f183eb94E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h4eb1fabeb362c295E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN41_$LT$bool$u20$as$u20$core..fmt..Debug$GT$3fmt17hb620321a489808caE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8fc3c848711e8923E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	mov	x2, x1
	ldr	x0, [x8]
	ldr	x1, [x8, #8]
	bl	__ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17hbaafd5d4e0b380e6E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h919a85546103f853E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN62_$LT$main..vm_util..StackValue$u20$as$u20$core..fmt..Debug$GT$3fmt17hafd7a73d9e63f38cE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9d648b5bd8239ea2E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i64$GT$3fmt17h70e019a2bdff87f3E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hbe496b7ef689d0ddE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN66_$LT$main..instructions..InstrData$u20$as$u20$core..fmt..Debug$GT$3fmt17hdb4ff904094b1a9aE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd33992b29ccb3899E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN70_$LT$main..instructions..InstructionV2$u20$as$u20$core..fmt..Debug$GT$3fmt17h1fc8fedb089aa03cE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17h6147861da6c066dcE:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	mov	x0, x2
	add	x8, sp, #16
	bl	__ZN4core3fmt9Formatter10debug_list17h0d0b1554eb22f7ccE
	ldr	x1, [sp]
	ldr	x0, [sp, #8]
	stur	x0, [x29, #-24]
	stur	x1, [x29, #-16]
	stur	x0, [x29, #-32]
	b	LBB18_1
LBB18_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	add	x8, x8, x9, lsl #3
	stur	x8, [x29, #-8]
	b	LBB18_2
LBB18_2:
	ldur	x8, [x29, #-8]
	ldur	x9, [x29, #-32]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x1, [sp, #32]
	ldr	x2, [sp, #40]
	add	x0, sp, #16
	bl	__ZN4core3fmt8builders9DebugList7entries17h131b3d847cf32dfbE
	bl	__ZN4core3fmt8builders9DebugList6finish17h113c11ea4e8abb88E
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17hbaafd5d4e0b380e6E:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	mov	x0, x2
	add	x8, sp, #16
	bl	__ZN4core3fmt9Formatter10debug_list17h0d0b1554eb22f7ccE
	ldr	x1, [sp]
	ldr	x0, [sp, #8]
	stur	x0, [x29, #-24]
	stur	x1, [x29, #-16]
	stur	x0, [x29, #-32]
	b	LBB19_1
LBB19_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	mov	w10, #48
	mul	x9, x9, x10
	add	x8, x8, x9
	stur	x8, [x29, #-8]
	b	LBB19_2
LBB19_2:
	ldur	x8, [x29, #-8]
	ldur	x9, [x29, #-32]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x1, [sp, #32]
	ldr	x2, [sp, #40]
	add	x0, sp, #16
	bl	__ZN4core3fmt8builders9DebugList7entries17h9c0381e6c5c47f8bE
	bl	__ZN4core3fmt8builders9DebugList6finish17h113c11ea4e8abb88E
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd9b6460679eaffa7E:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp]
	mov	x0, x2
	add	x8, sp, #16
	bl	__ZN4core3fmt9Formatter10debug_list17h0d0b1554eb22f7ccE
	ldr	x1, [sp]
	ldr	x0, [sp, #8]
	stur	x0, [x29, #-24]
	stur	x1, [x29, #-16]
	stur	x0, [x29, #-32]
	b	LBB20_1
LBB20_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	add	x8, x8, x9, lsl #4
	stur	x8, [x29, #-8]
	b	LBB20_2
LBB20_2:
	ldur	x8, [x29, #-8]
	ldur	x9, [x29, #-32]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x1, [sp, #32]
	ldr	x2, [sp, #40]
	add	x0, sp, #16
	bl	__ZN4core3fmt8builders9DebugList7entries17h481ed8b8c69f2ec0E
	bl	__ZN4core3fmt8builders9DebugList6finish17h113c11ea4e8abb88E
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics11write_bytes18precondition_check17hb8e92fd0efafaeb7E:
Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception1
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Ltmp4:
	bl	__ZN4core10intrinsics23is_aligned_and_not_null17hb603c5b18fd12102E
	stur	w0, [x29, #-4]
Ltmp5:
	b	LBB21_2
LBB21_1:
Ltmp6:
	bl	__ZN4core9panicking19panic_cannot_unwind17h1836202834f564f7E
LBB21_2:
	ldur	w8, [x29, #-4]
	tbnz	w8, #0, LBB21_4
	b	LBB21_3
LBB21_3:
	adrp	x0, l___unnamed_3@PAGE
	add	x0, x0, l___unnamed_3@PAGEOFF
	mov	w8, #111
	mov	x1, x8
	bl	__ZN4core9panicking14panic_nounwind17h765f3648d339fa95E
LBB21_4:
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end1:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table21:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp4-Lfunc_begin1
	.uleb128 Ltmp5-Ltmp4
	.uleb128 Ltmp6-Lfunc_begin1
	.byte	1
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core10intrinsics16retag_box_to_raw17h029517196ceba31fE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics16retag_box_to_raw17h2621a69217330d71E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics16retag_box_to_raw17h524749d3d092c407E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics16retag_box_to_raw17h5bee6c45725c9c82E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics16retag_box_to_raw17hc95473bcfbf489a7E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics16retag_box_to_raw17hf1c7cbc81a74151dE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics16retag_box_to_raw17hfe6dcf2a8466911aE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics17is_nonoverlapping7runtime17h5703f27a72fcacffE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	umulh	x8, x2, x3
	mul	x9, x2, x3
	str	x9, [sp, #24]
	subs	x8, x8, #0
	cset	w8, ne
	sturb	w8, [x29, #-1]
	ldurb	w8, [x29, #-1]
	tbnz	w8, #0, LBB29_2
	b	LBB29_1
LBB29_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp, #16]
	ldr	x10, [sp, #24]
	str	x10, [sp, #40]
	mov	w10, #1
	str	x10, [sp, #32]
	ldr	x10, [sp, #40]
	str	x10, [sp]
	subs	x8, x8, x9
	b.lo	LBB29_4
	b	LBB29_3
LBB29_2:
	adrp	x0, l___unnamed_4@PAGE
	add	x0, x0, l___unnamed_4@PAGEOFF
	mov	w8, #61
	mov	x1, x8
	bl	__ZN4core9panicking14panic_nounwind17h765f3648d339fa95E
LBB29_3:
	ldr	x8, [sp, #8]
	ldr	x9, [sp, #16]
	subs	x8, x8, x9
	stur	x8, [x29, #-16]
	b	LBB29_5
LBB29_4:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	subs	x8, x8, x9
	stur	x8, [x29, #-16]
	b	LBB29_5
LBB29_5:
	ldr	x9, [sp]
	ldur	x8, [x29, #-16]
	subs	x8, x8, x9
	cset	w0, hs
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h9c1c48ac54a9746cE:
Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception2
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	str	x2, [sp, #32]
	mov	x1, x3
	stur	x1, [x29, #-24]
	stur	x4, [x29, #-16]
Ltmp7:
	bl	__ZN4core10intrinsics23is_aligned_and_not_null17hb603c5b18fd12102E
	stur	w0, [x29, #-4]
Ltmp8:
	b	LBB30_2
LBB30_1:
Ltmp13:
	bl	__ZN4core9panicking19panic_cannot_unwind17h1836202834f564f7E
LBB30_2:
	ldur	w8, [x29, #-4]
	tbnz	w8, #0, LBB30_4
	b	LBB30_3
LBB30_3:
	adrp	x0, l___unnamed_5@PAGE
	add	x0, x0, l___unnamed_5@PAGEOFF
	mov	w8, #166
	mov	x1, x8
	bl	__ZN4core9panicking14panic_nounwind17h765f3648d339fa95E
LBB30_4:
Ltmp9:
	ldur	x1, [x29, #-24]
	ldr	x0, [sp, #24]
	bl	__ZN4core10intrinsics23is_aligned_and_not_null17hb603c5b18fd12102E
	str	w0, [sp, #12]
Ltmp10:
	b	LBB30_5
LBB30_5:
	ldr	w8, [sp, #12]
	tbz	w8, #0, LBB30_3
	b	LBB30_6
LBB30_6:
Ltmp11:
	ldur	x3, [x29, #-16]
	ldr	x2, [sp, #32]
	ldr	x1, [sp, #24]
	ldr	x0, [sp, #16]
	bl	__ZN4core10intrinsics17is_nonoverlapping7runtime17h5703f27a72fcacffE
	str	w0, [sp, #8]
Ltmp12:
	b	LBB30_7
LBB30_7:
	ldr	w8, [sp, #8]
	tbnz	w8, #0, LBB30_9
	b	LBB30_8
LBB30_8:
	b	LBB30_3
LBB30_9:
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end2:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table30:
Lexception2:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end2-Lcst_begin2
Lcst_begin2:
	.uleb128 Ltmp7-Lfunc_begin2
	.uleb128 Ltmp12-Ltmp7
	.uleb128 Ltmp13-Lfunc_begin2
	.byte	1
Lcst_end2:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core10intrinsics23is_aligned_and_not_null17hb603c5b18fd12102E:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp]
	str	x1, [sp, #8]
	cbnz	x0, LBB31_2
	b	LBB31_1
LBB31_1:
	strb	wzr, [sp, #23]
	b	LBB31_3
LBB31_2:
	ldr	x8, [sp, #8]
	fmov	d0, x8
	cnt.8b	v0, v0
	uaddlv.8b	h0, v0
	mov.s	w8, v0[0]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	subs	w8, w8, #1
	b.eq	LBB31_4
	b	LBB31_5
LBB31_3:
	ldrb	w8, [sp, #23]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB31_4:
	.cfi_restore_state
	ldr	x8, [sp]
	ldr	x9, [sp, #8]
	subs	x9, x9, #1
	ands	x8, x8, x9
	cset	w8, eq
	strb	w8, [sp, #23]
	b	LBB31_3
LBB31_5:
	add	x0, sp, #24
	adrp	x8, l___unnamed_6@PAGE
	add	x8, x8, l___unnamed_6@PAGEOFF
	str	x8, [sp, #24]
	mov	w8, #1
	str	x8, [sp, #32]
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #56]
	str	x8, [sp, #64]
	adrp	x8, l___unnamed_8@PAGE
	add	x8, x8, l___unnamed_8@PAGEOFF
	str	x8, [sp, #40]
	str	xzr, [sp, #48]
	adrp	x1, l___unnamed_9@PAGE
	add	x1, x1, l___unnamed_9@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h98bbf7bdf4994454E
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics23is_val_statically_known17hba828424c25d5449E:
	.cfi_startproc
	mov	w8, #0
	and	w0, w8, #0x1
	ret
	.cfi_endproc

	.p2align	2
__ZN4core10intrinsics8unlikely17h9e48c00f479c8a13E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hd42860831d83f422E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x8, [x0]
	str	x8, [sp, #8]
	ldr	x9, [x1]
	str	x9, [sp, #16]
	subs	x8, x8, x9
	b.lo	LBB34_2
	b	LBB34_1
LBB34_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp, #16]
	subs	x8, x8, x9
	b.eq	LBB34_4
	b	LBB34_3
LBB34_2:
	mov	w8, #255
	strb	w8, [sp, #31]
	b	LBB34_6
LBB34_3:
	mov	w8, #1
	strb	w8, [sp, #31]
	b	LBB34_5
LBB34_4:
	strb	wzr, [sp, #31]
	b	LBB34_5
LBB34_5:
	b	LBB34_6
LBB34_6:
	ldrb	w0, [sp, #31]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3cmp5impls70_$LT$impl$u20$core..cmp..PartialOrd$LT$$RF$B$GT$$u20$for$u20$$RF$A$GT$2lt17h888854cfd1fe6f76E:
	.cfi_startproc
	ldr	x8, [x0]
	ldr	x9, [x1]
	ldr	x8, [x8]
	ldr	x9, [x9]
	subs	x8, x8, x9
	cset	w0, lt
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3cmp5impls70_$LT$impl$u20$core..cmp..PartialOrd$LT$$RF$B$GT$$u20$for$u20$$RF$A$GT$2lt17h9bf4e6088cfa648cE:
	.cfi_startproc
	ldr	x8, [x0]
	ldr	x9, [x1]
	ldr	w8, [x8]
	ldr	w9, [x9]
	subs	w8, w8, w9
	cset	w0, lt
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3cmp6max_by17h43b9b5ac048f07d6E:
Lfunc_begin3:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception3
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	add	x9, sp, #8
	str	x0, [sp, #8]
	add	x8, sp, #16
	str	x1, [sp, #16]
	mov	w10, #1
	sturb	w10, [x29, #-17]
	str	x9, [sp, #40]
	str	x8, [sp, #48]
	ldr	x0, [sp, #40]
	ldr	x1, [sp, #48]
Ltmp15:
	bl	__ZN4core3ops8function6FnOnce9call_once17h2b29b13d06e50de0E
	str	w0, [sp, #4]
Ltmp16:
	b	LBB37_3
LBB37_1:
	b	LBB37_10
LBB37_2:
Ltmp17:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB37_1
LBB37_3:
	ldr	w8, [sp, #4]
	strb	w8, [sp, #39]
	ldrb	w8, [sp, #39]
	add	w8, w8, #1
	and	w8, w8, #0xff
	subs	w8, w8, #1
	b.ls	LBB37_6
	b	LBB37_4
LBB37_4:
	b	LBB37_7
LBB37_6:
	ldr	x8, [sp, #16]
	str	x8, [sp, #24]
	ldurb	w8, [x29, #-17]
	tbnz	w8, #0, LBB37_9
	b	LBB37_8
LBB37_7:
	sturb	wzr, [x29, #-17]
	ldr	x8, [sp, #8]
	str	x8, [sp, #24]
	b	LBB37_8
LBB37_8:
	ldr	x0, [sp, #24]
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB37_9:
	.cfi_restore_state
	b	LBB37_8
LBB37_10:
	ldurb	w8, [x29, #-17]
	tbnz	w8, #0, LBB37_12
	b	LBB37_11
LBB37_11:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB37_12:
	b	LBB37_11
Lfunc_end3:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table37:
Lexception3:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end3-Lcst_begin3
Lcst_begin3:
	.uleb128 Ltmp15-Lfunc_begin3
	.uleb128 Ltmp16-Ltmp15
	.uleb128 Ltmp17-Lfunc_begin3
	.byte	0
	.uleb128 Ltmp16-Lfunc_begin3
	.uleb128 Lfunc_end3-Ltmp16
	.byte	0
	.byte	0
Lcst_end3:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h9c2b11a1d0e71c2dE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	ldr	w8, [x1, #52]
	tbnz	w8, #4, LBB38_2
	b	LBB38_1
LBB38_1:
	ldr	x8, [sp, #16]
	ldr	w8, [x8, #52]
	tbz	w8, #5, LBB38_3
	b	LBB38_4
LBB38_2:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u8$GT$3fmt17hf80fea428aa49224E
	sturb	w0, [x29, #-1]
	b	LBB38_6
LBB38_3:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17h4b16b13010ac8fa6E
	sturb	w0, [x29, #-1]
	b	LBB38_5
LBB38_4:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u8$GT$3fmt17h85bbca09a1b509baE
	sturb	w0, [x29, #-1]
	b	LBB38_5
LBB38_5:
	b	LBB38_6
LBB38_6:
	ldurb	w8, [x29, #-1]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i32$GT$3fmt17hb20366c675940d9bE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	ldr	w8, [x1, #52]
	tbnz	w8, #4, LBB39_2
	b	LBB39_1
LBB39_1:
	ldr	x8, [sp, #16]
	ldr	w8, [x8, #52]
	tbz	w8, #5, LBB39_3
	b	LBB39_4
LBB39_2:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i32$GT$3fmt17h126d218a21b155cdE
	sturb	w0, [x29, #-1]
	b	LBB39_6
LBB39_3:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h879cd70b5b7c546eE
	sturb	w0, [x29, #-1]
	b	LBB39_5
LBB39_4:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i32$GT$3fmt17h84061024638f0cdeE
	sturb	w0, [x29, #-1]
	b	LBB39_5
LBB39_5:
	b	LBB39_6
LBB39_6:
	ldurb	w8, [x29, #-1]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i64$GT$3fmt17h70e019a2bdff87f3E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	ldr	w8, [x1, #52]
	tbnz	w8, #4, LBB40_2
	b	LBB40_1
LBB40_1:
	ldr	x8, [sp, #16]
	ldr	w8, [x8, #52]
	tbz	w8, #5, LBB40_3
	b	LBB40_4
LBB40_2:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num53_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i64$GT$3fmt17h477ed96f070bbb30E
	sturb	w0, [x29, #-1]
	b	LBB40_6
LBB40_3:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i64$GT$3fmt17h46b45c6d077fcbc6E
	sturb	w0, [x29, #-1]
	b	LBB40_5
LBB40_4:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h9fa62ad70354a71dE
	sturb	w0, [x29, #-1]
	b	LBB40_5
LBB40_5:
	b	LBB40_6
LBB40_6:
	ldurb	w8, [x29, #-1]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17h7033f97758f45be0E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	ldr	w8, [x1, #52]
	tbnz	w8, #4, LBB41_2
	b	LBB41_1
LBB41_1:
	ldr	x8, [sp, #16]
	ldr	w8, [x8, #52]
	tbz	w8, #5, LBB41_3
	b	LBB41_4
LBB41_2:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17hffeae19598650363E
	sturb	w0, [x29, #-1]
	b	LBB41_6
LBB41_3:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17h8fb7f92a2e17f7bbE
	sturb	w0, [x29, #-1]
	b	LBB41_5
LBB41_4:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num55_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$usize$GT$3fmt17hc40a7e3ed771aec9E
	sturb	w0, [x29, #-1]
	b	LBB41_5
LBB41_5:
	b	LBB41_6
LBB41_6:
	ldurb	w8, [x29, #-1]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt8builders9DebugList7entries17h131b3d847cf32dfbE:
Lfunc_begin4:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception4
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	mov	x0, x1
	mov	x1, x2
	bl	__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h2a5963f7401b2db4E
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	b	LBB42_1
LBB42_1:
Ltmp18:
	add	x0, sp, #16
	bl	__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h478cb3728ccf0c3cE
	str	x0, [sp]
Ltmp19:
	b	LBB42_4
LBB42_2:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB42_3:
Ltmp20:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB42_2
LBB42_4:
	ldr	x8, [sp]
	str	x8, [sp, #32]
	ldr	x8, [sp, #32]
	subs	x8, x8, #0
	cset	x8, ne
	cbnz	x8, LBB42_6
	b	LBB42_5
LBB42_5:
	ldr	x0, [sp, #8]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB42_6:
	.cfi_restore_state
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #32]
	sub	x1, x29, #24
	stur	x8, [x29, #-24]
Ltmp21:
	adrp	x2, l___unnamed_10@PAGE
	add	x2, x2, l___unnamed_10@PAGEOFF
	bl	__ZN4core3fmt8builders9DebugList5entry17hc73dd476d1443770E
Ltmp22:
	b	LBB42_9
LBB42_7:
	b	LBB42_2
LBB42_8:
Ltmp23:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB42_7
LBB42_9:
	b	LBB42_10
LBB42_10:
	b	LBB42_1
Lfunc_end4:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table42:
Lexception4:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end4-Lcst_begin4
Lcst_begin4:
	.uleb128 Lfunc_begin4-Lfunc_begin4
	.uleb128 Ltmp18-Lfunc_begin4
	.byte	0
	.byte	0
	.uleb128 Ltmp18-Lfunc_begin4
	.uleb128 Ltmp19-Ltmp18
	.uleb128 Ltmp20-Lfunc_begin4
	.byte	0
	.uleb128 Ltmp19-Lfunc_begin4
	.uleb128 Ltmp21-Ltmp19
	.byte	0
	.byte	0
	.uleb128 Ltmp21-Lfunc_begin4
	.uleb128 Ltmp22-Ltmp21
	.uleb128 Ltmp23-Lfunc_begin4
	.byte	0
Lcst_end4:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3fmt8builders9DebugList7entries17h481ed8b8c69f2ec0E:
Lfunc_begin5:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception5
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	mov	x0, x1
	mov	x1, x2
	bl	__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h7627f43ee7cd2253E
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	b	LBB43_1
LBB43_1:
Ltmp24:
	add	x0, sp, #16
	bl	__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hf0f48a1916b92d8eE
	str	x0, [sp]
Ltmp25:
	b	LBB43_4
LBB43_2:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB43_3:
Ltmp26:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB43_2
LBB43_4:
	ldr	x8, [sp]
	str	x8, [sp, #32]
	ldr	x8, [sp, #32]
	subs	x8, x8, #0
	cset	x8, ne
	cbnz	x8, LBB43_6
	b	LBB43_5
LBB43_5:
	ldr	x0, [sp, #8]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB43_6:
	.cfi_restore_state
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #32]
	sub	x1, x29, #24
	stur	x8, [x29, #-24]
Ltmp27:
	adrp	x2, l___unnamed_11@PAGE
	add	x2, x2, l___unnamed_11@PAGEOFF
	bl	__ZN4core3fmt8builders9DebugList5entry17hc73dd476d1443770E
Ltmp28:
	b	LBB43_9
LBB43_7:
	b	LBB43_2
LBB43_8:
Ltmp29:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB43_7
LBB43_9:
	b	LBB43_10
LBB43_10:
	b	LBB43_1
Lfunc_end5:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table43:
Lexception5:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end5-Lcst_begin5
Lcst_begin5:
	.uleb128 Lfunc_begin5-Lfunc_begin5
	.uleb128 Ltmp24-Lfunc_begin5
	.byte	0
	.byte	0
	.uleb128 Ltmp24-Lfunc_begin5
	.uleb128 Ltmp25-Ltmp24
	.uleb128 Ltmp26-Lfunc_begin5
	.byte	0
	.uleb128 Ltmp25-Lfunc_begin5
	.uleb128 Ltmp27-Ltmp25
	.byte	0
	.byte	0
	.uleb128 Ltmp27-Lfunc_begin5
	.uleb128 Ltmp28-Ltmp27
	.uleb128 Ltmp29-Lfunc_begin5
	.byte	0
Lcst_end5:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3fmt8builders9DebugList7entries17h9c0381e6c5c47f8bE:
Lfunc_begin6:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception6
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	mov	x0, x1
	mov	x1, x2
	bl	__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hc70e992a18f362a5E
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	b	LBB44_1
LBB44_1:
Ltmp30:
	add	x0, sp, #16
	bl	__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h8db6a8588b72297cE
	str	x0, [sp]
Ltmp31:
	b	LBB44_4
LBB44_2:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB44_3:
Ltmp32:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB44_2
LBB44_4:
	ldr	x8, [sp]
	str	x8, [sp, #32]
	ldr	x8, [sp, #32]
	subs	x8, x8, #0
	cset	x8, ne
	cbnz	x8, LBB44_6
	b	LBB44_5
LBB44_5:
	ldr	x0, [sp, #8]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB44_6:
	.cfi_restore_state
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #32]
	sub	x1, x29, #24
	stur	x8, [x29, #-24]
Ltmp33:
	adrp	x2, l___unnamed_12@PAGE
	add	x2, x2, l___unnamed_12@PAGEOFF
	bl	__ZN4core3fmt8builders9DebugList5entry17hc73dd476d1443770E
Ltmp34:
	b	LBB44_9
LBB44_7:
	b	LBB44_2
LBB44_8:
Ltmp35:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB44_7
LBB44_9:
	b	LBB44_10
LBB44_10:
	b	LBB44_1
Lfunc_end6:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table44:
Lexception6:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end6-Lcst_begin6
Lcst_begin6:
	.uleb128 Lfunc_begin6-Lfunc_begin6
	.uleb128 Ltmp30-Lfunc_begin6
	.byte	0
	.byte	0
	.uleb128 Ltmp30-Lfunc_begin6
	.uleb128 Ltmp31-Ltmp30
	.uleb128 Ltmp32-Lfunc_begin6
	.byte	0
	.uleb128 Ltmp31-Lfunc_begin6
	.uleb128 Ltmp33-Ltmp31
	.byte	0
	.byte	0
	.uleb128 Ltmp33-Lfunc_begin6
	.uleb128 Ltmp34-Ltmp33
	.uleb128 Ltmp35-Lfunc_begin6
	.byte	0
Lcst_end6:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3fmt9Arguments12as_const_str17h866d4e7a8cc409b5E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	ldr	x8, [x0]
	str	x8, [sp]
	ldr	x8, [x0, #8]
	str	x8, [sp, #8]
	ldr	x9, [x0, #24]
	str	x9, [sp, #16]
	cbnz	x8, LBB45_2
	b	LBB45_1
LBB45_1:
	ldr	x8, [sp, #16]
	cbz	x8, LBB45_3
	b	LBB45_4
LBB45_2:
	ldr	x8, [sp, #8]
	subs	x8, x8, #1
	b.eq	LBB45_6
	b	LBB45_4
LBB45_3:
	adrp	x8, l___unnamed_8@PAGE
	add	x8, x8, l___unnamed_8@PAGEOFF
	str	x8, [sp, #40]
	str	xzr, [sp, #48]
	b	LBB45_5
LBB45_4:
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #40]
	str	x8, [sp, #48]
	b	LBB45_5
LBB45_5:
	ldr	x8, [sp, #40]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.eq	LBB45_8
	b	LBB45_9
LBB45_6:
	ldr	x8, [sp, #16]
	cbnz	x8, LBB45_4
	b	LBB45_7
LBB45_7:
	ldr	x8, [sp]
	ldr	x9, [x8]
	ldr	x8, [x8, #8]
	str	x9, [sp, #40]
	str	x8, [sp, #48]
	b	LBB45_5
LBB45_8:
	mov	w8, #1
	strb	w8, [sp, #62]
	b	LBB45_10
LBB45_9:
	strb	wzr, [sp, #62]
	b	LBB45_10
LBB45_10:
	strb	wzr, [sp, #63]
	ldrb	w8, [sp, #63]
	tbnz	w8, #0, LBB45_12
	b	LBB45_11
LBB45_11:
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #24]
	str	x8, [sp, #32]
	b	LBB45_13
LBB45_12:
	ldr	x9, [sp, #40]
	ldr	x8, [sp, #48]
	str	x9, [sp, #24]
	str	x8, [sp, #32]
	b	LBB45_13
LBB45_13:
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Arguments16new_v1_formatted17he08a92c810cac804E:
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	str	x4, [sp]
	str	x5, [sp, #8]
	str	x0, [x8]
	str	x1, [x8, #8]
	ldr	x10, [sp]
	ldr	x9, [sp, #8]
	str	x10, [x8, #32]
	str	x9, [x8, #40]
	str	x2, [x8, #16]
	str	x3, [x8, #24]
	add	sp, sp, #16
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Arguments6new_v117h619f9bb2db3983b4E:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	str	x2, [sp, #32]
	str	x3, [sp, #40]
	subs	x8, x1, x3
	b.lo	LBB47_2
	b	LBB47_1
LBB47_1:
	ldr	x8, [sp, #24]
	ldr	x9, [sp, #40]
	add	x9, x9, #1
	subs	x8, x8, x9
	b.hi	LBB47_4
	b	LBB47_3
LBB47_2:
	add	x0, sp, #48
	adrp	x8, l___unnamed_13@PAGE
	add	x8, x8, l___unnamed_13@PAGEOFF
	str	x8, [sp, #48]
	mov	w8, #1
	str	x8, [sp, #56]
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #80]
	str	x8, [sp, #88]
	adrp	x8, l___unnamed_8@PAGE
	add	x8, x8, l___unnamed_8@PAGEOFF
	str	x8, [sp, #64]
	str	xzr, [sp, #72]
	adrp	x1, l___unnamed_14@PAGE
	add	x1, x1, l___unnamed_14@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h98bbf7bdf4994454E
LBB47_3:
	ldr	x8, [sp, #40]
	ldr	x9, [sp, #8]
	ldr	x10, [sp, #32]
	ldr	x11, [sp, #24]
	ldr	x12, [sp, #16]
	str	x12, [x9]
	str	x11, [x9, #8]
	adrp	x12, l___unnamed_7@PAGE
	adrp	x11, l___unnamed_7@PAGE
	add	x11, x11, l___unnamed_7@PAGEOFF
	ldr	x12, [x12, l___unnamed_7@PAGEOFF]
	ldr	x11, [x11, #8]
	str	x12, [x9, #32]
	str	x11, [x9, #40]
	str	x10, [x9, #16]
	str	x8, [x9, #24]
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB47_4:
	.cfi_restore_state
	b	LBB47_2
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Arguments9new_const17hb03f19c839408508E:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	subs	x8, x1, #1
	b.hi	LBB48_2
	b	LBB48_1
LBB48_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp, #24]
	ldr	x10, [sp, #16]
	str	x10, [x8]
	str	x9, [x8, #8]
	adrp	x10, l___unnamed_7@PAGE
	adrp	x9, l___unnamed_7@PAGE
	add	x9, x9, l___unnamed_7@PAGEOFF
	ldr	x10, [x10, l___unnamed_7@PAGEOFF]
	ldr	x9, [x9, #8]
	str	x10, [x8, #32]
	str	x9, [x8, #40]
	adrp	x9, l___unnamed_8@PAGE
	add	x9, x9, l___unnamed_8@PAGEOFF
	str	x9, [x8, #16]
	str	xzr, [x8, #24]
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB48_2:
	.cfi_restore_state
	add	x8, sp, #32
	str	x8, [sp]
	adrp	x0, l___unnamed_13@PAGE
	add	x0, x0, l___unnamed_13@PAGEOFF
	mov	w9, #1
	mov	x1, x9
	bl	__ZN4core3fmt9Arguments9new_const17hb03f19c839408508E
	ldr	x0, [sp]
	adrp	x1, l___unnamed_15@PAGE
	add	x1, x1, l___unnamed_15@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h98bbf7bdf4994454E
	.cfi_endproc

	.p2align	2
__ZN4core3fmt9Formatter9write_fmt17h153b963f3ea3120aE:
	.cfi_startproc
	sub	sp, sp, #128
	.cfi_def_cfa_offset 128
	stp	x29, x30, [sp, #112]
	add	x29, sp, #112
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #24]
	mov	x0, x1
	str	x0, [sp, #32]
	bl	__ZN4core3fmt9Arguments12as_const_str17h866d4e7a8cc409b5E
	str	x0, [sp, #48]
	str	x1, [sp, #56]
	ldr	x8, [sp, #48]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB49_2
	b	LBB49_1
LBB49_1:
	ldr	x8, [sp, #24]
	ldr	x1, [sp, #48]
	ldr	x2, [sp, #56]
	ldr	x0, [x8, #32]
	ldr	x8, [x8, #40]
	ldr	x8, [x8, #24]
	blr	x8
	strb	w0, [sp, #47]
	b	LBB49_3
LBB49_2:
	ldr	x1, [sp, #32]
	ldr	x8, [sp, #24]
	ldr	x9, [x8, #32]
	str	x9, [sp]
	ldr	x8, [x8, #40]
	str	x8, [sp, #8]
	sub	x0, x29, #48
	str	x0, [sp, #16]
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x0, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	bl	__ZN4core3fmt5write17hbadb443a71b75f23E
	strb	w0, [sp, #47]
	b	LBB49_3
LBB49_3:
	ldrb	w8, [sp, #47]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 128
	ldp	x29, x30, [sp, #112]
	add	sp, sp, #128
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hbd0bd907336b8d8eE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3ops8function6FnOnce9call_once17hdd445ba18d89ab56E
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h0a913df53902e0f9E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h2b29b13d06e50de0E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	str	x1, [sp, #8]
	ldr	x0, [sp]
	ldr	x1, [sp, #8]
	bl	__ZN4core3cmp5impls50_$LT$impl$u20$core..cmp..Ord$u20$for$u20$usize$GT$3cmp17hd42860831d83f422E
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17hdd445ba18d89ab56E:
Lfunc_begin7:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception7
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	add	x0, sp, #16
	str	x8, [sp, #16]
Ltmp38:
	bl	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h65f6febff2dbe018E
	str	w0, [sp, #12]
Ltmp39:
	b	LBB53_3
LBB53_1:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB53_2:
Ltmp40:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB53_1
LBB53_3:
	ldr	w0, [sp, #12]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end7:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table53:
Lexception7:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end7-Lcst_begin7
Lcst_begin7:
	.uleb128 Ltmp38-Lfunc_begin7
	.uleb128 Ltmp39-Ltmp38
	.uleb128 Ltmp40-Lfunc_begin7
	.byte	0
	.uleb128 Ltmp39-Lfunc_begin7
	.uleb128 Lfunc_end7-Ltmp39
	.byte	0
	.byte	0
Lcst_end7:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr100drop_in_place$LT$alloc..boxed..Box$LT$$u5b$main..instructions..Instruction2$u3b$$u20$17$u5d$$GT$$GT$17hcc250ad903e8ab72E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	b	LBB54_1
LBB54_1:
	ldr	x0, [sp, #8]
	bl	__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1f2c720a5dbee91aE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr101drop_in_place$LT$alloc..boxed..Box$LT$$u5b$main..instructions..InstructionV2$u3b$$u20$14$u5d$$GT$$GT$17hbadbccd766535580E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	b	LBB55_1
LBB55_1:
	ldr	x0, [sp, #8]
	bl	__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h95337d73c9fc504dE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr108drop_in_place$LT$alloc..vec..Vec$LT$$LP$usize$C$$u5b$main..instructions..Operand$u3b$$u20$4$u5d$$RP$$GT$$GT$17h4b2975b76c04d018E:
Lfunc_begin8:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception8
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp41:
	bl	__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h231c7dc41ab0bd21E
Ltmp42:
	b	LBB56_3
LBB56_1:
Ltmp44:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr115drop_in_place$LT$alloc..raw_vec..RawVec$LT$$LP$usize$C$$u5b$main..instructions..Operand$u3b$$u20$4$u5d$$RP$$GT$$GT$17hf3aeed6e3618d139E
Ltmp45:
	b	LBB56_5
LBB56_2:
Ltmp43:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB56_1
LBB56_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr115drop_in_place$LT$alloc..raw_vec..RawVec$LT$$LP$usize$C$$u5b$main..instructions..Operand$u3b$$u20$4$u5d$$RP$$GT$$GT$17hf3aeed6e3618d139E
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB56_4:
	.cfi_restore_state
Ltmp46:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB56_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end8:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table56:
Lexception8:
	.byte	255
	.byte	155
	.uleb128 Lttbase2-Lttbaseref2
Lttbaseref2:
	.byte	1
	.uleb128 Lcst_end8-Lcst_begin8
Lcst_begin8:
	.uleb128 Ltmp41-Lfunc_begin8
	.uleb128 Ltmp42-Ltmp41
	.uleb128 Ltmp43-Lfunc_begin8
	.byte	0
	.uleb128 Ltmp44-Lfunc_begin8
	.uleb128 Ltmp45-Ltmp44
	.uleb128 Ltmp46-Lfunc_begin8
	.byte	1
	.uleb128 Ltmp45-Lfunc_begin8
	.uleb128 Lfunc_end8-Ltmp45
	.byte	0
	.byte	0
Lcst_end8:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase2:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr115drop_in_place$LT$alloc..raw_vec..RawVec$LT$$LP$usize$C$$u5b$main..instructions..Operand$u3b$$u20$4$u5d$$RP$$GT$$GT$17hf3aeed6e3618d139E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hef91980bc59f6821E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr116drop_in_place$LT$alloc..boxed..Box$LT$$u5b$$LP$usize$C$main..instructions..Instruction2$RP$$u3b$$u20$17$u5d$$GT$$GT$17h7169344e8e3a6389E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	b	LBB58_1
LBB58_1:
	ldr	x0, [sp, #8]
	bl	__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h23d957b2c2c2dd12E
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr132drop_in_place$LT$alloc..boxed..Box$LT$$u5b$$LP$usize$C$$u5b$main..instructions..Operand$u3b$$u20$4$u5d$$RP$$u3b$$u20$17$u5d$$GT$$GT$17h7971495c0f700895E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	b	LBB59_1
LBB59_1:
	ldr	x0, [sp, #8]
	bl	__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h42ee07ad48cc5cbcE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr13read_volatile18precondition_check17haf6a701692d729fdE:
Lfunc_begin9:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception9
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Ltmp47:
	bl	__ZN4core10intrinsics23is_aligned_and_not_null17hb603c5b18fd12102E
	stur	w0, [x29, #-4]
Ltmp48:
	b	LBB60_2
LBB60_1:
Ltmp49:
	bl	__ZN4core9panicking19panic_cannot_unwind17h1836202834f564f7E
LBB60_2:
	ldur	w8, [x29, #-4]
	tbnz	w8, #0, LBB60_4
	b	LBB60_3
LBB60_3:
	adrp	x0, l___unnamed_16@PAGE
	add	x0, x0, l___unnamed_16@PAGEOFF
	mov	w8, #110
	mov	x1, x8
	bl	__ZN4core9panicking14panic_nounwind17h765f3648d339fa95E
LBB60_4:
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end9:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table60:
Lexception9:
	.byte	255
	.byte	155
	.uleb128 Lttbase3-Lttbaseref3
Lttbaseref3:
	.byte	1
	.uleb128 Lcst_end9-Lcst_begin9
Lcst_begin9:
	.uleb128 Ltmp47-Lfunc_begin9
	.uleb128 Ltmp48-Ltmp47
	.uleb128 Ltmp49-Lfunc_begin9
	.byte	1
Lcst_end9:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase3:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr23drop_in_place$LT$u8$GT$17h1763cab59d4cadfcE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr28drop_in_place$LT$$RF$i32$GT$17h99d783be20b987d1E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr28drop_in_place$LT$$RF$i64$GT$17he1f8dc4ad8c7c841E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h133c57e7d33347b9E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr29drop_in_place$LT$$RF$bool$GT$17hf9c10089fe597c86E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr29drop_in_place$LT$main..VM$GT$17h33d51f14d3fcf26dE:
Lfunc_begin10:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception10
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp50:
	bl	__ZN4core3ptr41drop_in_place$LT$main..vm_util..Stack$GT$17h156d0e55a5646dacE
Ltmp51:
	b	LBB66_3
LBB66_1:
	ldr	x8, [sp, #8]
	add	x0, x8, #48
Ltmp53:
	bl	__ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17hf7e818baffe1683fE
Ltmp54:
	b	LBB66_5
LBB66_2:
Ltmp52:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB66_1
LBB66_3:
	ldr	x8, [sp, #8]
	add	x0, x8, #48
	bl	__ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17hf7e818baffe1683fE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB66_4:
	.cfi_restore_state
Ltmp55:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB66_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end10:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table66:
Lexception10:
	.byte	255
	.byte	155
	.uleb128 Lttbase4-Lttbaseref4
Lttbaseref4:
	.byte	1
	.uleb128 Lcst_end10-Lcst_begin10
Lcst_begin10:
	.uleb128 Ltmp50-Lfunc_begin10
	.uleb128 Ltmp51-Ltmp50
	.uleb128 Ltmp52-Lfunc_begin10
	.byte	0
	.uleb128 Ltmp53-Lfunc_begin10
	.uleb128 Ltmp54-Ltmp53
	.uleb128 Ltmp55-Lfunc_begin10
	.byte	1
	.uleb128 Ltmp54-Lfunc_begin10
	.uleb128 Lfunc_end10-Ltmp54
	.byte	0
	.byte	0
Lcst_end10:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase4:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17hdfbb0672aebe3775E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr41drop_in_place$LT$main..vm_util..Stack$GT$17h156d0e55a5646dacE:
Lfunc_begin11:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception11
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp56:
	bl	__ZN4core3ptr69drop_in_place$LT$alloc..vec..Vec$LT$main..vm_util..StackValue$GT$$GT$17hc496bbea7a6bec5eE
Ltmp57:
	b	LBB68_3
LBB68_1:
	ldr	x8, [sp, #8]
	add	x0, x8, #24
Ltmp59:
	bl	__ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17hf7e818baffe1683fE
Ltmp60:
	b	LBB68_5
LBB68_2:
Ltmp58:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB68_1
LBB68_3:
	ldr	x8, [sp, #8]
	add	x0, x8, #24
	bl	__ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17hf7e818baffe1683fE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB68_4:
	.cfi_restore_state
Ltmp61:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB68_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end11:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table68:
Lexception11:
	.byte	255
	.byte	155
	.uleb128 Lttbase5-Lttbaseref5
Lttbaseref5:
	.byte	1
	.uleb128 Lcst_end11-Lcst_begin11
Lcst_begin11:
	.uleb128 Ltmp56-Lfunc_begin11
	.uleb128 Ltmp57-Ltmp56
	.uleb128 Ltmp58-Lfunc_begin11
	.byte	0
	.uleb128 Ltmp59-Lfunc_begin11
	.uleb128 Ltmp60-Ltmp59
	.uleb128 Ltmp61-Lfunc_begin11
	.byte	1
	.uleb128 Ltmp60-Lfunc_begin11
	.uleb128 Lfunc_end11-Ltmp60
	.byte	0
	.byte	0
Lcst_end11:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase5:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr45drop_in_place$LT$$RF$main..vm_util..Value$GT$17h84350351536a9094E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr49drop_in_place$LT$alloc..vec..Vec$LT$usize$GT$$GT$17hf7e818baffe1683fE:
Lfunc_begin12:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception12
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp62:
	bl	__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17ha0f625439a49ea68E
Ltmp63:
	b	LBB70_3
LBB70_1:
Ltmp65:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr56drop_in_place$LT$alloc..raw_vec..RawVec$LT$usize$GT$$GT$17h1f6eca46a1e089d9E
Ltmp66:
	b	LBB70_5
LBB70_2:
Ltmp64:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB70_1
LBB70_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr56drop_in_place$LT$alloc..raw_vec..RawVec$LT$usize$GT$$GT$17h1f6eca46a1e089d9E
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB70_4:
	.cfi_restore_state
Ltmp67:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB70_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end12:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table70:
Lexception12:
	.byte	255
	.byte	155
	.uleb128 Lttbase6-Lttbaseref6
Lttbaseref6:
	.byte	1
	.uleb128 Lcst_end12-Lcst_begin12
Lcst_begin12:
	.uleb128 Ltmp62-Lfunc_begin12
	.uleb128 Ltmp63-Ltmp62
	.uleb128 Ltmp64-Lfunc_begin12
	.byte	0
	.uleb128 Ltmp65-Lfunc_begin12
	.uleb128 Ltmp66-Ltmp65
	.uleb128 Ltmp67-Lfunc_begin12
	.byte	1
	.uleb128 Ltmp66-Lfunc_begin12
	.uleb128 Lfunc_end12-Ltmp66
	.byte	0
	.byte	0
Lcst_end12:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase6:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr50drop_in_place$LT$$RF$main..vm_util..StackValue$GT$17h73a9f2651e3f0c04E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr53drop_in_place$LT$$RF$alloc..vec..Vec$LT$usize$GT$$GT$17hf3708de75d14da17E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr54drop_in_place$LT$$RF$main..instructions..InstrData$GT$17hf66b31b500ac6387E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr56drop_in_place$LT$alloc..raw_vec..RawVec$LT$usize$GT$$GT$17h1f6eca46a1e089d9E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc00eaaabd0855f1eE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr58drop_in_place$LT$$RF$main..instructions..InstructionV2$GT$17hf58833b6e8a351a3E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr64drop_in_place$LT$alloc..vec..Vec$LT$main..vm_util..Value$GT$$GT$17h5e3a7fb19708ec5dE:
Lfunc_begin13:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception13
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp68:
	bl	__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h180872de86a57143E
Ltmp69:
	b	LBB76_3
LBB76_1:
Ltmp71:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr71drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..vm_util..Value$GT$$GT$17h2a842fbc93e66f9eE
Ltmp72:
	b	LBB76_5
LBB76_2:
Ltmp70:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB76_1
LBB76_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr71drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..vm_util..Value$GT$$GT$17h2a842fbc93e66f9eE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB76_4:
	.cfi_restore_state
Ltmp73:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB76_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end13:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table76:
Lexception13:
	.byte	255
	.byte	155
	.uleb128 Lttbase7-Lttbaseref7
Lttbaseref7:
	.byte	1
	.uleb128 Lcst_end13-Lcst_begin13
Lcst_begin13:
	.uleb128 Ltmp68-Lfunc_begin13
	.uleb128 Ltmp69-Ltmp68
	.uleb128 Ltmp70-Lfunc_begin13
	.byte	0
	.uleb128 Ltmp71-Lfunc_begin13
	.uleb128 Ltmp72-Ltmp71
	.uleb128 Ltmp73-Lfunc_begin13
	.byte	1
	.uleb128 Ltmp72-Lfunc_begin13
	.uleb128 Lfunc_end13-Ltmp72
	.byte	0
	.byte	0
Lcst_end13:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase7:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr69drop_in_place$LT$alloc..vec..Vec$LT$main..vm_util..StackValue$GT$$GT$17hc496bbea7a6bec5eE:
Lfunc_begin14:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception14
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp74:
	bl	__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbb1ff3e1f6843a73E
Ltmp75:
	b	LBB77_3
LBB77_1:
Ltmp77:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr76drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..vm_util..StackValue$GT$$GT$17hfbf04edc6e28f5d6E
Ltmp78:
	b	LBB77_5
LBB77_2:
Ltmp76:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB77_1
LBB77_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr76drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..vm_util..StackValue$GT$$GT$17hfbf04edc6e28f5d6E
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB77_4:
	.cfi_restore_state
Ltmp79:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB77_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end14:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table77:
Lexception14:
	.byte	255
	.byte	155
	.uleb128 Lttbase8-Lttbaseref8
Lttbaseref8:
	.byte	1
	.uleb128 Lcst_end14-Lcst_begin14
Lcst_begin14:
	.uleb128 Ltmp74-Lfunc_begin14
	.uleb128 Ltmp75-Ltmp74
	.uleb128 Ltmp76-Lfunc_begin14
	.byte	0
	.uleb128 Ltmp77-Lfunc_begin14
	.uleb128 Ltmp78-Ltmp77
	.uleb128 Ltmp79-Lfunc_begin14
	.byte	1
	.uleb128 Ltmp78-Lfunc_begin14
	.uleb128 Lfunc_end14-Ltmp78
	.byte	0
	.byte	0
Lcst_end14:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase8:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr71drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..vm_util..Value$GT$$GT$17h2a842fbc93e66f9eE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hdf0d79ed7ea6d829E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr72drop_in_place$LT$std..panicking..begin_panic..Payload$LT$$RF$str$GT$$GT$17hd6b5ec900259bc94E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr75drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..Instruction$GT$$GT$17hb330cbe1268ffd9aE:
Lfunc_begin15:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception15
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp80:
	bl	__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1a22e1d4b76bb115E
Ltmp81:
	b	LBB80_3
LBB80_1:
Ltmp83:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr82drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..Instruction$GT$$GT$17h8b7e95bc849700e7E
Ltmp84:
	b	LBB80_5
LBB80_2:
Ltmp82:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB80_1
LBB80_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr82drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..Instruction$GT$$GT$17h8b7e95bc849700e7E
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB80_4:
	.cfi_restore_state
Ltmp85:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB80_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end15:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table80:
Lexception15:
	.byte	255
	.byte	155
	.uleb128 Lttbase9-Lttbaseref9
Lttbaseref9:
	.byte	1
	.uleb128 Lcst_end15-Lcst_begin15
Lcst_begin15:
	.uleb128 Ltmp80-Lfunc_begin15
	.uleb128 Ltmp81-Ltmp80
	.uleb128 Ltmp82-Lfunc_begin15
	.byte	0
	.uleb128 Ltmp83-Lfunc_begin15
	.uleb128 Ltmp84-Ltmp83
	.uleb128 Ltmp85-Lfunc_begin15
	.byte	1
	.uleb128 Ltmp84-Lfunc_begin15
	.uleb128 Lfunc_end15-Ltmp84
	.byte	0
	.byte	0
Lcst_end15:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase9:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr76drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..vm_util..StackValue$GT$$GT$17hfbf04edc6e28f5d6E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h05557965ee1802d3E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr76drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..Instruction2$GT$$GT$17heda17ac1179fa6a0E:
Lfunc_begin16:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception16
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp86:
	bl	__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17ha4da5c1ab779e773E
Ltmp87:
	b	LBB82_3
LBB82_1:
Ltmp89:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr83drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..Instruction2$GT$$GT$17h5fa3e707a5fa79faE
Ltmp90:
	b	LBB82_5
LBB82_2:
Ltmp88:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB82_1
LBB82_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr83drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..Instruction2$GT$$GT$17h5fa3e707a5fa79faE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB82_4:
	.cfi_restore_state
Ltmp91:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB82_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end16:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table82:
Lexception16:
	.byte	255
	.byte	155
	.uleb128 Lttbase10-Lttbaseref10
Lttbaseref10:
	.byte	1
	.uleb128 Lcst_end16-Lcst_begin16
Lcst_begin16:
	.uleb128 Ltmp86-Lfunc_begin16
	.uleb128 Ltmp87-Ltmp86
	.uleb128 Ltmp88-Lfunc_begin16
	.byte	0
	.uleb128 Ltmp89-Lfunc_begin16
	.uleb128 Ltmp90-Ltmp89
	.uleb128 Ltmp91-Lfunc_begin16
	.byte	1
	.uleb128 Ltmp90-Lfunc_begin16
	.uleb128 Lfunc_end16-Ltmp90
	.byte	0
	.byte	0
Lcst_end16:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase10:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr77drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..InstructionV2$GT$$GT$17h570c83b0fdd350d5E:
Lfunc_begin17:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception17
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp92:
	bl	__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb5f518fa518ba4b4E
Ltmp93:
	b	LBB83_3
LBB83_1:
Ltmp95:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr84drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..InstructionV2$GT$$GT$17ha4f89cb7d7f1df4cE
Ltmp96:
	b	LBB83_5
LBB83_2:
Ltmp94:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB83_1
LBB83_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr84drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..InstructionV2$GT$$GT$17ha4f89cb7d7f1df4cE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB83_4:
	.cfi_restore_state
Ltmp97:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB83_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end17:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table83:
Lexception17:
	.byte	255
	.byte	155
	.uleb128 Lttbase11-Lttbaseref11
Lttbaseref11:
	.byte	1
	.uleb128 Lcst_end17-Lcst_begin17
Lcst_begin17:
	.uleb128 Ltmp92-Lfunc_begin17
	.uleb128 Ltmp93-Ltmp92
	.uleb128 Ltmp94-Lfunc_begin17
	.byte	0
	.uleb128 Ltmp95-Lfunc_begin17
	.uleb128 Ltmp96-Ltmp95
	.uleb128 Ltmp97-Lfunc_begin17
	.byte	1
	.uleb128 Ltmp96-Lfunc_begin17
	.uleb128 Lfunc_end17-Ltmp96
	.byte	0
	.byte	0
Lcst_end17:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase11:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr82drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..Instruction$GT$$GT$17h8b7e95bc849700e7E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h6bf902f7bfb0ce56E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr83drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..Instruction2$GT$$GT$17h5fa3e707a5fa79faE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h96d84aa33aca4a86E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr84drop_in_place$LT$alloc..raw_vec..RawVec$LT$main..instructions..InstructionV2$GT$$GT$17ha4f89cb7d7f1df4cE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h2c166a266ee37d1eE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he62c7a8a4e5d4ffeE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	cbnz	x0, LBB88_2
	b	LBB88_1
LBB88_1:
	adrp	x0, l___unnamed_17@PAGE
	add	x0, x0, l___unnamed_17@PAGEOFF
	mov	w8, #93
	mov	x1, x8
	bl	__ZN4core9panicking14panic_nounwind17h765f3648d339fa95E
LBB88_2:
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr92drop_in_place$LT$alloc..vec..Vec$LT$$LP$usize$C$main..instructions..Instruction2$RP$$GT$$GT$17hf0fdb457992ba510E:
Lfunc_begin18:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception18
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
Ltmp98:
	bl	__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17heedb86448cd32201E
Ltmp99:
	b	LBB89_3
LBB89_1:
Ltmp101:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr99drop_in_place$LT$alloc..raw_vec..RawVec$LT$$LP$usize$C$main..instructions..Instruction2$RP$$GT$$GT$17haea928a7d1f5fb1cE
Ltmp102:
	b	LBB89_5
LBB89_2:
Ltmp100:
	str	x0, [sp, #16]
	mov	x8, x1
	str	w8, [sp, #24]
	b	LBB89_1
LBB89_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr99drop_in_place$LT$alloc..raw_vec..RawVec$LT$$LP$usize$C$main..instructions..Instruction2$RP$$GT$$GT$17haea928a7d1f5fb1cE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB89_4:
	.cfi_restore_state
Ltmp103:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB89_5:
	ldr	x0, [sp, #16]
	bl	__Unwind_Resume
Lfunc_end18:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table89:
Lexception18:
	.byte	255
	.byte	155
	.uleb128 Lttbase12-Lttbaseref12
Lttbaseref12:
	.byte	1
	.uleb128 Lcst_end18-Lcst_begin18
Lcst_begin18:
	.uleb128 Ltmp98-Lfunc_begin18
	.uleb128 Ltmp99-Ltmp98
	.uleb128 Ltmp100-Lfunc_begin18
	.byte	0
	.uleb128 Ltmp101-Lfunc_begin18
	.uleb128 Ltmp102-Ltmp101
	.uleb128 Ltmp103-Lfunc_begin18
	.byte	1
	.uleb128 Ltmp102-Lfunc_begin18
	.uleb128 Lfunc_end18-Ltmp102
	.byte	0
	.byte	0
Lcst_end18:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase12:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr99drop_in_place$LT$alloc..boxed..Box$LT$$u5b$main..instructions..Instruction$u3b$$u20$12$u5d$$GT$$GT$17h607d9b4bfc4c1fe3E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	b	LBB90_1
LBB90_1:
	ldr	x0, [sp, #8]
	bl	__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5bdf4cd9537f880aE
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr99drop_in_place$LT$alloc..raw_vec..RawVec$LT$$LP$usize$C$main..instructions..Instruction2$RP$$GT$$GT$17haea928a7d1f5fb1cE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h88c485db797c154fE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core4hint16assert_unchecked18precondition_check17h3d2b16db344f9c42E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	tbnz	w0, #0, LBB92_2
	b	LBB92_1
LBB92_1:
	adrp	x0, l___unnamed_18@PAGE
	add	x0, x0, l___unnamed_18@PAGEOFF
	mov	w8, #104
	mov	x1, x8
	bl	__ZN4core9panicking14panic_nounwind17h765f3648d339fa95E
LBB92_2:
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5alloc6layout6Layout5array5inner17h6f4b5b5253e62597E:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	str	x2, [sp, #24]
	cbnz	x0, LBB93_2
	b	LBB93_1
LBB93_1:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	ldr	x10, [sp, #24]
	mul	x9, x9, x10
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	stur	x9, [x29, #-24]
	stur	x8, [x29, #-32]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	b	LBB93_7
LBB93_2:
	ldr	x8, [sp, #8]
	ldr	x9, [sp, #16]
	stur	x9, [x29, #-16]
	ldur	x9, [x29, #-16]
	subs	x10, x9, #1
	mov	x9, #9223372036854775807
	subs	x9, x9, x10
	str	x9, [sp]
	cbz	x8, LBB93_4
	b	LBB93_3
LBB93_3:
	ldr	x8, [sp, #24]
	ldr	x9, [sp]
	ldr	x10, [sp, #8]
	udiv	x9, x9, x10
	subs	x8, x8, x9
	b.hi	LBB93_6
	b	LBB93_5
LBB93_4:
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #25
	mov	x1, x8
	adrp	x2, l___unnamed_19@PAGE
	add	x2, x2, l___unnamed_19@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
LBB93_5:
	b	LBB93_1
LBB93_6:
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	b	LBB93_7
LBB93_7:
	ldr	x0, [sp, #32]
	ldr	x1, [sp, #40]
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5alloc6layout6Layout8dangling17habf52b833e2edaf7E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x8, [x0]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	str	x8, [sp, #8]
	b	LBB94_1
LBB94_1:
	ldr	x8, [sp, #8]
	add	x0, x8, #0
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB94_2
LBB94_2:
	ldr	x8, [sp, #8]
	add	x8, x8, #0
	str	x8, [sp, #16]
	ldr	x0, [sp, #16]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h21f22e255f836a66E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	mov	x0, x1
	ldr	x1, [sp]
	str	x0, [sp, #8]
	mov	x0, x2
	ldr	x2, [sp, #8]
	bl	__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hcd94d25b0092102dE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h244297f580e1b3d5E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	mov	x0, x1
	ldr	x1, [sp]
	str	x0, [sp, #8]
	mov	x0, x2
	ldr	x2, [sp, #8]
	bl	__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17h575587ebed97f6beE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17hc2b02be7afc34c85E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	mov	x0, x1
	ldr	x1, [sp]
	str	x0, [sp, #8]
	mov	x0, x2
	ldr	x2, [sp, #8]
	bl	__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hcb920bf91edc27bdE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	mov	x0, x1
	ldr	x1, [sp]
	str	x0, [sp, #8]
	mov	x0, x2
	ldr	x2, [sp, #8]
	bl	__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17h14d35e93823784dbE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5slice3raw14from_raw_parts18precondition_check17h44775545258936e7E:
Lfunc_begin19:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception19
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp]
	mov	x1, x2
	str	x3, [sp, #8]
Ltmp104:
	bl	__ZN4core10intrinsics23is_aligned_and_not_null17hb603c5b18fd12102E
	stur	w0, [x29, #-12]
Ltmp105:
	b	LBB99_2
LBB99_1:
Ltmp108:
	bl	__ZN4core9panicking19panic_cannot_unwind17h1836202834f564f7E
LBB99_2:
	ldur	w8, [x29, #-12]
	tbnz	w8, #0, LBB99_4
	b	LBB99_3
LBB99_3:
	b	LBB99_5
LBB99_4:
	ldr	x8, [sp]
	cbz	x8, LBB99_6
	b	LBB99_7
LBB99_5:
	adrp	x0, l___unnamed_20@PAGE
	add	x0, x0, l___unnamed_20@PAGEOFF
	mov	w8, #162
	mov	x1, x8
	bl	__ZN4core9panicking14panic_nounwind17h765f3648d339fa95E
LBB99_6:
	mov	x8, #-1
	stur	x8, [x29, #-8]
	b	LBB99_8
LBB99_7:
	ldr	x8, [sp]
	cbz	x8, LBB99_10
	b	LBB99_9
LBB99_8:
	ldr	x8, [sp, #8]
	ldur	x9, [x29, #-8]
	subs	x8, x8, x9
	b.ls	LBB99_13
	b	LBB99_12
LBB99_9:
	ldr	x9, [sp]
	mov	x8, #9223372036854775807
	udiv	x8, x8, x9
	stur	x8, [x29, #-8]
	b	LBB99_8
LBB99_10:
Ltmp106:
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #25
	mov	x1, x8
	adrp	x2, l___unnamed_21@PAGE
	add	x2, x2, l___unnamed_21@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
Ltmp107:
	b	LBB99_11
LBB99_11:
	brk	#0x1
LBB99_12:
	b	LBB99_5
LBB99_13:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end19:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table99:
Lexception19:
	.byte	255
	.byte	155
	.uleb128 Lttbase13-Lttbaseref13
Lttbaseref13:
	.byte	1
	.uleb128 Lcst_end19-Lcst_begin19
Lcst_begin19:
	.uleb128 Ltmp104-Lfunc_begin19
	.uleb128 Ltmp107-Ltmp104
	.uleb128 Ltmp108-Lfunc_begin19
	.byte	1
Lcst_end19:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase13:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core9panicking13assert_failed17h34e476f4fba23c5eE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x1
	mov	x5, x3
	mov	x6, x4
	mov	x1, sp
	str	x8, [sp]
	add	x3, sp, #8
	str	x2, [sp, #8]
	adrp	x4, l___unnamed_10@PAGE
	add	x4, x4, l___unnamed_10@PAGEOFF
	mov	x2, x4
	bl	__ZN4core9panicking19assert_failed_inner17hf761943632c466c1E
	.cfi_endproc

	.p2align	2
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6050c7b93a1c6573E:
	.cfi_startproc
	mov	w0, #0
	ret
	.cfi_endproc

	.p2align	2
__ZN59_$LT$$RF$u128$u20$as$u20$core..ops..bit..Shr$LT$i32$GT$$GT$3shr17h1514458bce170824E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	w1, [sp, #12]
	ldr	q0, [x0]
	str	q0, [sp, #16]
	subs	w8, w1, #128
	b.hs	LBB102_2
	b	LBB102_1
LBB102_1:
	ldr	q1, [sp, #16]
	ldr	w9, [sp, #12]
	mov	w8, #0
	mov	x13, x9
	bfi	x13, x8, #32, #32
	mov	x9, #0
	and	x10, x13, #0x7f
	mov	w8, #64
	fmov	d0, d1
	mov	d1, v1[1]
	subs	x14, x10, #64
	subs	x15, x8, x10
	fmov	x8, d1
	lsr	x8, x8, x10
	fmov	x11, d0
	lsr	x11, x11, x10
	fmov	x12, d1
	lsl	x12, x12, x15
	orr	x11, x11, x12
	fmov	x12, d1
	lsr	x12, x12, x14
	subs	x14, x10, #64
	csel	x12, x11, x12, lo
	fmov	x11, d0
	ands	x13, x13, #0x7f
	csel	x0, x11, x12, eq
	subs	x10, x10, #64
	csel	x1, x8, x9, lo
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB102_2:
	.cfi_restore_state
	adrp	x0, _str.1@PAGE
	add	x0, x0, _str.1@PAGEOFF
	mov	w8, #36
	mov	x1, x8
	adrp	x2, l___unnamed_22@PAGE
	add	x2, x2, l___unnamed_22@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN5alloc3vec12Vec$LT$T$GT$3new17h9a1b419d29aacfb7E:
	.cfi_startproc
	str	xzr, [x8]
	mov	w9, #8
	str	x9, [x8, #8]
	str	xzr, [x8, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc3vec12Vec$LT$T$GT$3new17ha107aee19cd17e30E:
	.cfi_startproc
	str	xzr, [x8]
	mov	w9, #8
	str	x9, [x8, #8]
	str	xzr, [x8, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17h27d589a0d32c4e01E:
	.cfi_startproc
	ldr	x0, [x0, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17haf3dcdb1269e75b5E:
	.cfi_startproc
	ldr	x0, [x0, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc3vec16Vec$LT$T$C$A$GT$3pop17h8ece746ea8cd90fbE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #16]
	ldr	x8, [x0, #16]
	cbnz	x8, LBB107_2
	b	LBB107_1
LBB107_1:
	str	xzr, [sp, #24]
	b	LBB107_3
LBB107_2:
	ldr	x8, [sp, #16]
	ldr	x9, [x8, #16]
	subs	x9, x9, #1
	str	x9, [x8, #16]
	ldr	x8, [x8, #16]
	str	x8, [sp, #8]
	b	LBB107_4
LBB107_3:
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB107_4:
	.cfi_restore_state
	ldr	x8, [sp, #16]
	ldr	x8, [x8]
	stur	x8, [x29, #-8]
	b	LBB107_5
LBB107_5:
	ldr	x8, [sp, #8]
	ldur	x9, [x29, #-8]
	subs	x8, x8, x9
	cset	w8, lo
	str	w8, [sp, #4]
	b	LBB107_6
LBB107_6:
	ldr	w0, [sp, #4]
	bl	__ZN4core4hint16assert_unchecked18precondition_check17h3d2b16db344f9c42E
	b	LBB107_7
LBB107_7:
	ldr	x9, [sp, #16]
	ldr	x8, [x9, #8]
	ldr	x9, [x9, #16]
	ldr	x8, [x8, x9, lsl #3]
	str	x8, [sp, #32]
	mov	w8, #1
	str	x8, [sp, #24]
	b	LBB107_3
	.cfi_endproc

	.p2align	2
__ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17h369134e97c19391dE:
Lfunc_begin20:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception20
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp]
	str	x1, [sp, #8]
	ldr	x8, [x0, #16]
	str	x8, [sp, #16]
	b	LBB108_1
LBB108_1:
	ldr	x8, [sp]
	ldr	x8, [x8]
	str	x8, [sp, #24]
	b	LBB108_2
LBB108_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #24]
	subs	x8, x8, x9
	b.eq	LBB108_4
	b	LBB108_3
LBB108_3:
	b	LBB108_5
LBB108_4:
	ldr	x0, [sp]
	ldr	x1, [x0, #16]
Ltmp109:
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17heb9b076b3a0b5e8eE
Ltmp110:
	b	LBB108_8
LBB108_5:
	ldr	x9, [sp]
	ldr	x11, [sp, #8]
	ldr	x8, [x9, #8]
	ldr	x10, [x9, #16]
	ldr	q0, [x11]
	str	q0, [sp, #32]
	ldr	q0, [sp, #32]
	str	q0, [x8, x10, lsl #4]
	ldr	x8, [x9, #16]
	add	x8, x8, #1
	str	x8, [x9, #16]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB108_6:
	.cfi_restore_state
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB108_7:
Ltmp111:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB108_6
LBB108_8:
	b	LBB108_5
Lfunc_end20:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table108:
Lexception20:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end20-Lcst_begin20
Lcst_begin20:
	.uleb128 Ltmp109-Lfunc_begin20
	.uleb128 Ltmp110-Ltmp109
	.uleb128 Ltmp111-Lfunc_begin20
	.byte	0
	.uleb128 Ltmp110-Lfunc_begin20
	.uleb128 Lfunc_end20-Ltmp110
	.byte	0
	.byte	0
Lcst_end20:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17h387bfeda0baaabafE:
Lfunc_begin21:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception21
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp]
	str	x1, [sp, #8]
	ldr	x8, [x0, #16]
	str	x8, [sp, #16]
	b	LBB109_1
LBB109_1:
	ldr	x8, [sp]
	ldr	x8, [x8]
	str	x8, [sp, #24]
	b	LBB109_2
LBB109_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #24]
	subs	x8, x8, x9
	b.eq	LBB109_4
	b	LBB109_3
LBB109_3:
	b	LBB109_5
LBB109_4:
	ldr	x0, [sp]
	ldr	x1, [x0, #16]
Ltmp112:
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hd59edee4349f66d8E
Ltmp113:
	b	LBB109_8
LBB109_5:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x10, [x9, #8]
	ldr	x11, [x9, #16]
	str	x8, [x10, x11, lsl #3]
	ldr	x8, [x9, #16]
	add	x8, x8, #1
	str	x8, [x9, #16]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB109_6:
	.cfi_restore_state
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB109_7:
Ltmp114:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB109_6
LBB109_8:
	b	LBB109_5
Lfunc_end21:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table109:
Lexception21:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end21-Lcst_begin21
Lcst_begin21:
	.uleb128 Ltmp112-Lfunc_begin21
	.uleb128 Ltmp113-Ltmp112
	.uleb128 Ltmp114-Lfunc_begin21
	.byte	0
	.uleb128 Ltmp113-Lfunc_begin21
	.uleb128 Lfunc_end21-Ltmp113
	.byte	0
	.byte	0
Lcst_end21:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN5alloc3vec16Vec$LT$T$C$A$GT$8truncate17hb431a5a89fa1cfadE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	str	x0, [sp]
	str	x1, [sp, #8]
	ldr	x8, [x0, #16]
	subs	x8, x1, x8
	b.hi	LBB110_2
	b	LBB110_1
LBB110_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	ldr	x10, [x9, #16]
	subs	x10, x10, x8
	ldr	x11, [x9, #8]
	add	x11, x11, x8, lsl #4
	str	x11, [sp, #32]
	str	x10, [sp, #40]
	ldr	x11, [sp, #32]
	ldr	x10, [sp, #40]
	str	x11, [sp, #16]
	str	x10, [sp, #24]
	str	x8, [x9, #16]
	b	LBB110_3
LBB110_2:
	b	LBB110_3
LBB110_3:
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5alloc15exchange_malloc17h941cdcdc6bb63bd7E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #16]
	str	x1, [sp, #8]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	adrp	x0, l___unnamed_8@PAGE
	add	x0, x0, l___unnamed_8@PAGEOFF
	mov	w8, #0
	and	w3, w8, #0x1
	bl	__ZN5alloc5alloc6Global10alloc_impl17h55da341c4b5c6e3dE
	str	x0, [sp, #24]
	str	x1, [sp, #32]
	ldr	x9, [sp, #24]
	mov	x8, #0
	subs	x9, x9, #0
	csinc	x8, x8, xzr, ne
	cbnz	x8, LBB111_2
	b	LBB111_1
LBB111_1:
	ldr	x0, [sp, #24]
	stur	x0, [x29, #-8]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB111_2:
	.cfi_restore_state
	ldr	x0, [sp, #8]
	ldr	x1, [sp, #16]
	bl	__ZN5alloc5alloc18handle_alloc_error17h2a00f3202443143aE
	.cfi_endproc

	.p2align	2
__ZN5alloc5alloc5alloc17hb1b9c72044e2280cE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	str	x1, [sp, #8]
	b	LBB112_1
LBB112_1:
	adrp	x0, ___rust_no_alloc_shim_is_unstable@GOTPAGE
	ldr	x0, [x0, ___rust_no_alloc_shim_is_unstable@GOTPAGEOFF]
	mov	w8, #1
	mov	x1, x8
	bl	__ZN4core3ptr13read_volatile18precondition_check17haf6a701692d729fdE
	b	LBB112_2
LBB112_2:
	adrp	x8, ___rust_no_alloc_shim_is_unstable@GOTPAGE
	ldr	x8, [x8, ___rust_no_alloc_shim_is_unstable@GOTPAGEOFF]
	ldrb	w8, [x8]
	sturb	w8, [x29, #-1]
	ldr	x0, [sp, #8]
	ldr	x8, [sp]
	str	x8, [sp, #16]
	ldr	x1, [sp, #16]
	bl	___rust_alloc
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5alloc6Global10alloc_impl17h55da341c4b5c6e3dE:
	.cfi_startproc
	sub	sp, sp, #288
	.cfi_def_cfa_offset 288
	stp	x28, x27, [sp, #256]
	stp	x29, x30, [sp, #272]
	add	x29, sp, #272
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	.cfi_remember_state
	str	w3, [sp, #52]
	str	x1, [sp, #64]
	str	x2, [sp, #72]
	ldr	x8, [sp, #72]
	str	x8, [sp, #56]
	cbnz	x8, LBB113_2
	b	LBB113_1
LBB113_1:
	add	x0, sp, #64
	bl	__ZN4core5alloc6layout6Layout8dangling17habf52b833e2edaf7E
	stur	x0, [x29, #-80]
	stur	xzr, [x29, #-72]
	ldur	x9, [x29, #-80]
	ldur	x8, [x29, #-72]
	stur	x9, [x29, #-96]
	stur	x8, [x29, #-88]
	ldur	x8, [x29, #-96]
	str	x8, [sp, #32]
	ldur	x8, [x29, #-88]
	str	x8, [sp, #40]
	b	LBB113_3
LBB113_2:
	ldr	w8, [sp, #52]
	tbnz	w8, #0, LBB113_7
	b	LBB113_6
LBB113_3:
	ldr	x0, [sp, #32]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB113_4
LBB113_4:
	ldr	x8, [sp, #40]
	ldr	x9, [sp, #32]
	str	x9, [sp, #96]
	str	x8, [sp, #104]
	ldr	x9, [sp, #96]
	ldr	x8, [sp, #104]
	str	x9, [sp, #80]
	str	x8, [sp, #88]
	b	LBB113_5
LBB113_5:
	ldr	x0, [sp, #80]
	ldr	x1, [sp, #88]
	.cfi_def_cfa wsp, 288
	ldp	x29, x30, [sp, #272]
	ldp	x28, x27, [sp, #256]
	add	sp, sp, #288
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB113_6:
	.cfi_restore_state
	ldr	x0, [sp, #64]
	ldr	x1, [sp, #72]
	bl	__ZN5alloc5alloc5alloc17hb1b9c72044e2280cE
	str	x0, [sp, #112]
	b	LBB113_8
LBB113_7:
	ldr	x9, [sp, #64]
	ldr	x8, [sp, #72]
	str	x9, [sp, #120]
	str	x8, [sp, #128]
	ldr	x0, [sp, #128]
	ldr	x8, [sp, #120]
	stur	x8, [x29, #-64]
	ldur	x1, [x29, #-64]
	bl	___rust_alloc_zeroed
	str	x0, [sp, #112]
	b	LBB113_8
LBB113_8:
	ldr	x8, [sp, #112]
	str	x8, [sp, #24]
	cbnz	x8, LBB113_10
	b	LBB113_9
LBB113_9:
	stur	xzr, [x29, #-120]
	stur	xzr, [x29, #-128]
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #80]
	str	x8, [sp, #88]
	b	LBB113_5
LBB113_10:
	b	LBB113_11
LBB113_11:
	ldr	x0, [sp, #24]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB113_12
LBB113_12:
	ldr	x8, [sp, #56]
	ldr	x9, [sp, #24]
	stur	x9, [x29, #-56]
	ldur	x9, [x29, #-56]
	stur	x9, [x29, #-120]
	ldur	x9, [x29, #-120]
	stur	x9, [x29, #-128]
	ldur	x9, [x29, #-128]
	str	x9, [sp, #136]
	ldr	x9, [sp, #136]
	stur	x9, [x29, #-32]
	stur	x8, [x29, #-24]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	ldur	x8, [x29, #-48]
	str	x8, [sp, #8]
	ldur	x8, [x29, #-40]
	str	x8, [sp, #16]
	b	LBB113_13
LBB113_13:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB113_14
LBB113_14:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	stur	x9, [x29, #-112]
	stur	x8, [x29, #-104]
	ldur	x9, [x29, #-112]
	ldur	x8, [x29, #-104]
	str	x9, [sp, #80]
	str	x8, [sp, #88]
	b	LBB113_5
	.cfi_endproc

	.p2align	2
__ZN5alloc5alloc6Global9grow_impl17h186b12e5261a8b98E:
	.cfi_startproc
	sub	sp, sp, #432
	.cfi_def_cfa_offset 432
	stp	x28, x27, [sp, #400]
	stp	x29, x30, [sp, #416]
	add	x29, sp, #416
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	.cfi_remember_state
	str	x0, [sp, #96]
	str	x1, [sp, #104]
	str	w6, [sp, #116]
	str	x2, [sp, #120]
	str	x3, [sp, #128]
	str	x4, [sp, #136]
	str	x5, [sp, #144]
	ldr	x8, [sp, #128]
	str	x8, [sp, #168]
	ldr	x8, [sp, #168]
	cbnz	x8, LBB114_2
	b	LBB114_1
LBB114_1:
	ldr	w3, [sp, #116]
	ldr	x0, [sp, #96]
	ldr	x1, [sp, #136]
	ldr	x2, [sp, #144]
	bl	__ZN5alloc5alloc6Global10alloc_impl17h55da341c4b5c6e3dE
	str	x0, [sp, #152]
	str	x1, [sp, #160]
	b	LBB114_3
LBB114_2:
	ldr	x8, [sp, #120]
	stur	x8, [x29, #-136]
	ldur	x8, [x29, #-136]
	ldr	x9, [sp, #136]
	stur	x9, [x29, #-128]
	ldur	x9, [x29, #-128]
	subs	x8, x8, x9
	b.eq	LBB114_5
	b	LBB114_4
LBB114_3:
	b	LBB114_15
LBB114_4:
	ldr	w3, [sp, #116]
	ldr	x0, [sp, #96]
	ldr	x8, [sp, #168]
	str	x8, [sp, #88]
	ldr	x1, [sp, #136]
	ldr	x2, [sp, #144]
	bl	__ZN5alloc5alloc6Global10alloc_impl17h55da341c4b5c6e3dE
	stur	x0, [x29, #-152]
	stur	x1, [x29, #-144]
	ldur	x9, [x29, #-152]
	mov	x8, #0
	subs	x9, x9, #0
	csinc	x8, x8, xzr, ne
	cbz	x8, LBB114_6
	b	LBB114_7
LBB114_5:
	ldr	x8, [sp, #168]
	str	x8, [sp, #64]
	ldr	x8, [sp, #144]
	str	x8, [sp, #72]
	ldr	x9, [sp, #128]
	subs	x8, x8, x9
	cset	w8, hs
	str	w8, [sp, #84]
	b	LBB114_11
LBB114_6:
	ldur	x9, [x29, #-152]
	ldur	x8, [x29, #-144]
	stur	x9, [x29, #-168]
	stur	x8, [x29, #-160]
	ldur	x8, [x29, #-168]
	str	x8, [sp, #48]
	ldur	x9, [x29, #-160]
	str	x9, [sp, #56]
	stur	x8, [x29, #-24]
	b	LBB114_8
LBB114_7:
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #152]
	str	x8, [sp, #160]
	b	LBB114_10
LBB114_8:
	ldr	x4, [sp, #88]
	ldr	x1, [sp, #48]
	ldr	x0, [sp, #104]
	mov	w8, #1
	mov	x3, x8
	mov	x2, x3
	bl	__ZN4core10intrinsics19copy_nonoverlapping18precondition_check17h9c1c48ac54a9746cE
	b	LBB114_9
LBB114_9:
	ldr	x0, [sp, #48]
	ldr	x1, [sp, #104]
	ldr	x8, [sp, #88]
	lsr	x2, x8, #0
	bl	_memcpy
	ldr	x0, [sp, #96]
	ldr	x1, [sp, #104]
	ldr	x2, [sp, #120]
	ldr	x3, [sp, #128]
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	ldr	x9, [sp, #48]
	ldr	x8, [sp, #56]
	str	x9, [sp, #152]
	str	x8, [sp, #160]
	b	LBB114_3
LBB114_10:
	b	LBB114_15
LBB114_11:
	ldr	w0, [sp, #84]
	bl	__ZN4core4hint16assert_unchecked18precondition_check17h3d2b16db344f9c42E
	b	LBB114_12
LBB114_12:
	ldr	x3, [sp, #72]
	ldr	x0, [sp, #104]
	ldr	x9, [sp, #120]
	ldr	x8, [sp, #128]
	str	x9, [sp, #176]
	str	x8, [sp, #184]
	ldr	x1, [sp, #184]
	ldr	x8, [sp, #176]
	stur	x8, [x29, #-120]
	ldur	x2, [x29, #-120]
	bl	___rust_realloc
	str	x0, [sp, #40]
	cbnz	x0, LBB114_14
	b	LBB114_13
LBB114_13:
	str	xzr, [sp, #208]
	str	xzr, [sp, #200]
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #152]
	str	x8, [sp, #160]
	b	LBB114_10
LBB114_14:
	b	LBB114_16
LBB114_15:
	ldr	x0, [sp, #152]
	ldr	x1, [sp, #160]
	.cfi_def_cfa wsp, 432
	ldp	x29, x30, [sp, #416]
	ldp	x28, x27, [sp, #400]
	add	sp, sp, #432
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB114_16:
	.cfi_restore_state
	ldr	x0, [sp, #40]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB114_17
LBB114_17:
	ldr	w8, [sp, #116]
	ldr	x9, [sp, #40]
	stur	x9, [x29, #-112]
	ldur	x9, [x29, #-112]
	str	x9, [sp, #208]
	ldr	x9, [sp, #208]
	str	x9, [sp, #200]
	ldr	x9, [sp, #200]
	str	x9, [sp, #192]
	ldr	x9, [sp, #192]
	str	x9, [sp, #32]
	tbnz	w8, #0, LBB114_19
	b	LBB114_18
LBB114_18:
	ldr	x8, [sp, #72]
	ldr	x9, [sp, #32]
	stur	x9, [x29, #-184]
	stur	x8, [x29, #-176]
	stur	x9, [x29, #-80]
	ldur	x9, [x29, #-80]
	stur	x9, [x29, #-88]
	ldur	x9, [x29, #-80]
	stur	x9, [x29, #-72]
	ldur	x9, [x29, #-80]
	stur	x9, [x29, #-32]
	ldur	x9, [x29, #-32]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	ldur	x9, [x29, #-48]
	ldur	x8, [x29, #-40]
	stur	x9, [x29, #-64]
	stur	x8, [x29, #-56]
	ldur	x9, [x29, #-64]
	ldur	x8, [x29, #-56]
	stur	x9, [x29, #-104]
	stur	x8, [x29, #-96]
	b	LBB114_21
LBB114_19:
	ldr	x8, [sp, #72]
	ldr	x9, [sp, #64]
	ldr	x10, [sp, #40]
	add	x10, x10, x9
	str	x10, [sp, #16]
	subs	x8, x8, x9
	str	x8, [sp, #24]
	b	LBB114_20
LBB114_20:
	ldr	x0, [sp, #16]
	mov	w8, #1
	mov	x1, x8
	str	x1, [sp, #8]
	bl	__ZN4core10intrinsics11write_bytes18precondition_check17hb8e92fd0efafaeb7E
	ldr	x9, [sp, #24]
	ldr	x8, [sp, #8]
	ldr	x0, [sp, #16]
	mul	x1, x8, x9
	bl	_bzero
	ldr	x9, [sp, #32]
	ldr	x8, [sp, #72]
	stur	x9, [x29, #-184]
	stur	x8, [x29, #-176]
	stur	x9, [x29, #-80]
	ldur	x9, [x29, #-80]
	stur	x9, [x29, #-88]
	ldur	x9, [x29, #-80]
	stur	x9, [x29, #-72]
	ldur	x9, [x29, #-80]
	stur	x9, [x29, #-32]
	ldur	x9, [x29, #-32]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	ldur	x9, [x29, #-48]
	ldur	x8, [x29, #-40]
	stur	x9, [x29, #-64]
	stur	x8, [x29, #-56]
	ldur	x9, [x29, #-64]
	ldur	x8, [x29, #-56]
	stur	x9, [x29, #-104]
	stur	x8, [x29, #-96]
	b	LBB114_21
LBB114_21:
	ldur	x0, [x29, #-104]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB114_22
LBB114_22:
	ldur	x9, [x29, #-104]
	ldur	x8, [x29, #-96]
	stur	x9, [x29, #-200]
	stur	x8, [x29, #-192]
	ldur	x9, [x29, #-200]
	ldur	x8, [x29, #-192]
	str	x9, [sp, #152]
	str	x8, [sp, #160]
	b	LBB114_3
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17h19babe651bef01fbE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN5alloc5slice4hack8into_vec17h9d90cd5a0bc09b8aE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17h7be03de987d6fb8bE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN5alloc5slice4hack8into_vec17hfa6b69c04823f469E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17h84bd0202cca5e00aE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN5alloc5slice4hack8into_vec17h0396a8e9e64907d2E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17h864456c55cce862dE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN5alloc5slice4hack8into_vec17he1598fa7922191a3E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17hb7db4ecec3366c8cE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN5alloc5slice4hack8into_vec17h20202821441f7e07E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17he0eec73e7b4c468bE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN5alloc5slice4hack8into_vec17h5887b2ddba310ec9E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice4hack8into_vec17h0396a8e9e64907d2E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp]
	str	x1, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	__ZN4core10intrinsics16retag_box_to_raw17h524749d3d092c407E
	ldr	x1, [sp, #8]
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17h63b182ec6bf518a6E
	ldr	x8, [sp]
	mov	x9, x1
	ldr	x1, [sp, #8]
	str	x0, [x8]
	str	x9, [x8, #8]
	str	x1, [x8, #16]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice4hack8into_vec17h20202821441f7e07E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp]
	str	x1, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	__ZN4core10intrinsics16retag_box_to_raw17hfe6dcf2a8466911aE
	ldr	x1, [sp, #8]
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17he190e64947a70b55E
	ldr	x8, [sp]
	mov	x9, x1
	ldr	x1, [sp, #8]
	str	x0, [x8]
	str	x9, [x8, #8]
	str	x1, [x8, #16]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice4hack8into_vec17h5887b2ddba310ec9E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp]
	str	x1, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	__ZN4core10intrinsics16retag_box_to_raw17h029517196ceba31fE
	ldr	x1, [sp, #8]
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17h1158bb01aeadba84E
	ldr	x8, [sp]
	mov	x9, x1
	ldr	x1, [sp, #8]
	str	x0, [x8]
	str	x9, [x8, #8]
	str	x1, [x8, #16]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice4hack8into_vec17h9d90cd5a0bc09b8aE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp]
	str	x1, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	__ZN4core10intrinsics16retag_box_to_raw17h2621a69217330d71E
	ldr	x1, [sp, #8]
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17hc7697f801d8f9e8eE
	ldr	x8, [sp]
	mov	x9, x1
	ldr	x1, [sp, #8]
	str	x0, [x8]
	str	x9, [x8, #8]
	str	x1, [x8, #16]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice4hack8into_vec17he1598fa7922191a3E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp]
	str	x1, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	__ZN4core10intrinsics16retag_box_to_raw17hc95473bcfbf489a7E
	ldr	x1, [sp, #8]
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17hf740983afe6d5cb7E
	ldr	x8, [sp]
	mov	x9, x1
	ldr	x1, [sp, #8]
	str	x0, [x8]
	str	x9, [x8, #8]
	str	x1, [x8, #16]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc5slice4hack8into_vec17hfa6b69c04823f469E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp]
	str	x1, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	__ZN4core10intrinsics16retag_box_to_raw17hf1c7cbc81a74151dE
	ldr	x1, [sp, #8]
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17h5326c119ec5fca9eE
	ldr	x8, [sp]
	mov	x9, x1
	ldr	x1, [sp, #8]
	str	x0, [x8]
	str	x9, [x8, #8]
	str	x1, [x8, #16]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec11finish_grow17hd3b44fe622d115baE:
	.cfi_startproc
	sub	sp, sp, #304
	.cfi_def_cfa_offset 304
	stp	x28, x27, [sp, #272]
	stp	x29, x30, [sp, #288]
	add	x29, sp, #288
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	str	x8, [sp, #24]
	str	x2, [sp, #32]
	str	x3, [sp, #40]
	str	x0, [sp, #48]
	str	x1, [sp, #56]
	ldr	x9, [sp, #48]
	mov	x8, #0
	subs	x9, x9, #0
	csinc	x8, x8, xzr, ne
	cbnz	x8, LBB127_2
	b	LBB127_1
LBB127_1:
	ldr	x8, [sp, #32]
	ldr	x10, [sp, #48]
	ldr	x9, [sp, #56]
	str	x10, [sp, #112]
	str	x9, [sp, #120]
	str	xzr, [sp, #104]
	ldr	x10, [sp, #112]
	ldr	x9, [sp, #120]
	str	x10, [sp, #88]
	str	x9, [sp, #96]
	str	xzr, [sp, #80]
	ldr	x10, [sp, #88]
	ldr	x9, [sp, #96]
	str	x10, [sp, #64]
	str	x9, [sp, #72]
	ldr	x8, [x8, #8]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.eq	LBB127_3
	b	LBB127_4
LBB127_2:
	ldr	x9, [sp, #24]
	adrp	x10, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x10, [x10, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x10, [sp, #112]
	str	x8, [sp, #120]
	mov	w8, #1
	str	x8, [sp, #104]
	ldr	x11, [sp, #112]
	ldr	x10, [sp, #120]
	stur	x11, [x29, #-96]
	stur	x10, [x29, #-88]
	ldur	x11, [x29, #-96]
	ldur	x10, [x29, #-88]
	str	x11, [sp, #88]
	str	x10, [sp, #96]
	str	x8, [sp, #80]
	ldr	x11, [sp, #88]
	ldr	x10, [sp, #96]
	str	x11, [sp, #128]
	str	x10, [sp, #136]
	ldr	x11, [sp, #128]
	ldr	x10, [sp, #136]
	stur	x11, [x29, #-80]
	stur	x10, [x29, #-72]
	ldur	x11, [x29, #-80]
	ldur	x10, [x29, #-72]
	str	x11, [x9, #8]
	str	x10, [x9, #16]
	str	x8, [x9]
	b	LBB127_11
LBB127_3:
	ldr	x8, [sp, #32]
	ldr	x9, [x8]
	str	x9, [sp, #8]
	ldr	x9, [x8, #8]
	ldr	x8, [x8, #16]
	stur	x9, [x29, #-128]
	stur	x8, [x29, #-120]
	ldur	x8, [x29, #-128]
	stur	x8, [x29, #-64]
	ldur	x8, [x29, #-64]
	ldr	x9, [sp, #64]
	stur	x9, [x29, #-56]
	ldur	x9, [x29, #-56]
	subs	x8, x8, x9
	cset	w8, eq
	str	w8, [sp, #20]
	b	LBB127_5
LBB127_4:
	ldr	x0, [sp, #40]
	ldr	x1, [sp, #64]
	ldr	x2, [sp, #72]
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17habc80d2b6492fff3E
	str	x0, [sp, #144]
	str	x1, [sp, #152]
	b	LBB127_7
LBB127_5:
	ldr	w0, [sp, #20]
	bl	__ZN4core4hint16assert_unchecked18precondition_check17h3d2b16db344f9c42E
	b	LBB127_6
LBB127_6:
	ldr	x1, [sp, #8]
	ldr	x0, [sp, #40]
	ldur	x2, [x29, #-128]
	ldur	x3, [x29, #-120]
	ldr	x4, [sp, #64]
	ldr	x5, [sp, #72]
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17h1104c178e6bcff75E
	str	x0, [sp, #144]
	str	x1, [sp, #152]
	b	LBB127_7
LBB127_7:
	ldr	x9, [sp, #144]
	ldr	x8, [sp, #152]
	stur	x9, [x29, #-112]
	stur	x8, [x29, #-104]
	ldur	x9, [x29, #-112]
	mov	x8, #0
	subs	x9, x9, #0
	csinc	x8, x8, xzr, ne
	cbnz	x8, LBB127_9
	b	LBB127_8
LBB127_8:
	ldr	x8, [sp, #24]
	ldur	x10, [x29, #-112]
	ldur	x9, [x29, #-104]
	str	x10, [x8, #8]
	str	x9, [x8, #16]
	str	xzr, [x8]
	b	LBB127_10
LBB127_9:
	ldr	x9, [sp, #24]
	ldr	x10, [sp, #64]
	ldr	x8, [sp, #72]
	stur	x10, [x29, #-32]
	stur	x8, [x29, #-24]
	ldur	x10, [x29, #-32]
	ldur	x8, [x29, #-24]
	stur	x10, [x29, #-48]
	stur	x8, [x29, #-40]
	ldur	x10, [x29, #-48]
	ldur	x8, [x29, #-40]
	str	x10, [x9, #8]
	str	x8, [x9, #16]
	mov	w8, #1
	str	x8, [x9]
	b	LBB127_10
LBB127_10:
	b	LBB127_11
LBB127_11:
	.cfi_def_cfa wsp, 304
	ldp	x29, x30, [sp, #288]
	ldp	x28, x27, [sp, #272]
	add	sp, sp, #304
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w27
	.cfi_restore w28
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec14handle_reserve17h533dacd39ebbbfbbE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp]
	str	x1, [sp, #8]
	ldr	x8, [sp]
	mov	x9, #-9223372036854775807
	subs	x8, x8, x9
	cset	x8, ne
	cbnz	x8, LBB128_2
	b	LBB128_1
LBB128_1:
	mov	x8, #-9223372036854775807
	str	x8, [sp, #16]
	b	LBB128_3
LBB128_2:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	stur	x9, [x29, #-16]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-16]
	subs	x8, x8, #0
	cset	x8, ne
	cbz	x8, LBB128_4
	b	LBB128_5
LBB128_3:
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB128_4:
	.cfi_restore_state
	bl	__ZN5alloc7raw_vec17capacity_overflow17hca84ee27eee447d1E
LBB128_5:
	ldur	x9, [x29, #-16]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x9, [sp, #32]
	ldr	x8, [sp, #40]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x8, [sp, #16]
	mov	x9, #-9223372036854775807
	subs	x8, x8, x9
	cset	x8, ne
	cbz	x8, LBB128_3
	b	LBB128_6
LBB128_6:
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	bl	__ZN5alloc5alloc18handle_alloc_error17h2a00f3202443143aE
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h06466c0943309046E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	x8, [sp]
	str	x0, [sp, #8]
	b	LBB129_1
LBB129_1:
	ldr	x8, [sp, #8]
	ldr	x8, [x8]
	cbnz	x8, LBB129_3
	b	LBB129_2
LBB129_2:
	b	LBB129_4
LBB129_3:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x11, [x8]
	mov	w10, #56
	mul	x10, x10, x11
	str	x10, [sp, #24]
	mov	w10, #8
	str	x10, [sp, #16]
	ldr	x8, [x8, #8]
	str	x8, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]
	ldr	x10, [sp, #16]
	ldr	x8, [sp, #24]
	str	x10, [sp, #40]
	str	x8, [sp, #48]
	ldr	q0, [sp, #32]
	str	q0, [x9]
	ldr	x8, [sp, #48]
	str	x8, [x9, #16]
	b	LBB129_5
LBB129_4:
	ldr	x8, [sp]
	str	xzr, [x8, #8]
	b	LBB129_5
LBB129_5:
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1496de62c603abeeE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	x8, [sp]
	str	x0, [sp, #8]
	b	LBB130_1
LBB130_1:
	ldr	x8, [sp, #8]
	ldr	x8, [x8]
	cbnz	x8, LBB130_3
	b	LBB130_2
LBB130_2:
	b	LBB130_4
LBB130_3:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x11, [x8]
	mov	w10, #56
	mul	x10, x10, x11
	str	x10, [sp, #24]
	mov	w10, #8
	str	x10, [sp, #16]
	ldr	x8, [x8, #8]
	str	x8, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]
	ldr	x10, [sp, #16]
	ldr	x8, [sp, #24]
	str	x10, [sp, #40]
	str	x8, [sp, #48]
	ldr	q0, [sp, #32]
	str	q0, [x9]
	ldr	x8, [sp, #48]
	str	x8, [x9, #16]
	b	LBB130_5
LBB130_4:
	ldr	x8, [sp]
	str	xzr, [x8, #8]
	b	LBB130_5
LBB130_5:
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1da824d8dc9d50e6E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	x8, [sp]
	str	x0, [sp, #8]
	b	LBB131_1
LBB131_1:
	ldr	x8, [sp, #8]
	ldr	x8, [x8]
	cbnz	x8, LBB131_3
	b	LBB131_2
LBB131_2:
	b	LBB131_4
LBB131_3:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x11, [x8]
	mov	w10, #48
	mul	x10, x10, x11
	str	x10, [sp, #24]
	mov	w10, #8
	str	x10, [sp, #16]
	ldr	x8, [x8, #8]
	str	x8, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]
	ldr	x10, [sp, #16]
	ldr	x8, [sp, #24]
	str	x10, [sp, #40]
	str	x8, [sp, #48]
	ldr	q0, [sp, #32]
	str	q0, [x9]
	ldr	x8, [sp, #48]
	str	x8, [x9, #16]
	b	LBB131_5
LBB131_4:
	ldr	x8, [sp]
	str	xzr, [x8, #8]
	b	LBB131_5
LBB131_5:
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7b2d212bfa12e6b0E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	x8, [sp]
	str	x0, [sp, #8]
	b	LBB132_1
LBB132_1:
	ldr	x8, [sp, #8]
	ldr	x8, [x8]
	cbnz	x8, LBB132_3
	b	LBB132_2
LBB132_2:
	b	LBB132_4
LBB132_3:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x11, [x8]
	mov	w10, #16
	mul	x10, x10, x11
	str	x10, [sp, #24]
	mov	w10, #8
	str	x10, [sp, #16]
	ldr	x8, [x8, #8]
	str	x8, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]
	ldr	x10, [sp, #16]
	ldr	x8, [sp, #24]
	str	x10, [sp, #40]
	str	x8, [sp, #48]
	ldr	q0, [sp, #32]
	str	q0, [x9]
	ldr	x8, [sp, #48]
	str	x8, [x9, #16]
	b	LBB132_5
LBB132_4:
	ldr	x8, [sp]
	str	xzr, [x8, #8]
	b	LBB132_5
LBB132_5:
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h89b16a4e698eba2bE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	x8, [sp]
	str	x0, [sp, #8]
	b	LBB133_1
LBB133_1:
	ldr	x8, [sp, #8]
	ldr	x8, [x8]
	cbnz	x8, LBB133_3
	b	LBB133_2
LBB133_2:
	b	LBB133_4
LBB133_3:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x11, [x8]
	mov	w10, #16
	mul	x10, x10, x11
	str	x10, [sp, #24]
	mov	w10, #8
	str	x10, [sp, #16]
	ldr	x8, [x8, #8]
	str	x8, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]
	ldr	x10, [sp, #16]
	ldr	x8, [sp, #24]
	str	x10, [sp, #40]
	str	x8, [sp, #48]
	ldr	q0, [sp, #32]
	str	q0, [x9]
	ldr	x8, [sp, #48]
	str	x8, [x9, #16]
	b	LBB133_5
LBB133_4:
	ldr	x8, [sp]
	str	xzr, [x8, #8]
	b	LBB133_5
LBB133_5:
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h98b026aacb6e4e14E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	x8, [sp]
	str	x0, [sp, #8]
	b	LBB134_1
LBB134_1:
	ldr	x8, [sp, #8]
	ldr	x8, [x8]
	cbnz	x8, LBB134_3
	b	LBB134_2
LBB134_2:
	b	LBB134_4
LBB134_3:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x11, [x8]
	mov	w10, #48
	mul	x10, x10, x11
	str	x10, [sp, #24]
	mov	w10, #8
	str	x10, [sp, #16]
	ldr	x8, [x8, #8]
	str	x8, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]
	ldr	x10, [sp, #16]
	ldr	x8, [sp, #24]
	str	x10, [sp, #40]
	str	x8, [sp, #48]
	ldr	q0, [sp, #32]
	str	q0, [x9]
	ldr	x8, [sp, #48]
	str	x8, [x9, #16]
	b	LBB134_5
LBB134_4:
	ldr	x8, [sp]
	str	xzr, [x8, #8]
	b	LBB134_5
LBB134_5:
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hc9a18b02e328c140E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	x8, [sp]
	str	x0, [sp, #8]
	b	LBB135_1
LBB135_1:
	ldr	x8, [sp, #8]
	ldr	x8, [x8]
	cbnz	x8, LBB135_3
	b	LBB135_2
LBB135_2:
	b	LBB135_4
LBB135_3:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x11, [x8]
	mov	w10, #8
	mul	x11, x10, x11
	str	x11, [sp, #24]
	str	x10, [sp, #16]
	ldr	x8, [x8, #8]
	str	x8, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]
	ldr	x10, [sp, #16]
	ldr	x8, [sp, #24]
	str	x10, [sp, #40]
	str	x8, [sp, #48]
	ldr	q0, [sp, #32]
	str	q0, [x9]
	ldr	x8, [sp, #48]
	str	x8, [x9, #16]
	b	LBB135_5
LBB135_4:
	ldr	x8, [sp]
	str	xzr, [x8, #8]
	b	LBB135_5
LBB135_5:
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hd12c8d1d3bd6f368E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	x8, [sp]
	str	x0, [sp, #8]
	b	LBB136_1
LBB136_1:
	ldr	x8, [sp, #8]
	ldr	x8, [x8]
	cbnz	x8, LBB136_3
	b	LBB136_2
LBB136_2:
	b	LBB136_4
LBB136_3:
	ldr	x9, [sp]
	ldr	x8, [sp, #8]
	ldr	x11, [x8]
	mov	w10, #104
	mul	x10, x10, x11
	str	x10, [sp, #24]
	mov	w10, #8
	str	x10, [sp, #16]
	ldr	x8, [x8, #8]
	str	x8, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #32]
	ldr	x10, [sp, #16]
	ldr	x8, [sp, #24]
	str	x10, [sp, #40]
	str	x8, [sp, #48]
	ldr	q0, [sp, #32]
	str	q0, [x9]
	ldr	x8, [sp, #48]
	str	x8, [x9, #16]
	b	LBB136_5
LBB136_4:
	ldr	x8, [sp]
	str	xzr, [x8, #8]
	b	LBB136_5
LBB136_5:
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h1a1c82e1be02e7d3E:
	.cfi_startproc
	sub	sp, sp, #352
	.cfi_def_cfa_offset 352
	stp	x28, x27, [sp, #320]
	stp	x29, x30, [sp, #336]
	add	x29, sp, #336
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	.cfi_remember_state
	str	x0, [sp, #48]
	str	x1, [sp, #56]
	str	x2, [sp, #64]
	b	LBB137_1
LBB137_1:
	ldr	x8, [sp, #56]
	ldr	x9, [sp, #64]
	adds	x8, x8, x9
	str	x8, [sp, #40]
	cset	w8, hs
	sturb	w8, [x29, #-17]
	ldurb	w8, [x29, #-17]
	tbnz	w8, #0, LBB137_3
	b	LBB137_2
LBB137_2:
	ldr	x8, [sp, #48]
	ldr	x9, [sp, #40]
	str	x9, [sp, #128]
	mov	w9, #1
	str	x9, [sp, #120]
	ldr	x9, [sp, #128]
	str	x9, [sp, #112]
	mov	x9, #-9223372036854775807
	str	x9, [sp, #104]
	ldr	x10, [sp, #112]
	str	x10, [sp, #96]
	str	x9, [sp, #88]
	ldr	x1, [sp, #96]
	ldr	x8, [x8]
	lsl	x0, x8, #1
	bl	__ZN4core3cmp6max_by17h43b9b5ac048f07d6E
	mov	x1, x0
	mov	w8, #4
	mov	x0, x8
	bl	__ZN4core3cmp6max_by17h43b9b5ac048f07d6E
	mov	x2, x0
	str	x2, [sp, #8]
	mov	w8, #16
	mov	x0, x8
	mov	w8, #8
	mov	x1, x8
	bl	__ZN4core5alloc6layout6Layout5array5inner17h6f4b5b5253e62597E
	mov	x2, x0
	ldr	x0, [sp, #48]
	str	x2, [sp, #16]
	str	x1, [sp, #24]
	sub	x8, x29, #136
	str	x8, [sp, #32]
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7b2d212bfa12e6b0E
	ldr	x8, [sp, #48]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	ldr	x2, [sp, #32]
	add	x3, x8, #16
	sub	x8, x29, #160
	bl	__ZN5alloc7raw_vec11finish_grow17hd3b44fe622d115baE
	ldur	x8, [x29, #-160]
	cbz	x8, LBB137_4
	b	LBB137_5
LBB137_3:
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x11, [x9, l___unnamed_7@PAGEOFF]
	ldr	x10, [x8, #8]
	str	x11, [sp, #120]
	str	x10, [sp, #128]
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #104]
	str	x8, [sp, #112]
	ldr	x9, [sp, #104]
	ldr	x8, [sp, #112]
	stur	x9, [x29, #-96]
	stur	x8, [x29, #-88]
	ldur	x9, [x29, #-96]
	ldur	x8, [x29, #-88]
	str	x9, [sp, #88]
	str	x8, [sp, #96]
	ldr	x9, [sp, #88]
	ldr	x8, [sp, #96]
	str	x9, [sp, #136]
	str	x8, [sp, #144]
	ldr	x9, [sp, #136]
	ldr	x8, [sp, #144]
	stur	x9, [x29, #-80]
	stur	x8, [x29, #-72]
	ldur	x9, [x29, #-80]
	ldur	x8, [x29, #-72]
	str	x9, [sp, #72]
	str	x8, [sp, #80]
	b	LBB137_7
LBB137_4:
	ldr	x9, [sp, #48]
	ldr	x8, [sp, #8]
	ldur	x11, [x29, #-152]
	ldur	x10, [x29, #-144]
	str	x11, [sp, #160]
	str	x10, [sp, #168]
	str	xzr, [sp, #152]
	ldr	x10, [sp, #160]
	stur	x10, [x29, #-40]
	ldur	x10, [x29, #-40]
	stur	x10, [x29, #-48]
	ldur	x10, [x29, #-48]
	str	x10, [x9, #8]
	stur	x8, [x29, #-32]
	ldur	x8, [x29, #-32]
	str	x8, [x9]
	adrp	x9, l___unnamed_23@PAGE
	adrp	x8, l___unnamed_23@PAGE
	add	x8, x8, l___unnamed_23@PAGEOFF
	ldr	x9, [x9, l___unnamed_23@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #72]
	str	x8, [sp, #80]
	b	LBB137_6
LBB137_5:
	ldur	x9, [x29, #-152]
	ldur	x8, [x29, #-144]
	stur	x9, [x29, #-64]
	stur	x8, [x29, #-56]
	ldur	x9, [x29, #-64]
	ldur	x8, [x29, #-56]
	str	x9, [sp, #160]
	str	x8, [sp, #168]
	mov	w8, #1
	str	x8, [sp, #152]
	ldr	x9, [sp, #160]
	ldr	x8, [sp, #168]
	stur	x9, [x29, #-112]
	stur	x8, [x29, #-104]
	ldur	x9, [x29, #-112]
	ldur	x8, [x29, #-104]
	str	x9, [sp, #72]
	str	x8, [sp, #80]
	b	LBB137_7
LBB137_6:
	ldr	x0, [sp, #72]
	ldr	x1, [sp, #80]
	.cfi_def_cfa wsp, 352
	ldp	x29, x30, [sp, #336]
	ldp	x28, x27, [sp, #320]
	add	sp, sp, #352
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB137_7:
	.cfi_restore_state
	b	LBB137_6
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h3f8eda4e4276eca6E:
	.cfi_startproc
	sub	sp, sp, #352
	.cfi_def_cfa_offset 352
	stp	x28, x27, [sp, #320]
	stp	x29, x30, [sp, #336]
	add	x29, sp, #336
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	.cfi_remember_state
	str	x0, [sp, #48]
	str	x1, [sp, #56]
	str	x2, [sp, #64]
	b	LBB138_1
LBB138_1:
	ldr	x8, [sp, #56]
	ldr	x9, [sp, #64]
	adds	x8, x8, x9
	str	x8, [sp, #40]
	cset	w8, hs
	sturb	w8, [x29, #-17]
	ldurb	w8, [x29, #-17]
	tbnz	w8, #0, LBB138_3
	b	LBB138_2
LBB138_2:
	ldr	x8, [sp, #48]
	ldr	x9, [sp, #40]
	str	x9, [sp, #128]
	mov	w9, #1
	str	x9, [sp, #120]
	ldr	x9, [sp, #128]
	str	x9, [sp, #112]
	mov	x9, #-9223372036854775807
	str	x9, [sp, #104]
	ldr	x10, [sp, #112]
	str	x10, [sp, #96]
	str	x9, [sp, #88]
	ldr	x1, [sp, #96]
	ldr	x8, [x8]
	lsl	x0, x8, #1
	bl	__ZN4core3cmp6max_by17h43b9b5ac048f07d6E
	mov	x1, x0
	mov	w8, #4
	mov	x0, x8
	bl	__ZN4core3cmp6max_by17h43b9b5ac048f07d6E
	mov	x2, x0
	str	x2, [sp, #8]
	mov	w8, #8
	mov	x1, x8
	mov	x0, x1
	bl	__ZN4core5alloc6layout6Layout5array5inner17h6f4b5b5253e62597E
	mov	x2, x0
	ldr	x0, [sp, #48]
	str	x2, [sp, #16]
	str	x1, [sp, #24]
	sub	x8, x29, #136
	str	x8, [sp, #32]
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hc9a18b02e328c140E
	ldr	x8, [sp, #48]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	ldr	x2, [sp, #32]
	add	x3, x8, #16
	sub	x8, x29, #160
	bl	__ZN5alloc7raw_vec11finish_grow17hd3b44fe622d115baE
	ldur	x8, [x29, #-160]
	cbz	x8, LBB138_4
	b	LBB138_5
LBB138_3:
	adrp	x9, l___unnamed_7@PAGE
	adrp	x8, l___unnamed_7@PAGE
	add	x8, x8, l___unnamed_7@PAGEOFF
	ldr	x11, [x9, l___unnamed_7@PAGEOFF]
	ldr	x10, [x8, #8]
	str	x11, [sp, #120]
	str	x10, [sp, #128]
	ldr	x9, [x9, l___unnamed_7@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #104]
	str	x8, [sp, #112]
	ldr	x9, [sp, #104]
	ldr	x8, [sp, #112]
	stur	x9, [x29, #-96]
	stur	x8, [x29, #-88]
	ldur	x9, [x29, #-96]
	ldur	x8, [x29, #-88]
	str	x9, [sp, #88]
	str	x8, [sp, #96]
	ldr	x9, [sp, #88]
	ldr	x8, [sp, #96]
	str	x9, [sp, #136]
	str	x8, [sp, #144]
	ldr	x9, [sp, #136]
	ldr	x8, [sp, #144]
	stur	x9, [x29, #-80]
	stur	x8, [x29, #-72]
	ldur	x9, [x29, #-80]
	ldur	x8, [x29, #-72]
	str	x9, [sp, #72]
	str	x8, [sp, #80]
	b	LBB138_7
LBB138_4:
	ldr	x9, [sp, #48]
	ldr	x8, [sp, #8]
	ldur	x11, [x29, #-152]
	ldur	x10, [x29, #-144]
	str	x11, [sp, #160]
	str	x10, [sp, #168]
	str	xzr, [sp, #152]
	ldr	x10, [sp, #160]
	stur	x10, [x29, #-40]
	ldur	x10, [x29, #-40]
	stur	x10, [x29, #-48]
	ldur	x10, [x29, #-48]
	str	x10, [x9, #8]
	stur	x8, [x29, #-32]
	ldur	x8, [x29, #-32]
	str	x8, [x9]
	adrp	x9, l___unnamed_23@PAGE
	adrp	x8, l___unnamed_23@PAGE
	add	x8, x8, l___unnamed_23@PAGEOFF
	ldr	x9, [x9, l___unnamed_23@PAGEOFF]
	ldr	x8, [x8, #8]
	str	x9, [sp, #72]
	str	x8, [sp, #80]
	b	LBB138_6
LBB138_5:
	ldur	x9, [x29, #-152]
	ldur	x8, [x29, #-144]
	stur	x9, [x29, #-64]
	stur	x8, [x29, #-56]
	ldur	x9, [x29, #-64]
	ldur	x8, [x29, #-56]
	str	x9, [sp, #160]
	str	x8, [sp, #168]
	mov	w8, #1
	str	x8, [sp, #152]
	ldr	x9, [sp, #160]
	ldr	x8, [sp, #168]
	stur	x9, [x29, #-112]
	stur	x8, [x29, #-104]
	ldur	x9, [x29, #-112]
	ldur	x8, [x29, #-104]
	str	x9, [sp, #72]
	str	x8, [sp, #80]
	b	LBB138_7
LBB138_6:
	ldr	x0, [sp, #72]
	ldr	x1, [sp, #80]
	.cfi_def_cfa wsp, 352
	ldp	x29, x30, [sp, #336]
	ldp	x28, x27, [sp, #320]
	add	sp, sp, #352
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB138_7:
	.cfi_restore_state
	b	LBB138_6
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17hd59edee4349f66d8E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w8, #1
	mov	x2, x8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h3f8eda4e4276eca6E
	bl	__ZN5alloc7raw_vec14handle_reserve17h533dacd39ebbbfbbE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16reserve_for_push17heb9b076b3a0b5e8eE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w8, #1
	mov	x2, x8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17h1a1c82e1be02e7d3E
	bl	__ZN5alloc7raw_vec14handle_reserve17h533dacd39ebbbfbbE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17h1158bb01aeadba84E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	b	LBB141_1
LBB141_1:
	ldr	x8, [sp, #16]
	stur	x8, [x29, #-24]
	b	LBB141_2
LBB141_2:
	b	LBB141_3
LBB141_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB141_4
LBB141_4:
	ldr	x8, [sp, #8]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	stur	x8, [x29, #-16]
	ldur	x8, [x29, #-24]
	ldur	x9, [x29, #-16]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17h5326c119ec5fca9eE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	b	LBB142_1
LBB142_1:
	ldr	x8, [sp, #16]
	stur	x8, [x29, #-24]
	b	LBB142_2
LBB142_2:
	b	LBB142_3
LBB142_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB142_4
LBB142_4:
	ldr	x8, [sp, #8]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	stur	x8, [x29, #-16]
	ldur	x8, [x29, #-24]
	ldur	x9, [x29, #-16]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17h63b182ec6bf518a6E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	b	LBB143_1
LBB143_1:
	ldr	x8, [sp, #16]
	stur	x8, [x29, #-24]
	b	LBB143_2
LBB143_2:
	b	LBB143_3
LBB143_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB143_4
LBB143_4:
	ldr	x8, [sp, #8]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	stur	x8, [x29, #-16]
	ldur	x8, [x29, #-24]
	ldur	x9, [x29, #-16]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17hc7697f801d8f9e8eE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	b	LBB144_1
LBB144_1:
	ldr	x8, [sp, #16]
	stur	x8, [x29, #-24]
	b	LBB144_2
LBB144_2:
	b	LBB144_3
LBB144_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB144_4
LBB144_4:
	ldr	x8, [sp, #8]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	stur	x8, [x29, #-16]
	ldur	x8, [x29, #-24]
	ldur	x9, [x29, #-16]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17he190e64947a70b55E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	b	LBB145_1
LBB145_1:
	ldr	x8, [sp, #16]
	stur	x8, [x29, #-24]
	b	LBB145_2
LBB145_2:
	b	LBB145_3
LBB145_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB145_4
LBB145_4:
	ldr	x8, [sp, #8]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	stur	x8, [x29, #-16]
	ldur	x8, [x29, #-24]
	ldur	x9, [x29, #-16]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$17from_raw_parts_in17hf740983afe6d5cb7E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	b	LBB146_1
LBB146_1:
	ldr	x8, [sp, #16]
	stur	x8, [x29, #-24]
	b	LBB146_2
LBB146_2:
	b	LBB146_3
LBB146_3:
	ldr	x0, [sp, #8]
	bl	__ZN4core3ptr8non_null16NonNull$LT$T$GT$13new_unchecked18precondition_check17h014695bd6199ce65E
	b	LBB146_4
LBB146_4:
	ldr	x8, [sp, #8]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	stur	x8, [x29, #-16]
	ldur	x8, [x29, #-24]
	ldur	x9, [x29, #-16]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h2a5963f7401b2db4E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h7627f43ee7cd2253E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17hc70e992a18f362a5E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp]
	str	x2, [sp, #8]
	str	x3, [sp, #16]
	ldr	x8, [sp, #16]
	cbnz	x8, LBB150_2
	b	LBB150_1
LBB150_1:
	b	LBB150_3
LBB150_2:
	ldr	x0, [sp]
	ldr	x9, [sp, #8]
	ldr	x8, [sp, #16]
	str	x9, [sp, #24]
	str	x8, [sp, #32]
	ldr	x1, [sp, #32]
	ldr	x8, [sp, #24]
	stur	x8, [x29, #-8]
	ldur	x2, [x29, #-8]
	bl	___rust_dealloc
	b	LBB150_3
LBB150_3:
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$4grow17h1104c178e6bcff75E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w8, #0
	and	w6, w8, #0x1
	bl	__ZN5alloc5alloc6Global9grow_impl17h186b12e5261a8b98E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17habc80d2b6492fff3E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w8, #0
	and	w3, w8, #0x1
	bl	__ZN5alloc5alloc6Global10alloc_impl17h55da341c4b5c6e3dE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN64_$LT$$RF$i32$u20$as$u20$core..ops..arith..Add$LT$$RF$i32$GT$$GT$3add17haffeed548419eb45E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x2, [sp]
	ldr	w8, [x0]
	ldr	w9, [x1]
	adds	w8, w8, w9
	stur	w8, [x29, #-4]
	cset	w8, vs
	tbnz	w8, #0, LBB153_2
	b	LBB153_1
LBB153_1:
	ldur	w0, [x29, #-4]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB153_2:
	.cfi_restore_state
	ldr	x2, [sp]
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN64_$LT$$RF$i32$u20$as$u20$core..ops..arith..Div$LT$$RF$i32$GT$$GT$3div17hd29fdd77f45d4c6cE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x2, [sp]
	ldr	w8, [x0]
	str	w8, [sp, #8]
	ldr	w8, [x1]
	stur	w8, [x29, #-4]
	cbz	w8, LBB154_2
	b	LBB154_1
LBB154_1:
	ldr	w9, [sp, #8]
	ldur	w8, [x29, #-4]
	adds	w8, w8, #1
	cset	w8, eq
	mov	w10, #-2147483648
	subs	w9, w9, w10
	cset	w9, eq
	and	w8, w8, w9
	tbnz	w8, #0, LBB154_4
	b	LBB154_3
LBB154_2:
	ldr	x2, [sp]
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #25
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
LBB154_3:
	ldr	w8, [sp, #8]
	ldur	w9, [x29, #-4]
	sdiv	w0, w8, w9
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB154_4:
	.cfi_restore_state
	ldr	x2, [sp]
	adrp	x0, _str.3@PAGE
	add	x0, x0, _str.3@PAGEOFF
	mov	w8, #31
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN64_$LT$$RF$i32$u20$as$u20$core..ops..arith..Mul$LT$$RF$i32$GT$$GT$3mul17h09851e64bde045bcE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x2, [sp]
	ldr	w9, [x0]
	ldr	w10, [x1]
	smull	x8, w9, w10
	asr	x8, x8, #32
	mul	w9, w9, w10
	stur	w9, [x29, #-4]
	subs	w8, w8, w9, asr #31
	b.ne	LBB155_2
	b	LBB155_1
LBB155_1:
	ldur	w0, [x29, #-4]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB155_2:
	.cfi_restore_state
	ldr	x2, [sp]
	adrp	x0, _str.4@PAGE
	add	x0, x0, _str.4@PAGEOFF
	mov	w8, #33
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN64_$LT$$RF$i32$u20$as$u20$core..ops..arith..Sub$LT$$RF$i32$GT$$GT$3sub17h94283ebb94365b9cE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x2, [sp]
	ldr	w8, [x0]
	ldr	w9, [x1]
	subs	w8, w8, w9
	stur	w8, [x29, #-4]
	cset	w8, vs
	tbnz	w8, #0, LBB156_2
	b	LBB156_1
LBB156_1:
	ldur	w0, [x29, #-4]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB156_2:
	.cfi_restore_state
	ldr	x2, [sp]
	adrp	x0, _str.5@PAGE
	add	x0, x0, _str.5@PAGEOFF
	mov	w8, #33
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN64_$LT$$RF$i64$u20$as$u20$core..ops..arith..Add$LT$$RF$i64$GT$$GT$3add17he18c087e1fb0253bE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x2, [sp]
	ldr	x8, [x0]
	ldr	x9, [x1]
	adds	x8, x8, x9
	str	x8, [sp, #8]
	cset	w8, vs
	tbnz	w8, #0, LBB157_2
	b	LBB157_1
LBB157_1:
	ldr	x0, [sp, #8]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB157_2:
	.cfi_restore_state
	ldr	x2, [sp]
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN64_$LT$$RF$i64$u20$as$u20$core..ops..arith..Div$LT$$RF$i64$GT$$GT$3div17ha0b42afe14a389baE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x2, [sp, #8]
	ldr	x8, [x0]
	str	x8, [sp, #16]
	ldr	x8, [x1]
	stur	x8, [x29, #-8]
	cbz	x8, LBB158_2
	b	LBB158_1
LBB158_1:
	ldr	x9, [sp, #16]
	ldur	x8, [x29, #-8]
	adds	x8, x8, #1
	cset	w8, eq
	mov	x10, #-9223372036854775808
	subs	x9, x9, x10
	cset	w9, eq
	and	w8, w8, w9
	tbnz	w8, #0, LBB158_4
	b	LBB158_3
LBB158_2:
	ldr	x2, [sp, #8]
	adrp	x0, _str.0@PAGE
	add	x0, x0, _str.0@PAGEOFF
	mov	w8, #25
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
LBB158_3:
	ldr	x8, [sp, #16]
	ldur	x9, [x29, #-8]
	sdiv	x0, x8, x9
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB158_4:
	.cfi_restore_state
	ldr	x2, [sp, #8]
	adrp	x0, _str.3@PAGE
	add	x0, x0, _str.3@PAGEOFF
	mov	w8, #31
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN64_$LT$$RF$i64$u20$as$u20$core..ops..arith..Mul$LT$$RF$i64$GT$$GT$3mul17h15a4e94dd23dc741E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x2, [sp]
	ldr	x9, [x0]
	ldr	x10, [x1]
	smulh	x8, x9, x10
	mul	x9, x9, x10
	str	x9, [sp, #8]
	subs	x8, x8, x9, asr #63
	b.ne	LBB159_2
	b	LBB159_1
LBB159_1:
	ldr	x0, [sp, #8]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB159_2:
	.cfi_restore_state
	ldr	x2, [sp]
	adrp	x0, _str.4@PAGE
	add	x0, x0, _str.4@PAGEOFF
	mov	w8, #33
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN64_$LT$$RF$i64$u20$as$u20$core..ops..arith..Sub$LT$$RF$i64$GT$$GT$3sub17h2d6f0cbc91a90a8fE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x2, [sp]
	ldr	x8, [x0]
	ldr	x9, [x1]
	subs	x8, x8, x9
	str	x8, [sp, #8]
	cset	w8, vs
	tbnz	w8, #0, LBB160_2
	b	LBB160_1
LBB160_1:
	ldr	x0, [sp, #8]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB160_2:
	.cfi_restore_state
	ldr	x2, [sp]
	adrp	x0, _str.5@PAGE
	add	x0, x0, _str.5@PAGEOFF
	mov	w8, #33
	mov	x1, x8
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h10da48dbecf85df7E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp, #8]
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h70cbdade11d89f06E
	ldr	x2, [sp, #8]
	bl	__ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd9b6460679eaffa7E
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hd52e55f810623b49E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp, #8]
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h6346f7eaf8ebae4cE
	ldr	x2, [sp, #8]
	bl	__ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17h6147861da6c066dcE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h180872de86a57143E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x9, [x0, #8]
	ldr	x8, [x0, #16]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1a22e1d4b76bb115E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x9, [x0, #8]
	ldr	x8, [x0, #16]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h231c7dc41ab0bd21E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x9, [x0, #8]
	ldr	x8, [x0, #16]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17ha0f625439a49ea68E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x9, [x0, #8]
	ldr	x8, [x0, #16]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17ha4da5c1ab779e773E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x9, [x0, #8]
	ldr	x8, [x0, #16]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hb5f518fa518ba4b4E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x9, [x0, #8]
	ldr	x8, [x0, #16]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hbb1ff3e1f6843a73E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x9, [x0, #8]
	ldr	x8, [x0, #16]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17heedb86448cd32201E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	ldr	x9, [x0, #8]
	ldr	x8, [x0, #16]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h1f2c720a5dbee91aE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x8, [x0]
	str	x8, [sp, #16]
	mov	w8, #816
	stur	x8, [x29, #-16]
	ldur	x9, [x29, #-16]
	mov	w8, #8
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x8, [sp, #32]
	cbnz	x8, LBB171_2
	b	LBB171_1
LBB171_1:
	b	LBB171_3
LBB171_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	add	x0, x9, #8
	stur	x8, [x29, #-24]
	ldr	x2, [sp, #24]
	ldr	x3, [sp, #32]
	ldur	x1, [x29, #-24]
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB171_3
LBB171_3:
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h23d957b2c2c2dd12E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x8, [x0]
	str	x8, [sp, #16]
	mov	w8, #952
	stur	x8, [x29, #-16]
	ldur	x9, [x29, #-16]
	mov	w8, #8
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x8, [sp, #32]
	cbnz	x8, LBB172_2
	b	LBB172_1
LBB172_1:
	b	LBB172_3
LBB172_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	add	x0, x9, #8
	stur	x8, [x29, #-24]
	ldr	x2, [sp, #24]
	ldr	x3, [sp, #32]
	ldur	x1, [x29, #-24]
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB172_3
LBB172_3:
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h42ee07ad48cc5cbcE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x8, [x0]
	str	x8, [sp, #16]
	mov	w8, #1768
	stur	x8, [x29, #-16]
	ldur	x9, [x29, #-16]
	mov	w8, #8
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x8, [sp, #32]
	cbnz	x8, LBB173_2
	b	LBB173_1
LBB173_1:
	b	LBB173_3
LBB173_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	add	x0, x9, #8
	stur	x8, [x29, #-24]
	ldr	x2, [sp, #24]
	ldr	x3, [sp, #32]
	ldur	x1, [x29, #-24]
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB173_3
LBB173_3:
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h5bdf4cd9537f880aE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x8, [x0]
	str	x8, [sp, #16]
	mov	w8, #672
	stur	x8, [x29, #-16]
	ldur	x9, [x29, #-16]
	mov	w8, #8
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x8, [sp, #32]
	cbnz	x8, LBB174_2
	b	LBB174_1
LBB174_1:
	b	LBB174_3
LBB174_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	add	x0, x9, #8
	stur	x8, [x29, #-24]
	ldr	x2, [sp, #24]
	ldr	x3, [sp, #32]
	ldur	x1, [x29, #-24]
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB174_3
LBB174_3:
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h95337d73c9fc504dE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x8, [x0]
	str	x8, [sp, #16]
	mov	w8, #672
	stur	x8, [x29, #-16]
	ldur	x9, [x29, #-16]
	mov	w8, #8
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x8, [sp, #32]
	cbnz	x8, LBB175_2
	b	LBB175_1
LBB175_1:
	b	LBB175_3
LBB175_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	add	x0, x9, #8
	stur	x8, [x29, #-24]
	ldr	x2, [sp, #24]
	ldr	x3, [sp, #32]
	ldur	x1, [x29, #-24]
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB175_3
LBB175_3:
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h39b8db764d9e69c3E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x8, [x0, #8]
	str	x8, [sp]
	ldr	x8, [x0, #16]
	str	x8, [sp, #8]
	b	LBB176_1
LBB176_1:
	ldr	x3, [sp, #8]
	ldr	x0, [sp]
	mov	w8, #16
	mov	x1, x8
	mov	w8, #8
	mov	x2, x8
	bl	__ZN4core5slice3raw14from_raw_parts18precondition_check17h44775545258936e7E
	b	LBB176_2
LBB176_2:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	stur	x9, [x29, #-16]
	stur	x8, [x29, #-8]
	ldur	x9, [x29, #-16]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h6346f7eaf8ebae4cE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x8, [x0, #8]
	str	x8, [sp]
	ldr	x8, [x0, #16]
	str	x8, [sp, #8]
	b	LBB177_1
LBB177_1:
	ldr	x3, [sp, #8]
	ldr	x0, [sp]
	mov	w8, #8
	mov	x2, x8
	mov	x1, x2
	bl	__ZN4core5slice3raw14from_raw_parts18precondition_check17h44775545258936e7E
	b	LBB177_2
LBB177_2:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	stur	x9, [x29, #-16]
	stur	x8, [x29, #-8]
	ldur	x9, [x29, #-16]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h70cbdade11d89f06E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x8, [x0, #8]
	str	x8, [sp]
	ldr	x8, [x0, #16]
	str	x8, [sp, #8]
	b	LBB178_1
LBB178_1:
	ldr	x3, [sp, #8]
	ldr	x0, [sp]
	mov	w8, #16
	mov	x1, x8
	mov	w8, #8
	mov	x2, x8
	bl	__ZN4core5slice3raw14from_raw_parts18precondition_check17h44775545258936e7E
	b	LBB178_2
LBB178_2:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	stur	x9, [x29, #-16]
	stur	x8, [x29, #-8]
	ldur	x9, [x29, #-16]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h750abde33ecac153E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x8, [x0, #8]
	str	x8, [sp]
	ldr	x8, [x0, #16]
	str	x8, [sp, #8]
	b	LBB179_1
LBB179_1:
	ldr	x3, [sp, #8]
	ldr	x0, [sp]
	mov	w8, #48
	mov	x1, x8
	mov	w8, #8
	mov	x2, x8
	bl	__ZN4core5slice3raw14from_raw_parts18precondition_check17h44775545258936e7E
	b	LBB179_2
LBB179_2:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	stur	x9, [x29, #-16]
	stur	x8, [x29, #-8]
	ldur	x9, [x29, #-16]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h8afc42426ac1b360E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x8, [x0, #8]
	str	x8, [sp]
	ldr	x8, [x0, #16]
	str	x8, [sp, #8]
	b	LBB180_1
LBB180_1:
	ldr	x3, [sp, #8]
	ldr	x0, [sp]
	mov	w8, #104
	mov	x1, x8
	mov	w8, #8
	mov	x2, x8
	bl	__ZN4core5slice3raw14from_raw_parts18precondition_check17h44775545258936e7E
	b	LBB180_2
LBB180_2:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	stur	x9, [x29, #-16]
	stur	x8, [x29, #-8]
	ldur	x9, [x29, #-16]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17hcf881085915acf78E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x8, [x0, #8]
	str	x8, [sp]
	ldr	x8, [x0, #16]
	str	x8, [sp, #8]
	b	LBB181_1
LBB181_1:
	ldr	x3, [sp, #8]
	ldr	x0, [sp]
	mov	w8, #56
	mov	x1, x8
	mov	w8, #8
	mov	x2, x8
	bl	__ZN4core5slice3raw14from_raw_parts18precondition_check17h44775545258936e7E
	b	LBB181_2
LBB181_2:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	stur	x9, [x29, #-16]
	stur	x8, [x29, #-8]
	ldur	x9, [x29, #-16]
	ldur	x8, [x29, #-8]
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x1, [sp, #24]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17h14d35e93823784dbE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	str	x2, [sp, #24]
	b	LBB182_1
LBB182_1:
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #24]
	ldr	x9, [sp, #16]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x1, [sp, #40]
	bl	__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked18precondition_check17h0d9f1c062821a537E
	b	LBB182_2
LBB182_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	ldr	x10, [sp, #24]
	stur	x8, [x29, #-16]
	stur	x10, [x29, #-8]
	mov	w10, #24
	mul	x9, x9, x10
	add	x0, x8, x9
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17h575587ebed97f6beE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	str	x2, [sp, #24]
	b	LBB183_1
LBB183_1:
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #24]
	ldr	x9, [sp, #16]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x1, [sp, #40]
	bl	__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked18precondition_check17h0d9f1c062821a537E
	b	LBB183_2
LBB183_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	ldr	x10, [sp, #24]
	stur	x8, [x29, #-16]
	stur	x10, [x29, #-8]
	add	x0, x8, x9, lsl #4
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hcb920bf91edc27bdE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	str	x2, [sp, #24]
	b	LBB184_1
LBB184_1:
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #24]
	ldr	x9, [sp, #16]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x1, [sp, #40]
	bl	__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked18precondition_check17h0d9f1c062821a537E
	b	LBB184_2
LBB184_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	ldr	x10, [sp, #24]
	stur	x8, [x29, #-16]
	stur	x10, [x29, #-8]
	add	x0, x8, x9, lsl #4
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked17hcd94d25b0092102dE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	str	x2, [sp, #24]
	b	LBB185_1
LBB185_1:
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #24]
	ldr	x9, [sp, #16]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x1, [sp, #40]
	bl	__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked18precondition_check17h0d9f1c062821a537E
	b	LBB185_2
LBB185_2:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	ldr	x10, [sp, #24]
	stur	x8, [x29, #-16]
	stur	x10, [x29, #-8]
	add	x0, x8, x9, lsl #3
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN75_$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$13get_unchecked18precondition_check17h0d9f1c062821a537E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	subs	x8, x0, x1
	b.lo	LBB186_2
	b	LBB186_1
LBB186_1:
	adrp	x0, l___unnamed_24@PAGE
	add	x0, x0, l___unnamed_24@PAGEOFF
	mov	w8, #97
	mov	x1, x8
	bl	__ZN4core9panicking14panic_nounwind17h765f3648d339fa95E
LBB186_2:
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h05557965ee1802d3E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	add	x8, sp, #8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h7b2d212bfa12e6b0E
	ldr	x8, [sp, #16]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB187_2
	b	LBB187_1
LBB187_1:
	ldr	x8, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	add	x0, x8, #16
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB187_2
LBB187_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h2c166a266ee37d1eE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	add	x8, sp, #8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1da824d8dc9d50e6E
	ldr	x8, [sp, #16]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB188_2
	b	LBB188_1
LBB188_1:
	ldr	x8, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	add	x0, x8, #16
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB188_2
LBB188_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h6bf902f7bfb0ce56E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	add	x8, sp, #8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h1496de62c603abeeE
	ldr	x8, [sp, #16]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB189_2
	b	LBB189_1
LBB189_1:
	ldr	x8, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	add	x0, x8, #16
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB189_2
LBB189_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h88c485db797c154fE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	add	x8, sp, #8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h06466c0943309046E
	ldr	x8, [sp, #16]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB190_2
	b	LBB190_1
LBB190_1:
	ldr	x8, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	add	x0, x8, #16
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB190_2
LBB190_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h96d84aa33aca4a86E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	add	x8, sp, #8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h98b026aacb6e4e14E
	ldr	x8, [sp, #16]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB191_2
	b	LBB191_1
LBB191_1:
	ldr	x8, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	add	x0, x8, #16
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB191_2
LBB191_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hc00eaaabd0855f1eE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	add	x8, sp, #8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hc9a18b02e328c140E
	ldr	x8, [sp, #16]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB192_2
	b	LBB192_1
LBB192_1:
	ldr	x8, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	add	x0, x8, #16
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB192_2
LBB192_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hdf0d79ed7ea6d829E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	add	x8, sp, #8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17h89b16a4e698eba2bE
	ldr	x8, [sp, #16]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB193_2
	b	LBB193_1
LBB193_1:
	ldr	x8, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	add	x0, x8, #16
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB193_2
LBB193_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17hef91980bc59f6821E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	add	x8, sp, #8
	bl	__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14current_memory17hd12c8d1d3bd6f368E
	ldr	x8, [sp, #16]
	subs	x8, x8, #0
	cset	x8, ne
	subs	x8, x8, #1
	b.ne	LBB194_2
	b	LBB194_1
LBB194_1:
	ldr	x8, [sp]
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	add	x0, x8, #16
	bl	__ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17hc5fadc8bee5dda52E
	b	LBB194_2
LBB194_2:
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h478cb3728ccf0c3cE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	str	x0, [sp]
	b	LBB195_1
LBB195_1:
	ldr	x8, [sp]
	ldr	x9, [x8, #8]
	str	x9, [sp, #24]
	ldr	x8, [x8]
	ldr	x9, [sp, #24]
	subs	x8, x8, x9
	cset	w8, eq
	strb	w8, [sp, #23]
	b	LBB195_2
LBB195_2:
	ldrb	w8, [sp, #23]
	tbnz	w8, #0, LBB195_4
	b	LBB195_3
LBB195_3:
	ldr	x8, [sp]
	ldr	x8, [x8]
	str	x8, [sp, #32]
	b	LBB195_5
LBB195_4:
	str	xzr, [sp, #8]
	b	LBB195_7
LBB195_5:
	ldr	x9, [sp]
	ldr	x8, [x9]
	add	x8, x8, #8
	str	x8, [sp, #40]
	ldr	x8, [sp, #40]
	str	x8, [x9]
	b	LBB195_6
LBB195_6:
	ldr	x8, [sp, #32]
	str	x8, [sp, #8]
	b	LBB195_7
LBB195_7:
	ldr	x0, [sp, #8]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h8db6a8588b72297cE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	str	x0, [sp]
	b	LBB196_1
LBB196_1:
	ldr	x8, [sp]
	ldr	x9, [x8, #8]
	str	x9, [sp, #24]
	ldr	x8, [x8]
	ldr	x9, [sp, #24]
	subs	x8, x8, x9
	cset	w8, eq
	strb	w8, [sp, #23]
	b	LBB196_2
LBB196_2:
	ldrb	w8, [sp, #23]
	tbnz	w8, #0, LBB196_4
	b	LBB196_3
LBB196_3:
	ldr	x8, [sp]
	ldr	x8, [x8]
	str	x8, [sp, #32]
	b	LBB196_5
LBB196_4:
	str	xzr, [sp, #8]
	b	LBB196_7
LBB196_5:
	ldr	x9, [sp]
	ldr	x8, [x9]
	add	x8, x8, #48
	str	x8, [sp, #40]
	ldr	x8, [sp, #40]
	str	x8, [x9]
	b	LBB196_6
LBB196_6:
	ldr	x8, [sp, #32]
	str	x8, [sp, #8]
	b	LBB196_7
LBB196_7:
	ldr	x0, [sp, #8]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hf0f48a1916b92d8eE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	str	x0, [sp]
	b	LBB197_1
LBB197_1:
	ldr	x8, [sp]
	ldr	x9, [x8, #8]
	str	x9, [sp, #24]
	ldr	x8, [x8]
	ldr	x9, [sp, #24]
	subs	x8, x8, x9
	cset	w8, eq
	strb	w8, [sp, #23]
	b	LBB197_2
LBB197_2:
	ldrb	w8, [sp, #23]
	tbnz	w8, #0, LBB197_4
	b	LBB197_3
LBB197_3:
	ldr	x8, [sp]
	ldr	x8, [x8]
	str	x8, [sp, #32]
	b	LBB197_5
LBB197_4:
	str	xzr, [sp, #8]
	b	LBB197_7
LBB197_5:
	ldr	x9, [sp]
	ldr	x8, [x9]
	add	x8, x8, #16
	str	x8, [sp, #40]
	ldr	x8, [sp, #40]
	str	x8, [x9]
	b	LBB197_6
LBB197_6:
	ldr	x8, [sp, #32]
	str	x8, [sp, #8]
	b	LBB197_7
LBB197_7:
	ldr	x0, [sp, #8]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN91_$LT$std..panicking..begin_panic..Payload$LT$A$GT$$u20$as$u20$core..panic..PanicPayload$GT$3get17hf57440273286eec3E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	ldr	x8, [x0]
	subs	x8, x8, #0
	cset	x8, ne
	cbnz	x8, LBB198_2
	b	LBB198_1
LBB198_1:
	bl	__ZN3std7process5abort17h881ea5f7c9561a59E
LBB198_2:
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_25@PAGE
	add	x1, x1, l___unnamed_25@PAGEOFF
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN91_$LT$std..panicking..begin_panic..Payload$LT$A$GT$$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hac5a8a1f2265d565E:
Lfunc_begin22:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception22
	sub	sp, sp, #128
	.cfi_def_cfa_offset 128
	stp	x29, x30, [sp, #112]
	add	x29, sp, #112
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	xzr, [sp, #48]
	ldr	x9, [x0]
	ldr	x8, [x0, #8]
	str	x9, [sp, #32]
	str	x8, [sp, #40]
	ldr	x9, [sp, #48]
	ldr	x8, [sp, #56]
	str	x9, [x0]
	str	x8, [x0, #8]
	ldr	x8, [sp, #32]
	subs	x8, x8, #0
	cset	x8, ne
	cbnz	x8, LBB199_2
	b	LBB199_1
LBB199_1:
	bl	__ZN3std7process5abort17h881ea5f7c9561a59E
LBB199_2:
	ldr	x8, [sp, #32]
	str	x8, [sp, #8]
	ldr	x8, [sp, #40]
	str	x8, [sp, #16]
Ltmp135:
	mov	w8, #16
	mov	x0, x8
	mov	w8, #8
	mov	x1, x8
	bl	__ZN5alloc5alloc15exchange_malloc17h941cdcdc6bb63bd7E
	str	x0, [sp, #24]
Ltmp136:
	b	LBB199_4
LBB199_3:
Ltmp137:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB199_4:
	ldr	x8, [sp, #24]
	ldr	x9, [sp, #16]
	ldr	x10, [sp, #8]
	str	x10, [x8]
	str	x9, [x8, #8]
	stur	x8, [x29, #-48]
	adrp	x8, l___unnamed_25@PAGE
	add	x8, x8, l___unnamed_25@PAGEOFF
	stur	x8, [x29, #-40]
	ldur	x0, [x29, #-48]
	ldur	x1, [x29, #-40]
	bl	__ZN4core10intrinsics16retag_box_to_raw17h5bee6c45725c9c82E
	.cfi_def_cfa wsp, 128
	ldp	x29, x30, [sp, #112]
	add	sp, sp, #128
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end22:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table199:
Lexception22:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end22-Lcst_begin22
Lcst_begin22:
	.uleb128 Lfunc_begin22-Lfunc_begin22
	.uleb128 Ltmp135-Lfunc_begin22
	.byte	0
	.byte	0
	.uleb128 Ltmp135-Lfunc_begin22
	.uleb128 Ltmp136-Ltmp135
	.uleb128 Ltmp137-Lfunc_begin22
	.byte	0
	.uleb128 Ltmp136-Lfunc_begin22
	.uleb128 Lfunc_end22-Ltmp136
	.byte	0
	.byte	0
Lcst_end22:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	ldrb	w8, [x0]
	cbnz	x8, LBB200_2
	b	LBB200_1
LBB200_1:
	ldr	x8, [sp, #8]
	add	x0, x8, #8
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB200_2:
	.cfi_restore_state
	adrp	x0, l___unnamed_26@PAGE
	add	x0, x0, l___unnamed_26@PAGEOFF
	mov	w8, #12
	mov	x1, x8
	adrp	x2, l___unnamed_27@PAGE
	add	x2, x2, l___unnamed_27@PAGEOFF
	bl	__ZN3std9panicking11begin_panic17had5db39b47896b4cE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions7Operand10unwrap_dst17h7efd48c761d53b64E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	ldrb	w8, [x0]
	subs	x8, x8, #1
	b.ne	LBB201_2
	b	LBB201_1
LBB201_1:
	ldr	x8, [sp, #8]
	add	x0, x8, #1
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB201_2:
	.cfi_restore_state
	adrp	x0, l___unnamed_28@PAGE
	add	x0, x0, l___unnamed_28@PAGEOFF
	mov	w8, #12
	mov	x1, x8
	adrp	x2, l___unnamed_29@PAGE
	add	x2, x2, l___unnamed_29@PAGEOFF
	bl	__ZN3std9panicking11begin_panic17had5db39b47896b4cE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions7Operand20unwrap_instrs_amount17h1886dad8a24573ecE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	ldrb	w8, [x0]
	subs	x8, x8, #2
	b.ne	LBB202_2
	b	LBB202_1
LBB202_1:
	ldr	x8, [sp, #8]
	ldr	x0, [x8, #8]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB202_2:
	.cfi_restore_state
	adrp	x0, l___unnamed_30@PAGE
	add	x0, x0, l___unnamed_30@PAGEOFF
	mov	w8, #22
	mov	x1, x8
	adrp	x2, l___unnamed_31@PAGE
	add	x2, x2, l___unnamed_31@PAGEOFF
	bl	__ZN3std9panicking11begin_panic17had5db39b47896b4cE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions7Operand16unwrap_instr_ptr17hc6519018ee9163c9E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	ldrb	w8, [x0]
	subs	x8, x8, #3
	b.ne	LBB203_2
	b	LBB203_1
LBB203_1:
	ldr	x8, [sp, #8]
	ldr	x0, [x8, #8]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB203_2:
	.cfi_restore_state
	adrp	x0, l___unnamed_32@PAGE
	add	x0, x0, l___unnamed_32@PAGEOFF
	mov	w8, #18
	mov	x1, x8
	adrp	x2, l___unnamed_33@PAGE
	add	x2, x2, l___unnamed_33@PAGEOFF
	bl	__ZN3std9panicking11begin_panic17had5db39b47896b4cE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions7Operand14unwrap_jmp_pos17h49d5dd2a8bc7be66E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	ldrb	w8, [x0]
	subs	x8, x8, #4
	b.ne	LBB204_2
	b	LBB204_1
LBB204_1:
	ldr	x8, [sp, #8]
	ldr	x0, [x8, #8]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB204_2:
	.cfi_restore_state
	adrp	x0, l___unnamed_34@PAGE
	add	x0, x0, l___unnamed_34@PAGEOFF
	mov	w8, #16
	mov	x1, x8
	adrp	x2, l___unnamed_35@PAGE
	add	x2, x2, l___unnamed_35@PAGEOFF
	bl	__ZN3std9panicking11begin_panic17had5db39b47896b4cE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions7Operand16unwrap_stack_ptr17h6d961735c953a191E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	ldrb	w8, [x0]
	subs	x8, x8, #5
	b.ne	LBB205_2
	b	LBB205_1
LBB205_1:
	ldr	x8, [sp, #8]
	add	x0, x8, #8
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB205_2:
	.cfi_restore_state
	adrp	x0, l___unnamed_36@PAGE
	add	x0, x0, l___unnamed_36@PAGEOFF
	mov	w8, #18
	mov	x1, x8
	adrp	x2, l___unnamed_37@PAGE
	add	x2, x2, l___unnamed_37@PAGEOFF
	bl	__ZN3std9panicking11begin_panic17had5db39b47896b4cE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions7Operand17unwrap_args_count17h383375d773e28d50E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	ldrb	w8, [x0]
	subs	x8, x8, #6
	b.ne	LBB206_2
	b	LBB206_1
LBB206_1:
	ldr	x8, [sp, #8]
	ldr	x0, [x8, #8]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB206_2:
	.cfi_restore_state
	adrp	x0, l___unnamed_38@PAGE
	add	x0, x0, l___unnamed_38@PAGEOFF
	mov	w8, #19
	mov	x1, x8
	adrp	x2, l___unnamed_39@PAGE
	add	x2, x2, l___unnamed_39@PAGEOFF
	bl	__ZN3std9panicking11begin_panic17had5db39b47896b4cE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions15HaltInstruction3new17hc69fe866ca78bfb8E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions15HaltInstruction7execute17h3d94b71409f21791E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x0, x4
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions14AddInstruction3new17h1fa494a4c80f5d48E:
	.cfi_startproc
	strb	w0, [x8, #32]
	ldr	q0, [x1]
	str	q0, [x8]
	ldr	q0, [x2]
	str	q0, [x8, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions14AddInstruction7execute17hbbf2fe13542e736cE:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	str	x1, [sp, #32]
	str	x2, [sp, #40]
	str	x3, [sp, #48]
	stur	x4, [x29, #-40]
	ldr	x1, [sp, #32]
	ldr	x2, [sp, #40]
	ldr	x3, [sp, #48]
	ldur	x4, [x29, #-40]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x1, x0
	ldr	x0, [sp]
	str	x1, [sp, #8]
	ldr	x1, [sp, #32]
	ldr	x2, [sp, #40]
	ldr	x3, [sp, #48]
	ldur	x4, [x29, #-40]
	add	x0, x0, #16
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x8, [sp]
	mov	x1, x0
	ldr	x0, [sp, #8]
	ldur	x9, [x29, #-40]
	str	x9, [sp, #16]
	ldrb	w8, [x8, #32]
	str	w8, [sp, #28]
	sub	x8, x29, #16
	bl	__ZN4main7vm_util5Value3add17hc9b1ea27128f8b31E
	ldr	x0, [sp, #16]
	ldr	w1, [sp, #28]
	ldur	q0, [x29, #-16]
	sub	x2, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE
	ldur	x0, [x29, #-40]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions14SubInstruction3new17ha4ff55e7ef410ce5E:
	.cfi_startproc
	strb	w0, [x8, #32]
	ldr	q0, [x1]
	str	q0, [x8]
	ldr	q0, [x2]
	str	q0, [x8, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions14SubInstruction7execute17h32bbd702aef68a41E:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #32]
	str	x1, [sp, #8]
	str	x2, [sp, #16]
	str	x3, [sp, #24]
	stur	x4, [x29, #-40]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	ldur	x4, [x29, #-40]
	mov	x5, x0
	ldr	x0, [sp, #32]
	str	x5, [sp, #40]
	add	x0, x0, #16
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x8, [sp, #32]
	mov	x1, x0
	ldr	x0, [sp, #40]
	ldrb	w8, [x8, #32]
	stur	w8, [x29, #-44]
	sub	x8, x29, #16
	bl	__ZN4main7vm_util5Value3sub17hd68d6234769dfb5dE
	ldur	w1, [x29, #-44]
	ldur	x0, [x29, #-40]
	ldur	q0, [x29, #-16]
	sub	x2, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE
	ldur	x0, [x29, #-40]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions14MulInstruction7execute17hc1927e975430bb56E:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #32]
	str	x1, [sp, #8]
	str	x2, [sp, #16]
	str	x3, [sp, #24]
	stur	x4, [x29, #-40]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	ldur	x4, [x29, #-40]
	mov	x5, x0
	ldr	x0, [sp, #32]
	str	x5, [sp, #40]
	add	x0, x0, #16
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x8, [sp, #32]
	mov	x1, x0
	ldr	x0, [sp, #40]
	ldrb	w8, [x8, #32]
	stur	w8, [x29, #-44]
	sub	x8, x29, #16
	bl	__ZN4main7vm_util5Value3mul17he602582e17797115E
	ldur	w1, [x29, #-44]
	ldur	x0, [x29, #-40]
	ldur	q0, [x29, #-16]
	sub	x2, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE
	ldur	x0, [x29, #-40]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions14DivInstruction7execute17h9b107b61b04a5120E:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #32]
	str	x1, [sp, #8]
	str	x2, [sp, #16]
	str	x3, [sp, #24]
	stur	x4, [x29, #-40]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x3, [sp, #24]
	ldur	x4, [x29, #-40]
	mov	x5, x0
	ldr	x0, [sp, #32]
	str	x5, [sp, #40]
	add	x0, x0, #16
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x8, [sp, #32]
	mov	x1, x0
	ldr	x0, [sp, #40]
	ldrb	w8, [x8, #32]
	stur	w8, [x29, #-44]
	sub	x8, x29, #16
	bl	__ZN4main7vm_util5Value3div17he68c9a11be5f0467E
	ldur	w1, [x29, #-44]
	ldur	x0, [x29, #-40]
	ldur	q0, [x29, #-16]
	sub	x2, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE
	ldur	x0, [x29, #-40]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions23PushFunctionInstruction3new17h931a333ebe77dcd9E:
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	add	sp, sp, #16
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions23PushFunctionInstruction7execute17h69237e0641c80bcfE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #16]
	str	x3, [sp, #8]
	mov	x0, x4
	str	x0, [sp]
	bl	__ZN4main9Registers6get_ip17hc33f2887d7bd86c3E
	mov	x8, x0
	ldr	x0, [sp, #8]
	sub	x1, x29, #16
	stur	x8, [x29, #-8]
	mov	w8, #3
	sturb	w8, [x29, #-16]
	bl	__ZN4main7vm_util5Stack4push17hf8474d57900a6c0fE
	ldr	x0, [sp, #16]
	ldr	x8, [x0]
	adds	x8, x8, #1
	str	x8, [sp, #24]
	cset	w8, hs
	tbnz	w8, #0, LBB216_2
	b	LBB216_1
LBB216_1:
	ldr	x1, [sp, #24]
	ldr	x0, [sp]
	bl	__ZN4main9Registers6add_ip17hb3c329094c540883E
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB216_2:
	.cfi_restore_state
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_40@PAGE
	add	x2, x2, l___unnamed_40@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions19CmpLtJmpInstruction3new17h9fe81541418d7fd0E:
	.cfi_startproc
	ldr	q0, [x0]
	str	q0, [x8, #16]
	ldr	q0, [x1]
	str	q0, [x8, #32]
	str	x2, [x8]
	str	x3, [x8, #8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions19CmpLtJmpInstruction7execute17h8adc5d8c8dfd3149E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	x0, [x29, #-16]
	str	x1, [sp]
	str	x2, [sp, #8]
	str	x3, [sp, #16]
	str	x4, [sp, #24]
	add	x0, x0, #16
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x1, [sp]
	ldr	x2, [sp, #8]
	ldr	x3, [sp, #16]
	ldr	x4, [sp, #24]
	mov	x5, x0
	ldur	x0, [x29, #-16]
	stur	x5, [x29, #-8]
	add	x0, x0, #32
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x1, x0
	ldur	x0, [x29, #-8]
	bl	__ZN4main7vm_util5Value5cmp_l17hf0d8db80eb6e44ccE
	tbnz	w0, #0, LBB218_2
	b	LBB218_1
LBB218_1:
	ldr	x0, [sp, #24]
	ldur	x8, [x29, #-16]
	ldr	x1, [x8, #8]
	bl	__ZN4main9Registers6set_ip17h5ac825b8f5ec62e9E
	b	LBB218_3
LBB218_2:
	ldr	x0, [sp, #24]
	ldur	x8, [x29, #-16]
	ldr	x1, [x8]
	bl	__ZN4main9Registers6set_ip17h5ac825b8f5ec62e9E
	b	LBB218_3
LBB218_3:
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions15CallInstruction3new17h81b5345dc87d0d3aE:
	.cfi_startproc
	str	x0, [x8, #8]
	strb	w1, [x8, #16]
	str	x2, [x8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions15CallInstruction7execute17h9b2d9394b717ebfbE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	stur	x0, [x29, #-16]
	mov	x0, x3
	str	x0, [sp, #16]
	str	x4, [sp, #24]
	bl	__ZN4main7vm_util5Stack3len17h2e0f01840a525a37E
	ldur	x8, [x29, #-16]
	ldr	x8, [x8]
	subs	x9, x0, x8
	stur	x9, [x29, #-8]
	subs	x8, x0, x8
	b.lo	LBB220_2
	b	LBB220_1
LBB220_1:
	ldr	x0, [sp, #16]
	ldur	x1, [x29, #-8]
	bl	__ZN4main7vm_util5Stack16push_stack_frame17ha9f296821220ecf7E
	ldur	x8, [x29, #-16]
	ldr	x0, [sp, #16]
	add	x1, x8, #8
	bl	__ZN4main7vm_util5Stack3get17h6f97994b4e633039E
	bl	__ZN4main7vm_util10StackValue13unwrap_fn_ptr17hc6af2877f010ef79E
	adds	x8, x0, #1
	str	x8, [sp, #8]
	cset	w8, hs
	tbnz	w8, #0, LBB220_4
	b	LBB220_3
LBB220_2:
	adrp	x0, _str.5@PAGE
	add	x0, x0, _str.5@PAGEOFF
	mov	w8, #33
	mov	x1, x8
	adrp	x2, l___unnamed_41@PAGE
	add	x2, x2, l___unnamed_41@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
LBB220_3:
	ldr	x1, [sp, #8]
	ldr	x0, [sp, #24]
	bl	__ZN4main9Registers6set_ip17h5ac825b8f5ec62e9E
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB220_4:
	.cfi_restore_state
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_42@PAGE
	add	x2, x2, l___unnamed_42@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN4main12instructions14RetInstruction3new17hba68c35196258340E:
	.cfi_startproc
	ldr	q0, [x0]
	str	q0, [x8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions14RetInstruction7execute17ha251383e91ffdd6aE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x3, [sp, #8]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x8, x0
	ldr	x0, [sp, #8]
	ldr	q0, [x8]
	str	q0, [sp, #16]
	bl	__ZN4main7vm_util5Stack16read_stack_frame17h9315cae95e177d90E
	mov	x1, x0
	ldr	x0, [sp, #8]
	bl	__ZN4main7vm_util5Stack8truncate17h0d9f5788a7945e3eE
	ldr	x0, [sp, #8]
	bl	__ZN4main7vm_util5Stack15pop_stack_frame17h14e601ae09d004d1E
	ldr	x0, [sp, #8]
	ldr	q0, [sp, #16]
	sub	x1, x29, #16
	stur	q0, [x29, #-16]
	bl	__ZN4main7vm_util5Stack4push17hf8474d57900a6c0fE
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions20PushStackInstruction3new17hfe7e45e50564c2e7E:
	.cfi_startproc
	ldr	q0, [x0]
	str	q0, [x8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions20PushStackInstruction7execute17hbbff7cce66887ca7E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x3, [sp]
	str	x4, [sp, #8]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x8, x0
	ldr	x0, [sp]
	ldr	q0, [x8]
	str	q0, [sp, #16]
	ldr	q0, [sp, #16]
	sub	x1, x29, #16
	stur	q0, [x29, #-16]
	bl	__ZN4main7vm_util5Stack4push17hf8474d57900a6c0fE
	ldr	x0, [sp, #8]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions23PushInstrPtrInstruction3new17h4d74fd4ece850b62E:
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	add	sp, sp, #16
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions23PushInstrPtrInstruction7execute17hb16ecd66cf1d65f2E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x0, x4
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp]
	mov	x8, x0
	ldr	x0, [sp]
	sturb	w8, [x29, #-1]
	ldurb	w8, [x29, #-1]
	strb	w8, [x0, #40]
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN66_$LT$main..instructions..InstrData$u20$as$u20$core..fmt..Debug$GT$3fmt17hdb4ff904094b1a9aE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp]
	add	x8, sp, #16
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_43@PAGE
	add	x0, x0, l___unnamed_43@PAGEOFF
	mov	w9, #1
	mov	x1, x9
	bl	__ZN4core3fmt9Arguments9new_const17hb03f19c839408508E
	ldr	x0, [sp]
	ldr	x1, [sp, #8]
	bl	__ZN4core3fmt9Formatter9write_fmt17h153b963f3ea3120aE
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData8new_halt17h11111db872084eb7E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData26new_binary_reg_stack_const17h2cc680dacedbc3dfE:
	.cfi_startproc
	ldr	q0, [x0]
	str	q0, [x8]
	ldr	q0, [x0, #16]
	str	q0, [x8, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData26new_binary_reg_stack_stack17hb190ab858b21569dE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp, #8]
	mov	x1, x0
	ldr	x0, [sp, #8]
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData8new_call17h9c0e33e6f3a1afa3E:
	.cfi_startproc
	ldr	q0, [x0]
	str	q0, [x8]
	ldr	q0, [x0, #16]
	str	q0, [x8, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData13new_ret_stack17hdf0883ccfb057066E:
	.cfi_startproc
	str	x0, [x8]
	strb	w1, [x8, #8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData11new_ret_reg17h6c59eedeb1f85c16E:
	.cfi_startproc
	strb	w0, [x8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData18new_push_stack_reg17h82a672bcbeea080eE:
	.cfi_startproc
	strb	w0, [x8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData20new_push_stack_const17hd40771eac948cb2fE:
	.cfi_startproc
	str	x0, [x8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData11new_cmp_jmp17hd109254bc2ebee4cE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x8, [sp, #8]
	mov	x1, x0
	ldr	x0, [sp, #8]
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main12instructions9InstrData11new_push_fn17h4fab9dbee3b9b2a5E:
	.cfi_startproc
	str	x0, [x8]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util8ConstPtr3new17h473b7f254df66ad8E:
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	str	x0, [sp, #8]
	ldr	x0, [sp, #8]
	add	sp, sp, #16
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E:
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	str	x0, [sp]
	strb	w1, [sp, #8]
	ldr	x0, [sp]
	ldrb	w8, [sp, #8]
	and	w1, w8, #0x1
	add	sp, sp, #16
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util8StackPtr7get_idx17h0e012aab37abcf33E:
	.cfi_startproc
	ldr	x0, [x0]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util8StackPtr15get_is_relative17h73bb4cab09f70a0aE:
	.cfi_startproc
	ldrb	w8, [x0, #8]
	and	w0, w8, #0x1
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	str	x2, [sp, #24]
	str	x3, [sp, #32]
	stur	x4, [x29, #-24]
	ldrb	w8, [x0, #8]
	subs	w9, w8, #2
	mov	x8, x9
	and	x8, x8, #0xff
	and	w10, w9, #0xff
	mov	w9, #2
	subs	w10, w10, #1
	csel	x8, x8, x9, ls
	stur	x8, [x29, #-16]
	cbz	x8, LBB243_4
	b	LBB243_1
LBB243_1:
	ldur	x8, [x29, #-16]
	subs	x8, x8, #1
	b.eq	LBB243_5
	b	LBB243_2
LBB243_2:
	b	LBB243_6
LBB243_4:
	ldur	x0, [x29, #-24]
	ldr	x8, [sp, #8]
	ldrb	w1, [x8]
	bl	__ZN4main9Registers7get_reg17h8a9834799835db69E
	ldr	x1, [sp, #16]
	ldr	x2, [sp, #24]
	bl	__ZN4main7vm_util13RegisterValue7get_val17h13f06b83c0370ad1E
	stur	x0, [x29, #-8]
	b	LBB243_7
LBB243_5:
	ldr	x1, [sp, #24]
	ldr	x0, [sp, #16]
	ldr	x8, [sp, #8]
	ldr	x2, [x8]
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17hc2b02be7afc34c85E
	stur	x0, [x29, #-8]
	b	LBB243_7
LBB243_6:
	ldr	x1, [sp, #8]
	ldr	x0, [sp, #32]
	bl	__ZN4main7vm_util5Stack3get17h6f97994b4e633039E
	bl	__ZN4main7vm_util10StackValue10unwrap_val17h6371c624edab9ae2E
	stur	x0, [x29, #-8]
	b	LBB243_7
LBB243_7:
	ldur	x0, [x29, #-8]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Value3add17hc9b1ea27128f8b31E:
	.cfi_startproc
	sub	sp, sp, #192
	.cfi_def_cfa_offset 192
	stp	x29, x30, [sp, #176]
	add	x29, sp, #176
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #16]
	str	x0, [sp, #32]
	str	x1, [sp, #40]
	ldr	x9, [sp, #32]
	ldr	x8, [sp, #40]
	str	x9, [sp, #48]
	str	x8, [sp, #56]
	ldr	x8, [sp, #48]
	ldrb	w8, [x8]
	str	x8, [sp, #24]
	cbz	x8, LBB244_4
	b	LBB244_1
LBB244_1:
	ldr	x8, [sp, #24]
	subs	x8, x8, #1
	b.eq	LBB244_5
	b	LBB244_2
LBB244_2:
	b	LBB244_6
LBB244_4:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	cbz	x8, LBB244_7
	b	LBB244_6
LBB244_5:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	subs	x8, x8, #1
	b.eq	LBB244_9
	b	LBB244_6
LBB244_6:
	add	x8, sp, #32
	stur	x8, [x29, #-16]
	adrp	x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGE
	add	x8, x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGEOFF
	mov	x9, x8
	stur	x9, [x29, #-8]
	ldur	x11, [x29, #-16]
	ldur	x10, [x29, #-8]
	add	x9, sp, #40
	stur	x9, [x29, #-32]
	stur	x8, [x29, #-24]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	sub	x2, x29, #64
	stur	x11, [x29, #-64]
	stur	x10, [x29, #-56]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	add	x8, sp, #64
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_44@PAGE
	add	x0, x0, l___unnamed_44@PAGEOFF
	mov	w9, #2
	mov	x3, x9
	mov	x1, x3
	bl	__ZN4core3fmt9Arguments6new_v117h619f9bb2db3983b4E
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_45@PAGE
	add	x1, x1, l___unnamed_45@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h98bbf7bdf4994454E
LBB244_7:
	ldr	x8, [sp, #48]
	add	x0, x8, #4
	ldr	x8, [sp, #56]
	add	x1, x8, #4
	adrp	x2, l___unnamed_46@PAGE
	add	x2, x2, l___unnamed_46@PAGEOFF
	bl	__ZN64_$LT$$RF$i32$u20$as$u20$core..ops..arith..Add$LT$$RF$i32$GT$$GT$3add17haffeed548419eb45E
	ldr	x8, [sp, #16]
	str	w0, [x8, #4]
	strb	wzr, [x8]
	b	LBB244_8
LBB244_8:
	.cfi_def_cfa wsp, 192
	ldp	x29, x30, [sp, #176]
	add	sp, sp, #192
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB244_9:
	.cfi_restore_state
	ldr	x8, [sp, #48]
	add	x0, x8, #8
	ldr	x8, [sp, #56]
	add	x1, x8, #8
	adrp	x2, l___unnamed_47@PAGE
	add	x2, x2, l___unnamed_47@PAGEOFF
	bl	__ZN64_$LT$$RF$i64$u20$as$u20$core..ops..arith..Add$LT$$RF$i64$GT$$GT$3add17he18c087e1fb0253bE
	ldr	x9, [sp, #16]
	str	x0, [x9, #8]
	mov	w8, #1
	strb	w8, [x9]
	b	LBB244_8
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Value3sub17hd68d6234769dfb5dE:
	.cfi_startproc
	sub	sp, sp, #192
	.cfi_def_cfa_offset 192
	stp	x29, x30, [sp, #176]
	add	x29, sp, #176
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #16]
	str	x0, [sp, #32]
	str	x1, [sp, #40]
	ldr	x9, [sp, #32]
	ldr	x8, [sp, #40]
	str	x9, [sp, #48]
	str	x8, [sp, #56]
	ldr	x8, [sp, #48]
	ldrb	w8, [x8]
	str	x8, [sp, #24]
	cbz	x8, LBB245_4
	b	LBB245_1
LBB245_1:
	ldr	x8, [sp, #24]
	subs	x8, x8, #1
	b.eq	LBB245_5
	b	LBB245_2
LBB245_2:
	b	LBB245_6
LBB245_4:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	cbz	x8, LBB245_7
	b	LBB245_6
LBB245_5:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	subs	x8, x8, #1
	b.eq	LBB245_9
	b	LBB245_6
LBB245_6:
	add	x8, sp, #32
	stur	x8, [x29, #-16]
	adrp	x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGE
	add	x8, x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGEOFF
	mov	x9, x8
	stur	x9, [x29, #-8]
	ldur	x11, [x29, #-16]
	ldur	x10, [x29, #-8]
	add	x9, sp, #40
	stur	x9, [x29, #-32]
	stur	x8, [x29, #-24]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	sub	x2, x29, #64
	stur	x11, [x29, #-64]
	stur	x10, [x29, #-56]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	add	x8, sp, #64
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_48@PAGE
	add	x0, x0, l___unnamed_48@PAGEOFF
	mov	w9, #2
	mov	x3, x9
	mov	x1, x3
	bl	__ZN4core3fmt9Arguments6new_v117h619f9bb2db3983b4E
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_49@PAGE
	add	x1, x1, l___unnamed_49@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h98bbf7bdf4994454E
LBB245_7:
	ldr	x8, [sp, #48]
	add	x0, x8, #4
	ldr	x8, [sp, #56]
	add	x1, x8, #4
	adrp	x2, l___unnamed_50@PAGE
	add	x2, x2, l___unnamed_50@PAGEOFF
	bl	__ZN64_$LT$$RF$i32$u20$as$u20$core..ops..arith..Sub$LT$$RF$i32$GT$$GT$3sub17h94283ebb94365b9cE
	ldr	x8, [sp, #16]
	str	w0, [x8, #4]
	strb	wzr, [x8]
	b	LBB245_8
LBB245_8:
	.cfi_def_cfa wsp, 192
	ldp	x29, x30, [sp, #176]
	add	sp, sp, #192
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB245_9:
	.cfi_restore_state
	ldr	x8, [sp, #48]
	add	x0, x8, #8
	ldr	x8, [sp, #56]
	add	x1, x8, #8
	adrp	x2, l___unnamed_51@PAGE
	add	x2, x2, l___unnamed_51@PAGEOFF
	bl	__ZN64_$LT$$RF$i64$u20$as$u20$core..ops..arith..Sub$LT$$RF$i64$GT$$GT$3sub17h2d6f0cbc91a90a8fE
	ldr	x9, [sp, #16]
	str	x0, [x9, #8]
	mov	w8, #1
	strb	w8, [x9]
	b	LBB245_8
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Value3mul17he602582e17797115E:
	.cfi_startproc
	sub	sp, sp, #192
	.cfi_def_cfa_offset 192
	stp	x29, x30, [sp, #176]
	add	x29, sp, #176
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #16]
	str	x0, [sp, #32]
	str	x1, [sp, #40]
	ldr	x9, [sp, #32]
	ldr	x8, [sp, #40]
	str	x9, [sp, #48]
	str	x8, [sp, #56]
	ldr	x8, [sp, #48]
	ldrb	w8, [x8]
	str	x8, [sp, #24]
	cbz	x8, LBB246_4
	b	LBB246_1
LBB246_1:
	ldr	x8, [sp, #24]
	subs	x8, x8, #1
	b.eq	LBB246_5
	b	LBB246_2
LBB246_2:
	b	LBB246_6
LBB246_4:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	cbz	x8, LBB246_7
	b	LBB246_6
LBB246_5:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	subs	x8, x8, #1
	b.eq	LBB246_9
	b	LBB246_6
LBB246_6:
	add	x8, sp, #32
	stur	x8, [x29, #-16]
	adrp	x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGE
	add	x8, x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGEOFF
	mov	x9, x8
	stur	x9, [x29, #-8]
	ldur	x11, [x29, #-16]
	ldur	x10, [x29, #-8]
	add	x9, sp, #40
	stur	x9, [x29, #-32]
	stur	x8, [x29, #-24]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	sub	x2, x29, #64
	stur	x11, [x29, #-64]
	stur	x10, [x29, #-56]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	add	x8, sp, #64
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_52@PAGE
	add	x0, x0, l___unnamed_52@PAGEOFF
	mov	w9, #2
	mov	x3, x9
	mov	x1, x3
	bl	__ZN4core3fmt9Arguments6new_v117h619f9bb2db3983b4E
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_53@PAGE
	add	x1, x1, l___unnamed_53@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h98bbf7bdf4994454E
LBB246_7:
	ldr	x8, [sp, #48]
	add	x0, x8, #4
	ldr	x8, [sp, #56]
	add	x1, x8, #4
	adrp	x2, l___unnamed_54@PAGE
	add	x2, x2, l___unnamed_54@PAGEOFF
	bl	__ZN64_$LT$$RF$i32$u20$as$u20$core..ops..arith..Mul$LT$$RF$i32$GT$$GT$3mul17h09851e64bde045bcE
	ldr	x8, [sp, #16]
	str	w0, [x8, #4]
	strb	wzr, [x8]
	b	LBB246_8
LBB246_8:
	.cfi_def_cfa wsp, 192
	ldp	x29, x30, [sp, #176]
	add	sp, sp, #192
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB246_9:
	.cfi_restore_state
	ldr	x8, [sp, #48]
	add	x0, x8, #8
	ldr	x8, [sp, #56]
	add	x1, x8, #8
	adrp	x2, l___unnamed_55@PAGE
	add	x2, x2, l___unnamed_55@PAGEOFF
	bl	__ZN64_$LT$$RF$i64$u20$as$u20$core..ops..arith..Mul$LT$$RF$i64$GT$$GT$3mul17h15a4e94dd23dc741E
	ldr	x9, [sp, #16]
	str	x0, [x9, #8]
	mov	w8, #1
	strb	w8, [x9]
	b	LBB246_8
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Value3div17he68c9a11be5f0467E:
	.cfi_startproc
	sub	sp, sp, #192
	.cfi_def_cfa_offset 192
	stp	x29, x30, [sp, #176]
	add	x29, sp, #176
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #16]
	str	x0, [sp, #32]
	str	x1, [sp, #40]
	ldr	x9, [sp, #32]
	ldr	x8, [sp, #40]
	str	x9, [sp, #48]
	str	x8, [sp, #56]
	ldr	x8, [sp, #48]
	ldrb	w8, [x8]
	str	x8, [sp, #24]
	cbz	x8, LBB247_4
	b	LBB247_1
LBB247_1:
	ldr	x8, [sp, #24]
	subs	x8, x8, #1
	b.eq	LBB247_5
	b	LBB247_2
LBB247_2:
	b	LBB247_6
LBB247_4:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	cbz	x8, LBB247_7
	b	LBB247_6
LBB247_5:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	subs	x8, x8, #1
	b.eq	LBB247_9
	b	LBB247_6
LBB247_6:
	add	x8, sp, #32
	stur	x8, [x29, #-16]
	adrp	x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGE
	add	x8, x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGEOFF
	mov	x9, x8
	stur	x9, [x29, #-8]
	ldur	x11, [x29, #-16]
	ldur	x10, [x29, #-8]
	add	x9, sp, #40
	stur	x9, [x29, #-32]
	stur	x8, [x29, #-24]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	sub	x2, x29, #64
	stur	x11, [x29, #-64]
	stur	x10, [x29, #-56]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	add	x8, sp, #64
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_56@PAGE
	add	x0, x0, l___unnamed_56@PAGEOFF
	mov	w9, #2
	mov	x3, x9
	mov	x1, x3
	bl	__ZN4core3fmt9Arguments6new_v117h619f9bb2db3983b4E
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_57@PAGE
	add	x1, x1, l___unnamed_57@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h98bbf7bdf4994454E
LBB247_7:
	ldr	x8, [sp, #48]
	add	x0, x8, #4
	ldr	x8, [sp, #56]
	add	x1, x8, #4
	adrp	x2, l___unnamed_58@PAGE
	add	x2, x2, l___unnamed_58@PAGEOFF
	bl	__ZN64_$LT$$RF$i32$u20$as$u20$core..ops..arith..Div$LT$$RF$i32$GT$$GT$3div17hd29fdd77f45d4c6cE
	ldr	x8, [sp, #16]
	str	w0, [x8, #4]
	strb	wzr, [x8]
	b	LBB247_8
LBB247_8:
	.cfi_def_cfa wsp, 192
	ldp	x29, x30, [sp, #176]
	add	sp, sp, #192
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB247_9:
	.cfi_restore_state
	ldr	x8, [sp, #48]
	add	x0, x8, #8
	ldr	x8, [sp, #56]
	add	x1, x8, #8
	adrp	x2, l___unnamed_59@PAGE
	add	x2, x2, l___unnamed_59@PAGEOFF
	bl	__ZN64_$LT$$RF$i64$u20$as$u20$core..ops..arith..Div$LT$$RF$i64$GT$$GT$3div17ha0b42afe14a389baE
	ldr	x9, [sp, #16]
	str	x0, [x9, #8]
	mov	w8, #1
	strb	w8, [x9]
	b	LBB247_8
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Value5cmp_l17hf0d8db80eb6e44ccE:
	.cfi_startproc
	sub	sp, sp, #224
	.cfi_def_cfa_offset 224
	stp	x29, x30, [sp, #208]
	add	x29, sp, #208
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #24]
	str	x1, [sp, #32]
	ldr	x9, [sp, #24]
	ldr	x8, [sp, #32]
	str	x9, [sp, #48]
	str	x8, [sp, #56]
	ldr	x8, [sp, #48]
	ldrb	w8, [x8]
	str	x8, [sp, #16]
	cbz	x8, LBB248_4
	b	LBB248_1
LBB248_1:
	ldr	x8, [sp, #16]
	subs	x8, x8, #1
	b.eq	LBB248_5
	b	LBB248_2
LBB248_2:
	b	LBB248_6
LBB248_4:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	cbz	x8, LBB248_7
	b	LBB248_6
LBB248_5:
	ldr	x8, [sp, #56]
	ldrb	w8, [x8]
	subs	x8, x8, #1
	b.eq	LBB248_9
	b	LBB248_6
LBB248_6:
	add	x8, sp, #24
	stur	x8, [x29, #-16]
	adrp	x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGE
	add	x8, x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E@PAGEOFF
	mov	x9, x8
	stur	x9, [x29, #-8]
	ldur	x11, [x29, #-16]
	ldur	x10, [x29, #-8]
	add	x9, sp, #32
	stur	x9, [x29, #-32]
	stur	x8, [x29, #-24]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	sub	x2, x29, #64
	stur	x11, [x29, #-64]
	stur	x10, [x29, #-56]
	stur	x9, [x29, #-48]
	stur	x8, [x29, #-40]
	add	x8, sp, #96
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_60@PAGE
	add	x0, x0, l___unnamed_60@PAGEOFF
	mov	w9, #2
	mov	x3, x9
	mov	x1, x3
	bl	__ZN4core3fmt9Arguments6new_v117h619f9bb2db3983b4E
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_61@PAGE
	add	x1, x1, l___unnamed_61@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17h98bbf7bdf4994454E
LBB248_7:
	ldr	x8, [sp, #48]
	add	x8, x8, #4
	add	x0, sp, #64
	str	x8, [sp, #64]
	ldr	x8, [sp, #56]
	add	x8, x8, #4
	add	x1, sp, #72
	str	x8, [sp, #72]
	bl	__ZN4core3cmp5impls70_$LT$impl$u20$core..cmp..PartialOrd$LT$$RF$B$GT$$u20$for$u20$$RF$A$GT$2lt17h9bf4e6088cfa648cE
	strb	w0, [sp, #47]
	b	LBB248_8
LBB248_8:
	ldrb	w8, [sp, #47]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 224
	ldp	x29, x30, [sp, #208]
	add	sp, sp, #224
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB248_9:
	.cfi_restore_state
	ldr	x8, [sp, #48]
	add	x8, x8, #8
	add	x0, sp, #80
	str	x8, [sp, #80]
	ldr	x8, [sp, #56]
	add	x8, x8, #8
	add	x1, sp, #88
	str	x8, [sp, #88]
	bl	__ZN4core3cmp5impls70_$LT$impl$u20$core..cmp..PartialOrd$LT$$RF$B$GT$$u20$for$u20$$RF$A$GT$2lt17h888854cfd1fe6f76E
	strb	w0, [sp, #47]
	b	LBB248_8
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Stack3new17h2ffa87df64a54c0aE:
Lfunc_begin23:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception23
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #8]
	add	x8, sp, #16
	bl	__ZN5alloc3vec12Vec$LT$T$GT$3new17h9a1b419d29aacfb7E
Ltmp142:
	add	x8, sp, #40
	bl	__ZN5alloc3vec12Vec$LT$T$GT$3new17ha107aee19cd17e30E
Ltmp143:
	b	LBB249_3
LBB249_1:
Ltmp145:
	add	x0, sp, #16
	bl	__ZN4core3ptr69drop_in_place$LT$alloc..vec..Vec$LT$main..vm_util..StackValue$GT$$GT$17hc496bbea7a6bec5eE
Ltmp146:
	b	LBB249_5
LBB249_2:
Ltmp144:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB249_1
LBB249_3:
	ldr	x9, [sp, #8]
	ldr	q0, [sp, #16]
	str	q0, [x9]
	ldr	x8, [sp, #32]
	str	x8, [x9, #16]
	ldur	q0, [sp, #40]
	stur	q0, [x9, #24]
	ldr	x8, [sp, #56]
	str	x8, [x9, #40]
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB249_4:
	.cfi_restore_state
Ltmp147:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB249_5:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
Lfunc_end23:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table249:
Lexception23:
	.byte	255
	.byte	155
	.uleb128 Lttbase14-Lttbaseref14
Lttbaseref14:
	.byte	1
	.uleb128 Lcst_end23-Lcst_begin23
Lcst_begin23:
	.uleb128 Lfunc_begin23-Lfunc_begin23
	.uleb128 Ltmp142-Lfunc_begin23
	.byte	0
	.byte	0
	.uleb128 Ltmp142-Lfunc_begin23
	.uleb128 Ltmp143-Ltmp142
	.uleb128 Ltmp144-Lfunc_begin23
	.byte	0
	.uleb128 Ltmp145-Lfunc_begin23
	.uleb128 Ltmp146-Ltmp145
	.uleb128 Ltmp147-Lfunc_begin23
	.byte	1
	.uleb128 Ltmp146-Lfunc_begin23
	.uleb128 Lfunc_end23-Ltmp146
	.byte	0
	.byte	0
Lcst_end23:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase14:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4main7vm_util5Stack3get17h6f97994b4e633039E:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	stur	x0, [x29, #-24]
	mov	x0, x1
	stur	x0, [x29, #-16]
	bl	__ZN4main7vm_util8StackPtr15get_is_relative17h73bb4cab09f70a0aE
	tbnz	w0, #0, LBB250_2
	b	LBB250_1
LBB250_1:
	ldur	x0, [x29, #-24]
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h70cbdade11d89f06E
	mov	x2, x0
	ldur	x0, [x29, #-16]
	stur	x2, [x29, #-32]
	str	x1, [sp, #40]
	bl	__ZN4main7vm_util8StackPtr7get_idx17h0e012aab37abcf33E
	ldr	x1, [sp, #40]
	mov	x2, x0
	ldur	x0, [x29, #-32]
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h244297f580e1b3d5E
	stur	x0, [x29, #-8]
	b	LBB250_3
LBB250_2:
	ldur	x0, [x29, #-24]
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h70cbdade11d89f06E
	mov	x2, x0
	ldur	x0, [x29, #-16]
	str	x2, [sp, #8]
	str	x1, [sp, #16]
	bl	__ZN4main7vm_util8StackPtr7get_idx17h0e012aab37abcf33E
	mov	x8, x0
	ldur	x0, [x29, #-24]
	str	x8, [sp, #24]
	bl	__ZN4main7vm_util5Stack16read_stack_frame17h9315cae95e177d90E
	mov	x8, x0
	ldr	x0, [sp, #24]
	adds	x8, x0, x8
	str	x8, [sp, #32]
	cset	w8, hs
	tbnz	w8, #0, LBB250_5
	b	LBB250_4
LBB250_3:
	ldur	x0, [x29, #-8]
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB250_4:
	.cfi_restore_state
	ldr	x2, [sp, #32]
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h244297f580e1b3d5E
	stur	x0, [x29, #-8]
	b	LBB250_3
LBB250_5:
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_62@PAGE
	add	x2, x2, l___unnamed_62@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Stack4push17hf8474d57900a6c0fE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	q0, [x1]
	mov	x1, sp
	str	q0, [sp]
	bl	__ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17h369134e97c19391dE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Stack8truncate17h0d9f5788a7945e3eE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN5alloc3vec16Vec$LT$T$C$A$GT$8truncate17hb431a5a89fa1cfadE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Stack16push_stack_frame17ha9f296821220ecf7E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	add	x0, x0, #24
	bl	__ZN5alloc3vec16Vec$LT$T$C$A$GT$4push17h387bfeda0baaabafE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Stack16read_stack_frame17h9315cae95e177d90E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp]
	add	x0, x0, #24
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h6346f7eaf8ebae4cE
	mov	x2, x0
	ldr	x0, [sp]
	str	x2, [sp, #8]
	str	x1, [sp, #16]
	add	x0, x0, #24
	bl	__ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17h27d589a0d32c4e01E
	subs	x8, x0, #1
	stur	x8, [x29, #-8]
	subs	x8, x0, #1
	b.lo	LBB254_2
	b	LBB254_1
LBB254_1:
	ldur	x2, [x29, #-8]
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17h21f22e255f836a66E
	ldr	x0, [x0]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB254_2:
	.cfi_restore_state
	adrp	x0, _str.5@PAGE
	add	x0, x0, _str.5@PAGEOFF
	mov	w8, #33
	mov	x1, x8
	adrp	x2, l___unnamed_63@PAGE
	add	x2, x2, l___unnamed_63@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Stack15pop_stack_frame17h14e601ae09d004d1E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	add	x0, x0, #24
	bl	__ZN5alloc3vec16Vec$LT$T$C$A$GT$3pop17h8ece746ea8cd90fbE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util5Stack3len17h2e0f01840a525a37E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN5alloc3vec16Vec$LT$T$C$A$GT$3len17haf3dcdb1269e75b5E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util10StackValue10unwrap_val17h6371c624edab9ae2E:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	adrp	x8, l___unnamed_64@PAGE
	add	x8, x8, l___unnamed_64@PAGEOFF
	mov	x9, x8
	str	x9, [sp, #24]
	str	x8, [sp, #32]
	ldr	x8, [sp, #24]
	str	x8, [sp, #8]
	ldr	x9, [sp, #32]
	str	x9, [sp, #16]
	ldr	x8, [x8]
	ldr	x9, [x9]
	subs	x8, x8, x9
	b.eq	LBB257_2
	b	LBB257_1
LBB257_1:
	ldr	x2, [sp, #16]
	ldr	x1, [sp, #8]
	strb	wzr, [sp, #47]
	add	x3, sp, #48
	str	xzr, [sp, #48]
	ldrb	w0, [sp, #47]
	adrp	x4, l___unnamed_65@PAGE
	add	x4, x4, l___unnamed_65@PAGEOFF
	bl	__ZN4core9panicking13assert_failed17h34e476f4fba23c5eE
LBB257_2:
	ldr	x0, [sp]
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util10StackValue13unwrap_fn_ptr17hc6af2877f010ef79E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w1, #64
	adrp	x2, l___unnamed_66@PAGE
	add	x2, x2, l___unnamed_66@PAGEOFF
	bl	__ZN59_$LT$$RF$u128$u20$as$u20$core..ops..bit..Shr$LT$i32$GT$$GT$3shr17h1514458bce170824E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main7vm_util13RegisterValue7get_val17h13f06b83c0370ad1E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	str	x1, [sp, #8]
	str	x2, [sp, #16]
	ldrb	w8, [x0]
	subs	w8, w8, #3
	cset	x8, ne
	cbnz	x8, LBB259_2
	b	LBB259_1
LBB259_1:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	ldr	x8, [sp]
	ldr	x2, [x8, #8]
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17hc2b02be7afc34c85E
	stur	x0, [x29, #-8]
	b	LBB259_3
LBB259_2:
	ldr	x8, [sp]
	stur	x8, [x29, #-8]
	b	LBB259_3
LBB259_3:
	ldur	x0, [x29, #-8]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_39halt_rule17hbb075f246d4b0f3dE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	mov	x0, x4
	str	x0, [sp]
	bl	__ZN4main9Registers6get_ip17hc33f2887d7bd86c3E
	adds	x8, x0, #1
	str	x8, [sp, #8]
	cset	w8, hs
	tbnz	w8, #0, LBB260_2
	b	LBB260_1
LBB260_1:
	ldr	x1, [sp, #8]
	ldr	x0, [sp]
	bl	__ZN4main9Registers6set_ip17h5ac825b8f5ec62e9E
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB260_2:
	.cfi_restore_state
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_67@PAGE
	add	x2, x2, l___unnamed_67@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_38add_rule17hdffbdf5f034a7464E:
	.cfi_startproc
	sub	sp, sp, #176
	.cfi_def_cfa_offset 176
	stp	x29, x30, [sp, #160]
	add	x29, sp, #160
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #72]
	str	x2, [sp, #80]
	stur	x3, [x29, #-72]
	stur	x4, [x29, #-64]
	mov	w8, #4
	mov	x1, x8
	str	x1, [sp]
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_dst17h7efd48c761d53b64E
	ldr	x1, [sp]
	mov	x2, x0
	ldr	x0, [sp, #8]
	str	x2, [sp, #16]
	mov	w8, #1
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x1, [sp]
	mov	x2, x0
	ldr	x0, [sp, #8]
	str	x2, [sp, #24]
	mov	w8, #2
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	stur	x9, [x29, #-56]
	stur	x8, [x29, #-48]
	stur	x0, [x29, #-40]
	ldur	x8, [x29, #-56]
	str	x8, [sp, #40]
	ldur	x0, [x29, #-48]
	ldur	x8, [x29, #-40]
	str	x8, [sp, #32]
	ldr	x1, [sp, #72]
	ldr	x2, [sp, #80]
	ldur	x3, [x29, #-72]
	ldur	x4, [x29, #-64]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x1, x0
	ldr	x0, [sp, #32]
	str	x1, [sp, #48]
	ldr	x1, [sp, #72]
	ldr	x2, [sp, #80]
	ldur	x3, [x29, #-72]
	ldur	x4, [x29, #-64]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x8, [sp, #40]
	mov	x1, x0
	ldr	x0, [sp, #48]
	ldur	x9, [x29, #-64]
	str	x9, [sp, #56]
	ldrb	w8, [x8]
	str	w8, [sp, #68]
	sub	x8, x29, #16
	bl	__ZN4main7vm_util5Value3add17hc9b1ea27128f8b31E
	ldr	x0, [sp, #56]
	ldr	w1, [sp, #68]
	ldur	q0, [x29, #-16]
	sub	x2, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE
	ldur	x0, [x29, #-64]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 176
	ldp	x29, x30, [sp, #160]
	add	sp, sp, #176
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_38sub_rule17hb9b5f4ea22098fd7E:
	.cfi_startproc
	sub	sp, sp, #176
	.cfi_def_cfa_offset 176
	stp	x29, x30, [sp, #160]
	add	x29, sp, #160
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #72]
	str	x2, [sp, #80]
	stur	x3, [x29, #-72]
	stur	x4, [x29, #-64]
	mov	w8, #4
	mov	x1, x8
	str	x1, [sp]
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_dst17h7efd48c761d53b64E
	ldr	x1, [sp]
	mov	x2, x0
	ldr	x0, [sp, #8]
	str	x2, [sp, #16]
	mov	w8, #1
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x1, [sp]
	mov	x2, x0
	ldr	x0, [sp, #8]
	str	x2, [sp, #24]
	mov	w8, #2
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	stur	x9, [x29, #-56]
	stur	x8, [x29, #-48]
	stur	x0, [x29, #-40]
	ldur	x8, [x29, #-56]
	str	x8, [sp, #40]
	ldur	x0, [x29, #-48]
	ldur	x8, [x29, #-40]
	str	x8, [sp, #32]
	ldr	x1, [sp, #72]
	ldr	x2, [sp, #80]
	ldur	x3, [x29, #-72]
	ldur	x4, [x29, #-64]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x1, x0
	ldr	x0, [sp, #32]
	str	x1, [sp, #48]
	ldr	x1, [sp, #72]
	ldr	x2, [sp, #80]
	ldur	x3, [x29, #-72]
	ldur	x4, [x29, #-64]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x8, [sp, #40]
	mov	x1, x0
	ldr	x0, [sp, #48]
	ldur	x9, [x29, #-64]
	str	x9, [sp, #56]
	ldrb	w8, [x8]
	str	w8, [sp, #68]
	sub	x8, x29, #16
	bl	__ZN4main7vm_util5Value3sub17hd68d6234769dfb5dE
	ldr	x0, [sp, #56]
	ldr	w1, [sp, #68]
	ldur	q0, [x29, #-16]
	sub	x2, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE
	ldur	x0, [x29, #-64]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 176
	ldp	x29, x30, [sp, #160]
	add	sp, sp, #176
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_38mul_rule17ha851ec2311b00d75E:
	.cfi_startproc
	sub	sp, sp, #176
	.cfi_def_cfa_offset 176
	stp	x29, x30, [sp, #160]
	add	x29, sp, #160
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #72]
	str	x2, [sp, #80]
	stur	x3, [x29, #-72]
	stur	x4, [x29, #-64]
	mov	w8, #4
	mov	x1, x8
	str	x1, [sp]
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_dst17h7efd48c761d53b64E
	ldr	x1, [sp]
	mov	x2, x0
	ldr	x0, [sp, #8]
	str	x2, [sp, #16]
	mov	w8, #1
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x1, [sp]
	mov	x2, x0
	ldr	x0, [sp, #8]
	str	x2, [sp, #24]
	mov	w8, #2
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	stur	x9, [x29, #-56]
	stur	x8, [x29, #-48]
	stur	x0, [x29, #-40]
	ldur	x8, [x29, #-56]
	str	x8, [sp, #40]
	ldur	x0, [x29, #-48]
	ldur	x8, [x29, #-40]
	str	x8, [sp, #32]
	ldr	x1, [sp, #72]
	ldr	x2, [sp, #80]
	ldur	x3, [x29, #-72]
	ldur	x4, [x29, #-64]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x1, x0
	ldr	x0, [sp, #32]
	str	x1, [sp, #48]
	ldr	x1, [sp, #72]
	ldr	x2, [sp, #80]
	ldur	x3, [x29, #-72]
	ldur	x4, [x29, #-64]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x8, [sp, #40]
	mov	x1, x0
	ldr	x0, [sp, #48]
	ldur	x9, [x29, #-64]
	str	x9, [sp, #56]
	ldrb	w8, [x8]
	str	w8, [sp, #68]
	sub	x8, x29, #16
	bl	__ZN4main7vm_util5Value3mul17he602582e17797115E
	ldr	x0, [sp, #56]
	ldr	w1, [sp, #68]
	ldur	q0, [x29, #-16]
	sub	x2, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE
	ldur	x0, [x29, #-64]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 176
	ldp	x29, x30, [sp, #160]
	add	sp, sp, #176
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_38div_rule17h0e3ddd5be354696bE:
	.cfi_startproc
	sub	sp, sp, #176
	.cfi_def_cfa_offset 176
	stp	x29, x30, [sp, #160]
	add	x29, sp, #160
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #72]
	str	x2, [sp, #80]
	stur	x3, [x29, #-72]
	stur	x4, [x29, #-64]
	mov	w8, #4
	mov	x1, x8
	str	x1, [sp]
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_dst17h7efd48c761d53b64E
	ldr	x1, [sp]
	mov	x2, x0
	ldr	x0, [sp, #8]
	str	x2, [sp, #16]
	mov	w8, #1
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x1, [sp]
	mov	x2, x0
	ldr	x0, [sp, #8]
	str	x2, [sp, #24]
	mov	w8, #2
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x9, [sp, #16]
	ldr	x8, [sp, #24]
	stur	x9, [x29, #-56]
	stur	x8, [x29, #-48]
	stur	x0, [x29, #-40]
	ldur	x8, [x29, #-56]
	str	x8, [sp, #40]
	ldur	x0, [x29, #-48]
	ldur	x8, [x29, #-40]
	str	x8, [sp, #32]
	ldr	x1, [sp, #72]
	ldr	x2, [sp, #80]
	ldur	x3, [x29, #-72]
	ldur	x4, [x29, #-64]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x1, x0
	ldr	x0, [sp, #32]
	str	x1, [sp, #48]
	ldr	x1, [sp, #72]
	ldr	x2, [sp, #80]
	ldur	x3, [x29, #-72]
	ldur	x4, [x29, #-64]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x8, [sp, #40]
	mov	x1, x0
	ldr	x0, [sp, #48]
	ldur	x9, [x29, #-64]
	str	x9, [sp, #56]
	ldrb	w8, [x8]
	str	w8, [sp, #68]
	sub	x8, x29, #16
	bl	__ZN4main7vm_util5Value3div17he68c9a11be5f0467E
	ldr	x0, [sp, #56]
	ldr	w1, [sp, #68]
	ldur	q0, [x29, #-16]
	sub	x2, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE
	ldur	x0, [x29, #-64]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 176
	ldp	x29, x30, [sp, #160]
	add	sp, sp, #176
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_318push_function_rule17h0abe9084717f69c6E:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x3, [sp, #8]
	str	x4, [sp]
	mov	w8, #4
	mov	x1, x8
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand20unwrap_instrs_amount17h1886dad8a24573ecE
	mov	x8, x0
	ldr	x0, [sp]
	str	x8, [sp, #16]
	bl	__ZN4main9Registers6get_ip17hc33f2887d7bd86c3E
	mov	x8, x0
	ldr	x0, [sp, #8]
	sub	x1, x29, #16
	stur	x8, [x29, #-8]
	mov	w8, #3
	sturb	w8, [x29, #-16]
	bl	__ZN4main7vm_util5Stack4push17hf8474d57900a6c0fE
	ldr	x0, [sp, #16]
	adds	x8, x0, #1
	str	x8, [sp, #24]
	cset	w8, hs
	tbnz	w8, #0, LBB265_2
	b	LBB265_1
LBB265_1:
	ldr	x1, [sp, #24]
	ldr	x0, [sp]
	bl	__ZN4main9Registers6add_ip17hb3c329094c540883E
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB265_2:
	.cfi_restore_state
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_68@PAGE
	add	x2, x2, l___unnamed_68@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_316cmp_lt_jump_rule17h3eccff231d922f4aE:
	.cfi_startproc
	sub	sp, sp, #160
	.cfi_def_cfa_offset 160
	stp	x29, x30, [sp, #144]
	add	x29, sp, #144
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #16]
	str	x1, [sp, #64]
	str	x2, [sp, #72]
	stur	x3, [x29, #-64]
	stur	x4, [x29, #-56]
	mov	w8, #4
	mov	x1, x8
	str	x1, [sp, #8]
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x1, [sp, #8]
	mov	x2, x0
	ldr	x0, [sp, #16]
	str	x2, [sp, #24]
	mov	w8, #1
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x1, [sp, #8]
	mov	x2, x0
	ldr	x0, [sp, #16]
	str	x2, [sp, #32]
	mov	w8, #2
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand14unwrap_jmp_pos17h49d5dd2a8bc7be66E
	ldr	x1, [sp, #8]
	mov	x8, x0
	ldr	x0, [sp, #16]
	str	x8, [sp, #40]
	mov	w8, #3
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand14unwrap_jmp_pos17h49d5dd2a8bc7be66E
	ldr	x10, [sp, #24]
	ldr	x9, [sp, #32]
	ldr	x8, [sp, #40]
	ldr	x1, [sp, #64]
	ldr	x2, [sp, #72]
	ldur	x3, [x29, #-64]
	ldur	x4, [x29, #-56]
	stur	x10, [x29, #-32]
	stur	x9, [x29, #-24]
	stur	x8, [x29, #-16]
	stur	x0, [x29, #-8]
	ldur	x0, [x29, #-32]
	ldur	x8, [x29, #-24]
	stur	x8, [x29, #-48]
	ldur	x8, [x29, #-16]
	str	x8, [sp, #48]
	ldur	x8, [x29, #-8]
	str	x8, [sp, #56]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	ldr	x1, [sp, #64]
	ldr	x2, [sp, #72]
	ldur	x3, [x29, #-64]
	ldur	x4, [x29, #-56]
	mov	x5, x0
	ldur	x0, [x29, #-48]
	stur	x5, [x29, #-40]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	mov	x1, x0
	ldur	x0, [x29, #-40]
	bl	__ZN4main7vm_util5Value5cmp_l17hf0d8db80eb6e44ccE
	tbnz	w0, #0, LBB266_2
	b	LBB266_1
LBB266_1:
	ldr	x1, [sp, #56]
	ldur	x0, [x29, #-56]
	bl	__ZN4main9Registers6set_ip17h5ac825b8f5ec62e9E
	b	LBB266_3
LBB266_2:
	ldr	x1, [sp, #48]
	ldur	x0, [x29, #-56]
	bl	__ZN4main9Registers6set_ip17h5ac825b8f5ec62e9E
	b	LBB266_3
LBB266_3:
	.cfi_def_cfa wsp, 160
	ldp	x29, x30, [sp, #144]
	add	sp, sp, #160
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_39call_rule17hab79b569da00cc74E:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #32]
	str	x3, [sp, #48]
	str	x4, [sp, #16]
	mov	w8, #4
	mov	x1, x8
	str	x1, [sp, #24]
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand16unwrap_stack_ptr17h6d961735c953a191E
	ldr	x1, [sp, #24]
	mov	x2, x0
	ldr	x0, [sp, #32]
	str	x2, [sp, #40]
	mov	w8, #1
	mov	x2, x8
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand17unwrap_args_count17h383375d773e28d50E
	ldr	x9, [sp, #40]
	mov	x8, x0
	ldr	x0, [sp, #48]
	stur	x9, [x29, #-16]
	stur	x8, [x29, #-8]
	ldur	x8, [x29, #-16]
	stur	x8, [x29, #-40]
	ldur	x8, [x29, #-8]
	stur	x8, [x29, #-32]
	bl	__ZN4main7vm_util5Stack3len17h2e0f01840a525a37E
	ldur	x8, [x29, #-32]
	subs	x9, x0, x8
	stur	x9, [x29, #-24]
	subs	x8, x0, x8
	b.lo	LBB267_2
	b	LBB267_1
LBB267_1:
	ldr	x0, [sp, #48]
	ldur	x1, [x29, #-24]
	bl	__ZN4main7vm_util5Stack16push_stack_frame17ha9f296821220ecf7E
	ldr	x0, [sp, #48]
	ldur	x1, [x29, #-40]
	bl	__ZN4main7vm_util5Stack3get17h6f97994b4e633039E
	bl	__ZN4main7vm_util10StackValue13unwrap_fn_ptr17hc6af2877f010ef79E
	adds	x8, x0, #1
	str	x8, [sp, #8]
	cset	w8, hs
	tbnz	w8, #0, LBB267_4
	b	LBB267_3
LBB267_2:
	adrp	x0, _str.5@PAGE
	add	x0, x0, _str.5@PAGEOFF
	mov	w8, #33
	mov	x1, x8
	adrp	x2, l___unnamed_69@PAGE
	add	x2, x2, l___unnamed_69@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
LBB267_3:
	ldr	x1, [sp, #8]
	ldr	x0, [sp, #16]
	bl	__ZN4main9Registers6set_ip17h5ac825b8f5ec62e9E
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB267_4:
	.cfi_restore_state
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_70@PAGE
	add	x2, x2, l___unnamed_70@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_38ret_rule17h7e5c19167dfe3b7bE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp]
	str	x2, [sp, #8]
	str	x3, [sp, #24]
	str	x4, [sp, #16]
	mov	w8, #4
	mov	x1, x8
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x1, [sp]
	ldr	x2, [sp, #8]
	ldr	x4, [sp, #16]
	ldr	x3, [sp, #24]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	add	x8, sp, #32
	bl	__ZN59_$LT$main..vm_util..Value$u20$as$u20$core..clone..Clone$GT$5clone17h5a17eabc18bf908aE
	ldr	x0, [sp, #24]
	bl	__ZN4main7vm_util5Stack16read_stack_frame17h9315cae95e177d90E
	mov	x1, x0
	ldr	x0, [sp, #24]
	bl	__ZN4main7vm_util5Stack8truncate17h0d9f5788a7945e3eE
	ldr	x0, [sp, #24]
	bl	__ZN4main7vm_util5Stack15pop_stack_frame17h14e601ae09d004d1E
	ldr	x0, [sp, #24]
	ldr	q0, [sp, #32]
	sub	x1, x29, #16
	stur	q0, [x29, #-16]
	bl	__ZN4main7vm_util5Stack4push17hf8474d57900a6c0fE
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_315push_stack_rule17hd93ea35243e08d64E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp]
	str	x2, [sp, #8]
	str	x3, [sp, #16]
	str	x4, [sp, #24]
	mov	w8, #4
	mov	x1, x8
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand10unwrap_src17h8ce2eb25021ad200E
	ldr	x1, [sp]
	ldr	x2, [sp, #8]
	ldr	x3, [sp, #16]
	ldr	x4, [sp, #24]
	bl	__ZN4main7vm_util3Src7get_val17h89eb451d16de3c73E
	add	x8, sp, #32
	bl	__ZN59_$LT$main..vm_util..Value$u20$as$u20$core..clone..Clone$GT$5clone17h5a17eabc18bf908aE
	ldr	x0, [sp, #16]
	ldr	q0, [sp, #32]
	sub	x1, x29, #16
	stur	q0, [x29, #-16]
	bl	__ZN4main7vm_util5Stack4push17hf8474d57900a6c0fE
	ldr	x0, [sp, #24]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_319push_instr_ptr_rule17ha831a1e5e5f90844E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x4, [sp, #8]
	mov	w8, #4
	mov	x1, x8
	mov	x2, #0
	bl	__ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$13get_unchecked17heef6f6ced702b0f7E
	bl	__ZN4main12instructions7Operand16unwrap_instr_ptr17hc6519018ee9163c9E
	ldr	x0, [sp, #8]
	bl	__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_49halt_rule17h5c00d6745cba7c6eE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions15HaltInstruction7execute17h3d94b71409f21791E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_48add_rule17hedda029abc2b5849E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions14AddInstruction7execute17hbbf2fe13542e736cE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_48sub_rule17h9f46d4406a21e1b8E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions14SubInstruction7execute17h32bbd702aef68a41E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_48mul_rule17h729dc08828dc9e55E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions14MulInstruction7execute17hc1927e975430bb56E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_48div_rule17h46423645fb2543edE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions14DivInstruction7execute17h9b107b61b04a5120E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_418push_function_rule17h6e921980851f1ef6E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions23PushFunctionInstruction7execute17h69237e0641c80bcfE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_416cmp_lt_jump_rule17had435da49cfbf434E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions19CmpLtJmpInstruction7execute17h8adc5d8c8dfd3149E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_49call_rule17h8da5b9cd672b6eb3E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions15CallInstruction7execute17h9b2d9394b717ebfbE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_48ret_rule17h27f3b67f5d4c60ceE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions14RetInstruction7execute17ha251383e91ffdd6aE
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_415push_stack_rule17h7d20b29f30772283E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions20PushStackInstruction7execute17hbbff7cce66887ca7E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main16rules_approach_419push_instr_ptr_rule17h311ff0bbf4171056E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4main12instructions23PushInstrPtrInstruction7execute17hb16ecd66cf1d65f2E
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main4main17h8bd578f601f0fa06E:
Lfunc_begin24:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception24
	stp	x28, x27, [sp, #-32]!
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	.cfi_remember_state
	sub	sp, sp, #1, lsl #12
	str	xzr, [sp]
	sub	sp, sp, #1, lsl #12
	str	xzr, [sp]
	sub	sp, sp, #1, lsl #12
	str	xzr, [sp]
	sub	sp, sp, #192
	add	x8, sp, #2, lsl #12
	add	x8, x8, #64
	str	x8, [sp, #1240]
	add	x8, sp, #3920
	str	x8, [sp, #1248]
	mov	w8, #48
	mov	x0, x8
	mov	w8, #8
	mov	x1, x8
	bl	__ZN5alloc5alloc15exchange_malloc17h941cdcdc6bb63bd7E
	str	x0, [sp, #1256]
	mov	w8, #35
	str	w8, [sp, #1300]
	strb	wzr, [sp, #1296]
	mov	w8, #2
	str	w8, [sp, #1316]
	strb	wzr, [sp, #1312]
	mov	w8, #1
	str	w8, [sp, #1332]
	strb	wzr, [sp, #1328]
	str	x0, [sp, #1264]
	and	x8, x0, #0x7
	cbnz	x8, LBB282_2
	b	LBB282_1
LBB282_1:
	ldr	x0, [sp, #1256]
	ldr	q0, [sp, #1296]
	str	q0, [x0]
	ldr	q0, [sp, #1312]
	str	q0, [x0, #16]
	ldr	q0, [sp, #1328]
	str	q0, [x0, #32]
	add	x8, sp, #1272
	mov	w9, #3
	mov	x1, x9
	bl	__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17hb7db4ecec3366c8cE
Ltmp148:
	mov	w8, #672
	mov	x0, x8
	mov	w8, #8
	mov	x1, x8
	bl	__ZN5alloc5alloc15exchange_malloc17h941cdcdc6bb63bd7E
	str	x0, [sp, #1232]
Ltmp149:
	b	LBB282_5
LBB282_2:
	ldr	x1, [sp, #1264]
	mov	w8, #8
	mov	x0, x8
	adrp	x2, l___unnamed_71@PAGE
	add	x2, x2, l___unnamed_71@PAGEOFF
	bl	__ZN4core9panicking36panic_misaligned_pointer_dereference17hdc38b89bae20a49aE
LBB282_3:
Ltmp466:
	add	x0, sp, #1272
	bl	__ZN4core3ptr64drop_in_place$LT$alloc..vec..Vec$LT$main..vm_util..Value$GT$$GT$17h5e3a7fb19708ec5dE
Ltmp467:
	b	LBB282_182
LBB282_4:
Ltmp465:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_3
LBB282_5:
	ldr	x8, [sp, #1232]
	str	x8, [sp, #1376]
	mov	w8, #3
	strb	w8, [sp, #1439]
Ltmp150:
	add	x8, sp, #1440
	mov	w9, #10
	mov	x0, x9
	bl	__ZN4main12instructions9InstrData11new_push_fn17h4fab9dbee3b9b2a5E
Ltmp151:
	b	LBB282_8
LBB282_6:
Ltmp233:
	add	x0, sp, #1376
	bl	__ZN4core3ptr101drop_in_place$LT$alloc..boxed..Box$LT$$u5b$main..instructions..InstructionV2$u3b$$u20$14$u5d$$GT$$GT$17hbadbccd766535580E
Ltmp234:
	b	LBB282_3
LBB282_7:
Ltmp232:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_6
LBB282_8:
	ldrb	w0, [sp, #1439]
Ltmp152:
	add	x8, sp, #1384
	add	x1, sp, #1440
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp153:
	b	LBB282_9
LBB282_9:
	mov	w8, #14
	strb	w8, [sp, #1535]
Ltmp154:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1216]
	str	w1, [sp, #1228]
Ltmp155:
	b	LBB282_10
LBB282_10:
Ltmp156:
	mov	w8, #1
	mov	x0, x8
	bl	__ZN4main7vm_util8ConstPtr3new17h473b7f254df66ad8E
	str	x0, [sp, #1208]
Ltmp157:
	b	LBB282_11
LBB282_11:
	ldr	x8, [sp, #1208]
	ldr	w9, [sp, #1228]
	ldr	x10, [sp, #1216]
	add	x0, sp, #1576
	str	x10, [sp, #1576]
	and	w9, w9, #0x1
	strb	w9, [sp, #1584]
	str	x8, [sp, #1592]
	mov	w8, #2
	str	x8, [sp, #1600]
	mov	w8, #3
	str	x8, [sp, #1608]
Ltmp158:
	add	x8, sp, #1536
	bl	__ZN4main12instructions9InstrData11new_cmp_jmp17hd109254bc2ebee4cE
Ltmp159:
	b	LBB282_12
LBB282_12:
	ldrb	w0, [sp, #1535]
Ltmp160:
	add	x8, sp, #1480
	add	x1, sp, #1536
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp161:
	b	LBB282_13
LBB282_13:
	mov	w8, #16
	strb	w8, [sp, #1671]
Ltmp162:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1192]
	str	w1, [sp, #1204]
Ltmp163:
	b	LBB282_14
LBB282_14:
Ltmp164:
	ldr	w9, [sp, #1204]
	ldr	x0, [sp, #1192]
	add	x8, sp, #1672
	and	w1, w9, #0x1
	bl	__ZN4main12instructions9InstrData13new_ret_stack17hdf0883ccfb057066E
Ltmp165:
	b	LBB282_15
LBB282_15:
	ldrb	w0, [sp, #1671]
Ltmp166:
	add	x8, sp, #1616
	add	x1, sp, #1672
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp167:
	b	LBB282_16
LBB282_16:
	mov	w8, #2
	strb	w8, [sp, #1767]
	strb	wzr, [sp, #1847]
Ltmp168:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1176]
	str	w1, [sp, #1188]
Ltmp169:
	b	LBB282_17
LBB282_17:
Ltmp170:
	mov	w8, #1
	mov	x0, x8
	bl	__ZN4main7vm_util8ConstPtr3new17h473b7f254df66ad8E
	str	x0, [sp, #1168]
Ltmp171:
	b	LBB282_18
LBB282_18:
	ldr	x8, [sp, #1168]
	ldr	w9, [sp, #1188]
	ldr	x10, [sp, #1176]
	ldrb	w11, [sp, #1847]
	add	x0, sp, #1808
	strb	w11, [sp, #1824]
	str	x10, [sp, #1808]
	and	w9, w9, #0x1
	strb	w9, [sp, #1816]
	str	x8, [sp, #1832]
Ltmp172:
	add	x8, sp, #1768
	bl	__ZN4main12instructions9InstrData26new_binary_reg_stack_const17h2cc680dacedbc3dfE
Ltmp173:
	b	LBB282_19
LBB282_19:
	ldrb	w0, [sp, #1767]
Ltmp174:
	add	x8, sp, #1712
	add	x1, sp, #1768
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp175:
	b	LBB282_20
LBB282_20:
	mov	w8, #19
	strb	w8, [sp, #1903]
	strb	wzr, [sp, #1951]
	ldrb	w0, [sp, #1951]
Ltmp176:
	add	x8, sp, #1904
	bl	__ZN4main12instructions9InstrData18new_push_stack_reg17h82a672bcbeea080eE
Ltmp177:
	b	LBB282_21
LBB282_21:
	ldrb	w0, [sp, #1903]
Ltmp178:
	add	x8, sp, #1848
	add	x1, sp, #1904
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp179:
	b	LBB282_22
LBB282_22:
	mov	w8, #20
	strb	w8, [sp, #2007]
Ltmp180:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1152]
	str	w1, [sp, #1164]
Ltmp181:
	b	LBB282_23
LBB282_23:
	ldr	w8, [sp, #1164]
	ldr	x9, [sp, #1152]
	add	x0, sp, #2048
	str	x9, [sp, #2048]
	and	w8, w8, #0x1
	strb	w8, [sp, #2056]
	mov	w8, #1
	str	x8, [sp, #2064]
	strb	wzr, [sp, #2072]
Ltmp182:
	add	x8, sp, #2008
	bl	__ZN4main12instructions9InstrData8new_call17h9c0e33e6f3a1afa3E
Ltmp183:
	b	LBB282_24
LBB282_24:
	ldrb	w0, [sp, #2007]
Ltmp184:
	add	x8, sp, #1952
	add	x1, sp, #2008
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp185:
	b	LBB282_25
LBB282_25:
	mov	w8, #2
	strb	w8, [sp, #2135]
	strb	wzr, [sp, #2215]
Ltmp186:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1136]
	str	w1, [sp, #1148]
Ltmp187:
	b	LBB282_26
LBB282_26:
Ltmp188:
	mov	w8, #2
	mov	x0, x8
	bl	__ZN4main7vm_util8ConstPtr3new17h473b7f254df66ad8E
	str	x0, [sp, #1128]
Ltmp189:
	b	LBB282_27
LBB282_27:
	ldr	x8, [sp, #1128]
	ldr	w9, [sp, #1148]
	ldr	x10, [sp, #1136]
	ldrb	w11, [sp, #2215]
	add	x0, sp, #2176
	strb	w11, [sp, #2192]
	str	x10, [sp, #2176]
	and	w9, w9, #0x1
	strb	w9, [sp, #2184]
	str	x8, [sp, #2200]
Ltmp190:
	add	x8, sp, #2136
	bl	__ZN4main12instructions9InstrData26new_binary_reg_stack_const17h2cc680dacedbc3dfE
Ltmp191:
	b	LBB282_28
LBB282_28:
	ldrb	w0, [sp, #2135]
Ltmp192:
	add	x8, sp, #2080
	add	x1, sp, #2136
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp193:
	b	LBB282_29
LBB282_29:
	mov	w8, #19
	strb	w8, [sp, #2271]
	strb	wzr, [sp, #2319]
	ldrb	w0, [sp, #2319]
Ltmp194:
	add	x8, sp, #2272
	bl	__ZN4main12instructions9InstrData18new_push_stack_reg17h82a672bcbeea080eE
Ltmp195:
	b	LBB282_30
LBB282_30:
	ldrb	w0, [sp, #2271]
Ltmp196:
	add	x8, sp, #2216
	add	x1, sp, #2272
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp197:
	b	LBB282_31
LBB282_31:
	mov	w8, #20
	strb	w8, [sp, #2375]
Ltmp198:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1112]
	str	w1, [sp, #1124]
Ltmp199:
	b	LBB282_32
LBB282_32:
	ldr	w8, [sp, #1124]
	ldr	x9, [sp, #1112]
	add	x0, sp, #2416
	str	x9, [sp, #2416]
	and	w8, w8, #0x1
	strb	w8, [sp, #2424]
	mov	w8, #1
	str	x8, [sp, #2432]
	strb	wzr, [sp, #2440]
Ltmp200:
	add	x8, sp, #2376
	bl	__ZN4main12instructions9InstrData8new_call17h9c0e33e6f3a1afa3E
Ltmp201:
	b	LBB282_33
LBB282_33:
	ldrb	w0, [sp, #2375]
Ltmp202:
	add	x8, sp, #2320
	add	x1, sp, #2376
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp203:
	b	LBB282_34
LBB282_34:
	mov	w8, #1
	strb	w8, [sp, #2503]
	strb	wzr, [sp, #2591]
Ltmp204:
	mov	w8, #1
	mov	x0, x8
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1096]
	str	w1, [sp, #1108]
Ltmp205:
	b	LBB282_35
LBB282_35:
Ltmp206:
	mov	w8, #2
	mov	x0, x8
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1080]
	str	w1, [sp, #1092]
Ltmp207:
	b	LBB282_36
LBB282_36:
	ldr	w8, [sp, #1092]
	ldr	x9, [sp, #1080]
	ldr	w10, [sp, #1108]
	ldr	x11, [sp, #1096]
	ldrb	w12, [sp, #2591]
	add	x0, sp, #2544
	strb	w12, [sp, #2560]
	str	x11, [sp, #2544]
	mov	w11, #1
	and	w10, w10, w11
	strb	w10, [sp, #2552]
	str	x9, [sp, #2568]
	and	w8, w8, #0x1
	strb	w8, [sp, #2576]
Ltmp208:
	add	x8, sp, #2504
	bl	__ZN4main12instructions9InstrData26new_binary_reg_stack_stack17hb190ab858b21569dE
Ltmp209:
	b	LBB282_37
LBB282_37:
	ldrb	w0, [sp, #2503]
Ltmp210:
	add	x8, sp, #2448
	add	x1, sp, #2504
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp211:
	b	LBB282_38
LBB282_38:
	mov	w8, #17
	strb	w8, [sp, #2647]
	strb	wzr, [sp, #2695]
	ldrb	w0, [sp, #2695]
Ltmp212:
	add	x8, sp, #2648
	bl	__ZN4main12instructions9InstrData11new_ret_reg17h6c59eedeb1f85c16E
Ltmp213:
	b	LBB282_39
LBB282_39:
	ldrb	w0, [sp, #2647]
Ltmp214:
	add	x8, sp, #2592
	add	x1, sp, #2648
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp215:
	b	LBB282_40
LBB282_40:
	mov	w8, #18
	strb	w8, [sp, #2751]
Ltmp216:
	mov	x0, #0
	bl	__ZN4main7vm_util8ConstPtr3new17h473b7f254df66ad8E
	str	x0, [sp, #1072]
Ltmp217:
	b	LBB282_41
LBB282_41:
Ltmp218:
	ldr	x0, [sp, #1072]
	add	x8, sp, #2752
	bl	__ZN4main12instructions9InstrData20new_push_stack_const17hd40771eac948cb2fE
Ltmp219:
	b	LBB282_42
LBB282_42:
	ldrb	w0, [sp, #2751]
Ltmp220:
	add	x8, sp, #2696
	add	x1, sp, #2752
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp221:
	b	LBB282_43
LBB282_43:
	mov	w8, #20
	strb	w8, [sp, #2847]
Ltmp222:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1056]
	str	w1, [sp, #1068]
Ltmp223:
	b	LBB282_44
LBB282_44:
	ldr	w8, [sp, #1068]
	ldr	x9, [sp, #1056]
	add	x0, sp, #2888
	str	x9, [sp, #2888]
	and	w8, w8, #0x1
	strb	w8, [sp, #2896]
	mov	w8, #1
	str	x8, [sp, #2904]
	strb	wzr, [sp, #2912]
Ltmp224:
	add	x8, sp, #2848
	bl	__ZN4main12instructions9InstrData8new_call17h9c0e33e6f3a1afa3E
Ltmp225:
	b	LBB282_45
LBB282_45:
	ldrb	w0, [sp, #2847]
Ltmp226:
	add	x8, sp, #2792
	add	x1, sp, #2848
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp227:
	b	LBB282_46
LBB282_46:
	strb	wzr, [sp, #2975]
Ltmp228:
	add	x8, sp, #2976
	bl	__ZN4main12instructions9InstrData8new_halt17h11111db872084eb7E
Ltmp229:
	b	LBB282_47
LBB282_47:
	ldrb	w0, [sp, #2975]
Ltmp230:
	add	x8, sp, #2920
	add	x1, sp, #2976
	bl	__ZN4main12instructions13InstructionV23new17hd6541c5f8eab9999E
Ltmp231:
	b	LBB282_48
LBB282_48:
	ldr	x8, [sp, #1376]
	str	x8, [sp, #1048]
	and	x8, x8, #0x7
	cbnz	x8, LBB282_50
	b	LBB282_49
LBB282_49:
	ldr	x0, [sp, #1048]
	add	x1, sp, #1384
	mov	w8, #48
	mov	x2, x8
	str	x2, [sp, #1040]
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #48
	add	x1, sp, #1480
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #96
	add	x1, sp, #1616
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #144
	add	x1, sp, #1712
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #192
	add	x1, sp, #1848
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #240
	add	x1, sp, #1952
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #288
	add	x1, sp, #2080
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #336
	add	x1, sp, #2216
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #384
	add	x1, sp, #2320
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #432
	add	x1, sp, #2448
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #480
	add	x1, sp, #2592
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #528
	add	x1, sp, #2696
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #576
	add	x1, sp, #2792
	bl	_memcpy
	ldr	x8, [sp, #1048]
	ldr	x2, [sp, #1040]
	add	x0, x8, #624
	add	x1, sp, #2920
	bl	_memcpy
	ldr	x0, [sp, #1376]
Ltmp235:
	add	x8, sp, #1352
	mov	w9, #14
	mov	x1, x9
	bl	__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17h19babe651bef01fbE
Ltmp236:
	b	LBB282_51
LBB282_50:
	ldr	x1, [sp, #1048]
	mov	w8, #8
	mov	x0, x8
	adrp	x2, l___unnamed_72@PAGE
	add	x2, x2, l___unnamed_72@PAGEOFF
	bl	__ZN4core9panicking36panic_misaligned_pointer_dereference17hdc38b89bae20a49aE
LBB282_51:
	add	x8, sp, #1352
	str	x8, [sp, #1344]
Ltmp237:
	mov	w8, #672
	mov	x0, x8
	mov	w8, #8
	mov	x1, x8
	bl	__ZN5alloc5alloc15exchange_malloc17h941cdcdc6bb63bd7E
	str	x0, [sp, #1032]
Ltmp238:
	b	LBB282_54
LBB282_52:
Ltmp461:
	add	x0, sp, #1352
	bl	__ZN4core3ptr77drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..InstructionV2$GT$$GT$17h570c83b0fdd350d5E
Ltmp462:
	b	LBB282_3
LBB282_53:
Ltmp460:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_52
LBB282_54:
	ldr	x8, [sp, #1032]
	str	x8, [sp, #3040]
	mov	w8, #8
	str	x8, [sp, #3056]
	mov	w8, #5
	strb	w8, [sp, #3048]
Ltmp239:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1016]
	str	w1, [sp, #1028]
Ltmp240:
	b	LBB282_57
LBB282_55:
Ltmp264:
	add	x0, sp, #3040
	bl	__ZN4core3ptr99drop_in_place$LT$alloc..boxed..Box$LT$$u5b$main..instructions..Instruction$u3b$$u20$12$u5d$$GT$$GT$17h607d9b4bfc4c1fe3E
Ltmp265:
	b	LBB282_52
LBB282_56:
Ltmp263:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_55
LBB282_57:
	ldr	w9, [sp, #1028]
	ldr	x8, [sp, #1016]
	str	x8, [sp, #3160]
	mov	w8, #1
	and	w9, w9, w8
	strb	w9, [sp, #3168]
	mov	w9, #1
	str	x9, [sp, #3176]
	mov	w9, #3
	strb	w9, [sp, #3184]
	add	x9, sp, #3104
	add	x10, sp, #2905
	ldur	q0, [x10, #255]
	stur	q0, [x9, #24]
	add	x10, sp, #2921
	ldur	q0, [x10, #255]
	stur	q0, [x9, #40]
	mov	w9, #2
	str	x9, [sp, #3112]
	mov	w9, #3
	str	x9, [sp, #3120]
	mov	w9, #8
	strb	w9, [sp, #3104]
Ltmp241:
	mov	x0, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #1000]
	str	w1, [sp, #1012]
Ltmp242:
	b	LBB282_58
LBB282_58:
	ldr	w9, [sp, #1012]
	ldr	x8, [sp, #1000]
	str	x8, [sp, #3200]
	mov	w8, #1
	and	w9, w9, w8
	strb	w9, [sp, #3208]
	mov	w9, #19
	strb	w9, [sp, #3192]
	mov	w9, #4
	strb	w9, [sp, #3311]
Ltmp243:
	mov	x0, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #984]
	str	w1, [sp, #996]
Ltmp244:
	b	LBB282_59
LBB282_59:
Ltmp245:
	mov	w8, #1
	mov	x0, x8
	bl	__ZN4main7vm_util8ConstPtr3new17h473b7f254df66ad8E
	str	x0, [sp, #976]
Ltmp246:
	b	LBB282_60
LBB282_60:
	ldr	x8, [sp, #976]
	ldr	w9, [sp, #996]
	ldr	x10, [sp, #984]
	ldrb	w11, [sp, #3311]
	strb	w11, [sp, #3249]
	str	x10, [sp, #3264]
	mov	w10, #1
	and	w9, w9, w10
	strb	w9, [sp, #3272]
	str	x8, [sp, #3256]
	mov	w8, #2
	strb	w8, [sp, #3248]
Ltmp247:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #960]
	str	w1, [sp, #972]
Ltmp248:
	b	LBB282_61
LBB282_61:
	ldr	w9, [sp, #972]
	ldr	x8, [sp, #960]
	str	x8, [sp, #3328]
	mov	w8, #1
	and	w9, w9, w8
	strb	w9, [sp, #3336]
	mov	w9, #1
	str	x9, [sp, #3320]
	strb	wzr, [sp, #3313]
	mov	w9, #18
	strb	w9, [sp, #3312]
	mov	w9, #4
	strb	w9, [sp, #3431]
Ltmp249:
	mov	x0, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #944]
	str	w1, [sp, #956]
Ltmp250:
	b	LBB282_62
LBB282_62:
Ltmp251:
	mov	w8, #2
	mov	x0, x8
	bl	__ZN4main7vm_util8ConstPtr3new17h473b7f254df66ad8E
	str	x0, [sp, #936]
Ltmp252:
	b	LBB282_63
LBB282_63:
	ldr	x8, [sp, #936]
	ldr	w9, [sp, #956]
	ldr	x10, [sp, #944]
	ldrb	w11, [sp, #3431]
	strb	w11, [sp, #3369]
	str	x10, [sp, #3384]
	mov	w10, #1
	and	w9, w9, w10
	strb	w9, [sp, #3392]
	str	x8, [sp, #3376]
	mov	w8, #2
	strb	w8, [sp, #3368]
Ltmp253:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #920]
	str	w1, [sp, #932]
Ltmp254:
	b	LBB282_64
LBB282_64:
	ldr	w9, [sp, #932]
	ldr	x8, [sp, #920]
	str	x8, [sp, #3448]
	mov	w8, #1
	and	w9, w9, w8
	strb	w9, [sp, #3456]
	mov	w9, #1
	mov	x0, x9
	str	x0, [sp, #3440]
	strb	wzr, [sp, #3433]
	mov	w9, #18
	strb	w9, [sp, #3432]
	strb	wzr, [sp, #3551]
Ltmp255:
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #904]
	str	w1, [sp, #916]
Ltmp256:
	b	LBB282_65
LBB282_65:
Ltmp257:
	mov	w8, #2
	mov	x0, x8
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #888]
	str	w1, [sp, #900]
Ltmp258:
	b	LBB282_66
LBB282_66:
	ldr	w9, [sp, #900]
	ldr	x10, [sp, #888]
	ldr	w11, [sp, #916]
	ldr	x8, [sp, #904]
	ldrb	w12, [sp, #3551]
	strb	w12, [sp, #3489]
	str	x8, [sp, #3496]
	mov	w8, #1
	mov	w12, #1
	and	w11, w11, w12
	strb	w11, [sp, #3504]
	str	x10, [sp, #3512]
	and	w9, w9, #0x1
	strb	w9, [sp, #3520]
	strb	w8, [sp, #3488]
	strb	wzr, [sp, #3615]
	ldrb	w8, [sp, #3615]
	strb	w8, [sp, #3553]
	mov	w8, #21
	strb	w8, [sp, #3552]
Ltmp259:
	mov	x0, #0
	bl	__ZN4main7vm_util8ConstPtr3new17h473b7f254df66ad8E
	str	x0, [sp, #880]
Ltmp260:
	b	LBB282_67
LBB282_67:
	ldr	x8, [sp, #880]
	str	x8, [sp, #3624]
	mov	w8, #22
	strb	w8, [sp, #3616]
Ltmp261:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #864]
	str	w1, [sp, #876]
Ltmp262:
	b	LBB282_68
LBB282_68:
	ldr	w8, [sp, #876]
	ldr	x9, [sp, #864]
	str	x9, [sp, #3688]
	and	w8, w8, #0x1
	strb	w8, [sp, #3696]
	mov	w8, #1
	str	x8, [sp, #3680]
	strb	wzr, [sp, #3673]
	mov	w8, #18
	strb	w8, [sp, #3672]
	strb	wzr, [sp, #3728]
	ldr	x8, [sp, #3040]
	str	x8, [sp, #856]
	and	x8, x8, #0x7
	cbnz	x8, LBB282_70
	b	LBB282_69
LBB282_69:
	ldr	x0, [sp, #856]
	add	x1, sp, #3048
	mov	w8, #56
	mov	x2, x8
	str	x2, [sp, #848]
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #56
	add	x1, sp, #3104
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #112
	add	x1, sp, #3192
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #168
	add	x1, sp, #3248
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #224
	add	x1, sp, #3312
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #280
	add	x1, sp, #3368
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #336
	add	x1, sp, #3432
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #392
	add	x1, sp, #3488
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #448
	add	x1, sp, #3552
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #504
	add	x1, sp, #3616
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #560
	add	x1, sp, #3672
	bl	_memcpy
	ldr	x8, [sp, #856]
	ldr	x2, [sp, #848]
	add	x0, x8, #616
	add	x1, sp, #3728
	bl	_memcpy
	ldr	x0, [sp, #3040]
Ltmp266:
	add	x8, sp, #3016
	mov	w9, #12
	mov	x1, x9
	bl	__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17he0eec73e7b4c468bE
Ltmp267:
	b	LBB282_71
LBB282_70:
	ldr	x1, [sp, #856]
	mov	w8, #8
	mov	x0, x8
	adrp	x2, l___unnamed_73@PAGE
	add	x2, x2, l___unnamed_73@PAGEOFF
	bl	__ZN4core9panicking36panic_misaligned_pointer_dereference17hdc38b89bae20a49aE
LBB282_71:
Ltmp268:
	mov	w8, #1768
	mov	x0, x8
	mov	w8, #8
	mov	x1, x8
	bl	__ZN5alloc5alloc15exchange_malloc17h941cdcdc6bb63bd7E
	str	x0, [sp, #840]
Ltmp269:
	b	LBB282_74
LBB282_72:
Ltmp456:
	add	x0, sp, #3016
	bl	__ZN4core3ptr75drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..Instruction$GT$$GT$17hb330cbe1268ffd9aE
Ltmp457:
	b	LBB282_52
LBB282_73:
Ltmp455:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_72
LBB282_74:
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #840]
	str	x8, [sp, #3808]
	mov	w8, #12
	str	x8, [sp, #4024]
	mov	w8, #2
	strb	w8, [sp, #4016]
	ldr	q0, [x9, #96]
	add	x1, sp, #3920
	str	q0, [x9]
	ldr	x8, [sp, #4032]
	str	x8, [sp, #3936]
	adrp	x8, l___unnamed_74@PAGE
	add	x8, x8, l___unnamed_74@PAGEOFF
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #3960]
	ldr	q0, [x8]
	str	q0, [x9, #48]
	ldr	x9, [x8, #16]
	str	x9, [sp, #3984]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #4008]
	add	x8, sp, #3816
	mov	w9, #5
	str	x9, [sp, #3816]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	bl	_memcpy
Ltmp270:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #824]
	str	w1, [sp, #836]
Ltmp271:
	b	LBB282_77
LBB282_75:
Ltmp289:
	add	x0, sp, #3808
	bl	__ZN4core3ptr132drop_in_place$LT$alloc..boxed..Box$LT$$u5b$$LP$usize$C$$u5b$main..instructions..Operand$u3b$$u20$4$u5d$$RP$$u3b$$u20$17$u5d$$GT$$GT$17h7971495c0f700895E
Ltmp290:
	b	LBB282_72
LBB282_76:
Ltmp288:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_75
LBB282_77:
	ldr	x8, [sp, #1248]
	ldr	w9, [sp, #836]
	ldr	x10, [sp, #824]
	str	x10, [sp, #4272]
	mov	w10, #1
	str	w10, [sp, #804]
	and	w9, w9, w10
	add	x10, sp, #185
	strb	w9, [x10, #4095]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #152
	ldr	q0, [x8, #352]
	stur	q0, [x9, #8]
	add	x9, sp, #153
	strb	wzr, [x9, #4095]
	mov	w9, #1
	str	x9, [sp, #4312]
	mov	w9, #3
	add	x10, sp, #225
	strb	w9, [x10, #4095]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #192
	add	x10, sp, #4057
	ldur	q0, [x10, #255]
	stur	q0, [x9, #8]
	add	x9, sp, #193
	strb	wzr, [x9, #4095]
	mov	w9, #2
	str	x9, [sp, #4336]
	mov	w9, #4
	add	x10, sp, #233
	strb	w9, [x10, #4095]
	mov	w10, #3
	str	x10, [sp, #4360]
	add	x10, sp, #257
	strb	w9, [x10, #4095]
	add	x9, sp, #3993
	ldur	q0, [x9, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #48
	str	q0, [x8, #224]
	ldr	x9, [sp, #4264]
	str	x9, [sp, #4160]
	ldr	q0, [x8, #368]
	stur	q0, [x1, #24]
	ldr	x9, [sp, #4304]
	str	x9, [sp, #4184]
	add	x9, sp, #4073
	ldur	q0, [x9, #255]
	str	q0, [x8, #272]
	ldr	x9, [sp, #4344]
	str	x9, [sp, #4208]
	ldr	q0, [x8, #432]
	stur	q0, [x1, #72]
	ldr	x8, [sp, #4368]
	str	x8, [sp, #4232]
	add	x8, sp, #4040
	mov	w9, #6
	str	x9, [sp, #4040]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	bl	_memcpy
	ldr	w8, [sp, #804]
Ltmp272:
	mov	x0, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #808]
	str	w1, [sp, #820]
Ltmp273:
	b	LBB282_78
LBB282_78:
	ldr	x9, [sp, #1248]
	ldr	w8, [sp, #820]
	ldr	x10, [sp, #808]
	str	x10, [sp, #4600]
	mov	w10, #1
	str	w10, [sp, #784]
	mov	w10, #1
	str	w10, [sp, #788]
	and	w8, w8, w10
	add	x10, sp, #513
	strb	w8, [x10, #4095]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #480
	add	x10, sp, #1, lsl #12
	add	x10, x10, #249
	ldur	q0, [x10, #255]
	stur	q0, [x8, #8]
	add	x8, sp, #481
	strb	wzr, [x8, #4095]
	ldr	q0, [x9, #656]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #384
	str	q0, [x9, #560]
	ldr	x8, [sp, #4592]
	str	x8, [sp, #4496]
	adrp	x8, l___unnamed_74@PAGE
	add	x8, x8, l___unnamed_74@PAGEOFF
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #4520]
	ldr	q0, [x8]
	str	q0, [x9, #608]
	ldr	x9, [x8, #16]
	str	x9, [sp, #4544]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #4568]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #280
	mov	w9, #8
	str	x9, [sp, #4376]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	bl	_memcpy
	ldr	w9, [sp, #784]
	ldr	w8, [sp, #788]
	add	x10, sp, #760
	strb	wzr, [x10, #4095]
	add	x10, sp, #760
	ldrb	w10, [x10, #4095]
	add	x11, sp, #730
	strb	w10, [x11, #4095]
	add	x10, sp, #729
	strb	w9, [x10, #4095]
Ltmp274:
	mov	x0, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #792]
	str	w1, [sp, #800]
Ltmp275:
	b	LBB282_79
LBB282_79:
	ldr	x8, [sp, #1248]
	ldr	w9, [sp, #800]
	ldr	x10, [sp, #792]
	str	x10, [sp, #4880]
	mov	w10, #1
	and	w9, w9, w10
	add	x10, sp, #793
	strb	w9, [x10, #4095]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #760
	ldr	q0, [x8, #960]
	stur	q0, [x9, #8]
	add	x9, sp, #761
	strb	wzr, [x9, #4095]
	mov	w9, #1
	str	x9, [sp, #4920]
	mov	w9, #3
	str	w9, [sp, #748]
	add	x10, sp, #833
	strb	w9, [x10, #4095]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #800
	add	x10, sp, #1, lsl #12
	add	x10, x10, #569
	ldur	q0, [x10, #255]
	stur	q0, [x9, #8]
	add	x9, sp, #801
	strb	wzr, [x9, #4095]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #473
	ldur	q0, [x9, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #624
	str	q0, [x8, #800]
	ldr	x9, [sp, #4840]
	str	x9, [sp, #4736]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #505
	ldur	q0, [x9, #255]
	stur	q0, [x1, #24]
	ldr	x9, [sp, #4872]
	str	x9, [sp, #4760]
	ldr	q0, [x8, #976]
	str	q0, [x8, #848]
	ldr	x8, [sp, #4912]
	str	x8, [sp, #4784]
	adrp	x8, l___unnamed_74@PAGE
	add	x8, x8, l___unnamed_74@PAGEOFF
	str	x8, [sp, #752]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #4808]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #520
	mov	w9, #2
	str	x9, [sp, #4616]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	str	x2, [sp, #760]
	bl	_memcpy
	ldr	w10, [sp, #748]
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #752]
	ldr	x2, [sp, #760]
	mov	w11, #7
	str	x11, [sp, #5144]
	add	x11, sp, #1041
	strb	w10, [x11, #4095]
	ldr	q0, [x9, #1216]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #944
	str	q0, [x9, #1120]
	ldr	x10, [sp, #5152]
	str	x10, [sp, #5056]
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #5080]
	ldr	q0, [x8]
	str	q0, [x9, #1168]
	ldr	x9, [x8, #16]
	str	x9, [sp, #5104]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #5128]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #840
	mov	w9, #10
	str	x9, [sp, #4936]
	add	x0, x8, #8
	bl	_memcpy
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #752]
	ldr	x2, [sp, #760]
	strb	wzr, [x9, #1495]
	ldrb	w10, [x9, #1495]
	strb	w10, [x9, #1472]
	mov	w10, #2
	strb	w10, [x9, #1480]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #1272
	ldr	q0, [x9, #1472]
	stur	q0, [x10, #8]
	strb	wzr, [x9, #1448]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #1017
	ldur	q0, [x10, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #1168
	str	q0, [x9, #1344]
	ldr	x10, [sp, #5384]
	str	x10, [sp, #5280]
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #5304]
	ldr	q0, [x8]
	str	q0, [x9, #1392]
	ldr	x9, [x8, #16]
	str	x9, [sp, #5328]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #5352]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #1064
	mov	w9, #9
	str	x9, [sp, #5160]
	add	x0, x8, #8
	bl	_memcpy
Ltmp276:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #768]
	str	w1, [sp, #780]
Ltmp277:
	b	LBB282_80
LBB282_80:
	ldr	x9, [sp, #1248]
	ldr	w8, [sp, #780]
	ldr	x10, [sp, #768]
	str	x10, [sp, #5632]
	mov	w10, #1
	str	w10, [sp, #728]
	mov	w10, #1
	str	w10, [sp, #732]
	and	w8, w8, w10
	strb	w8, [x9, #1720]
	mov	w8, #5
	strb	w8, [x9, #1704]
	mov	w8, #1
	str	x8, [sp, #5656]
	mov	w8, #6
	strb	w8, [x9, #1728]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #1273
	ldur	q0, [x8, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #1424
	str	q0, [x9, #1600]
	ldr	x8, [sp, #5640]
	str	x8, [sp, #5536]
	ldr	q0, [x9, #1728]
	stur	q0, [x1, #24]
	ldr	x8, [sp, #5664]
	str	x8, [sp, #5560]
	adrp	x8, l___unnamed_74@PAGE
	add	x8, x8, l___unnamed_74@PAGEOFF
	ldr	q0, [x8]
	str	q0, [x9, #1648]
	ldr	x9, [x8, #16]
	str	x9, [sp, #5584]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #5608]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #1320
	mov	w9, #7
	str	x9, [sp, #5416]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	bl	_memcpy
	ldr	x10, [sp, #1248]
	ldr	w9, [sp, #728]
	ldr	w8, [sp, #732]
	strb	w9, [x10, #1991]
	ldrb	w11, [x10, #1991]
	strb	w11, [x10, #1961]
	strb	w9, [x10, #1960]
Ltmp278:
	mov	x0, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #736]
	str	w1, [sp, #744]
Ltmp279:
	b	LBB282_81
LBB282_81:
	ldr	x8, [sp, #1248]
	ldr	w9, [sp, #744]
	ldr	x10, [sp, #736]
	str	x10, [sp, #5936]
	mov	w10, #1
	str	w10, [sp, #692]
	mov	w10, #1
	and	w9, w9, w10
	strb	w9, [x8, #2024]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #1816
	ldr	q0, [x8, #2016]
	stur	q0, [x9, #8]
	strb	wzr, [x8, #1992]
	mov	w9, #2
	str	x9, [sp, #5976]
	mov	w10, #3
	str	w10, [sp, #688]
	strb	w10, [x8, #2064]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #1856
	add	x11, sp, #1, lsl #12
	add	x11, x11, #1625
	ldur	q0, [x11, #255]
	stur	q0, [x10, #8]
	strb	wzr, [x8, #2032]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #1529
	ldur	q0, [x10, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #1680
	str	q0, [x8, #1856]
	ldr	x10, [sp, #5896]
	str	x10, [sp, #5792]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #1561
	ldur	q0, [x10, #255]
	stur	q0, [x1, #24]
	ldr	x10, [sp, #5928]
	str	x10, [sp, #5816]
	ldr	q0, [x8, #2032]
	str	q0, [x8, #1904]
	ldr	x8, [sp, #5968]
	str	x8, [sp, #5840]
	adrp	x8, l___unnamed_74@PAGE
	add	x8, x8, l___unnamed_74@PAGEOFF
	str	x8, [sp, #696]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #5864]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #1576
	str	x9, [sp, #5672]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	str	x2, [sp, #704]
	bl	_memcpy
	ldr	w10, [sp, #688]
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #696]
	ldr	x2, [sp, #704]
	mov	w11, #11
	str	x11, [sp, #6200]
	strb	w10, [x9, #2272]
	ldr	q0, [x9, #2272]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #2000
	str	q0, [x9, #2176]
	ldr	x10, [sp, #6208]
	str	x10, [sp, #6112]
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #6136]
	ldr	q0, [x8]
	str	q0, [x9, #2224]
	ldr	x9, [x8, #16]
	str	x9, [sp, #6160]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #6184]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #1896
	mov	w9, #10
	str	x9, [sp, #5992]
	add	x0, x8, #8
	bl	_memcpy
	ldr	w10, [sp, #692]
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #696]
	ldr	x2, [sp, #704]
	strb	w10, [x9, #2551]
	ldrb	w10, [x9, #2551]
	strb	w10, [x9, #2528]
	mov	w10, #2
	strb	w10, [x9, #2536]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #2328
	ldr	q0, [x9, #2528]
	stur	q0, [x10, #8]
	strb	wzr, [x9, #2504]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #2073
	ldur	q0, [x10, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #2224
	str	q0, [x9, #2400]
	ldr	x10, [sp, #6440]
	str	x10, [sp, #6336]
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #6360]
	ldr	q0, [x8]
	str	q0, [x9, #2448]
	ldr	x9, [x8, #16]
	str	x9, [sp, #6384]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #6408]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #2120
	mov	w9, #9
	str	x9, [sp, #6216]
	add	x0, x8, #8
	bl	_memcpy
Ltmp280:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #712]
	str	w1, [sp, #724]
Ltmp281:
	b	LBB282_82
LBB282_82:
	ldr	x9, [sp, #1248]
	ldr	w8, [sp, #724]
	ldr	x10, [sp, #712]
	str	x10, [sp, #6688]
	mov	w10, #1
	str	w10, [sp, #664]
	mov	w10, #1
	str	w10, [sp, #668]
	and	w8, w8, w10
	strb	w8, [x9, #2776]
	mov	w8, #5
	strb	w8, [x9, #2760]
	mov	w8, #1
	str	x8, [sp, #656]
	str	x8, [sp, #6712]
	mov	w8, #6
	strb	w8, [x9, #2784]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #2329
	ldur	q0, [x8, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #2480
	str	q0, [x9, #2656]
	ldr	x8, [sp, #6696]
	str	x8, [sp, #6592]
	ldr	q0, [x9, #2784]
	stur	q0, [x1, #24]
	ldr	x8, [sp, #6720]
	str	x8, [sp, #6616]
	adrp	x8, l___unnamed_74@PAGE
	add	x8, x8, l___unnamed_74@PAGEOFF
	ldr	q0, [x8]
	str	q0, [x9, #2704]
	ldr	x9, [x8, #16]
	str	x9, [sp, #6640]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #6664]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #2376
	mov	w9, #7
	str	x9, [sp, #6472]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	bl	_memcpy
	ldr	x0, [sp, #656]
	ldr	x10, [sp, #1248]
	ldr	w9, [sp, #664]
	ldr	w8, [sp, #668]
	strb	wzr, [x10, #3047]
	ldrb	w11, [x10, #3047]
	strb	w11, [x10, #3017]
	strb	w9, [x10, #3016]
Ltmp282:
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #672]
	str	w1, [sp, #684]
Ltmp283:
	b	LBB282_83
LBB282_83:
	ldr	x9, [sp, #1248]
	ldr	w10, [sp, #684]
	ldr	x8, [sp, #672]
	str	x8, [sp, #6992]
	mov	w8, #1
	and	w10, w10, w8
	strb	w10, [x9, #3080]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #2872
	ldr	q0, [x9, #3072]
	stur	q0, [x10, #8]
	strb	wzr, [x9, #3048]
Ltmp284:
	mov	w9, #2
	mov	x0, x9
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #640]
	str	w1, [sp, #652]
Ltmp285:
	b	LBB282_84
LBB282_84:
	ldr	x8, [sp, #1248]
	ldr	w9, [sp, #652]
	ldr	x10, [sp, #640]
	str	x10, [sp, #7032]
	mov	w10, #1
	and	w9, w9, w10
	strb	w9, [x8, #3120]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #2912
	add	x10, sp, #1, lsl #12
	add	x10, x10, #2681
	ldur	q0, [x10, #255]
	stur	q0, [x9, #8]
	strb	wzr, [x8, #3088]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #2585
	ldur	q0, [x9, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #2736
	str	q0, [x8, #2912]
	ldr	x9, [sp, #6952]
	str	x9, [sp, #6848]
	add	x9, sp, #1, lsl #12
	add	x9, x9, #2617
	ldur	q0, [x9, #255]
	stur	q0, [x1, #24]
	ldr	x9, [sp, #6984]
	str	x9, [sp, #6872]
	ldr	q0, [x8, #3088]
	str	q0, [x8, #2960]
	ldr	x8, [sp, #7024]
	str	x8, [sp, #6896]
	adrp	x8, l___unnamed_74@PAGE
	add	x8, x8, l___unnamed_74@PAGEOFF
	str	x8, [sp, #600]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #6920]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #2632
	mov	w9, #1
	str	x9, [sp, #6728]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	str	x2, [sp, #608]
	bl	_memcpy
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #600]
	ldr	x2, [sp, #608]
	strb	wzr, [x9, #3383]
	ldrb	w10, [x9, #3383]
	strb	w10, [x9, #3360]
	mov	w10, #2
	strb	w10, [x9, #3368]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #3160
	ldr	q0, [x9, #3360]
	stur	q0, [x10, #8]
	strb	wzr, [x9, #3336]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #2905
	ldur	q0, [x10, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #3056
	str	q0, [x9, #3232]
	ldr	x10, [sp, #7272]
	str	x10, [sp, #7168]
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #7192]
	ldr	q0, [x8]
	str	q0, [x9, #3280]
	ldr	x9, [x8, #16]
	str	x9, [sp, #7216]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #7240]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #2952
	mov	w9, #8
	str	x9, [sp, #7048]
	add	x0, x8, #8
	bl	_memcpy
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #600]
	ldr	x2, [sp, #608]
	mov	w10, #16
	str	x10, [sp, #7512]
	mov	w10, #3
	str	w10, [sp, #596]
	strb	w10, [x9, #3584]
	ldr	q0, [x9, #3584]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #3312
	str	q0, [x9, #3488]
	ldr	x10, [sp, #7520]
	str	x10, [sp, #7424]
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #7448]
	ldr	q0, [x8]
	str	q0, [x9, #3536]
	ldr	x9, [x8, #16]
	str	x9, [sp, #7472]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #7496]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #3208
	mov	w9, #10
	str	x9, [sp, #7304]
	add	x0, x8, #8
	bl	_memcpy
	ldr	w10, [sp, #596]
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #600]
	ldr	x2, [sp, #608]
	mov	x11, #0
	str	x11, [sp, #616]
	str	xzr, [sp, #7752]
	strb	w10, [x9, #3840]
	add	x10, sp, #1, lsl #12
	add	x10, x10, #3632
	add	x11, sp, #1, lsl #12
	add	x11, x11, #3401
	ldur	q0, [x11, #255]
	stur	q0, [x10, #8]
	strb	wzr, [x9, #3808]
	ldr	q0, [x9, #3808]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #3536
	str	q0, [x9, #3712]
	ldr	x10, [sp, #7744]
	str	x10, [sp, #7648]
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #7672]
	ldr	q0, [x8]
	str	q0, [x9, #3760]
	ldr	x9, [x8, #16]
	str	x9, [sp, #7696]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #7720]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #3432
	mov	w9, #9
	str	x9, [sp, #7528]
	add	x0, x8, #8
	bl	_memcpy
	ldr	x0, [sp, #616]
Ltmp286:
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #624]
	str	w1, [sp, #636]
Ltmp287:
	b	LBB282_85
LBB282_85:
	ldr	x9, [sp, #1248]
	ldr	w8, [sp, #636]
	ldr	x10, [sp, #624]
	str	x10, [sp, #7984]
	and	w8, w8, #0x1
	strb	w8, [x9, #4072]
	mov	w8, #5
	strb	w8, [x9, #4056]
	mov	w8, #1
	str	x8, [sp, #8008]
	mov	w8, #6
	strb	w8, [x9, #4080]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #3625
	ldur	q0, [x8, #255]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #3776
	str	q0, [x9, #3952]
	ldr	x8, [sp, #7992]
	str	x8, [sp, #7888]
	ldr	q0, [x9, #4080]
	stur	q0, [x1, #24]
	ldr	x8, [sp, #8016]
	str	x8, [sp, #7912]
	adrp	x8, l___unnamed_74@PAGE
	add	x8, x8, l___unnamed_74@PAGEOFF
	str	x8, [sp, #568]
	ldr	q0, [x8]
	str	q0, [x9, #4000]
	ldr	x9, [x8, #16]
	str	x9, [sp, #7936]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #7960]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #3672
	mov	w9, #7
	str	x9, [sp, #7768]
	add	x0, x8, #8
	mov	w8, #96
	mov	x2, x8
	str	x2, [sp, #576]
	bl	_memcpy
	ldr	x9, [sp, #1248]
	ldr	x8, [sp, #568]
	ldr	x2, [sp, #576]
	ldr	q0, [x8]
	add	x1, sp, #1, lsl #12
	add	x1, x1, #4032
	str	q0, [x9, #4208]
	ldr	x10, [x8, #16]
	str	x10, [sp, #8144]
	ldr	q0, [x8]
	stur	q0, [x1, #24]
	ldr	x10, [x8, #16]
	str	x10, [sp, #8168]
	ldr	q0, [x8]
	str	q0, [x9, #4256]
	ldr	x9, [x8, #16]
	str	x9, [sp, #8192]
	ldr	q0, [x8]
	stur	q0, [x1, #72]
	ldr	x8, [x8, #16]
	str	x8, [sp, #8216]
	add	x8, sp, #1, lsl #12
	add	x8, x8, #3928
	str	xzr, [sp, #8024]
	add	x0, x8, #8
	bl	_memcpy
	ldr	x8, [sp, #3808]
	str	x8, [sp, #584]
	and	x8, x8, #0x7
	cbnz	x8, LBB282_87
	b	LBB282_86
LBB282_86:
	ldr	x0, [sp, #584]
	add	x1, sp, #3816
	mov	w8, #104
	mov	x2, x8
	str	x2, [sp, #560]
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #104
	add	x1, sp, #4040
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #208
	add	x1, sp, #1, lsl #12
	add	x1, x1, #280
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #312
	add	x1, sp, #1, lsl #12
	add	x1, x1, #520
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #416
	add	x1, sp, #1, lsl #12
	add	x1, x1, #840
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #520
	add	x1, sp, #1, lsl #12
	add	x1, x1, #1064
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #624
	add	x1, sp, #1, lsl #12
	add	x1, x1, #1320
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #728
	add	x1, sp, #1, lsl #12
	add	x1, x1, #1576
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #832
	add	x1, sp, #1, lsl #12
	add	x1, x1, #1896
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #936
	add	x1, sp, #1, lsl #12
	add	x1, x1, #2120
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #1040
	add	x1, sp, #1, lsl #12
	add	x1, x1, #2376
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #1144
	add	x1, sp, #1, lsl #12
	add	x1, x1, #2632
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #1248
	add	x1, sp, #1, lsl #12
	add	x1, x1, #2952
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #1352
	add	x1, sp, #1, lsl #12
	add	x1, x1, #3208
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #1456
	add	x1, sp, #1, lsl #12
	add	x1, x1, #3432
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #1560
	add	x1, sp, #1, lsl #12
	add	x1, x1, #3672
	bl	_memcpy
	ldr	x8, [sp, #584]
	ldr	x2, [sp, #560]
	add	x0, x8, #1664
	add	x1, sp, #1, lsl #12
	add	x1, x1, #3928
	bl	_memcpy
	ldr	x0, [sp, #3808]
Ltmp291:
	add	x8, sp, #3784
	mov	w9, #17
	mov	x1, x9
	bl	__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17h7be03de987d6fb8bE
Ltmp292:
	b	LBB282_88
LBB282_87:
	ldr	x1, [sp, #584]
	mov	w8, #8
	mov	x0, x8
	adrp	x2, l___unnamed_75@PAGE
	add	x2, x2, l___unnamed_75@PAGEOFF
	bl	__ZN4core9panicking36panic_misaligned_pointer_dereference17hdc38b89bae20a49aE
LBB282_88:
Ltmp293:
	add	x0, sp, #3784
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h8afc42426ac1b360E
Ltmp294:
	b	LBB282_91
LBB282_89:
Ltmp451:
	add	x0, sp, #3784
	bl	__ZN4core3ptr108drop_in_place$LT$alloc..vec..Vec$LT$$LP$usize$C$$u5b$main..instructions..Operand$u3b$$u20$4$u5d$$RP$$GT$$GT$17h4b2975b76c04d018E
Ltmp452:
	b	LBB282_72
LBB282_90:
Ltmp450:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_89
LBB282_91:
Ltmp295:
	mov	w8, #816
	mov	x0, x8
	mov	w8, #8
	mov	x1, x8
	bl	__ZN5alloc5alloc15exchange_malloc17h941cdcdc6bb63bd7E
	str	x0, [sp, #552]
Ltmp296:
	b	LBB282_92
LBB282_92:
	ldr	x8, [sp, #552]
	str	x8, [sp, #8248]
Ltmp297:
	mov	w8, #12
	mov	x0, x8
	bl	__ZN4main12instructions23PushFunctionInstruction3new17h931a333ebe77dcd9E
	str	x0, [sp, #544]
Ltmp298:
	b	LBB282_95
LBB282_93:
Ltmp350:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #56
	bl	__ZN4core3ptr100drop_in_place$LT$alloc..boxed..Box$LT$$u5b$main..instructions..Instruction2$u3b$$u20$17$u5d$$GT$$GT$17hcc250ad903e8ab72E
Ltmp351:
	b	LBB282_89
LBB282_94:
Ltmp349:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_93
LBB282_95:
	ldr	x9, [sp, #1240]
	ldr	x8, [sp, #544]
	str	x8, [sp, #8256]
	mov	w8, #9
	strb	w8, [x9, #40]
Ltmp299:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #528]
	str	w1, [sp, #540]
Ltmp300:
	b	LBB282_96
LBB282_96:
	ldr	x9, [sp, #1240]
	ldr	w8, [sp, #540]
	ldr	x10, [sp, #528]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #208
	str	x10, [sp, #8400]
	and	w8, w8, #0x1
	strb	w8, [x9, #152]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #224
	mov	w8, #1
	str	x8, [sp, #8416]
	mov	w8, #3
	strb	w8, [x9, #168]
Ltmp301:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #160
	mov	w9, #2
	mov	x2, x9
	mov	w9, #3
	mov	x3, x9
	bl	__ZN4main12instructions19CmpLtJmpInstruction3new17h9fe81541418d7fd0E
Ltmp302:
	b	LBB282_97
LBB282_97:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #112
	add	x1, sp, #2, lsl #12
	add	x1, x1, #160
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp303:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #512]
	str	w1, [sp, #524]
Ltmp304:
	b	LBB282_98
LBB282_98:
	ldr	x9, [sp, #1240]
	ldr	w8, [sp, #524]
	ldr	x10, [sp, #512]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #304
	str	x10, [sp, #8496]
	and	w8, w8, #0x1
	strb	w8, [x9, #248]
Ltmp305:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #288
	bl	__ZN4main12instructions14RetInstruction3new17hba68c35196258340E
Ltmp306:
	b	LBB282_99
LBB282_99:
	ldr	x8, [sp, #1240]
	ldr	q0, [x8, #224]
	str	q0, [x8, #176]
	mov	w9, #12
	strb	w9, [x8, #216]
	strb	wzr, [x8, #351]
Ltmp307:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #496]
	str	w1, [sp, #508]
Ltmp308:
	b	LBB282_100
LBB282_100:
	ldr	x8, [sp, #1240]
	ldr	w9, [sp, #508]
	ldr	x10, [sp, #496]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #416
	str	x10, [sp, #8608]
	and	w9, w9, #0x1
	strb	w9, [x8, #360]
	add	x2, sp, #2, lsl #12
	add	x2, x2, #432
	mov	w9, #1
	str	x9, [sp, #8624]
	mov	w9, #3
	strb	w9, [x8, #376]
	ldrb	w0, [x8, #351]
Ltmp309:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #368
	bl	__ZN4main12instructions14SubInstruction3new17ha4ff55e7ef410ce5E
Ltmp310:
	b	LBB282_101
LBB282_101:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #320
	add	x1, sp, #2, lsl #12
	add	x1, x1, #368
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	mov	w8, #6
	strb	w8, [x9, #296]
Ltmp311:
	mov	w8, #7
	mov	x0, x8
	bl	__ZN4main12instructions23PushInstrPtrInstruction3new17h4d74fd4ece850b62E
	str	x0, [sp, #488]
Ltmp312:
	b	LBB282_102
LBB282_102:
	ldr	x9, [sp, #1240]
	ldr	x8, [sp, #488]
	str	x8, [sp, #8640]
	mov	w8, #14
	strb	w8, [x9, #424]
	strb	wzr, [x9, #527]
	ldrb	w8, [x9, #527]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #568
	strb	w8, [x9, #504]
	mov	w8, #2
	strb	w8, [x9, #512]
Ltmp313:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #552
	bl	__ZN4main12instructions20PushStackInstruction3new17hfe7e45e50564c2e7E
Ltmp314:
	b	LBB282_103
LBB282_103:
	ldr	x9, [sp, #1240]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #297
	ldur	q0, [x8, #255]
	str	q0, [x9, #432]
	mov	w8, #13
	strb	w8, [x9, #472]
Ltmp315:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #472]
	str	w1, [sp, #484]
Ltmp316:
	b	LBB282_104
LBB282_104:
Ltmp317:
	ldr	w9, [sp, #484]
	ldr	x0, [sp, #472]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #648
	and	w1, w9, #0x1
	mov	w9, #1
	mov	x2, x9
	bl	__ZN4main12instructions15CallInstruction3new17h81b5345dc87d0d3aE
Ltmp318:
	b	LBB282_105
LBB282_105:
	ldr	x9, [sp, #1240]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #393
	ldur	q0, [x8, #255]
	str	q0, [x9, #528]
	ldr	x8, [sp, #8856]
	str	x8, [sp, #8800]
	mov	w8, #11
	strb	w8, [x9, #568]
	mov	w8, #1
	strb	w8, [x9, #703]
Ltmp319:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #456]
	str	w1, [sp, #468]
Ltmp320:
	b	LBB282_106
LBB282_106:
	ldr	x8, [sp, #1240]
	ldr	w9, [sp, #468]
	ldr	x10, [sp, #456]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #768
	str	x10, [sp, #8960]
	and	w9, w9, #0x1
	strb	w9, [x8, #712]
	add	x2, sp, #2, lsl #12
	add	x2, x2, #784
	mov	w9, #2
	str	x9, [sp, #8976]
	mov	w9, #3
	strb	w9, [x8, #728]
	ldrb	w0, [x8, #703]
Ltmp321:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #720
	bl	__ZN4main12instructions14SubInstruction3new17ha4ff55e7ef410ce5E
Ltmp322:
	b	LBB282_107
LBB282_107:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #672
	add	x1, sp, #2, lsl #12
	add	x1, x1, #720
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	mov	w8, #6
	strb	w8, [x9, #648]
Ltmp323:
	mov	w8, #11
	mov	x0, x8
	bl	__ZN4main12instructions23PushInstrPtrInstruction3new17h4d74fd4ece850b62E
	str	x0, [sp, #448]
Ltmp324:
	b	LBB282_108
LBB282_108:
	ldr	x9, [sp, #1240]
	ldr	x8, [sp, #448]
	str	x8, [sp, #8992]
	mov	w8, #14
	strb	w8, [x9, #776]
	mov	w8, #1
	strb	w8, [x9, #879]
	ldrb	w8, [x9, #879]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #920
	strb	w8, [x9, #856]
	mov	w8, #2
	strb	w8, [x9, #864]
Ltmp325:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #904
	bl	__ZN4main12instructions20PushStackInstruction3new17hfe7e45e50564c2e7E
Ltmp326:
	b	LBB282_109
LBB282_109:
	ldr	x9, [sp, #1240]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #649
	ldur	q0, [x8, #255]
	str	q0, [x9, #784]
	mov	w8, #13
	strb	w8, [x9, #824]
Ltmp327:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #432]
	str	w1, [sp, #444]
Ltmp328:
	b	LBB282_110
LBB282_110:
Ltmp329:
	ldr	w9, [sp, #444]
	ldr	x0, [sp, #432]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1000
	and	w1, w9, #0x1
	mov	w9, #1
	mov	x2, x9
	bl	__ZN4main12instructions15CallInstruction3new17h81b5345dc87d0d3aE
Ltmp330:
	b	LBB282_111
LBB282_111:
	ldr	x8, [sp, #1240]
	add	x9, sp, #2, lsl #12
	add	x9, x9, #745
	ldur	q0, [x9, #255]
	str	q0, [x8, #880]
	ldr	x9, [sp, #9208]
	str	x9, [sp, #9152]
	mov	w9, #11
	strb	w9, [x8, #920]
	strb	wzr, [x8, #1055]
Ltmp331:
	mov	w8, #1
	mov	x0, x8
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #416]
	str	w1, [sp, #428]
Ltmp332:
	b	LBB282_112
LBB282_112:
	ldr	x10, [sp, #1240]
	ldr	w9, [sp, #428]
	ldr	x8, [sp, #416]
	str	x8, [sp, #9312]
	mov	w8, #1
	and	w9, w9, w8
	strb	w9, [x10, #1064]
Ltmp333:
	mov	w9, #2
	mov	x0, x9
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #400]
	str	w1, [sp, #412]
Ltmp334:
	b	LBB282_113
LBB282_113:
	ldr	x8, [sp, #1240]
	ldr	w9, [sp, #412]
	ldr	x10, [sp, #400]
	add	x2, sp, #2, lsl #12
	add	x2, x2, #1136
	str	x10, [sp, #9328]
	and	w9, w9, #0x1
	strb	w9, [x8, #1080]
	ldrb	w0, [x8, #1055]
Ltmp335:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1072
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1120
	bl	__ZN4main12instructions14AddInstruction3new17h1fa494a4c80f5d48E
Ltmp336:
	b	LBB282_114
LBB282_114:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1024
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1072
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	mov	w8, #5
	strb	w8, [x9, #1000]
	strb	wzr, [x9, #1183]
	ldrb	w8, [x9, #1183]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1224
	strb	w8, [x9, #1160]
	mov	w8, #2
	strb	w8, [x9, #1168]
Ltmp337:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1208
	bl	__ZN4main12instructions14RetInstruction3new17hba68c35196258340E
Ltmp338:
	b	LBB282_115
LBB282_115:
	ldr	x9, [sp, #1240]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #953
	ldur	q0, [x8, #255]
	str	q0, [x9, #1088]
	mov	w8, #12
	strb	w8, [x9, #1128]
Ltmp339:
	mov	w8, #16
	mov	x0, x8
	bl	__ZN4main12instructions23PushInstrPtrInstruction3new17h4d74fd4ece850b62E
	str	x0, [sp, #392]
Ltmp340:
	b	LBB282_116
LBB282_116:
	ldr	x9, [sp, #1240]
	ldr	x8, [sp, #392]
	str	x8, [sp, #9440]
	mov	w8, #14
	strb	w8, [x9, #1224]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1360
	str	xzr, [sp, #9552]
	mov	w8, #3
	strb	w8, [x9, #1304]
Ltmp341:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1344
	bl	__ZN4main12instructions20PushStackInstruction3new17hfe7e45e50564c2e7E
Ltmp342:
	b	LBB282_117
LBB282_117:
	ldr	x9, [sp, #1240]
	ldr	q0, [x9, #1280]
	str	q0, [x9, #1232]
	mov	w8, #13
	strb	w8, [x9, #1272]
Ltmp343:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #376]
	str	w1, [sp, #388]
Ltmp344:
	b	LBB282_118
LBB282_118:
Ltmp345:
	ldr	w9, [sp, #388]
	ldr	x0, [sp, #376]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1424
	and	w1, w9, #0x1
	mov	w9, #1
	mov	x2, x9
	bl	__ZN4main12instructions15CallInstruction3new17h81b5345dc87d0d3aE
Ltmp346:
	b	LBB282_119
LBB282_119:
	ldr	x9, [sp, #1240]
	ldr	q0, [x9, #1360]
	str	q0, [x9, #1312]
	ldr	x8, [sp, #9632]
	str	x8, [sp, #9584]
	mov	w8, #11
	strb	w8, [x9, #1352]
Ltmp347:
	bl	__ZN4main12instructions15HaltInstruction3new17hc69fe866ca78bfb8E
Ltmp348:
	b	LBB282_120
LBB282_120:
	ldr	x9, [sp, #1240]
	mov	w8, #4
	strb	w8, [x9, #1424]
	ldr	x8, [sp, #8248]
	str	x8, [sp, #368]
	and	x8, x8, #0x7
	cbnz	x8, LBB282_122
	b	LBB282_121
LBB282_121:
	ldr	x0, [sp, #368]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #64
	mov	w8, #48
	mov	x2, x8
	str	x2, [sp, #360]
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #48
	add	x1, sp, #2, lsl #12
	add	x1, x1, #112
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #96
	add	x1, sp, #2, lsl #12
	add	x1, x1, #240
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #144
	add	x1, sp, #2, lsl #12
	add	x1, x1, #320
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #192
	add	x1, sp, #2, lsl #12
	add	x1, x1, #448
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #240
	add	x1, sp, #2, lsl #12
	add	x1, x1, #496
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #288
	add	x1, sp, #2, lsl #12
	add	x1, x1, #592
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #336
	add	x1, sp, #2, lsl #12
	add	x1, x1, #672
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #384
	add	x1, sp, #2, lsl #12
	add	x1, x1, #800
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #432
	add	x1, sp, #2, lsl #12
	add	x1, x1, #848
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #480
	add	x1, sp, #2, lsl #12
	add	x1, x1, #944
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #528
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1024
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #576
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1152
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #624
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1248
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #672
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1296
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #720
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1376
	bl	_memcpy
	ldr	x8, [sp, #368]
	ldr	x2, [sp, #360]
	add	x0, x8, #768
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1448
	bl	_memcpy
	ldr	x0, [sp, #8248]
Ltmp352:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #32
	mov	w9, #17
	mov	x1, x9
	bl	__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17h864456c55cce862dE
Ltmp353:
	b	LBB282_123
LBB282_122:
	ldr	x1, [sp, #368]
	mov	w8, #8
	mov	x0, x8
	adrp	x2, l___unnamed_76@PAGE
	add	x2, x2, l___unnamed_76@PAGEOFF
	bl	__ZN4core9panicking36panic_misaligned_pointer_dereference17hdc38b89bae20a49aE
LBB282_123:
Ltmp354:
	mov	w8, #952
	mov	x0, x8
	mov	w8, #8
	mov	x1, x8
	bl	__ZN5alloc5alloc15exchange_malloc17h941cdcdc6bb63bd7E
	str	x0, [sp, #352]
Ltmp355:
	b	LBB282_126
LBB282_124:
Ltmp446:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #32
	bl	__ZN4core3ptr76drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..Instruction2$GT$$GT$17heda17ac1179fa6a0E
Ltmp447:
	b	LBB282_89
LBB282_125:
Ltmp445:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_124
LBB282_126:
	ldr	x8, [sp, #352]
	str	x8, [sp, #9712]
Ltmp356:
	mov	w8, #12
	mov	x0, x8
	bl	__ZN4main12instructions23PushFunctionInstruction3new17h931a333ebe77dcd9E
	str	x0, [sp, #344]
Ltmp357:
	b	LBB282_129
LBB282_127:
Ltmp409:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1520
	bl	__ZN4core3ptr116drop_in_place$LT$alloc..boxed..Box$LT$$u5b$$LP$usize$C$main..instructions..Instruction2$RP$$u3b$$u20$17$u5d$$GT$$GT$17h7169344e8e3a6389E
Ltmp410:
	b	LBB282_124
LBB282_128:
Ltmp408:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_127
LBB282_129:
	ldr	x9, [sp, #1240]
	ldr	x8, [sp, #344]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1584
	str	x8, [sp, #9776]
	mov	w8, #9
	strb	w8, [x9, #1560]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1528
	mov	w9, #5
	str	x9, [sp, #9720]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp358:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #328]
	str	w1, [sp, #340]
Ltmp359:
	b	LBB282_130
LBB282_130:
	ldr	x9, [sp, #1240]
	ldr	w8, [sp, #340]
	ldr	x10, [sp, #328]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1784
	str	x10, [sp, #9976]
	and	w8, w8, #0x1
	strb	w8, [x9, #1728]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1800
	mov	w8, #1
	str	x8, [sp, #9992]
	mov	w8, #3
	strb	w8, [x9, #1744]
Ltmp360:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1736
	mov	w9, #2
	mov	x2, x9
	mov	w9, #3
	mov	x3, x9
	bl	__ZN4main12instructions19CmpLtJmpInstruction3new17h9fe81541418d7fd0E
Ltmp361:
	b	LBB282_131
LBB282_131:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1688
	str	x0, [sp, #296]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1736
	mov	w8, #48
	mov	x2, x8
	str	x2, [sp, #304]
	bl	_memcpy
	ldr	x1, [sp, #296]
	ldr	x2, [sp, #304]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1632
	mov	w9, #6
	str	x9, [sp, #9824]
	add	x0, x8, #8
	bl	_memcpy
Ltmp362:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #312]
	str	w1, [sp, #324]
Ltmp363:
	b	LBB282_132
LBB282_132:
	ldr	x9, [sp, #1240]
	ldr	w8, [sp, #324]
	ldr	x10, [sp, #312]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1944
	str	x10, [sp, #10136]
	and	w8, w8, #0x1
	strb	w8, [x9, #1888]
Ltmp364:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1928
	bl	__ZN4main12instructions14RetInstruction3new17hba68c35196258340E
Ltmp365:
	b	LBB282_133
LBB282_133:
	ldr	x9, [sp, #1240]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1673
	ldur	q0, [x8, #255]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1872
	str	q0, [x9, #1808]
	mov	w8, #12
	strb	w8, [x9, #1848]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1816
	mov	w9, #8
	str	x9, [sp, #10008]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x8, [sp, #1240]
	strb	wzr, [x8, #2047]
Ltmp366:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #280]
	str	w1, [sp, #292]
Ltmp367:
	b	LBB282_134
LBB282_134:
	ldr	x8, [sp, #1240]
	ldr	w9, [sp, #292]
	ldr	x10, [sp, #280]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2112
	str	x10, [sp, #10304]
	and	w9, w9, #0x1
	strb	w9, [x8, #2056]
	add	x2, sp, #2, lsl #12
	add	x2, x2, #2128
	mov	w9, #1
	str	x9, [sp, #10320]
	mov	w9, #3
	strb	w9, [x8, #2072]
	ldrb	w0, [x8, #2047]
Ltmp368:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2064
	bl	__ZN4main12instructions14SubInstruction3new17ha4ff55e7ef410ce5E
Ltmp369:
	b	LBB282_135
LBB282_135:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #2016
	str	x0, [sp, #264]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2064
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	ldr	x1, [sp, #264]
	mov	w8, #6
	strb	w8, [x9, #1992]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1960
	mov	w9, #2
	str	x9, [sp, #10152]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp370:
	mov	w8, #7
	mov	x0, x8
	bl	__ZN4main12instructions23PushInstrPtrInstruction3new17h4d74fd4ece850b62E
	str	x0, [sp, #272]
Ltmp371:
	b	LBB282_136
LBB282_136:
	ldr	x9, [sp, #1240]
	ldr	x8, [sp, #272]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2200
	str	x8, [sp, #10392]
	mov	w8, #14
	strb	w8, [x9, #2176]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2144
	mov	w9, #10
	str	x9, [sp, #10336]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	strb	wzr, [x9, #2327]
	ldrb	w8, [x9, #2327]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #2368
	strb	w8, [x9, #2304]
	mov	w8, #2
	strb	w8, [x9, #2312]
Ltmp372:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2352
	bl	__ZN4main12instructions20PushStackInstruction3new17hfe7e45e50564c2e7E
Ltmp373:
	b	LBB282_137
LBB282_137:
	ldr	x9, [sp, #1240]
	ldr	q0, [x9, #2288]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2304
	str	q0, [x9, #2240]
	mov	w8, #13
	strb	w8, [x9, #2280]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2248
	mov	w9, #9
	str	x9, [sp, #10440]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp374:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #248]
	str	w1, [sp, #260]
Ltmp375:
	b	LBB282_138
LBB282_138:
Ltmp376:
	ldr	w9, [sp, #260]
	ldr	x0, [sp, #248]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2496
	and	w1, w9, #0x1
	mov	w9, #1
	mov	x2, x9
	bl	__ZN4main12instructions15CallInstruction3new17h81b5345dc87d0d3aE
Ltmp377:
	b	LBB282_139
LBB282_139:
	ldr	x9, [sp, #1240]
	ldr	q0, [x9, #2432]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2448
	str	q0, [x9, #2384]
	ldr	x8, [sp, #10704]
	str	x8, [sp, #10656]
	mov	w8, #11
	strb	w8, [x9, #2424]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2392
	mov	w9, #7
	str	x9, [sp, #10584]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	mov	w8, #1
	strb	w8, [x9, #2607]
Ltmp378:
	mov	x0, #0
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #232]
	str	w1, [sp, #244]
Ltmp379:
	b	LBB282_140
LBB282_140:
	ldr	x8, [sp, #1240]
	ldr	w9, [sp, #244]
	ldr	x10, [sp, #232]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2672
	str	x10, [sp, #10864]
	and	w9, w9, #0x1
	strb	w9, [x8, #2616]
	add	x2, sp, #2, lsl #12
	add	x2, x2, #2688
	mov	w9, #2
	str	x9, [sp, #10880]
	mov	w9, #3
	strb	w9, [x8, #2632]
	ldrb	w0, [x8, #2607]
Ltmp380:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2624
	bl	__ZN4main12instructions14SubInstruction3new17ha4ff55e7ef410ce5E
Ltmp381:
	b	LBB282_141
LBB282_141:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #2576
	str	x0, [sp, #216]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2624
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	ldr	x1, [sp, #216]
	mov	w8, #6
	strb	w8, [x9, #2552]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2520
	mov	w9, #2
	str	x9, [sp, #10712]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp382:
	mov	w8, #11
	mov	x0, x8
	bl	__ZN4main12instructions23PushInstrPtrInstruction3new17h4d74fd4ece850b62E
	str	x0, [sp, #224]
Ltmp383:
	b	LBB282_142
LBB282_142:
	ldr	x9, [sp, #1240]
	ldr	x8, [sp, #224]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2760
	str	x8, [sp, #10952]
	mov	w8, #14
	strb	w8, [x9, #2736]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2704
	mov	w9, #10
	str	x9, [sp, #10896]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	mov	w8, #1
	strb	w8, [x9, #2887]
	ldrb	w8, [x9, #2887]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #2928
	strb	w8, [x9, #2864]
	mov	w8, #2
	strb	w8, [x9, #2872]
Ltmp384:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2912
	bl	__ZN4main12instructions20PushStackInstruction3new17hfe7e45e50564c2e7E
Ltmp385:
	b	LBB282_143
LBB282_143:
	ldr	x9, [sp, #1240]
	ldr	q0, [x9, #2848]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2864
	str	q0, [x9, #2800]
	mov	w8, #13
	strb	w8, [x9, #2840]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2808
	mov	w9, #9
	str	x9, [sp, #11000]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp386:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #200]
	str	w1, [sp, #212]
Ltmp387:
	b	LBB282_144
LBB282_144:
Ltmp388:
	ldr	w9, [sp, #212]
	ldr	x0, [sp, #200]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3064
	and	w1, w9, #0x1
	mov	w9, #1
	mov	x2, x9
	bl	__ZN4main12instructions15CallInstruction3new17h81b5345dc87d0d3aE
Ltmp389:
	b	LBB282_145
LBB282_145:
	ldr	x9, [sp, #1240]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2809
	ldur	q0, [x8, #255]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3008
	str	q0, [x9, #2944]
	ldr	x8, [sp, #11272]
	str	x8, [sp, #11216]
	mov	w8, #11
	strb	w8, [x9, #2984]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #2952
	mov	w9, #7
	str	x9, [sp, #11144]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x8, [sp, #1240]
	strb	wzr, [x8, #3175]
Ltmp390:
	mov	w8, #1
	mov	x0, x8
	mov	w8, #1
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #184]
	str	w1, [sp, #196]
Ltmp391:
	b	LBB282_146
LBB282_146:
	ldr	x10, [sp, #1240]
	ldr	w9, [sp, #196]
	ldr	x8, [sp, #184]
	str	x8, [sp, #11432]
	mov	w8, #1
	and	w9, w9, w8
	strb	w9, [x10, #3184]
Ltmp392:
	mov	w9, #2
	mov	x0, x9
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #168]
	str	w1, [sp, #180]
Ltmp393:
	b	LBB282_147
LBB282_147:
	ldr	x8, [sp, #1240]
	ldr	w9, [sp, #180]
	ldr	x10, [sp, #168]
	add	x2, sp, #2, lsl #12
	add	x2, x2, #3256
	str	x10, [sp, #11448]
	and	w9, w9, #0x1
	strb	w9, [x8, #3200]
	ldrb	w0, [x8, #3175]
Ltmp394:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3192
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3240
	bl	__ZN4main12instructions14AddInstruction3new17h1fa494a4c80f5d48E
Ltmp395:
	b	LBB282_148
LBB282_148:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #3144
	str	x0, [sp, #160]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3192
	mov	w8, #40
	mov	x2, x8
	bl	_memcpy
	ldr	x1, [sp, #160]
	ldr	x9, [sp, #1240]
	mov	w8, #5
	strb	w8, [x9, #3120]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3088
	mov	w9, #1
	str	x9, [sp, #11280]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	strb	wzr, [x9, #3359]
	ldrb	w8, [x9, #3359]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #3400
	strb	w8, [x9, #3336]
	mov	w8, #2
	strb	w8, [x9, #3344]
Ltmp396:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3384
	bl	__ZN4main12instructions14RetInstruction3new17hba68c35196258340E
Ltmp397:
	b	LBB282_149
LBB282_149:
	ldr	x9, [sp, #1240]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3129
	ldur	q0, [x8, #255]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3328
	str	q0, [x9, #3264]
	mov	w8, #12
	strb	w8, [x9, #3304]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3272
	mov	w9, #8
	str	x9, [sp, #11464]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp398:
	mov	w8, #16
	mov	x0, x8
	bl	__ZN4main12instructions23PushInstrPtrInstruction3new17h4d74fd4ece850b62E
	str	x0, [sp, #152]
Ltmp399:
	b	LBB282_150
LBB282_150:
	ldr	x9, [sp, #1240]
	ldr	x8, [sp, #152]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3480
	str	x8, [sp, #11672]
	mov	w8, #14
	strb	w8, [x9, #3456]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3424
	mov	w9, #10
	str	x9, [sp, #11616]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #1240]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #3656
	str	xzr, [sp, #11848]
	mov	w8, #3
	strb	w8, [x9, #3600]
Ltmp400:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3640
	bl	__ZN4main12instructions20PushStackInstruction3new17hfe7e45e50564c2e7E
Ltmp401:
	b	LBB282_151
LBB282_151:
	ldr	x9, [sp, #1240]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3385
	ldur	q0, [x8, #255]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3584
	str	q0, [x9, #3520]
	mov	w8, #13
	strb	w8, [x9, #3560]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3528
	mov	w9, #9
	str	x9, [sp, #11720]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp402:
	mov	x0, #0
	mov	w8, #0
	and	w1, w8, #0x1
	bl	__ZN4main7vm_util8StackPtr3new17h8e5f5f64b99665f5E
	str	x0, [sp, #136]
	str	w1, [sp, #148]
Ltmp403:
	b	LBB282_152
LBB282_152:
Ltmp404:
	ldr	w9, [sp, #148]
	ldr	x0, [sp, #136]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3776
	and	w1, w9, #0x1
	mov	w9, #1
	mov	x2, x9
	bl	__ZN4main12instructions15CallInstruction3new17h81b5345dc87d0d3aE
Ltmp405:
	b	LBB282_153
LBB282_153:
	ldr	x9, [sp, #1240]
	ldr	q0, [x9, #3712]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3728
	str	q0, [x9, #3664]
	ldr	x8, [sp, #11984]
	str	x8, [sp, #11936]
	mov	w8, #11
	strb	w8, [x9, #3704]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3672
	mov	w9, #7
	str	x9, [sp, #11864]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
Ltmp406:
	bl	__ZN4main12instructions15HaltInstruction3new17hc69fe866ca78bfb8E
Ltmp407:
	b	LBB282_154
LBB282_154:
	ldr	x9, [sp, #1240]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3856
	mov	w8, #4
	strb	w8, [x9, #3832]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3800
	str	xzr, [sp, #11992]
	add	x0, x8, #8
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x8, [sp, #9712]
	str	x8, [sp, #128]
	and	x8, x8, #0x7
	cbnz	x8, LBB282_156
	b	LBB282_155
LBB282_155:
	ldr	x0, [sp, #128]
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1528
	mov	w8, #56
	mov	x2, x8
	str	x2, [sp, #120]
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #56
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1632
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #112
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1816
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #168
	add	x1, sp, #2, lsl #12
	add	x1, x1, #1960
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #224
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2144
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #280
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2248
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #336
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2392
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #392
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2520
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #448
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2704
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #504
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2808
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #560
	add	x1, sp, #2, lsl #12
	add	x1, x1, #2952
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #616
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3088
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #672
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3272
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #728
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3424
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #784
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3528
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #840
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3672
	bl	_memcpy
	ldr	x8, [sp, #128]
	ldr	x2, [sp, #120]
	add	x0, x8, #896
	add	x1, sp, #2, lsl #12
	add	x1, x1, #3800
	bl	_memcpy
	ldr	x0, [sp, #9712]
Ltmp411:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #1496
	mov	w9, #17
	mov	x1, x9
	bl	__ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$8into_vec17h84bd0202cca5e00aE
Ltmp412:
	b	LBB282_157
LBB282_156:
	ldr	x1, [sp, #128]
	mov	w8, #8
	mov	x0, x8
	adrp	x2, l___unnamed_77@PAGE
	add	x2, x2, l___unnamed_77@PAGEOFF
	bl	__ZN4core9panicking36panic_misaligned_pointer_dereference17hdc38b89bae20a49aE
LBB282_157:
Ltmp413:
	add	x0, sp, #3016
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17hcf881085915acf78E
	str	x0, [sp, #104]
	str	x1, [sp, #112]
Ltmp414:
	b	LBB282_160
LBB282_158:
Ltmp441:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1496
	bl	__ZN4core3ptr92drop_in_place$LT$alloc..vec..Vec$LT$$LP$usize$C$main..instructions..Instruction2$RP$$GT$$GT$17hf0fdb457992ba510E
Ltmp442:
	b	LBB282_124
LBB282_159:
Ltmp440:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_158
LBB282_160:
Ltmp415:
	add	x0, sp, #1272
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h39b8db764d9e69c3E
	str	x0, [sp, #88]
	str	x1, [sp, #96]
Ltmp416:
	b	LBB282_161
LBB282_161:
Ltmp417:
	ldr	x3, [sp, #96]
	ldr	x2, [sp, #88]
	ldr	x1, [sp, #112]
	ldr	x0, [sp, #104]
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3904
	bl	__ZN4main2VM3new17h26d5f5b8a06145e0E
Ltmp418:
	b	LBB282_162
LBB282_162:
Ltmp419:
	bl	__ZN3std4time7Instant3now17hd53424a4232df949E
	str	x0, [sp, #72]
	str	w1, [sp, #84]
Ltmp420:
	b	LBB282_165
LBB282_163:
Ltmp436:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #3904
	bl	__ZN4core3ptr29drop_in_place$LT$main..VM$GT$17h33d51f14d3fcf26dE
Ltmp437:
	b	LBB282_158
LBB282_164:
Ltmp435:
	stur	x0, [x29, #-64]
	mov	x8, x1
	stur	w8, [x29, #-56]
	b	LBB282_163
LBB282_165:
	ldr	w8, [sp, #84]
	ldr	x9, [sp, #72]
	stur	x9, [x29, #-224]
	stur	w8, [x29, #-216]
	ldr	x0, [sp, #1344]
Ltmp421:
	bl	__ZN72_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..deref..Deref$GT$5deref17h750abde33ecac153E
	str	x0, [sp, #56]
	str	x1, [sp, #64]
Ltmp422:
	b	LBB282_166
LBB282_166:
Ltmp423:
	ldr	x2, [sp, #64]
	ldr	x1, [sp, #56]
	add	x0, sp, #2, lsl #12
	add	x0, x0, #3904
	bl	__ZN4main2VM7run_1v217h0666c051b3da9926E
Ltmp424:
	b	LBB282_167
LBB282_167:
Ltmp425:
	sub	x0, x29, #224
	bl	__ZN3std4time7Instant7elapsed17h39d822248735d68eE
	str	x0, [sp, #40]
	str	w1, [sp, #52]
Ltmp426:
	b	LBB282_168
LBB282_168:
	ldr	w9, [sp, #52]
	ldr	x10, [sp, #40]
	sub	x8, x29, #144
	stur	x10, [x29, #-144]
	stur	w9, [x29, #-136]
	stur	x8, [x29, #-32]
	adrp	x8, __ZN57_$LT$core..time..Duration$u20$as$u20$core..fmt..Debug$GT$3fmt17h6867f81edf14bf6dE@GOTPAGE
	ldr	x8, [x8, __ZN57_$LT$core..time..Duration$u20$as$u20$core..fmt..Debug$GT$3fmt17h6867f81edf14bf6dE@GOTPAGEOFF]
	stur	x8, [x29, #-24]
	ldur	x8, [x29, #-32]
	str	x8, [sp, #24]
	ldur	x8, [x29, #-24]
	str	x8, [sp, #32]
	b	LBB282_169
LBB282_169:
	ldr	x8, [sp, #32]
	ldr	x9, [sp, #24]
	sub	x2, x29, #160
	stur	x9, [x29, #-160]
	stur	x8, [x29, #-152]
Ltmp427:
	sub	x8, x29, #208
	adrp	x0, l___unnamed_78@PAGE
	add	x0, x0, l___unnamed_78@PAGEOFF
	mov	w9, #2
	mov	x1, x9
	mov	w9, #1
	mov	x3, x9
	bl	__ZN4core3fmt9Arguments6new_v117h619f9bb2db3983b4E
Ltmp428:
	b	LBB282_170
LBB282_170:
Ltmp429:
	sub	x0, x29, #208
	bl	__ZN3std2io5stdio6_print17hd9ffb3f73dfcc2b3E
Ltmp430:
	b	LBB282_171
LBB282_171:
	add	x8, sp, #2, lsl #12
	add	x8, x8, #3904
	stur	x8, [x29, #-48]
	adrp	x8, __ZN57_$LT$main..vm_util..Stack$u20$as$u20$core..fmt..Debug$GT$3fmt17h9759a1e612de70c1E@PAGE
	add	x8, x8, __ZN57_$LT$main..vm_util..Stack$u20$as$u20$core..fmt..Debug$GT$3fmt17h9759a1e612de70c1E@PAGEOFF
	stur	x8, [x29, #-40]
	ldur	x8, [x29, #-48]
	str	x8, [sp, #8]
	ldur	x8, [x29, #-40]
	str	x8, [sp, #16]
	b	LBB282_172
LBB282_172:
	ldr	x8, [sp, #16]
	ldr	x9, [sp, #8]
	sub	x2, x29, #80
	stur	x9, [x29, #-80]
	stur	x8, [x29, #-72]
Ltmp431:
	sub	x8, x29, #128
	adrp	x0, l___unnamed_78@PAGE
	add	x0, x0, l___unnamed_78@PAGEOFF
	mov	w9, #2
	mov	x1, x9
	mov	w9, #1
	mov	x3, x9
	bl	__ZN4core3fmt9Arguments6new_v117h619f9bb2db3983b4E
Ltmp432:
	b	LBB282_173
LBB282_173:
Ltmp433:
	sub	x0, x29, #128
	bl	__ZN3std2io5stdio6_print17hd9ffb3f73dfcc2b3E
Ltmp434:
	b	LBB282_174
LBB282_174:
Ltmp438:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #3904
	bl	__ZN4core3ptr29drop_in_place$LT$main..VM$GT$17h33d51f14d3fcf26dE
Ltmp439:
	b	LBB282_175
LBB282_175:
Ltmp443:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #1496
	bl	__ZN4core3ptr92drop_in_place$LT$alloc..vec..Vec$LT$$LP$usize$C$main..instructions..Instruction2$RP$$GT$$GT$17hf0fdb457992ba510E
Ltmp444:
	b	LBB282_176
LBB282_176:
Ltmp448:
	add	x0, sp, #2, lsl #12
	add	x0, x0, #32
	bl	__ZN4core3ptr76drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..Instruction2$GT$$GT$17heda17ac1179fa6a0E
Ltmp449:
	b	LBB282_177
LBB282_177:
Ltmp453:
	add	x0, sp, #3784
	bl	__ZN4core3ptr108drop_in_place$LT$alloc..vec..Vec$LT$$LP$usize$C$$u5b$main..instructions..Operand$u3b$$u20$4$u5d$$RP$$GT$$GT$17h4b2975b76c04d018E
Ltmp454:
	b	LBB282_178
LBB282_178:
Ltmp458:
	add	x0, sp, #3016
	bl	__ZN4core3ptr75drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..Instruction$GT$$GT$17hb330cbe1268ffd9aE
Ltmp459:
	b	LBB282_179
LBB282_179:
Ltmp463:
	add	x0, sp, #1352
	bl	__ZN4core3ptr77drop_in_place$LT$alloc..vec..Vec$LT$main..instructions..InstructionV2$GT$$GT$17h570c83b0fdd350d5E
Ltmp464:
	b	LBB282_180
LBB282_180:
	add	x0, sp, #1272
	bl	__ZN4core3ptr64drop_in_place$LT$alloc..vec..Vec$LT$main..vm_util..Value$GT$$GT$17h5e3a7fb19708ec5dE
	add	sp, sp, #3, lsl #12
	add	sp, sp, #192
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	ldp	x28, x27, [sp], #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB282_181:
	.cfi_restore_state
Ltmp468:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB282_182:
	ldur	x0, [x29, #-64]
	bl	__Unwind_Resume
Lfunc_end24:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table282:
Lexception24:
	.byte	255
	.byte	155
	.uleb128 Lttbase15-Lttbaseref15
Lttbaseref15:
	.byte	1
	.uleb128 Lcst_end24-Lcst_begin24
Lcst_begin24:
	.uleb128 Lfunc_begin24-Lfunc_begin24
	.uleb128 Ltmp148-Lfunc_begin24
	.byte	0
	.byte	0
	.uleb128 Ltmp148-Lfunc_begin24
	.uleb128 Ltmp149-Ltmp148
	.uleb128 Ltmp465-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp466-Lfunc_begin24
	.uleb128 Ltmp467-Ltmp466
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp150-Lfunc_begin24
	.uleb128 Ltmp151-Ltmp150
	.uleb128 Ltmp232-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp233-Lfunc_begin24
	.uleb128 Ltmp234-Ltmp233
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp152-Lfunc_begin24
	.uleb128 Ltmp231-Ltmp152
	.uleb128 Ltmp232-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp231-Lfunc_begin24
	.uleb128 Ltmp235-Ltmp231
	.byte	0
	.byte	0
	.uleb128 Ltmp235-Lfunc_begin24
	.uleb128 Ltmp236-Ltmp235
	.uleb128 Ltmp465-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp237-Lfunc_begin24
	.uleb128 Ltmp238-Ltmp237
	.uleb128 Ltmp460-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp461-Lfunc_begin24
	.uleb128 Ltmp462-Ltmp461
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp239-Lfunc_begin24
	.uleb128 Ltmp240-Ltmp239
	.uleb128 Ltmp263-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp264-Lfunc_begin24
	.uleb128 Ltmp265-Ltmp264
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp241-Lfunc_begin24
	.uleb128 Ltmp262-Ltmp241
	.uleb128 Ltmp263-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp262-Lfunc_begin24
	.uleb128 Ltmp266-Ltmp262
	.byte	0
	.byte	0
	.uleb128 Ltmp266-Lfunc_begin24
	.uleb128 Ltmp267-Ltmp266
	.uleb128 Ltmp460-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp268-Lfunc_begin24
	.uleb128 Ltmp269-Ltmp268
	.uleb128 Ltmp455-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp456-Lfunc_begin24
	.uleb128 Ltmp457-Ltmp456
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp457-Lfunc_begin24
	.uleb128 Ltmp270-Ltmp457
	.byte	0
	.byte	0
	.uleb128 Ltmp270-Lfunc_begin24
	.uleb128 Ltmp271-Ltmp270
	.uleb128 Ltmp288-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp289-Lfunc_begin24
	.uleb128 Ltmp290-Ltmp289
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp290-Lfunc_begin24
	.uleb128 Ltmp272-Ltmp290
	.byte	0
	.byte	0
	.uleb128 Ltmp272-Lfunc_begin24
	.uleb128 Ltmp273-Ltmp272
	.uleb128 Ltmp288-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp273-Lfunc_begin24
	.uleb128 Ltmp274-Ltmp273
	.byte	0
	.byte	0
	.uleb128 Ltmp274-Lfunc_begin24
	.uleb128 Ltmp275-Ltmp274
	.uleb128 Ltmp288-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp275-Lfunc_begin24
	.uleb128 Ltmp276-Ltmp275
	.byte	0
	.byte	0
	.uleb128 Ltmp276-Lfunc_begin24
	.uleb128 Ltmp277-Ltmp276
	.uleb128 Ltmp288-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp277-Lfunc_begin24
	.uleb128 Ltmp278-Ltmp277
	.byte	0
	.byte	0
	.uleb128 Ltmp278-Lfunc_begin24
	.uleb128 Ltmp279-Ltmp278
	.uleb128 Ltmp288-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp279-Lfunc_begin24
	.uleb128 Ltmp280-Ltmp279
	.byte	0
	.byte	0
	.uleb128 Ltmp280-Lfunc_begin24
	.uleb128 Ltmp281-Ltmp280
	.uleb128 Ltmp288-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp281-Lfunc_begin24
	.uleb128 Ltmp282-Ltmp281
	.byte	0
	.byte	0
	.uleb128 Ltmp282-Lfunc_begin24
	.uleb128 Ltmp285-Ltmp282
	.uleb128 Ltmp288-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp285-Lfunc_begin24
	.uleb128 Ltmp286-Ltmp285
	.byte	0
	.byte	0
	.uleb128 Ltmp286-Lfunc_begin24
	.uleb128 Ltmp287-Ltmp286
	.uleb128 Ltmp288-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp287-Lfunc_begin24
	.uleb128 Ltmp291-Ltmp287
	.byte	0
	.byte	0
	.uleb128 Ltmp291-Lfunc_begin24
	.uleb128 Ltmp292-Ltmp291
	.uleb128 Ltmp455-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp293-Lfunc_begin24
	.uleb128 Ltmp294-Ltmp293
	.uleb128 Ltmp450-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp451-Lfunc_begin24
	.uleb128 Ltmp452-Ltmp451
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp295-Lfunc_begin24
	.uleb128 Ltmp296-Ltmp295
	.uleb128 Ltmp450-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp297-Lfunc_begin24
	.uleb128 Ltmp298-Ltmp297
	.uleb128 Ltmp349-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp350-Lfunc_begin24
	.uleb128 Ltmp351-Ltmp350
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp299-Lfunc_begin24
	.uleb128 Ltmp302-Ltmp299
	.uleb128 Ltmp349-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp302-Lfunc_begin24
	.uleb128 Ltmp303-Ltmp302
	.byte	0
	.byte	0
	.uleb128 Ltmp303-Lfunc_begin24
	.uleb128 Ltmp310-Ltmp303
	.uleb128 Ltmp349-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp310-Lfunc_begin24
	.uleb128 Ltmp311-Ltmp310
	.byte	0
	.byte	0
	.uleb128 Ltmp311-Lfunc_begin24
	.uleb128 Ltmp322-Ltmp311
	.uleb128 Ltmp349-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp322-Lfunc_begin24
	.uleb128 Ltmp323-Ltmp322
	.byte	0
	.byte	0
	.uleb128 Ltmp323-Lfunc_begin24
	.uleb128 Ltmp336-Ltmp323
	.uleb128 Ltmp349-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp336-Lfunc_begin24
	.uleb128 Ltmp337-Ltmp336
	.byte	0
	.byte	0
	.uleb128 Ltmp337-Lfunc_begin24
	.uleb128 Ltmp348-Ltmp337
	.uleb128 Ltmp349-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp348-Lfunc_begin24
	.uleb128 Ltmp352-Ltmp348
	.byte	0
	.byte	0
	.uleb128 Ltmp352-Lfunc_begin24
	.uleb128 Ltmp353-Ltmp352
	.uleb128 Ltmp450-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp354-Lfunc_begin24
	.uleb128 Ltmp355-Ltmp354
	.uleb128 Ltmp445-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp446-Lfunc_begin24
	.uleb128 Ltmp447-Ltmp446
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp356-Lfunc_begin24
	.uleb128 Ltmp357-Ltmp356
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp409-Lfunc_begin24
	.uleb128 Ltmp410-Ltmp409
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp410-Lfunc_begin24
	.uleb128 Ltmp358-Ltmp410
	.byte	0
	.byte	0
	.uleb128 Ltmp358-Lfunc_begin24
	.uleb128 Ltmp361-Ltmp358
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp361-Lfunc_begin24
	.uleb128 Ltmp362-Ltmp361
	.byte	0
	.byte	0
	.uleb128 Ltmp362-Lfunc_begin24
	.uleb128 Ltmp365-Ltmp362
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp365-Lfunc_begin24
	.uleb128 Ltmp366-Ltmp365
	.byte	0
	.byte	0
	.uleb128 Ltmp366-Lfunc_begin24
	.uleb128 Ltmp369-Ltmp366
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp369-Lfunc_begin24
	.uleb128 Ltmp370-Ltmp369
	.byte	0
	.byte	0
	.uleb128 Ltmp370-Lfunc_begin24
	.uleb128 Ltmp371-Ltmp370
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp371-Lfunc_begin24
	.uleb128 Ltmp372-Ltmp371
	.byte	0
	.byte	0
	.uleb128 Ltmp372-Lfunc_begin24
	.uleb128 Ltmp373-Ltmp372
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp373-Lfunc_begin24
	.uleb128 Ltmp374-Ltmp373
	.byte	0
	.byte	0
	.uleb128 Ltmp374-Lfunc_begin24
	.uleb128 Ltmp377-Ltmp374
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp377-Lfunc_begin24
	.uleb128 Ltmp378-Ltmp377
	.byte	0
	.byte	0
	.uleb128 Ltmp378-Lfunc_begin24
	.uleb128 Ltmp381-Ltmp378
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp381-Lfunc_begin24
	.uleb128 Ltmp382-Ltmp381
	.byte	0
	.byte	0
	.uleb128 Ltmp382-Lfunc_begin24
	.uleb128 Ltmp383-Ltmp382
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp383-Lfunc_begin24
	.uleb128 Ltmp384-Ltmp383
	.byte	0
	.byte	0
	.uleb128 Ltmp384-Lfunc_begin24
	.uleb128 Ltmp385-Ltmp384
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp385-Lfunc_begin24
	.uleb128 Ltmp386-Ltmp385
	.byte	0
	.byte	0
	.uleb128 Ltmp386-Lfunc_begin24
	.uleb128 Ltmp389-Ltmp386
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp389-Lfunc_begin24
	.uleb128 Ltmp390-Ltmp389
	.byte	0
	.byte	0
	.uleb128 Ltmp390-Lfunc_begin24
	.uleb128 Ltmp395-Ltmp390
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp395-Lfunc_begin24
	.uleb128 Ltmp396-Ltmp395
	.byte	0
	.byte	0
	.uleb128 Ltmp396-Lfunc_begin24
	.uleb128 Ltmp397-Ltmp396
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp397-Lfunc_begin24
	.uleb128 Ltmp398-Ltmp397
	.byte	0
	.byte	0
	.uleb128 Ltmp398-Lfunc_begin24
	.uleb128 Ltmp399-Ltmp398
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp399-Lfunc_begin24
	.uleb128 Ltmp400-Ltmp399
	.byte	0
	.byte	0
	.uleb128 Ltmp400-Lfunc_begin24
	.uleb128 Ltmp401-Ltmp400
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp401-Lfunc_begin24
	.uleb128 Ltmp402-Ltmp401
	.byte	0
	.byte	0
	.uleb128 Ltmp402-Lfunc_begin24
	.uleb128 Ltmp405-Ltmp402
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp405-Lfunc_begin24
	.uleb128 Ltmp406-Ltmp405
	.byte	0
	.byte	0
	.uleb128 Ltmp406-Lfunc_begin24
	.uleb128 Ltmp407-Ltmp406
	.uleb128 Ltmp408-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp407-Lfunc_begin24
	.uleb128 Ltmp411-Ltmp407
	.byte	0
	.byte	0
	.uleb128 Ltmp411-Lfunc_begin24
	.uleb128 Ltmp412-Ltmp411
	.uleb128 Ltmp445-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp413-Lfunc_begin24
	.uleb128 Ltmp414-Ltmp413
	.uleb128 Ltmp440-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp441-Lfunc_begin24
	.uleb128 Ltmp442-Ltmp441
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp415-Lfunc_begin24
	.uleb128 Ltmp418-Ltmp415
	.uleb128 Ltmp440-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp419-Lfunc_begin24
	.uleb128 Ltmp420-Ltmp419
	.uleb128 Ltmp435-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp436-Lfunc_begin24
	.uleb128 Ltmp437-Ltmp436
	.uleb128 Ltmp468-Lfunc_begin24
	.byte	1
	.uleb128 Ltmp421-Lfunc_begin24
	.uleb128 Ltmp434-Ltmp421
	.uleb128 Ltmp435-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp438-Lfunc_begin24
	.uleb128 Ltmp439-Ltmp438
	.uleb128 Ltmp440-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp443-Lfunc_begin24
	.uleb128 Ltmp444-Ltmp443
	.uleb128 Ltmp445-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp448-Lfunc_begin24
	.uleb128 Ltmp449-Ltmp448
	.uleb128 Ltmp450-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp453-Lfunc_begin24
	.uleb128 Ltmp454-Ltmp453
	.uleb128 Ltmp455-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp458-Lfunc_begin24
	.uleb128 Ltmp459-Ltmp458
	.uleb128 Ltmp460-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp463-Lfunc_begin24
	.uleb128 Ltmp464-Ltmp463
	.uleb128 Ltmp465-Lfunc_begin24
	.byte	0
	.uleb128 Ltmp464-Lfunc_begin24
	.uleb128 Lfunc_end24-Ltmp464
	.byte	0
	.byte	0
Lcst_end24:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase15:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4main2VM3new17h26d5f5b8a06145e0E:
Lfunc_begin25:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception25
	sub	sp, sp, #224
	.cfi_def_cfa_offset 224
	stp	x29, x30, [sp, #208]
	add	x29, sp, #208
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #8]
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	str	x2, [sp, #32]
	str	x3, [sp, #40]
	add	x8, sp, #48
	bl	__ZN4main7vm_util5Stack3new17h2ffa87df64a54c0aE
Ltmp469:
	add	x8, sp, #96
	bl	__ZN4main9Registers3new17h9b0ce7e64fcbf2efE
Ltmp470:
	b	LBB283_3
LBB283_1:
Ltmp474:
	add	x0, sp, #48
	bl	__ZN4core3ptr41drop_in_place$LT$main..vm_util..Stack$GT$17h156d0e55a5646dacE
Ltmp475:
	b	LBB283_6
LBB283_2:
Ltmp473:
	stur	x0, [x29, #-16]
	mov	x8, x1
	stur	w8, [x29, #-8]
	b	LBB283_1
LBB283_3:
Ltmp471:
	sub	x8, x29, #40
	bl	__ZN5alloc3vec12Vec$LT$T$GT$3new17ha107aee19cd17e30E
Ltmp472:
	b	LBB283_4
LBB283_4:
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #40]
	ldr	x9, [sp, #32]
	ldr	x10, [sp, #24]
	ldr	x11, [sp, #16]
	str	x11, [x0, #144]
	str	x10, [x0, #152]
	str	x9, [x0, #160]
	str	x8, [x0, #168]
	add	x1, sp, #48
	mov	w8, #48
	mov	x2, x8
	bl	_memcpy
	ldr	x8, [sp, #8]
	add	x0, x8, #72
	add	x1, sp, #96
	mov	w8, #72
	mov	x2, x8
	bl	_memcpy
	ldr	x9, [sp, #8]
	ldur	q0, [x29, #-40]
	str	q0, [x9, #48]
	ldur	x8, [x29, #-24]
	str	x8, [x9, #64]
	.cfi_def_cfa wsp, 224
	ldp	x29, x30, [sp, #208]
	add	sp, sp, #224
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB283_5:
	.cfi_restore_state
Ltmp476:
	bl	__ZN4core9panicking16panic_in_cleanup17he9ef3195c438193cE
LBB283_6:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
Lfunc_end25:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table283:
Lexception25:
	.byte	255
	.byte	155
	.uleb128 Lttbase16-Lttbaseref16
Lttbaseref16:
	.byte	1
	.uleb128 Lcst_end25-Lcst_begin25
Lcst_begin25:
	.uleb128 Lfunc_begin25-Lfunc_begin25
	.uleb128 Ltmp469-Lfunc_begin25
	.byte	0
	.byte	0
	.uleb128 Ltmp469-Lfunc_begin25
	.uleb128 Ltmp470-Ltmp469
	.uleb128 Ltmp473-Lfunc_begin25
	.byte	0
	.uleb128 Ltmp474-Lfunc_begin25
	.uleb128 Ltmp475-Ltmp474
	.uleb128 Ltmp476-Lfunc_begin25
	.byte	1
	.uleb128 Ltmp471-Lfunc_begin25
	.uleb128 Ltmp472-Ltmp471
	.uleb128 Ltmp473-Lfunc_begin25
	.byte	0
	.uleb128 Ltmp472-Lfunc_begin25
	.uleb128 Lfunc_end25-Ltmp472
	.byte	0
	.byte	0
Lcst_end25:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase16:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4main2VM7run_1v217h0666c051b3da9926E:
	.cfi_startproc
	sub	sp, sp, #320
	.cfi_def_cfa_offset 320
	stp	x28, x27, [sp, #288]
	stp	x29, x30, [sp, #304]
	add	x29, sp, #304
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w27, -24
	.cfi_offset w28, -32
	add	x8, sp, #40
	str	x1, [sp, #40]
	str	x2, [sp, #48]
	stur	x8, [x29, #-32]
	adrp	x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8fc3c848711e8923E@PAGE
	add	x8, x8, __ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h8fc3c848711e8923E@PAGEOFF
	stur	x8, [x29, #-24]
	ldur	x9, [x29, #-32]
	ldur	x8, [x29, #-24]
	add	x10, sp, #104
	str	x10, [sp, #16]
	str	x9, [sp, #104]
	str	x8, [sp, #112]
	mov	w8, #3
	sturb	w8, [x29, #-65]
	mov	w8, #2
	str	x8, [sp, #8]
	stur	x8, [x29, #-64]
	stur	x8, [x29, #-48]
	ldurb	w12, [x29, #-65]
	ldur	x11, [x29, #-64]
	ldur	x10, [x29, #-56]
	ldur	x9, [x29, #-48]
	ldur	x8, [x29, #-40]
	sub	x1, x29, #128
	stur	xzr, [x29, #-96]
	mov	w13, #32
	stur	w13, [x29, #-88]
	sturb	w12, [x29, #-80]
	mov	w12, #4
	stur	w12, [x29, #-84]
	stur	x11, [x29, #-128]
	stur	x10, [x29, #-120]
	stur	x9, [x29, #-112]
	stur	x8, [x29, #-104]
	add	x0, sp, #120
	str	x0, [sp, #24]
	mov	w8, #56
	mov	x2, x8
	bl	_memcpy
	ldr	x1, [sp, #8]
	ldr	x2, [sp, #16]
	ldr	x4, [sp, #24]
	add	x8, sp, #56
	str	x8, [sp, #32]
	adrp	x0, l___unnamed_78@PAGE
	add	x0, x0, l___unnamed_78@PAGEOFF
	mov	w9, #1
	mov	x5, x9
	mov	x3, x5
	bl	__ZN4core3fmt9Arguments16new_v1_formatted17he08a92c810cac804E
	ldr	x0, [sp, #32]
	bl	__ZN3std2io5stdio6_print17hd9ffb3f73dfcc2b3E
	.cfi_def_cfa wsp, 320
	ldp	x29, x30, [sp, #304]
	ldp	x28, x27, [sp, #288]
	add	sp, sp, #320
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w27
	.cfi_restore w28
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers3new17h9b0ce7e64fcbf2efE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	str	xzr, [sp, #8]
	mov	w9, #3
	strb	w9, [sp]
	str	xzr, [sp, #24]
	strb	w9, [sp, #16]
	str	xzr, [sp, #40]
	strb	w9, [sp, #32]
	str	xzr, [sp, #56]
	strb	w9, [sp, #48]
	ldr	q0, [sp]
	str	q0, [x8]
	ldr	q0, [sp, #16]
	str	q0, [x8, #16]
	ldr	q0, [sp, #32]
	str	q0, [x8, #32]
	ldr	q0, [sp, #48]
	str	q0, [x8, #48]
	str	xzr, [x8, #64]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers7get_reg17h8a9834799835db69E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	str	x0, [sp]
	strb	w1, [sp, #23]
	ldrb	w8, [sp, #23]
	str	x8, [sp, #8]
	cbz	x8, LBB286_5
	b	LBB286_1
LBB286_1:
	ldr	x8, [sp, #8]
	subs	x8, x8, #1
	b.eq	LBB286_6
	b	LBB286_2
LBB286_2:
	ldr	x8, [sp, #8]
	subs	x8, x8, #2
	b.eq	LBB286_7
	b	LBB286_3
LBB286_3:
	b	LBB286_8
LBB286_5:
	ldr	x8, [sp]
	str	x8, [sp, #24]
	b	LBB286_9
LBB286_6:
	ldr	x8, [sp]
	add	x8, x8, #16
	str	x8, [sp, #24]
	b	LBB286_9
LBB286_7:
	ldr	x8, [sp]
	add	x8, x8, #32
	str	x8, [sp, #24]
	b	LBB286_9
LBB286_8:
	ldr	x8, [sp]
	add	x8, x8, #48
	str	x8, [sp, #24]
	b	LBB286_9
LBB286_9:
	ldr	x0, [sp, #24]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers7set_reg17h7064e2aa42c06f9eE:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	str	x2, [sp, #8]
	strb	w1, [sp, #31]
	ldrb	w8, [sp, #31]
	str	x8, [sp, #16]
	cbz	x8, LBB287_5
	b	LBB287_1
LBB287_1:
	ldr	x8, [sp, #16]
	subs	x8, x8, #1
	b.eq	LBB287_6
	b	LBB287_2
LBB287_2:
	ldr	x8, [sp, #16]
	subs	x8, x8, #2
	b.eq	LBB287_7
	b	LBB287_3
LBB287_3:
	b	LBB287_8
LBB287_5:
	ldr	x0, [sp]
	ldr	x8, [sp, #8]
	ldr	q0, [x8]
	add	x1, sp, #32
	str	q0, [sp, #32]
	bl	__ZN4main9Registers6set_r017hb485b446a43a6b79E
	b	LBB287_9
LBB287_6:
	ldr	x0, [sp]
	ldr	x8, [sp, #8]
	ldr	q0, [x8]
	add	x1, sp, #48
	str	q0, [sp, #48]
	bl	__ZN4main9Registers6set_r117h4f7701557c65d9e1E
	b	LBB287_9
LBB287_7:
	ldr	x0, [sp]
	ldr	x8, [sp, #8]
	ldr	q0, [x8]
	sub	x1, x29, #32
	stur	q0, [x29, #-32]
	bl	__ZN4main9Registers6set_r217h0dcb72f38d925504E
	b	LBB287_9
LBB287_8:
	ldr	x0, [sp]
	ldr	x8, [sp, #8]
	ldr	q0, [x8]
	sub	x1, x29, #16
	stur	q0, [x29, #-16]
	bl	__ZN4main9Registers6set_r317hc0704d92a7139ce8E
	b	LBB287_9
LBB287_9:
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers6set_r017hb485b446a43a6b79E:
	.cfi_startproc
	ldr	q0, [x1]
	str	q0, [x0]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers6set_r117h4f7701557c65d9e1E:
	.cfi_startproc
	ldr	q0, [x1]
	str	q0, [x0, #16]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers6set_r217h0dcb72f38d925504E:
	.cfi_startproc
	ldr	q0, [x1]
	str	q0, [x0, #32]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers6set_r317hc0704d92a7139ce8E:
	.cfi_startproc
	ldr	q0, [x1]
	str	q0, [x0, #48]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers6get_ip17hc33f2887d7bd86c3E:
	.cfi_startproc
	ldr	x0, [x0, #64]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers6set_ip17h5ac825b8f5ec62e9E:
	.cfi_startproc
	str	x1, [x0, #64]
	ret
	.cfi_endproc

	.p2align	2
__ZN4main9Registers6add_ip17hb3c329094c540883E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp]
	ldr	x8, [x0, #64]
	adds	x8, x8, x1
	str	x8, [sp, #8]
	cset	w8, hs
	tbnz	w8, #0, LBB294_2
	b	LBB294_1
LBB294_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	str	x8, [x9, #64]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB294_2:
	.cfi_restore_state
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_79@PAGE
	add	x2, x2, l___unnamed_79@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN4main9Registers6inc_ip17hb67aa2a2bee90fdaE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp]
	ldr	x8, [x0, #64]
	adds	x8, x8, #1
	str	x8, [sp, #8]
	cset	w8, hs
	tbnz	w8, #0, LBB295_2
	b	LBB295_1
LBB295_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp]
	str	x8, [x9, #64]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB295_2:
	.cfi_restore_state
	adrp	x0, _str.2@PAGE
	add	x0, x0, _str.2@PAGEOFF
	mov	w8, #28
	mov	x1, x8
	adrp	x2, l___unnamed_80@PAGE
	add	x2, x2, l___unnamed_80@PAGEOFF
	bl	__ZN4core9panicking5panic17hc59c8a709a9b37aeE
	.cfi_endproc

	.p2align	2
__ZN70_$LT$main..instructions..InstructionV2$u20$as$u20$core..fmt..Debug$GT$3fmt17h1fc8fedb089aa03cE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	mov	x0, x1
	add	x5, sp, #8
	str	x8, [sp, #8]
	adrp	x1, l___unnamed_81@PAGE
	add	x1, x1, l___unnamed_81@PAGEOFF
	mov	w9, #13
	mov	x2, x9
	add	x3, x8, #40
	adrp	x4, l___unnamed_82@PAGE
	add	x4, x4, l___unnamed_82@PAGEOFF
	adrp	x6, l___unnamed_83@PAGE
	add	x6, x6, l___unnamed_83@PAGEOFF
	bl	__ZN4core3fmt9Formatter25debug_tuple_field2_finish17hdbc837dd714253efE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN57_$LT$main..vm_util..Value$u20$as$u20$core..fmt..Debug$GT$3fmt17hbf95ed63f183eb94E:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	ldrb	w8, [x0]
	str	x8, [sp, #24]
	cbz	x8, LBB297_4
	b	LBB297_1
LBB297_1:
	ldr	x8, [sp, #24]
	subs	x8, x8, #1
	b.eq	LBB297_5
	b	LBB297_2
LBB297_2:
	b	LBB297_6
LBB297_4:
	ldr	x0, [sp, #16]
	ldr	x8, [sp, #8]
	add	x8, x8, #4
	sub	x3, x29, #24
	stur	x8, [x29, #-24]
	adrp	x1, l___unnamed_84@PAGE
	add	x1, x1, l___unnamed_84@PAGEOFF
	mov	w8, #5
	mov	x2, x8
	adrp	x4, l___unnamed_85@PAGE
	add	x4, x4, l___unnamed_85@PAGEOFF
	bl	__ZN4core3fmt9Formatter25debug_tuple_field1_finish17h357a6e68d70d3705E
	sturb	w0, [x29, #-25]
	b	LBB297_7
LBB297_5:
	ldr	x0, [sp, #16]
	ldr	x8, [sp, #8]
	add	x8, x8, #8
	sub	x3, x29, #16
	stur	x8, [x29, #-16]
	adrp	x1, l___unnamed_86@PAGE
	add	x1, x1, l___unnamed_86@PAGEOFF
	mov	w8, #5
	mov	x2, x8
	adrp	x4, l___unnamed_87@PAGE
	add	x4, x4, l___unnamed_87@PAGEOFF
	bl	__ZN4core3fmt9Formatter25debug_tuple_field1_finish17h357a6e68d70d3705E
	sturb	w0, [x29, #-25]
	b	LBB297_7
LBB297_6:
	ldr	x0, [sp, #16]
	ldr	x8, [sp, #8]
	add	x8, x8, #1
	sub	x3, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l___unnamed_88@PAGE
	add	x1, x1, l___unnamed_88@PAGEOFF
	mov	w8, #4
	mov	x2, x8
	adrp	x4, l___unnamed_89@PAGE
	add	x4, x4, l___unnamed_89@PAGEOFF
	bl	__ZN4core3fmt9Formatter25debug_tuple_field1_finish17h357a6e68d70d3705E
	sturb	w0, [x29, #-25]
	b	LBB297_7
LBB297_7:
	ldurb	w8, [x29, #-25]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN59_$LT$main..vm_util..Value$u20$as$u20$core..clone..Clone$GT$5clone17h5a17eabc18bf908aE:
	.cfi_startproc
	ldr	q0, [x0]
	str	q0, [x8]
	ret
	.cfi_endproc

	.p2align	2
__ZN57_$LT$main..vm_util..Stack$u20$as$u20$core..fmt..Debug$GT$3fmt17h9759a1e612de70c1E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x5, x0
	mov	x0, x1
	mov	x8, x5
	add	x9, x8, #24
	sub	x8, x29, #8
	stur	x9, [x29, #-8]
	mov	x9, sp
	mov	w10, #12
	str	x10, [x9]
	str	x8, [x9, #8]
	adrp	x8, l___unnamed_90@PAGE
	add	x8, x8, l___unnamed_90@PAGEOFF
	str	x8, [x9, #16]
	adrp	x1, l___unnamed_91@PAGE
	add	x1, x1, l___unnamed_91@PAGEOFF
	mov	w8, #5
	mov	x4, x8
	mov	x2, x4
	adrp	x3, l___unnamed_92@PAGE
	add	x3, x3, l___unnamed_92@PAGEOFF
	adrp	x6, l___unnamed_93@PAGE
	add	x6, x6, l___unnamed_93@PAGEOFF
	adrp	x7, l___unnamed_94@PAGE
	add	x7, x7, l___unnamed_94@PAGEOFF
	bl	__ZN4core3fmt9Formatter26debug_struct_field2_finish17h548d01a5a6b12c92E
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN62_$LT$main..vm_util..StackValue$u20$as$u20$core..fmt..Debug$GT$3fmt17hafd7a73d9e63f38cE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	ldrb	w8, [x0]
	subs	w8, w8, #3
	cset	x8, ne
	cbnz	x8, LBB300_2
	b	LBB300_1
LBB300_1:
	ldr	x0, [sp, #16]
	ldr	x8, [sp, #8]
	add	x8, x8, #8
	sub	x3, x29, #16
	stur	x8, [x29, #-16]
	adrp	x1, l___unnamed_95@PAGE
	add	x1, x1, l___unnamed_95@PAGEOFF
	mov	w8, #5
	mov	x2, x8
	adrp	x4, l___unnamed_10@PAGE
	add	x4, x4, l___unnamed_10@PAGEOFF
	bl	__ZN4core3fmt9Formatter25debug_tuple_field1_finish17h357a6e68d70d3705E
	sturb	w0, [x29, #-17]
	b	LBB300_3
LBB300_2:
	ldr	x0, [sp, #16]
	ldr	x8, [sp, #8]
	sub	x3, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l___unnamed_96@PAGE
	add	x1, x1, l___unnamed_96@PAGEOFF
	mov	w8, #5
	mov	x2, x8
	adrp	x4, l___unnamed_97@PAGE
	add	x4, x4, l___unnamed_97@PAGEOFF
	bl	__ZN4core3fmt9Formatter25debug_tuple_field1_finish17h357a6e68d70d3705E
	sturb	w0, [x29, #-17]
	b	LBB300_3
LBB300_3:
	ldurb	w8, [x29, #-17]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x1
	mov	x8, x0
	sxtw	x1, w8
	adrp	x0, __ZN4main4main17h8bd578f601f0fa06E@PAGE
	add	x0, x0, __ZN4main4main17h8bd578f601f0fa06E@PAGEOFF
	mov	w3, #0
	bl	__ZN3std2rt10lang_start17h3539623d15ccbd4aE
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17he62c7a8a4e5d4ffeE
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hbd0bd907336b8d8eE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h65f6febff2dbe018E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h65f6febff2dbe018E

	.p2align	3, 0x0
l___unnamed_2:
	.quad	__ZN4core3ptr72drop_in_place$LT$std..panicking..begin_panic..Payload$LT$$RF$str$GT$$GT$17hd6b5ec900259bc94E
	.asciz	"\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN91_$LT$std..panicking..begin_panic..Payload$LT$A$GT$$u20$as$u20$core..panic..PanicPayload$GT$8take_box17hac5a8a1f2265d565E
	.quad	__ZN91_$LT$std..panicking..begin_panic..Payload$LT$A$GT$$u20$as$u20$core..panic..PanicPayload$GT$3get17hf57440273286eec3E

	.section	__TEXT,__const
l___unnamed_3:
	.ascii	"unsafe precondition(s) violated: ptr::write_bytes requires that the destination pointer is aligned and non-null"

l___unnamed_4:
	.ascii	"is_nonoverlapping: `size_of::<T>() * count` overflows a usize"

l___unnamed_5:
	.ascii	"unsafe precondition(s) violated: ptr::copy_nonoverlapping requires that both pointer arguments are aligned and non-null and the specified memory ranges do not overlap"

l___unnamed_98:
	.ascii	"is_aligned_to: align is not a power-of-two"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_6:
	.quad	l___unnamed_98
	.asciz	"*\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l___unnamed_8:
	.byte	0

	.section	__TEXT,__literal16,16byte_literals
	.p2align	3, 0x0
l___unnamed_7:
	.space	8
	.space	8

	.section	__TEXT,__const
l___unnamed_99:
	.ascii	"/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ptr/const_ptr.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_9:
	.quad	l___unnamed_99
	.asciz	"Q\000\000\000\000\000\000\000b\006\000\000\r\000\000"

	.p2align	3, 0x0
l___unnamed_10:
	.quad	__ZN4core3ptr30drop_in_place$LT$$RF$usize$GT$17hdfbb0672aebe3775E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h07d103376f470b26E

	.p2align	3, 0x0
l___unnamed_11:
	.quad	__ZN4core3ptr50drop_in_place$LT$$RF$main..vm_util..StackValue$GT$17h73a9f2651e3f0c04E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h919a85546103f853E

	.p2align	3, 0x0
l___unnamed_12:
	.quad	__ZN4core3ptr58drop_in_place$LT$$RF$main..instructions..InstructionV2$GT$17hf58833b6e8a351a3E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hd33992b29ccb3899E

	.section	__TEXT,__const
l___unnamed_100:
	.ascii	"invalid args"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_13:
	.quad	l___unnamed_100
	.asciz	"\f\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_101:
	.ascii	"/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/fmt/mod.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_14:
	.quad	l___unnamed_101
	.asciz	"K\000\000\000\000\000\000\000U\001\000\000\r\000\000"

	.p2align	3, 0x0
l___unnamed_15:
	.quad	l___unnamed_101
	.asciz	"K\000\000\000\000\000\000\000K\001\000\000\r\000\000"

	.section	__TEXT,__const
l___unnamed_16:
	.ascii	"unsafe precondition(s) violated: ptr::read_volatile requires that the pointer argument is aligned and non-null"

l___unnamed_17:
	.ascii	"unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null"

l___unnamed_18:
	.ascii	"unsafe precondition(s) violated: hint::assert_unchecked must never be called when the condition is false"

l___unnamed_102:
	.ascii	"/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/alloc/layout.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_19:
	.quad	l___unnamed_102
	.asciz	"P\000\000\000\000\000\000\000\301\001\000\000)\000\000"

	.section	__TEXT,__const
	.p2align	4, 0x0
_str.0:
	.ascii	"attempt to divide by zero"

l___unnamed_103:
	.ascii	"/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/intrinsics.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_21:
	.quad	l___unnamed_103
	.asciz	"N\000\000\000\000\000\000\000\n\013\000\0006\000\000"

	.section	__TEXT,__const
l___unnamed_20:
	.ascii	"unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`"

l___unnamed_104:
	.ascii	"/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/ops/bit.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_22:
	.quad	l___unnamed_104
	.asciz	"K\000\000\000\000\000\000\000f\002\000\000\001\000\000"

	.section	__TEXT,__const
	.p2align	4, 0x0
_str.1:
	.ascii	"attempt to shift right with overflow"

	.section	__TEXT,__literal16,16byte_literals
	.p2align	3, 0x0
l___unnamed_23:
	.ascii	"\001\000\000\000\000\000\000\200"
	.space	8

	.section	__TEXT,__const
	.p2align	4, 0x0
_str.2:
	.ascii	"attempt to add with overflow"

	.p2align	4, 0x0
_str.3:
	.ascii	"attempt to divide with overflow"

	.p2align	4, 0x0
_str.4:
	.ascii	"attempt to multiply with overflow"

	.p2align	4, 0x0
_str.5:
	.ascii	"attempt to subtract with overflow"

l___unnamed_24:
	.ascii	"unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_25:
	.quad	__ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h133c57e7d33347b9E
	.asciz	"\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h3d711e262771b209E

	.section	__TEXT,__const
l___unnamed_26:
	.ascii	"Expected src"

l___unnamed_105:
	.ascii	"src/instructions.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_27:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000:\000\000\000\022\000\000"

	.section	__TEXT,__const
l___unnamed_28:
	.ascii	"Expected dst"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_29:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000B\000\000\000\022\000\000"

	.section	__TEXT,__const
l___unnamed_30:
	.ascii	"Expected instrs_amount"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_31:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000J\000\000\000\022\000\000"

	.section	__TEXT,__const
l___unnamed_32:
	.ascii	"Expected instr_ptr"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_33:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000R\000\000\000\022\000\000"

	.section	__TEXT,__literal16,16byte_literals
l___unnamed_34:
	.ascii	"Expected jmp_pos"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_35:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000Z\000\000\000\022\000\000"

	.section	__TEXT,__const
l___unnamed_36:
	.ascii	"Expected stack_ptr"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_37:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000b\000\000\000\022\000\000"

	.section	__TEXT,__const
l___unnamed_38:
	.ascii	"Expected args_count"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_39:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000j\000\000\000\022\000\000"

	.p2align	3, 0x0
l___unnamed_40:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000\007\001\000\000\032\000\000"

	.p2align	3, 0x0
l___unnamed_41:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\0007\001\000\000 \000\000"

	.p2align	3, 0x0
l___unnamed_42:
	.quad	l___unnamed_105
	.asciz	"\023\000\000\000\000\000\000\000;\001\000\000\032\000\000"

	.section	__TEXT,__const
l___unnamed_106:
	.ascii	"instr_data"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_43:
	.quad	l___unnamed_106
	.asciz	"\n\000\000\000\000\000\000"

	.section	__TEXT,__const
l___unnamed_107:
	.ascii	"src/vm_util.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_46:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\212\000\000\000A\000\000"

	.p2align	3, 0x0
l___unnamed_47:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\213\000\000\000A\000\000"

	.section	__TEXT,__const
l___unnamed_108:
	.ascii	"Addition not defined for "

l___unnamed_109:
	.ascii	" and "

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_44:
	.quad	l___unnamed_108
	.asciz	"\031\000\000\000\000\000\000"
	.quad	l___unnamed_109
	.asciz	"\005\000\000\000\000\000\000"

	.p2align	3, 0x0
l___unnamed_45:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\214\000\000\000\022\000\000"

	.p2align	3, 0x0
l___unnamed_50:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\223\000\000\000A\000\000"

	.p2align	3, 0x0
l___unnamed_51:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\224\000\000\000A\000\000"

	.section	__TEXT,__const
l___unnamed_110:
	.ascii	"Subtraction not defined for "

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_48:
	.quad	l___unnamed_110
	.asciz	"\034\000\000\000\000\000\000"
	.quad	l___unnamed_109
	.asciz	"\005\000\000\000\000\000\000"

	.p2align	3, 0x0
l___unnamed_49:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\225\000\000\000\022\000\000"

	.p2align	3, 0x0
l___unnamed_54:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\234\000\000\000A\000\000"

	.p2align	3, 0x0
l___unnamed_55:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\235\000\000\000A\000\000"

	.section	__TEXT,__const
l___unnamed_111:
	.ascii	"Mul not defined for "

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_52:
	.quad	l___unnamed_111
	.asciz	"\024\000\000\000\000\000\000"
	.quad	l___unnamed_109
	.asciz	"\005\000\000\000\000\000\000"

	.p2align	3, 0x0
l___unnamed_53:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\236\000\000\000\022\000\000"

	.p2align	3, 0x0
l___unnamed_58:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\245\000\000\000A\000\000"

	.p2align	3, 0x0
l___unnamed_59:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\246\000\000\000A\000\000"

	.section	__TEXT,__const
l___unnamed_112:
	.ascii	"Div not defined for "

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_56:
	.quad	l___unnamed_112
	.asciz	"\024\000\000\000\000\000\000"
	.quad	l___unnamed_109
	.asciz	"\005\000\000\000\000\000\000"

	.p2align	3, 0x0
l___unnamed_57:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\247\000\000\000\022\000\000"

	.section	__TEXT,__const
l___unnamed_113:
	.ascii	"CmpLt not defined for "

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_60:
	.quad	l___unnamed_113
	.asciz	"\026\000\000\000\000\000\000"
	.quad	l___unnamed_109
	.asciz	"\005\000\000\000\000\000\000"

	.p2align	3, 0x0
l___unnamed_61:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\302\000\000\000\022\000\000"

	.p2align	3, 0x0
l___unnamed_62:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\371\000\000\000*\000\000"

	.p2align	3, 0x0
l___unnamed_63:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\000\025\001\000\0003\000\000"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0
l___unnamed_64:
	.asciz	"\020\000\000\000\000\000\000"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_65:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\0001\001\000\000\t\000\000"

	.p2align	3, 0x0
l___unnamed_66:
	.quad	l___unnamed_107
	.asciz	"\016\000\000\000\000\000\000\0007\001\000\000\023\000\000"

	.section	__TEXT,__const
l___unnamed_114:
	.ascii	"src/rules_approach_3.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_67:
	.quad	l___unnamed_114
	.asciz	"\027\000\000\000\000\000\000\000\b\000\000\000\026\000\000"

	.p2align	3, 0x0
l___unnamed_68:
	.quad	l___unnamed_114
	.asciz	"\027\000\000\000\000\000\000\000m\000\000\000\026\000\000"

	.p2align	3, 0x0
l___unnamed_69:
	.quad	l___unnamed_114
	.asciz	"\027\000\000\000\000\000\000\000\225\000\000\000\034\000\000"

	.p2align	3, 0x0
l___unnamed_70:
	.quad	l___unnamed_114
	.asciz	"\027\000\000\000\000\000\000\000\231\000\000\000\026\000\000"

	.section	__TEXT,__const
l___unnamed_115:
	.ascii	"src/main.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_71:
	.quad	l___unnamed_115
	.asciz	"\013\000\000\000\000\000\000\0008\000\000\000\032\000\000"

	.p2align	3, 0x0
l___unnamed_72:
	.quad	l___unnamed_115
	.asciz	"\013\000\000\000\000\000\000\000:\000\000\000\033\000\000"

	.p2align	3, 0x0
l___unnamed_73:
	.quad	l___unnamed_115
	.asciz	"\013\000\000\000\000\000\000\000c\000\000\000\032\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l___unnamed_74:
	.byte	7
	.space	23

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_75:
	.quad	l___unnamed_115
	.asciz	"\013\000\000\000\000\000\000\000\232\000\000\0004\000\000"

	.p2align	3, 0x0
l___unnamed_76:
	.quad	l___unnamed_115
	.asciz	"\013\000\000\000\000\000\000\000\025\001\000\000\032\000\000"

	.p2align	3, 0x0
l___unnamed_77:
	.quad	l___unnamed_115
	.asciz	"\013\000\000\000\000\000\000\000B\001\000\000\032\000\000"

	.section	__TEXT,__const
l___unnamed_116:
	.byte	10

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_78:
	.quad	l___unnamed_8
	.space	8
	.quad	l___unnamed_116
	.asciz	"\001\000\000\000\000\000\000"

	.p2align	3, 0x0
l___unnamed_79:
	.quad	l___unnamed_115
	.asciz	"\013\000\000\000\000\000\000\000\247\003\000\000\t\000\000"

	.p2align	3, 0x0
l___unnamed_80:
	.quad	l___unnamed_115
	.asciz	"\013\000\000\000\000\000\000\000\254\003\000\000\t\000\000"

	.section	__TEXT,__const
l___unnamed_81:
	.ascii	"InstructionV2"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_82:
	.quad	__ZN4core3ptr23drop_in_place$LT$u8$GT$17h1763cab59d4cadfcE
	.asciz	"\001\000\000\000\000\000\000\000\001\000\000\000\000\000\000"
	.quad	__ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h9c2b11a1d0e71c2dE

	.p2align	3, 0x0
l___unnamed_83:
	.quad	__ZN4core3ptr54drop_in_place$LT$$RF$main..instructions..InstrData$GT$17hf66b31b500ac6387E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hbe496b7ef689d0ddE

	.section	__TEXT,__const
l___unnamed_84:
	.ascii	"Int32"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_85:
	.quad	__ZN4core3ptr28drop_in_place$LT$$RF$i32$GT$17h99d783be20b987d1E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h1bb02d7b7e96dff5E

	.section	__TEXT,__const
l___unnamed_86:
	.ascii	"Int64"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_87:
	.quad	__ZN4core3ptr28drop_in_place$LT$$RF$i64$GT$17he1f8dc4ad8c7c841E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h9d648b5bd8239ea2E

	.section	__TEXT,__literal4,4byte_literals
l___unnamed_88:
	.ascii	"Bool"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_89:
	.quad	__ZN4core3ptr29drop_in_place$LT$$RF$bool$GT$17hf9c10089fe597c86E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h4eb1fabeb362c295E

	.section	__TEXT,__const
l___unnamed_91:
	.ascii	"Stack"

l___unnamed_92:
	.ascii	"stack"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_93:
	.quad	__ZN4core3ptr69drop_in_place$LT$alloc..vec..Vec$LT$main..vm_util..StackValue$GT$$GT$17hc496bbea7a6bec5eE
	.asciz	"\030\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17h10da48dbecf85df7E

	.section	__TEXT,__const
l___unnamed_94:
	.ascii	"stack_frames"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_90:
	.quad	__ZN4core3ptr53drop_in_place$LT$$RF$alloc..vec..Vec$LT$usize$GT$$GT$17hf3708de75d14da17E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h04d0284cf44c688bE

	.section	__TEXT,__const
l___unnamed_95:
	.ascii	"FnPtr"

l___unnamed_96:
	.ascii	"Value"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_97:
	.quad	__ZN4core3ptr45drop_in_place$LT$$RF$main..vm_util..Value$GT$17h84350351536a9094E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h33c2319c71cf6599E

.subsections_via_symbols
