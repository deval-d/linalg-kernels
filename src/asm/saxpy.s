.section __TEXT,__text,regular,pure_instructions
	.p2align	2
saxpy_asm::saxpy:
Lfunc_begin5:
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	cmp x1, x3
	b.ne LBB5_16
	cbz x1, LBB5_15
	cmp x1, #3
	b.hi LBB5_4
	mov x8, #0
	b LBB5_13
LBB5_4:
	cmp x1, #16
	b.hs LBB5_6
	mov x8, #0
	b LBB5_10
LBB5_6:
	and x8, x1, #0x1ffffffffffffff0
	add x9, x0, #32
	add x10, x2, #32
	movi.4s v0, #64, lsl #24
	mov x11, x8
LBB5_7:
	ldp q1, q2, [x9, #-32]
	ldp q3, q4, [x9], #64
	ldp q5, q6, [x10, #-32]
	ldp q7, q16, [x10]
	fmla.4s v5, v0, v1
	fmla.4s v6, v0, v2
	fmla.4s v7, v0, v3
	fmla.4s v16, v0, v4
	stp q5, q6, [x10, #-32]
	stp q7, q16, [x10], #64
	subs x11, x11, #16
	b.ne LBB5_7
	cmp x1, x8
	b.eq LBB5_15
	tst x1, #0xc
	b.eq LBB5_13
LBB5_10:
	mov x10, x8
	and x8, x1, #0x1ffffffffffffffc
	sub x9, x10, x8
	lsl x11, x10, #2
	add x10, x2, x11
	add x11, x0, x11
	movi.4s v0, #64, lsl #24
LBB5_11:
	ldr q1, [x11], #16
	ldr q2, [x10]
	fmla.4s v2, v0, v1
	str q2, [x10], #16
	adds x9, x9, #4
	b.ne LBB5_11
	cmp x1, x8
	b.eq LBB5_15
LBB5_13:
	lsl x10, x8, #2
	add x9, x2, x10
	add x10, x0, x10
	sub x8, x1, x8
	fmov s0, #2.00000000
LBB5_14:
	ldr s1, [x10], #4
	ldr s2, [x9]
	fmadd s1, s1, s0, s2
	str s1, [x9], #4
	subs x8, x8, #1
	b.ne LBB5_14
LBB5_15:
	.cfi_def_cfa wsp, 16
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB5_16:
	.cfi_restore_state
Lloh6:
	adrp x0, l_anon.e2e6a2c20164fb1851ad220beafa7444.1@PAGE
Lloh7:
	add x0, x0, l_anon.e2e6a2c20164fb1851ad220beafa7444.1@PAGEOFF
Lloh8:
	adrp x2, l_anon.e2e6a2c20164fb1851ad220beafa7444.3@PAGE
Lloh9:
	add x2, x2, l_anon.e2e6a2c20164fb1851ad220beafa7444.3@PAGEOFF
	mov w1, #65
	bl core::panicking::panic_fmt
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
