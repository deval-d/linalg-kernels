.section __TEXT,__text,regular,pure_instructions
	.p2align	2
asm_compare::call_faxpy_struct_kernel_local:
Lfunc_begin12:
	.cfi_startproc
	sub sp, sp, #144
	.cfi_def_cfa_offset 144
	stp x29, x30, [sp, #128]
	add x29, sp, #128
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	stp x0, x0, [sp, #32]
	mul x8, x0, x0
	stp x2, x8, [sp, #48]
	cmp x8, x2
	b.ne LBB12_2
	stp x0, x0, [sp, #16]
	stp x1, x2, [sp]
	mov x0, sp
	mov x1, x3
	mov x2, x4
	mov x3, x5
	mov x4, x6
	bl asm_compare::faxpy_struct_f32_local
	.cfi_def_cfa wsp, 144
	ldp x29, x30, [sp, #128]
	add sp, sp, #144
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB12_2:
	.cfi_restore_state
Lloh102:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh103:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	add x9, sp, #48
	stp x9, x8, [sp, #64]
	add x9, sp, #32
	stp x9, x8, [sp, #80]
	add x9, sp, #40
	stp x9, x8, [sp, #96]
	add x9, sp, #56
	stp x9, x8, [sp, #112]
Lloh104:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.56@PAGE
Lloh105:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.56@PAGEOFF
Lloh106:
	adrp x5, l_anon.6efade69f4e207c800bc74b0e469f61c.57@PAGE
Lloh107:
	add x5, x5, l_anon.6efade69f4e207c800bc74b0e469f61c.57@PAGEOFF
	add x1, sp, #56
	add x2, sp, #48
	add x4, sp, #64
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
	.loh AdrpAdd	Lloh106, Lloh107
	.loh AdrpAdd	Lloh104, Lloh105
	.loh AdrpLdrGot	Lloh102, Lloh103
Lfunc_end12:
	.cfi_endproc

	.p2align	2
asm_compare::faxpy_panel64_struct_f32_local:
Lfunc_begin13:
	.cfi_startproc
	sub sp, sp, #160
	.cfi_def_cfa_offset 160
	stp d9, d8, [sp, #48]
	stp x28, x27, [sp, #64]
	stp x26, x25, [sp, #80]
	stp x24, x23, [sp, #96]
	stp x22, x21, [sp, #112]
	stp x20, x19, [sp, #128]
	stp x29, x30, [sp, #144]
	add x29, sp, #144
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_offset b8, -104
	.cfi_offset b9, -112
	.cfi_remember_state
	str x3, [sp, #40]
	mov x24, x2
	mov x22, x1
	fmov d8, d0
	ldp x21, x20, [x0, #16]
	lsr x26, x21, #6
	lsr x12, x20, #2
	ldp x27, x19, [x0]
	cmp x20, #4
	b.hs LBB13_16
LBB13_1:
	stp x26, x27, [sp, #8]
	stp x24, x4, [sp, #24]
	and x25, x21, #0xffffffffffffffc0
	and x8, x20, #0xfffffffffffffffc
	cmp x8, x20
	b.eq LBB13_8
	ldr x2, [sp, #32]
	cmp x25, x2
	b.hi LBB13_41
	ldp x10, x9, [sp, #16]
	cmp x9, x8
	csel x27, x9, x8, hi
	mul x9, x21, x12
	lsl x26, x9, #2
	add x28, x10, x9, lsl #4
	lsl x23, x21, #2
LBB13_4:
	adds x9, x25, x26
	b.hs LBB13_39
	cmp x9, x19
	b.hi LBB13_39
	cmp x27, x8
	b.eq LBB13_48
	add x24, x8, #1
	ldr s0, [x22, x8, lsl #2]
	fmul s0, s8, s0
	mov x0, x28
	mov x1, x25
	ldr x2, [sp, #40]
	mov x3, x25
	bl asm_compare::axpy_slice_panel64_local
	add x26, x26, x21
	add x28, x28, x23
	mov x8, x24
	cmp x24, x20
	b.lo LBB13_4
LBB13_8:
	cmp x25, x21
	ccmp x20, #0, #4, ne
	b.eq LBB13_15
	ldr x2, [sp, #32]
	cmp x21, x2
	b.hi LBB13_44
	ldr x8, [sp, #40]
	add x24, x8, x25, lsl #2
	lsl x27, x21, #2
	mov x23, x21
	ldp x9, x28, [sp, #16]
	ldr x8, [sp, #8]
	add x26, x9, x8, lsl #8
LBB13_11:
	cbz x28, LBB13_45
	cmp x23, x25
	b.lo LBB13_54
	cmp x23, x19
	b.hi LBB13_54
	ldr s0, [x22], #4
	fmul s0, s8, s0
	and x1, x21, #0x3f
	and x3, x21, #0x3f
	mov x0, x26
	mov x2, x24
	bl asm_compare::axpy_slice_panel64_local
	add x25, x25, x21
	add x26, x26, x27
	sub x28, x28, #1
	add x23, x23, x21
	subs x20, x20, #1
	b.ne LBB13_11
LBB13_15:
	.cfi_def_cfa wsp, 160
	ldp x29, x30, [sp, #144]
	ldp x20, x19, [sp, #128]
	ldp x22, x21, [sp, #112]
	ldp x24, x23, [sp, #96]
	ldp x26, x25, [sp, #80]
	ldp x28, x27, [sp, #64]
	ldp d9, d8, [sp, #48]
	add sp, sp, #160
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	.cfi_restore b8
	.cfi_restore b9
	ret
LBB13_16:
	.cfi_restore_state
	mov x13, #0
	mov x14, #0
	lsl x15, x21, #1
	add x16, x15, x21
	lsl x17, x16, #2
	and x2, x4, #0xffffffffffffffc0
	lsl x3, x21, #4
	lsl x25, x21, #2
	lsl x5, x21, #3
	mov x6, x21
	mov x7, x27
	b LBB13_18
LBB13_17:
	add x14, x14, #1
	add x7, x7, x3
	add x16, x16, x25
	add x15, x15, x25
	add x6, x6, x25
	add x13, x13, x25
	cmp x14, x12
	b.eq LBB13_1
LBB13_18:
	lsl x8, x14, #2
	cmp x8, x24
	b.hs LBB13_51
	orr x9, x8, #0x1
	cmp x9, x24
	b.hs LBB13_52
	orr x10, x8, #0x2
	cmp x10, x24
	b.hs LBB13_49
	orr x11, x8, #0x3
	cmp x11, x24
	b.hs LBB13_50
	cmp x21, #64
	b.lo LBB13_17
	mov x0, #0
	ldr s0, [x22, x8, lsl #2]
	fmul s0, s8, s0
	ldr s1, [x22, x9, lsl #2]
	fmul s1, s8, s1
	ldr s2, [x22, x10, lsl #2]
	fmul s2, s8, s2
	ldr s3, [x22, x11, lsl #2]
	fmul s3, s8, s3
	mov x9, x26
	mov x10, x7
	ldr x11, [sp, #40]
LBB13_24:
	add x8, x13, x0
	add x1, x8, #64
	cmn x8, #65
	b.hi LBB13_34
	cmp x1, x19
	b.hi LBB13_34
	add x23, x6, x0
	add x8, x6, x0
	add x1, x8, #64
	cmn x23, #65
	b.hi LBB13_35
	cmp x1, x19
	b.hi LBB13_35
	add x23, x15, x0
	add x8, x15, x0
	add x1, x8, #64
	cmn x23, #65
	b.hi LBB13_36
	cmp x1, x19
	b.hi LBB13_36
	add x23, x16, x0
	add x8, x16, x0
	add x1, x8, #64
	cmn x23, #65
	b.hi LBB13_37
	cmp x1, x19
	b.hi LBB13_37
	cmp x2, x0
	b.eq LBB13_38
	ldp q29, q28, [x10, #224]
	ldp q27, q26, [x10, #192]
	ldp q25, q24, [x10, #160]
	ldp q23, q22, [x10, #128]
	ldp q21, q20, [x10, #96]
	ldp q19, q18, [x10, #64]
	ldp q17, q16, [x10, #32]
	ldp q6, q7, [x10]
	add x8, x10, x25
	ldp q4, q5, [x11]
	fmla.4s v5, v7, v0[0]
	fmla.4s v4, v6, v0[0]
	ldp q7, q6, [x11, #32]
	fmla.4s v7, v17, v0[0]
	fmla.4s v6, v16, v0[0]
	ldp q17, q16, [x11, #64]
	fmla.4s v17, v19, v0[0]
	fmla.4s v16, v18, v0[0]
	ldp q19, q18, [x11, #96]
	fmla.4s v19, v21, v0[0]
	fmla.4s v18, v20, v0[0]
	ldp q21, q20, [x11, #128]
	fmla.4s v21, v23, v0[0]
	fmla.4s v20, v22, v0[0]
	ldp q23, q22, [x11, #160]
	fmla.4s v23, v25, v0[0]
	fmla.4s v22, v24, v0[0]
	ldp q25, q24, [x11, #192]
	fmla.4s v25, v27, v0[0]
	fmla.4s v24, v26, v0[0]
	ldp q27, q26, [x11, #224]
	fmla.4s v27, v29, v0[0]
	fmla.4s v26, v28, v0[0]
	ldp q29, q28, [x8, #224]
	fmla.4s v26, v28, v1[0]
	fmla.4s v27, v29, v1[0]
	ldp q29, q28, [x8, #192]
	fmla.4s v24, v28, v1[0]
	fmla.4s v25, v29, v1[0]
	ldp q29, q28, [x8, #160]
	fmla.4s v22, v28, v1[0]
	fmla.4s v23, v29, v1[0]
	ldp q29, q28, [x8, #128]
	fmla.4s v20, v28, v1[0]
	fmla.4s v21, v29, v1[0]
	ldp q29, q28, [x8, #96]
	fmla.4s v18, v28, v1[0]
	fmla.4s v19, v29, v1[0]
	ldp q29, q28, [x8, #64]
	fmla.4s v16, v28, v1[0]
	fmla.4s v17, v29, v1[0]
	ldp q29, q28, [x8, #32]
	fmla.4s v6, v28, v1[0]
	fmla.4s v7, v29, v1[0]
	ldp q29, q28, [x8]
	add x8, x10, x5
	fmla.4s v4, v29, v1[0]
	fmla.4s v5, v28, v1[0]
	ldp q29, q28, [x8]
	fmla.4s v5, v28, v2[0]
	fmla.4s v4, v29, v2[0]
	ldp q29, q28, [x8, #32]
	fmla.4s v7, v29, v2[0]
	fmla.4s v6, v28, v2[0]
	ldp q29, q28, [x8, #64]
	fmla.4s v17, v29, v2[0]
	fmla.4s v16, v28, v2[0]
	ldp q29, q28, [x8, #96]
	fmla.4s v19, v29, v2[0]
	fmla.4s v18, v28, v2[0]
	ldp q29, q28, [x8, #128]
	fmla.4s v21, v29, v2[0]
	fmla.4s v20, v28, v2[0]
	ldp q29, q28, [x8, #160]
	fmla.4s v23, v29, v2[0]
	fmla.4s v22, v28, v2[0]
	ldp q29, q28, [x8, #192]
	fmla.4s v25, v29, v2[0]
	fmla.4s v24, v28, v2[0]
	ldp q29, q28, [x8, #224]
	fmla.4s v27, v29, v2[0]
	add x8, x10, x17
	fmla.4s v26, v28, v2[0]
	ldp q28, q29, [x8, #224]
	fmla.4s v26, v29, v3[0]
	fmla.4s v27, v28, v3[0]
	ldp q28, q29, [x8, #192]
	fmla.4s v24, v29, v3[0]
	fmla.4s v25, v28, v3[0]
	ldp q28, q29, [x8, #160]
	fmla.4s v22, v29, v3[0]
	fmla.4s v23, v28, v3[0]
	ldp q28, q29, [x8, #128]
	fmla.4s v20, v29, v3[0]
	fmla.4s v21, v28, v3[0]
	ldp q28, q29, [x8, #96]
	fmla.4s v18, v29, v3[0]
	fmla.4s v19, v28, v3[0]
	ldp q28, q29, [x8, #64]
	fmla.4s v16, v29, v3[0]
	fmla.4s v17, v28, v3[0]
	ldp q28, q29, [x8, #32]
	fmla.4s v6, v29, v3[0]
	fmla.4s v7, v28, v3[0]
	ldp q29, q28, [x8]
	fmla.4s v4, v29, v3[0]
	fmla.4s v5, v28, v3[0]
	stp q4, q5, [x11]
	stp q7, q6, [x11, #32]
	stp q17, q16, [x11, #64]
	stp q19, q18, [x11, #96]
	stp q21, q20, [x11, #128]
	stp q23, q22, [x11, #160]
	stp q25, q24, [x11, #192]
	stp q27, q26, [x11, #224]
	add x0, x0, #64
	add x11, x11, #256
	add x10, x10, #256
	subs x9, x9, #1
	b.ne LBB13_24
	b LBB13_17
LBB13_34:
Lloh108:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.53@PAGE
Lloh109:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.53@PAGEOFF
	mov x0, x8
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB13_35:
Lloh110:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.52@PAGE
Lloh111:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.52@PAGEOFF
	mov x0, x8
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB13_36:
Lloh112:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.51@PAGE
Lloh113:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.51@PAGEOFF
	mov x0, x8
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB13_37:
Lloh114:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.50@PAGE
Lloh115:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.50@PAGEOFF
	mov x0, x8
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB13_38:
	and x8, x4, #0x1fffffffffffffc0
Lloh116:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.49@PAGE
Lloh117:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.49@PAGEOFF
	add x1, x8, #64
	mov x2, x4
	bl core::slice::index::slice_index_fail
LBB13_39:
	add x1, x25, x26
LBB13_40:
Lloh118:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.44@PAGE
Lloh119:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.44@PAGEOFF
	mov x0, x26
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB13_41:
	mul x26, x8, x21
	adds x1, x26, x25
	b.hs LBB13_40
	cmp x1, x19
	b.hi LBB13_40
Lloh120:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.43@PAGE
Lloh121:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.43@PAGEOFF
	mov x0, #0
	mov x1, x25
	bl core::slice::index::slice_index_fail
LBB13_44:
	ldr x8, [sp, #24]
	cbnz x8, LBB13_46
LBB13_45:
Lloh122:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.39@PAGE
Lloh123:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.39@PAGEOFF
	ldr x0, [sp, #24]
	mov x1, x0
	bl core::panicking::panic_bounds_check
LBB13_46:
	cmp x21, x19
	b.hi LBB13_53
Lloh124:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.40@PAGE
Lloh125:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.40@PAGEOFF
	mov x0, x25
	mov x1, x21
	bl core::slice::index::slice_index_fail
LBB13_48:
Lloh126:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.42@PAGE
Lloh127:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.42@PAGEOFF
	mov x0, x27
	ldr x1, [sp, #24]
	bl core::panicking::panic_bounds_check
LBB13_49:
Lloh128:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.47@PAGE
Lloh129:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.47@PAGEOFF
	mov x0, x10
	mov x1, x24
	bl core::panicking::panic_bounds_check
LBB13_50:
Lloh130:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.48@PAGE
Lloh131:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.48@PAGEOFF
	mov x0, x11
	mov x1, x24
	bl core::panicking::panic_bounds_check
LBB13_51:
Lloh132:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.45@PAGE
Lloh133:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.45@PAGEOFF
	mov x0, x8
	mov x1, x24
	bl core::panicking::panic_bounds_check
LBB13_52:
Lloh134:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.46@PAGE
Lloh135:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.46@PAGEOFF
	mov x0, x9
	mov x1, x24
	bl core::panicking::panic_bounds_check
LBB13_53:
	mov x23, x21
LBB13_54:
Lloh136:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.41@PAGE
Lloh137:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.41@PAGEOFF
	mov x0, x25
	mov x1, x23
	mov x2, x19
	bl core::slice::index::slice_index_fail
	.loh AdrpAdd	Lloh108, Lloh109
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpAdd	Lloh112, Lloh113
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh116, Lloh117
	.loh AdrpAdd	Lloh118, Lloh119
	.loh AdrpAdd	Lloh120, Lloh121
	.loh AdrpAdd	Lloh122, Lloh123
	.loh AdrpAdd	Lloh124, Lloh125
	.loh AdrpAdd	Lloh126, Lloh127
	.loh AdrpAdd	Lloh128, Lloh129
	.loh AdrpAdd	Lloh130, Lloh131
	.loh AdrpAdd	Lloh132, Lloh133
	.loh AdrpAdd	Lloh134, Lloh135
	.loh AdrpAdd	Lloh136, Lloh137
