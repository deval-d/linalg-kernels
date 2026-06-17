.section __TEXT,__text,regular,pure_instructions
	.p2align	2
sdot_asm::sdot:
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
	b.ne LBB5_10
	movi.2d v6, #0000000000000000
	cmp x1, #32
	b.hs LBB5_3
	movi.2d v7, #0000000000000000
	movi.2d v5, #0000000000000000
	movi.2d v4, #0000000000000000
	movi.2d v3, #0000000000000000
	movi.2d v2, #0000000000000000
	movi.2d v1, #0000000000000000
	movi.2d v0, #0000000000000000
	b LBB5_5
LBB5_3:
	lsr x8, x1, #5
	movi.2d v7, #0000000000000000
	mov x9, x0
	mov x10, x2
	movi.2d v5, #0000000000000000
	movi.2d v4, #0000000000000000
	movi.2d v3, #0000000000000000
	movi.2d v2, #0000000000000000
	movi.2d v1, #0000000000000000
	movi.2d v0, #0000000000000000
LBB5_4:
	ldp q17, q16, [x9, #64]
	ldp q19, q18, [x9, #32]
	ldp q20, q21, [x9]
	ldp q23, q22, [x10, #32]
	ldp q24, q25, [x10]
	fmla.4s v7, v25, v21
	fmla.4s v6, v24, v20
	fmla.4s v5, v23, v19
	ldp q20, q19, [x10, #64]
	fmla.4s v4, v22, v18
	fmla.4s v3, v20, v17
	fmla.4s v2, v19, v16
	ldp q17, q16, [x9, #96]
	ldp q19, q18, [x10, #96]
	fmla.4s v1, v19, v17
	fmla.4s v0, v18, v16
	add x10, x10, #128
	add x9, x9, #128
	subs x8, x8, #1
	b.ne LBB5_4
LBB5_5:
	ands x8, x1, #0x1f
	b.eq LBB5_8
	lsl x9, x1, #2
	and x10, x9, #0x7fffffffffffff80
	add x9, x0, x10
	add x10, x2, x10
	movi d16, #0000000000000000
LBB5_7:
	ldr s17, [x9], #4
	ldr s18, [x10], #4
	fmadd s16, s17, s18, s16
	subs x8, x8, #1
	b.ne LBB5_7
	b LBB5_9
LBB5_8:
	movi d16, #0000000000000000
LBB5_9:
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
	.cfi_def_cfa wsp, 16
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB5_10:
	.cfi_restore_state
Lloh6:
	adrp x0, l_anon.0b97df70fb62afaf5dedca7c962fa45f.1@PAGE
Lloh7:
	add x0, x0, l_anon.0b97df70fb62afaf5dedca7c962fa45f.1@PAGEOFF
Lloh8:
	adrp x2, l_anon.0b97df70fb62afaf5dedca7c962fa45f.3@PAGE
Lloh9:
	add x2, x2, l_anon.0b97df70fb62afaf5dedca7c962fa45f.3@PAGEOFF
	mov w1, #65
	bl core::panicking::panic_fmt
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
