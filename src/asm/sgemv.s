.section __TEXT,__text,regular,pure_instructions
	.p2align	2
sgemv_asm::sgemv:
Lfunc_begin5:
	.cfi_startproc
	sub sp, sp, #112
	.cfi_def_cfa_offset 112
	stp x29, x30, [sp, #96]
	add x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	mov w8, #128
	stp x8, x8, [sp]
	mov w8, #16384
	stp x1, x8, [sp, #16]
	cmp x1, #4, lsl #12
	b.ne LBB5_25
	cmp x3, #128
	b.ne LBB5_26
	cmp x5, #128
	b.ne LBB5_27
	mov x9, #0
	mov x10, #0
	ldp q1, q2, [x4]
	ldp q3, q4, [x4, #32]
	mov w11, #4058
	movk w11, #16457, lsl #16
	dup.4s v0, w11
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x4]
	stp q3, q4, [x4, #32]
	ldp q1, q2, [x4, #64]
	ldp q3, q4, [x4, #96]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x4, #64]
	stp q3, q4, [x4, #96]
	ldp q1, q2, [x4, #128]
	ldp q3, q4, [x4, #160]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x4, #128]
	stp q3, q4, [x4, #160]
	ldp q1, q2, [x4, #192]
	ldp q3, q4, [x4, #224]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x4, #192]
	stp q3, q4, [x4, #224]
	ldp q1, q2, [x4, #256]
	ldp q3, q4, [x4, #288]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x4, #256]
	stp q3, q4, [x4, #288]
	ldp q1, q2, [x4, #320]
	ldp q3, q4, [x4, #352]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x4, #320]
	stp q3, q4, [x4, #352]
	ldp q1, q2, [x4, #384]
	ldp q3, q4, [x4, #416]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x4, #384]
	stp q3, q4, [x4, #416]
	ldp q1, q2, [x4, #448]
	ldp q3, q4, [x4, #480]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v0, v4, v0
	stp q1, q2, [x4, #448]
	add x12, x4, #512
	mov x13, x0
	stp q3, q0, [x4, #480]
LBB5_4:
	lsl x8, x10, #2
	cmp x10, #32
	b.hs LBB5_28
	mov x14, #0
	add x10, x10, #1
	add x8, x2, x8, lsl #2
	fmov s3, w11
	ldp s0, s1, [x8]
	fmul s0, s0, s3
	fmul s1, s1, s3
	ldp s2, s4, [x8, #8]
	fmul s2, s2, s3
	fmul s3, s4, s3
	mov x15, x13
	mov x16, x4
LBB5_6:
	add x8, x9, x14
	cmp x8, #4, lsl #12
	b.hs LBB5_18
	add x17, x8, #128
	cmp x17, #4, lsl #12
	b.hs LBB5_19
	add x17, x8, #256
	cmp x17, #4, lsl #12
	b.hs LBB5_20
	add x8, x8, #384
	cmp x8, #4, lsl #12
	b.hs LBB5_21
	ldp q5, q4, [x15, #32]
	ldp q7, q6, [x15]
	ldp q17, q16, [x15, #544]
	ldp q19, q18, [x15, #512]
	ldr q20, [x15, #1072]
	ldr q21, [x15, #1056]
	ldr q22, [x15, #1040]
	ldr q23, [x15, #1024]
	ldr q24, [x15, #1536]
	ldr q25, [x15, #1552]
	ldr q26, [x15, #1568]
	ldr q27, [x15, #1584]
	ldp q29, q28, [x16, #32]
	ldp q31, q30, [x16]
	fmla.4s v31, v7, v0[0]
	fmla.4s v30, v6, v0[0]
	fmla.4s v29, v5, v0[0]
	fmla.4s v28, v4, v0[0]
	fmla.4s v28, v16, v1[0]
	fmla.4s v29, v17, v1[0]
	fmla.4s v30, v18, v1[0]
	fmla.4s v31, v19, v1[0]
	fmla.4s v31, v23, v2[0]
	fmla.4s v30, v22, v2[0]
	fmla.4s v29, v21, v2[0]
	fmla.4s v28, v20, v2[0]
	fmla.4s v28, v27, v3[0]
	fmla.4s v29, v26, v3[0]
	fmla.4s v30, v25, v3[0]
	fmla.4s v31, v24, v3[0]
	stp q31, q30, [x16]
	stp q29, q28, [x16, #32]
	add x16, x16, #64
	add x14, x14, #16
	add x15, x15, #64
	cmp x14, #128
	b.ne LBB5_6
	add x9, x9, #512
	add x13, x13, #2048
	cmp x10, #32
	b.ne LBB5_4
	add x8, x0, #512
	mov w0, #128
	mov w9, #1
	mov w10, #16512
LBB5_13:
	cbnz wzr, LBB5_24
	cmp x0, #4, lsl #12
	b.hi LBB5_24
	cbz w9, LBB5_22
	add x8, x8, #512
	add x2, x2, #4
	add x0, x0, #128
	cmp x0, x10
	b.ne LBB5_13
	.cfi_def_cfa wsp, 112
	ldp x29, x30, [sp, #96]
	add sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB5_18:
	.cfi_restore_state
Lloh6:
	adrp x3, l_anon.092cb5656245a349b83e0cc004f34217.12@PAGE
Lloh7:
	add x3, x3, l_anon.092cb5656245a349b83e0cc004f34217.12@PAGEOFF
	add x1, x8, #16
	mov x0, x8
	mov w2, #16384
	bl core::slice::index::slice_index_fail
LBB5_19:
	add x8, x9, x14
Lloh8:
	adrp x3, l_anon.092cb5656245a349b83e0cc004f34217.11@PAGE
Lloh9:
	add x3, x3, l_anon.092cb5656245a349b83e0cc004f34217.11@PAGEOFF
	add x0, x8, #128
	add x1, x8, #144
	mov w2, #16384
	bl core::slice::index::slice_index_fail
LBB5_20:
	add x8, x9, x14
Lloh10:
	adrp x3, l_anon.092cb5656245a349b83e0cc004f34217.10@PAGE
Lloh11:
	add x3, x3, l_anon.092cb5656245a349b83e0cc004f34217.10@PAGEOFF
	add x0, x8, #256
	add x1, x8, #272
	mov w2, #16384
	bl core::slice::index::slice_index_fail
LBB5_21:
	add x8, x9, x14
Lloh12:
	adrp x3, l_anon.092cb5656245a349b83e0cc004f34217.9@PAGE
Lloh13:
	add x3, x3, l_anon.092cb5656245a349b83e0cc004f34217.9@PAGEOFF
	add x0, x8, #384
	add x1, x8, #400
	mov w2, #16384
	bl core::slice::index::slice_index_fail
LBB5_22:
	ldr s0, [x2]
	mov w9, #4058
	movk w9, #16457, lsl #16
	fmov s1, w9
	fmul s0, s0, s1
LBB5_23:
	ldr s1, [x8], #4
	ldr s2, [x12]
	fmadd s1, s0, s1, s2
	str s1, [x12], #4
	b LBB5_23
LBB5_24:
Lloh14:
	adrp x3, l_anon.092cb5656245a349b83e0cc004f34217.7@PAGE
Lloh15:
	add x3, x3, l_anon.092cb5656245a349b83e0cc004f34217.7@PAGEOFF
	mov x1, x0
	mov w2, #16384
	bl core::slice::index::slice_index_fail
LBB5_25:
Lloh16:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh17:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	add x9, sp, #16
	stp x9, x8, [sp, #32]
	mov x9, sp
	stp x9, x8, [sp, #48]
	add x9, sp, #8
	stp x9, x8, [sp, #64]
	add x9, sp, #24
	stp x9, x8, [sp, #80]
Lloh18:
	adrp x3, l_anon.092cb5656245a349b83e0cc004f34217.14@PAGE
Lloh19:
	add x3, x3, l_anon.092cb5656245a349b83e0cc004f34217.14@PAGEOFF
Lloh20:
	adrp x5, l_anon.092cb5656245a349b83e0cc004f34217.15@PAGE
Lloh21:
	add x5, x5, l_anon.092cb5656245a349b83e0cc004f34217.15@PAGEOFF
	add x1, sp, #24
	add x2, sp, #16
	add x4, sp, #32
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
LBB5_26:
Lloh22:
	adrp x0, l_anon.092cb5656245a349b83e0cc004f34217.1@PAGE
Lloh23:
	add x0, x0, l_anon.092cb5656245a349b83e0cc004f34217.1@PAGEOFF
Lloh24:
	adrp x2, l_anon.092cb5656245a349b83e0cc004f34217.3@PAGE
Lloh25:
	add x2, x2, l_anon.092cb5656245a349b83e0cc004f34217.3@PAGEOFF
	mov w1, #93
	bl core::panicking::panic_fmt
LBB5_27:
Lloh26:
	adrp x0, l_anon.092cb5656245a349b83e0cc004f34217.4@PAGE
Lloh27:
	add x0, x0, l_anon.092cb5656245a349b83e0cc004f34217.4@PAGEOFF
Lloh28:
	adrp x2, l_anon.092cb5656245a349b83e0cc004f34217.5@PAGE
Lloh29:
	add x2, x2, l_anon.092cb5656245a349b83e0cc004f34217.5@PAGEOFF
	mov w1, #93
	bl core::panicking::panic_fmt
LBB5_28:
Lloh30:
	adrp x2, l_anon.092cb5656245a349b83e0cc004f34217.8@PAGE
Lloh31:
	add x2, x2, l_anon.092cb5656245a349b83e0cc004f34217.8@PAGEOFF
	mov x0, x8
	mov w1, #128
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpLdrGot	Lloh16, Lloh17
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh30, Lloh31
