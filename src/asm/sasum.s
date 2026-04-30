	.globl	asm_testing::main
	.p2align	2
asm_testing::main:
Lfunc_begin4:
	.cfi_startproc
	sub sp, sp, #48
	.cfi_def_cfa_offset 48
	stp x20, x19, [sp, #16]
	stp x29, x30, [sp, #32]
	add x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_remember_state
	bl __rustc::__rust_no_alloc_shim_is_unstable_v2
	mov w0, #4096
	mov w1, #4
	bl __rustc::__rust_alloc
	cbz x0, LBB4_2
Lloh2:
	adrp x1, l_.memset_pattern@PAGE
Lloh3:
	add x1, x1, l_.memset_pattern@PAGEOFF
	mov x19, x0
	mov w2, #4092
	bl _memset_pattern16
	mov w8, #1065353216
	str w8, [x19, #4092]
	mov x0, x19
	mov w1, #1024
	bl asm_testing::sasum
	str s0, [sp, #12]
	add x8, sp, #12
	; InlineAsm Start
	; InlineAsm End
	mov x0, x19
	mov w1, #4096
	mov w2, #4
	bl __rustc::__rust_dealloc
	.cfi_def_cfa wsp, 48
	ldp x29, x30, [sp, #32]
	ldp x20, x19, [sp, #16]
	add sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	ret
LBB4_2:
	.cfi_restore_state
	mov w0, #4
	mov w1, #4096
	bl alloc::raw_vec::handle_error
	.loh AdrpAdd	Lloh2, Lloh3
Lfunc_end4:
	.cfi_endproc

	.p2align	2
asm_testing::sasum:
Lfunc_begin5:
	.cfi_startproc
	stp d9, d8, [sp, #-16]!
	.cfi_def_cfa_offset 16
	.cfi_offset b8, -8
	.cfi_offset b9, -16
	lsl x9, x1, #2
	movi.2d v6, #0000000000000000
	ands x8, x9, #0x7fffffffffffff80
	b.eq LBB5_6
	mov x10, x8
	movi.2d v7, #0000000000000000
	mov x11, x0
	movi.2d v5, #0000000000000000
	movi.2d v4, #0000000000000000
	movi.2d v3, #0000000000000000
	movi.2d v2, #0000000000000000
	movi.2d v1, #0000000000000000
	movi.2d v0, #0000000000000000
LBB5_2:
	ldp q17, q16, [x11]
	ldp q18, q19, [x11, #32]
	ldp q20, q21, [x11, #64]
	ldp q22, q23, [x11, #96]
	add x11, x11, #128
	fabs.4s v23, v23
	fabs.4s v22, v22
	fabs.4s v21, v21
	fabs.4s v20, v20
	fabs.4s v19, v19
	fabs.4s v17, v17
	fabs.4s v16, v16
	fabs.4s v18, v18
	fadd.4s v5, v5, v18
	fadd.4s v7, v7, v16
	fadd.4s v6, v6, v17
	fadd.4s v4, v4, v19
	fadd.4s v3, v3, v20
	fadd.4s v2, v2, v21
	fadd.4s v1, v1, v22
	fadd.4s v0, v0, v23
	subs x10, x10, #128
	b.ne LBB5_2
	ands x9, x9, #0x7c
	b.eq LBB5_7
LBB5_4:
	and x10, x1, #0x1fffffffffffffe0
	add x10, x0, x10, lsl #2
	sub x12, x9, #4
	cmp x12, #12
	b.hs LBB5_8
	movi d16, #0000000000000000
	mov x13, x10
	b LBB5_18
LBB5_6:
	movi.2d v7, #0000000000000000
	movi.2d v5, #0000000000000000
	movi.2d v4, #0000000000000000
	movi.2d v3, #0000000000000000
	movi.2d v2, #0000000000000000
	movi.2d v1, #0000000000000000
	movi.2d v0, #0000000000000000
	ands x9, x9, #0x7c
	b.ne LBB5_4
LBB5_7:
	movi d16, #0000000000000000
	b LBB5_20
LBB5_8:
	lsr x11, x12, #2
	add x11, x11, #1
	cmp x12, #60
	b.hs LBB5_10
	mov x12, #0
	movi d16, #0000000000000000
	b LBB5_14
LBB5_10:
	and x12, x11, #0x7ffffffffffffff0
	add x13, x8, x0
	add x13, x13, #32
	movi d16, #0000000000000000
	mov x14, x12
LBB5_11:
	ldp q17, q18, [x13, #-32]
	ldp q19, q20, [x13], #64
	fabs.4s v17, v17
	mov s21, v17[3]
	mov s22, v17[2]
	mov s23, v17[1]
	fabs.4s v18, v18
	mov s24, v18[3]
	mov s25, v18[2]
	mov s26, v18[1]
	fabs.4s v19, v19
	mov s27, v19[3]
	mov s28, v19[2]
	mov s29, v19[1]
	fabs.4s v20, v20
	mov s30, v20[3]
	mov s31, v20[2]
	mov s8, v20[1]
	fadd s16, s16, s17
	fadd s16, s16, s23
	fadd s16, s16, s22
	fadd s16, s16, s21
	fadd s16, s16, s18
	fadd s16, s16, s26
	fadd s16, s16, s25
	fadd s16, s16, s24
	fadd s16, s16, s19
	fadd s16, s16, s29
	fadd s16, s16, s28
	fadd s16, s16, s27
	fadd s16, s16, s20
	fadd s16, s16, s8
	fadd s16, s16, s31
	fadd s16, s16, s30
	subs x14, x14, #16
	b.ne LBB5_11
	cmp x11, x12
	b.eq LBB5_20
	tst x11, #0xc
	b.eq LBB5_17
LBB5_14:
	and x14, x11, #0x7ffffffffffffffc
	add x13, x10, x14, lsl #2
	sub x15, x12, x14
	add x8, x8, x12, lsl #2
	add x8, x0, x8
LBB5_15:
	ldr q17, [x8], #16
	fabs.4s v17, v17
	mov s18, v17[3]
	mov s19, v17[2]
	mov s20, v17[1]
	fadd s16, s16, s17
	fadd s16, s16, s20
	fadd s16, s16, s19
	fadd s16, s16, s18
	adds x15, x15, #4
	b.ne LBB5_15
	cmp x11, x14
	b.ne LBB5_18
	b LBB5_20
LBB5_17:
	add x13, x10, x12, lsl #2
LBB5_18:
	add x8, x10, x9
LBB5_19:
	ldr s17, [x13], #4
	fabs s17, s17
	fadd s16, s16, s17
	cmp x13, x8
	b.ne LBB5_19
LBB5_20:
	mov s17, v6[2]
	faddp.2s s18, v6
	fadd s17, s18, s17
	mov s6, v6[3]
	fadd s6, s17, s6
	fadd s6, s6, s7
	mov s17, v7[1]
	fadd s6, s6, s17
	mov s17, v7[2]
	fadd s6, s6, s17
	mov s7, v7[3]
	fadd s6, s6, s7
	fadd s6, s6, s5
	mov s7, v5[1]
	fadd s6, s6, s7
	mov s7, v5[2]
	fadd s6, s6, s7
	mov s5, v5[3]
	fadd s5, s6, s5
	fadd s5, s5, s4
	mov s6, v4[1]
	fadd s5, s5, s6
	mov s6, v4[2]
	fadd s5, s5, s6
	mov s4, v4[3]
	fadd s4, s5, s4
	fadd s4, s4, s3
	mov s5, v3[1]
	fadd s4, s4, s5
	mov s5, v3[2]
	fadd s4, s4, s5
	mov s3, v3[3]
	fadd s3, s4, s3
	fadd s3, s3, s2
	mov s4, v2[1]
	fadd s3, s3, s4
	mov s4, v2[2]
	fadd s3, s3, s4
	mov s2, v2[3]
	fadd s2, s3, s2
	fadd s2, s2, s1
	mov s3, v1[1]
	fadd s2, s2, s3
	mov s3, v1[2]
	fadd s2, s2, s3
	mov s1, v1[3]
	fadd s1, s2, s1
	fadd s1, s1, s0
	mov s2, v0[1]
	fadd s1, s1, s2
	mov s2, v0[2]
	fadd s1, s1, s2
	mov s0, v0[3]
	fadd s0, s1, s0
	fadd s0, s0, s16
	ldp d9, d8, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore b8
	.cfi_restore b9
	ret
