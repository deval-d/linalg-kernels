.section __TEXT,__text,regular,pure_instructions
	.p2align	2
sgemv_asm::sgemv_n:
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
	mov w8, #1024
	stp x8, x8, [sp]
	mov w8, #1048576
	stp x1, x8, [sp, #16]
	cmp x1, #256, lsl #12
	b.ne LBB5_30
	cmp x3, #1024
	b.ne LBB5_31
	cmp x5, #1024
	b.ne LBB5_32
	add x8, x4, #32
	mov w9, #1024
	fmov.4s v0, #3.00000000
LBB5_4:
	ldp q1, q2, [x8, #-32]
	ldp q3, q4, [x8]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x8, #-32]
	stp q3, q4, [x8], #64
	subs x9, x9, #16
	b.ne LBB5_4
	mov w13, #8
	mov w14, #1024
	b LBB5_7
LBB5_6:
	add x0, x0, #128, lsl #12
	mov x9, x15
	sub x13, x13, #1
	cbz x13, LBB5_27
LBB5_7:
	add x15, x9, #128
	cmp x15, #1024
	csel x10, x15, x14, lo
	lsl x11, x9, #10
	lsl x1, x10, #10
	subs x8, x1, x11
	b.lo LBB5_28
	sub x1, x10, x9
	stp x14, x1, [sp]
	lsl x11, x1, #10
	stp x8, x11, [sp, #16]
	cmp x11, x8
	b.ne LBB5_30
	cmp x9, #1025
	b.hs LBB5_29
	cmp x9, #1024
	b.eq LBB5_6
	mov x16, #0
	mov x17, #0
	lsr x3, x1, #2
	mov x5, x0
	mov w10, #1
	add x6, x2, x9, lsl #2
LBB5_12:
	lsl x9, x17, #2
	cmp x9, x1
	b.hs LBB5_35
	mov x17, x10
	orr x10, x9, #0x1
	cmp x10, x1
	b.hs LBB5_36
	orr x11, x9, #0x2
	cmp x11, x1
	b.hs LBB5_33
	orr x12, x9, #0x3
	cmp x12, x1
	b.hs LBB5_34
	mov x7, #0
	ldr s0, [x6, x9, lsl #2]
	fadd s0, s0, s0
	ldr s1, [x6, x10, lsl #2]
	fadd s1, s1, s1
	ldr s2, [x6, x11, lsl #2]
	fadd s2, s2, s2
	ldr s3, [x6, x12, lsl #2]
	fadd s3, s3, s3
	mov x10, x5
	mov x11, x4
LBB5_17:
	add x9, x16, x7
	add x12, x9, #63
	cmp x12, x8
	b.hs LBB5_23
	add x9, x9, #1087
	cmp x9, x8
	b.hs LBB5_24
	add x9, x16, x7
	add x12, x9, #2111
	cmp x12, x8
	b.hs LBB5_25
	add x9, x9, #3135
	cmp x9, x8
	b.hs LBB5_26
	ldp q27, q26, [x10, #192]
	ldp q24, q28, [x10, #160]
	ldp q22, q25, [x10, #128]
	ldp q20, q23, [x10, #96]
	ldp q18, q21, [x10, #64]
	ldp q16, q19, [x10, #32]
	ldp q17, q5, [x10]
	ldp q4, q6, [x11]
	fmla.4s v6, v5, v0[0]
	ldp q7, q5, [x11, #32]
	fmla.4s v4, v17, v0[0]
	fmla.4s v7, v16, v0[0]
	ldp q17, q16, [x11, #64]
	fmla.4s v5, v19, v0[0]
	fmla.4s v17, v18, v0[0]
	ldp q19, q18, [x11, #96]
	fmla.4s v16, v21, v0[0]
	fmla.4s v19, v20, v0[0]
	ldp q21, q20, [x11, #128]
	fmla.4s v18, v23, v0[0]
	fmla.4s v21, v22, v0[0]
	ldp q23, q22, [x11, #160]
	fmla.4s v20, v25, v0[0]
	fmla.4s v23, v24, v0[0]
	ldp q25, q24, [x11, #192]
	fmla.4s v22, v28, v0[0]
	fmla.4s v25, v27, v0[0]
	ldp q27, q28, [x10, #224]
	fmla.4s v24, v26, v0[0]
	ldr q26, [x11, #224]
	fmla.4s v26, v27, v0[0]
	ldr q27, [x11, #240]
	fmla.4s v27, v28, v0[0]
	ldr q28, [x10, #4336]
	fmla.4s v27, v28, v1[0]
	ldr q28, [x10, #4320]
	fmla.4s v26, v28, v1[0]
	ldr q28, [x10, #4304]
	fmla.4s v24, v28, v1[0]
	ldr q28, [x10, #4288]
	fmla.4s v25, v28, v1[0]
	ldr q28, [x10, #4272]
	fmla.4s v22, v28, v1[0]
	ldr q28, [x10, #4256]
	fmla.4s v23, v28, v1[0]
	ldr q28, [x10, #4240]
	fmla.4s v20, v28, v1[0]
	ldr q28, [x10, #4224]
	fmla.4s v21, v28, v1[0]
	ldr q28, [x10, #4208]
	fmla.4s v18, v28, v1[0]
	ldr q28, [x10, #4192]
	fmla.4s v19, v28, v1[0]
	ldr q28, [x10, #4176]
	fmla.4s v16, v28, v1[0]
	ldr q28, [x10, #4160]
	fmla.4s v17, v28, v1[0]
	ldr q28, [x10, #4144]
	fmla.4s v5, v28, v1[0]
	ldr q28, [x10, #4128]
	fmla.4s v7, v28, v1[0]
	ldr q28, [x10, #4096]
	fmla.4s v4, v28, v1[0]
	ldr q28, [x10, #4112]
	fmla.4s v6, v28, v1[0]
	ldr q28, [x10, #8208]
	fmla.4s v6, v28, v2[0]
	ldr q28, [x10, #8192]
	fmla.4s v4, v28, v2[0]
	ldr q28, [x10, #8224]
	fmla.4s v7, v28, v2[0]
	ldr q28, [x10, #8240]
	fmla.4s v5, v28, v2[0]
	ldr q28, [x10, #8256]
	fmla.4s v17, v28, v2[0]
	ldr q28, [x10, #8272]
	fmla.4s v16, v28, v2[0]
	ldr q28, [x10, #8288]
	fmla.4s v19, v28, v2[0]
	ldr q28, [x10, #8304]
	fmla.4s v18, v28, v2[0]
	ldr q28, [x10, #8320]
	fmla.4s v21, v28, v2[0]
	ldr q28, [x10, #8336]
	fmla.4s v20, v28, v2[0]
	ldr q28, [x10, #8352]
	fmla.4s v23, v28, v2[0]
	ldr q28, [x10, #8368]
	fmla.4s v22, v28, v2[0]
	ldr q28, [x10, #8384]
	fmla.4s v25, v28, v2[0]
	ldr q28, [x10, #8400]
	fmla.4s v24, v28, v2[0]
	ldr q28, [x10, #8416]
	fmla.4s v26, v28, v2[0]
	ldr q28, [x10, #8432]
	fmla.4s v27, v28, v2[0]
	ldr q28, [x10, #12528]
	fmla.4s v27, v28, v3[0]
	ldr q28, [x10, #12512]
	fmla.4s v26, v28, v3[0]
	ldr q28, [x10, #12496]
	fmla.4s v24, v28, v3[0]
	ldr q28, [x10, #12480]
	fmla.4s v25, v28, v3[0]
	ldr q28, [x10, #12464]
	fmla.4s v22, v28, v3[0]
	ldr q28, [x10, #12448]
	fmla.4s v23, v28, v3[0]
	ldr q28, [x10, #12432]
	fmla.4s v20, v28, v3[0]
	ldr q28, [x10, #12416]
	fmla.4s v21, v28, v3[0]
	ldr q28, [x10, #12400]
	fmla.4s v18, v28, v3[0]
	ldr q28, [x10, #12384]
	fmla.4s v19, v28, v3[0]
	ldr q28, [x10, #12368]
	fmla.4s v16, v28, v3[0]
	ldr q28, [x10, #12352]
	fmla.4s v17, v28, v3[0]
	ldr q28, [x10, #12336]
	fmla.4s v5, v28, v3[0]
	ldr q28, [x10, #12320]
	fmla.4s v7, v28, v3[0]
	ldr q28, [x10, #12288]
	fmla.4s v4, v28, v3[0]
	ldr q28, [x10, #12304]
	fmla.4s v6, v28, v3[0]
	stp q4, q6, [x11]
	stp q7, q5, [x11, #32]
	stp q17, q16, [x11, #64]
	stp q19, q18, [x11, #96]
	stp q21, q20, [x11, #128]
	stp q23, q22, [x11, #160]
	stp q25, q24, [x11, #192]
	stp q26, q27, [x11, #224]
	add x7, x7, #64
	add x11, x11, #256
	add x10, x10, #256
	cmp x7, #1024
	b.ne LBB5_17
	add x16, x16, #1, lsl #12
	add x5, x5, #4, lsl #12
	cmp x17, x3
	cinc x10, x17, lo
	b.lo LBB5_12
	b LBB5_6
LBB5_23:
Lloh8:
	adrp x3, l_anon.6d029413f5d07e6592737053f2658f24.14@PAGE
Lloh9:
	add x3, x3, l_anon.6d029413f5d07e6592737053f2658f24.14@PAGEOFF
	add x1, x9, #64
	mov x0, x9
	mov x2, x8
	bl core::slice::index::slice_index_fail
LBB5_24:
	add x9, x16, x7
Lloh10:
	adrp x3, l_anon.6d029413f5d07e6592737053f2658f24.13@PAGE
Lloh11:
	add x3, x3, l_anon.6d029413f5d07e6592737053f2658f24.13@PAGEOFF
	add x0, x9, #1024
	add x1, x9, #1088
	mov x2, x8
	bl core::slice::index::slice_index_fail
LBB5_25:
Lloh12:
	adrp x3, l_anon.6d029413f5d07e6592737053f2658f24.12@PAGE
Lloh13:
	add x3, x3, l_anon.6d029413f5d07e6592737053f2658f24.12@PAGEOFF
	add x0, x9, #2048
	add x1, x9, #2112
	mov x2, x8
	bl core::slice::index::slice_index_fail
LBB5_26:
	add x9, x16, x7
Lloh14:
	adrp x3, l_anon.6d029413f5d07e6592737053f2658f24.11@PAGE
Lloh15:
	add x3, x3, l_anon.6d029413f5d07e6592737053f2658f24.11@PAGEOFF
	add x0, x9, #3072
	add x1, x9, #3136
	mov x2, x8
	bl core::slice::index::slice_index_fail
LBB5_27:
	.cfi_def_cfa wsp, 112
	ldp x29, x30, [sp, #96]
	add sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB5_28:
	.cfi_restore_state
Lloh16:
	adrp x3, l_anon.6d029413f5d07e6592737053f2658f24.19@PAGE
Lloh17:
	add x3, x3, l_anon.6d029413f5d07e6592737053f2658f24.19@PAGEOFF
	mov x0, x11
	mov w2, #1048576
	bl core::slice::index::slice_index_fail
LBB5_29:
Lloh18:
	adrp x3, l_anon.6d029413f5d07e6592737053f2658f24.16@PAGE
Lloh19:
	add x3, x3, l_anon.6d029413f5d07e6592737053f2658f24.16@PAGEOFF
	mov x0, x9
	mov x1, x10
	mov w2, #1024
	bl core::slice::index::slice_index_fail
LBB5_30:
Lloh20:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh21:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	add x9, sp, #16
	stp x9, x8, [sp, #32]
	mov x9, sp
	stp x9, x8, [sp, #48]
	add x9, sp, #8
	stp x9, x8, [sp, #64]
	add x9, sp, #24
	stp x9, x8, [sp, #80]
Lloh22:
	adrp x3, l_anon.6d029413f5d07e6592737053f2658f24.17@PAGE
Lloh23:
	add x3, x3, l_anon.6d029413f5d07e6592737053f2658f24.17@PAGEOFF
Lloh24:
	adrp x5, l_anon.6d029413f5d07e6592737053f2658f24.18@PAGE
Lloh25:
	add x5, x5, l_anon.6d029413f5d07e6592737053f2658f24.18@PAGEOFF
	add x1, sp, #24
	add x2, sp, #16
	add x4, sp, #32
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
LBB5_31:
Lloh26:
	adrp x0, l_anon.6d029413f5d07e6592737053f2658f24.1@PAGE
Lloh27:
	add x0, x0, l_anon.6d029413f5d07e6592737053f2658f24.1@PAGEOFF
Lloh28:
	adrp x2, l_anon.6d029413f5d07e6592737053f2658f24.3@PAGE
Lloh29:
	add x2, x2, l_anon.6d029413f5d07e6592737053f2658f24.3@PAGEOFF
	mov w1, #93
	bl core::panicking::panic_fmt
LBB5_32:
Lloh30:
	adrp x0, l_anon.6d029413f5d07e6592737053f2658f24.4@PAGE
Lloh31:
	add x0, x0, l_anon.6d029413f5d07e6592737053f2658f24.4@PAGEOFF
Lloh32:
	adrp x2, l_anon.6d029413f5d07e6592737053f2658f24.5@PAGE
Lloh33:
	add x2, x2, l_anon.6d029413f5d07e6592737053f2658f24.5@PAGEOFF
	mov w1, #93
	bl core::panicking::panic_fmt
LBB5_33:
Lloh34:
	adrp x2, l_anon.6d029413f5d07e6592737053f2658f24.9@PAGE
Lloh35:
	add x2, x2, l_anon.6d029413f5d07e6592737053f2658f24.9@PAGEOFF
	mov x0, x11
	bl core::panicking::panic_bounds_check
LBB5_34:
Lloh36:
	adrp x2, l_anon.6d029413f5d07e6592737053f2658f24.10@PAGE
Lloh37:
	add x2, x2, l_anon.6d029413f5d07e6592737053f2658f24.10@PAGEOFF
	mov x0, x12
	bl core::panicking::panic_bounds_check
LBB5_35:
Lloh38:
	adrp x2, l_anon.6d029413f5d07e6592737053f2658f24.7@PAGE
Lloh39:
	add x2, x2, l_anon.6d029413f5d07e6592737053f2658f24.7@PAGEOFF
	mov x0, x9
	bl core::panicking::panic_bounds_check
LBB5_36:
Lloh40:
	adrp x2, l_anon.6d029413f5d07e6592737053f2658f24.8@PAGE
Lloh41:
	add x2, x2, l_anon.6d029413f5d07e6592737053f2658f24.8@PAGEOFF
	mov x0, x10
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpLdrGot	Lloh20, Lloh21
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpAdd	Lloh40, Lloh41
