.section __TEXT,__text,regular,pure_instructions
	.private_extern	std::rt::lang_start::<()>
	.globl	std::rt::lang_start::<()>
	.p2align	2
std::rt::lang_start::<()>:
Lfunc_begin0:
	.cfi_startproc
	sub sp, sp, #32
	.cfi_def_cfa_offset 32
	stp x29, x30, [sp, #16]
	add x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov x4, x3
	mov x3, x2
	mov x2, x1
	str x0, [sp, #8]
Lloh0:
	adrp x1, l_anon.6efade69f4e207c800bc74b0e469f61c.0@PAGE
Lloh1:
	add x1, x1, l_anon.6efade69f4e207c800bc74b0e469f61c.0@PAGEOFF
	add x0, sp, #8
	bl std::rt::lang_start_internal
	.cfi_def_cfa wsp, 32
	ldp x29, x30, [sp, #16]
	add sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.loh AdrpAdd	Lloh0, Lloh1
Lfunc_end0:
	.cfi_endproc

	.p2align	2
std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>:
Lfunc_begin1:
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr x0
	; InlineAsm Start
	; InlineAsm End
	.cfi_def_cfa wsp, 16
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end1:
	.cfi_endproc

	.p2align	2
std::rt::lang_start::<()>::{closure#0}:
Lfunc_begin2:
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr x0, [x0]
	bl std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
	mov w0, #0
	.cfi_def_cfa wsp, 16
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end2:
	.cfi_endproc

	.p2align	2
<std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}:
Lfunc_begin3:
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr x0, [x0]
	bl std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
	mov w0, #0
	.cfi_def_cfa wsp, 16
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end3:
	.cfi_endproc

	.p2align	2
asm_compare::axpy_slice_local:
Lfunc_begin4:
	.cfi_startproc
	stp x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	sub x8, x1, #1
	cmp x3, x8
	csel x8, x3, x8, lo
	cmp x8, #15
	b.hi LBB4_2
	mov x8, #0
	b LBB4_4
LBB4_2:
	add x8, x8, #1
	ands x9, x8, #0xf
	mov w10, #16
	csel x9, x10, x9, eq
	sub x8, x8, x9
	add x9, x0, #32
	add x10, x2, #32
	mov x11, x8
LBB4_3:
	ldp q1, q2, [x9, #-32]
	ldp q3, q4, [x9], #64
	fmul.4s v1, v1, v0[0]
	fmul.4s v2, v2, v0[0]
	fmul.4s v3, v3, v0[0]
	fmul.4s v4, v4, v0[0]
	ldp q5, q6, [x10, #-32]
	ldp q7, q16, [x10]
	fadd.4s v1, v5, v1
	fadd.4s v2, v6, v2
	fadd.4s v3, v7, v3
	fadd.4s v4, v16, v4
	stp q1, q2, [x10, #-32]
	stp q3, q4, [x10], #64
	subs x11, x11, #16
	b.ne LBB4_3
LBB4_4:
	cmp x3, x8
	b.eq LBB4_7
	ldr s1, [x0, x8, lsl #2]
	fmul s1, s0, s1
	ldr s2, [x2, x8, lsl #2]
	fadd s1, s2, s1
	str s1, [x2, x8, lsl #2]
	add x8, x8, #1
	cmp x1, x8
	b.ne LBB4_4
	.cfi_def_cfa wsp, 16
	ldp x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB4_7:
	.cfi_restore_state
Lloh2:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.2@PAGE
Lloh3:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.2@PAGEOFF
	mov x0, x3
	mov x1, x3
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh2, Lloh3
Lfunc_end4:
	.cfi_endproc

	.p2align	2
asm_compare::faxpy_struct_f32_local:
Lfunc_begin5:
	.cfi_startproc
	sub sp, sp, #224
	.cfi_def_cfa_offset 224
	stp d9, d8, [sp, #112]
	stp x28, x27, [sp, #128]
	stp x26, x25, [sp, #144]
	stp x24, x23, [sp, #160]
	stp x22, x21, [sp, #176]
	stp x20, x19, [sp, #192]
	stp x29, x30, [sp, #208]
	add x29, sp, #208
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
	stp x3, x4, [sp, #72]
	ldr x21, [x0, #16]
	cbz x21, LBB5_39
	mov x22, x2
	mov x20, x1
	fmov d8, d0
	mov x9, #0
	ldr x25, [x0, #24]
	ldp x10, x19, [x0]
	lsl x26, x21, #1
	add x27, x26, x21
	lsl x11, x2, #2
	mov w8, #12
	madd x8, x21, x8, x10
	stp x8, x11, [sp, #16]
	lsl x24, x21, #4
	stp x25, x10, [sp, #56]
	add x8, x10, x21, lsl #3
	str x8, [sp, #8]
	lsl x8, x21, #2
	str x8, [sp, #104]
	str x2, [sp, #88]
	stp x27, x26, [sp, #40]
	b LBB5_3
LBB5_2:
	ldp x22, x9, [sp, #88]
	cmp x9, x21
	ldp x26, x25, [sp, #48]
	ldr x27, [sp, #40]
	b.hs LBB5_39
LBB5_3:
	mov x0, x9
	sub x28, x21, x9
	cmp x28, #256
	mov w8, #256
	csel x16, x28, x8, lo
	adds x23, x16, x9
	b.hs LBB5_46
	ldr x8, [sp, #80]
	cmp x23, x8
	b.hi LBB5_46
	lsr x12, x16, #5
	ldr x8, [sp, #72]
	add x17, x8, x0, lsl #2
	cmp x28, #32
	b.lo LBB5_33
	cmp x25, #4
	b.hs LBB5_8
	mov x2, #0
	b LBB5_24
LBB5_8:
	mov x8, #0
	lsl x3, x12, #7
	lsl x9, x0, #2
	ldp x10, x11, [sp, #8]
	add x4, x11, x9
	add x5, x10, x9
	add x10, x21, x0
	ldr x11, [sp, #64]
	add x6, x11, x9
	mov w9, #4
	add x7, x11, x10, lsl #2
LBB5_9:
	cmp x8, x22
	b.hs LBB5_50
	mov x2, x9
	orr x9, x8, #0x1
	cmp x9, x22
	b.hs LBB5_52
	orr x10, x8, #0x2
	cmp x10, x22
	b.hs LBB5_51
	orr x11, x8, #0x3
	cmp x11, x22
	b.hs LBB5_53
	madd x13, x8, x21, x0
	adds x1, x13, x16
	b.hs LBB5_42
	cmp x1, x19
	b.hi LBB5_42
	add x14, x13, x21
	add x15, x1, x21
	cmp x15, x14
	ccmp x15, x19, #2, hs
	b.hi LBB5_43
	add x14, x13, x26
	add x15, x1, x26
	cmp x15, x14
	b.lo LBB5_44
	cmp x15, x19
	b.hi LBB5_44
	add x13, x13, x27
	add x1, x1, x27
	cmp x1, x13
	b.lo LBB5_45
	cmp x1, x19
	b.hi LBB5_45
	mov x13, #0
	ldr s0, [x20, x8, lsl #2]
	ldr s1, [x20, x9, lsl #2]
	ldr s2, [x20, x10, lsl #2]
	ldr s3, [x20, x11, lsl #2]
	fmul s0, s8, s0
	fmul s1, s8, s1
	fmul s2, s8, s2
	fmul s3, s8, s3
LBB5_21:
	cmp x3, x13
	b.eq LBB5_48
	add x8, x17, x13
	ldp q5, q4, [x8, #32]
	ldp q7, q6, [x8, #64]
	ldp q17, q16, [x8, #96]
	add x9, x6, x13
	ldp q19, q18, [x9, #96]
	fmla.4s v17, v19, v0[0]
	ldp q20, q19, [x9, #64]
	fmla.4s v16, v18, v0[0]
	fmla.4s v7, v20, v0[0]
	ldp q18, q20, [x9, #32]
	fmla.4s v6, v19, v0[0]
	fmla.4s v5, v18, v0[0]
	ldp q19, q18, [x8]
	fmla.4s v4, v20, v0[0]
	ldp q21, q20, [x9]
	fmla.4s v19, v21, v0[0]
	add x9, x7, x13
	fmla.4s v18, v20, v0[0]
	ldp q20, q21, [x9]
	fmla.4s v18, v21, v1[0]
	fmla.4s v19, v20, v1[0]
	ldp q20, q21, [x9, #32]
	fmla.4s v4, v21, v1[0]
	fmla.4s v5, v20, v1[0]
	ldp q20, q21, [x9, #64]
	fmla.4s v6, v21, v1[0]
	fmla.4s v7, v20, v1[0]
	ldp q20, q21, [x9, #96]
	fmla.4s v16, v21, v1[0]
	add x9, x5, x13
	fmla.4s v17, v20, v1[0]
	ldp q21, q20, [x9, #96]
	fmla.4s v17, v21, v2[0]
	fmla.4s v16, v20, v2[0]
	ldp q21, q20, [x9, #64]
	fmla.4s v7, v21, v2[0]
	fmla.4s v6, v20, v2[0]
	ldp q21, q20, [x9, #32]
	fmla.4s v5, v21, v2[0]
	fmla.4s v4, v20, v2[0]
	ldp q21, q20, [x9]
	fmla.4s v19, v21, v2[0]
	add x9, x4, x13
	fmla.4s v18, v20, v2[0]
	ldp q20, q21, [x9]
	fmla.4s v18, v21, v3[0]
	fmla.4s v19, v20, v3[0]
	ldp q20, q21, [x9, #32]
	fmla.4s v4, v21, v3[0]
	fmla.4s v5, v20, v3[0]
	ldp q20, q21, [x9, #64]
	fmla.4s v6, v21, v3[0]
	fmla.4s v7, v20, v3[0]
	ldp q20, q21, [x9, #96]
	fmla.4s v16, v21, v3[0]
	fmla.4s v17, v20, v3[0]
	stp q17, q16, [x8, #96]
	stp q7, q6, [x8, #64]
	stp q5, q4, [x8, #32]
	stp q19, q18, [x8]
	add x13, x13, #128
	cmp x3, x13
	b.ne LBB5_21
	add x9, x2, #4
	add x4, x4, x24
	add x5, x5, x24
	add x7, x7, x24
	add x6, x6, x24
	mov x8, x2
	cmp x9, x25
	b.ls LBB5_9
LBB5_24:
	cmp x2, x25
	b.hs LBB5_33
	lsl x10, x12, #7
	cmp x2, x22
	csel x8, x2, x22, hi
	madd x9, x21, x2, x0
	ldr x11, [sp, #64]
	add x11, x11, x9, lsl #2
LBB5_26:
	cmp x2, x8
	b.eq LBB5_49
	madd x9, x2, x21, x0
	adds x1, x9, x16
	b.hs LBB5_41
	cmp x1, x19
	b.hi LBB5_41
	ldr s0, [x20, x2, lsl #2]
	fmul s0, s8, s0
	mov x9, x10
	mov x13, x11
	mov x14, x17
LBB5_30:
	cbz x9, LBB5_47
	ldp q1, q2, [x14, #96]
	ldp q3, q4, [x14, #64]
	ldp q5, q6, [x14, #32]
	ldp q7, q16, [x14]
	ldp q17, q18, [x13, #96]
	ldp q19, q20, [x13, #64]
	ldp q21, q22, [x13, #32]
	ldp q23, q24, [x13], #128
	fmla.4s v16, v24, v0[0]
	fmla.4s v7, v23, v0[0]
	fmla.4s v6, v22, v0[0]
	fmla.4s v5, v21, v0[0]
	fmla.4s v4, v20, v0[0]
	fmla.4s v3, v19, v0[0]
	fmla.4s v2, v18, v0[0]
	fmla.4s v1, v17, v0[0]
	stp q1, q2, [x14, #96]
	stp q3, q4, [x14, #64]
	stp q5, q6, [x14, #32]
	stp q7, q16, [x14], #128
	subs x9, x9, #128
	b.ne LBB5_30
	add x2, x2, #1
	ldr x9, [sp, #104]
	add x11, x11, x9
	cmp x2, x25
	b.ne LBB5_26
LBB5_33:
	str x28, [sp, #32]
	str x23, [sp, #96]
	and x26, x16, #0x1f
	cbz x26, LBB5_2
	ldr x8, [sp, #88]
	cbz x8, LBB5_2
	and x8, x16, #0x1e0
	add x27, x17, x8, lsl #2
	add x25, x0, x8
	lsl x8, x12, #7
	add x8, x8, x0, lsl #2
	ldr x9, [sp, #64]
	add x28, x9, x8
	ldr x22, [sp, #24]
	mov x23, x20
LBB5_36:
	adds x8, x26, x25
	b.hs LBB5_40
	cmp x8, x19
	b.hi LBB5_40
	ldr s0, [x23], #4
	fmul s0, s8, s0
	mov x0, x28
	mov x1, x26
	mov x2, x27
	mov x3, x26
	bl asm_compare::axpy_slice_local
	add x25, x25, x21
	ldr x8, [sp, #104]
	add x28, x28, x8
	subs x22, x22, #4
	b.ne LBB5_36
	b LBB5_2
LBB5_39:
	.cfi_def_cfa wsp, 224
	ldp x29, x30, [sp, #208]
	ldp x20, x19, [sp, #192]
	ldp x22, x21, [sp, #176]
	ldp x24, x23, [sp, #160]
	ldp x26, x25, [sp, #144]
	ldp x28, x27, [sp, #128]
	ldp d9, d8, [sp, #112]
	add sp, sp, #224
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
LBB5_40:
	.cfi_restore_state
	mov w8, #256
	ldr x9, [sp, #32]
	cmp x9, #256
	csel x8, x9, x8, lo
	and x8, x8, #0x1f
Lloh4:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.3@PAGE
Lloh5:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.3@PAGEOFF
	add x1, x8, x25
	mov x0, x25
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB5_41:
Lloh6:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.6@PAGE
Lloh7:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.6@PAGEOFF
	mov x0, x9
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB5_42:
Lloh8:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.15@PAGE
Lloh9:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.15@PAGEOFF
	mov x0, x13
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB5_43:
Lloh10:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.14@PAGE
Lloh11:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.14@PAGEOFF
	mov x0, x14
	mov x1, x15
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB5_44:
Lloh12:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.13@PAGE
Lloh13:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.13@PAGEOFF
	mov x0, x14
	mov x1, x15
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB5_45:
Lloh14:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.12@PAGE
Lloh15:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.12@PAGEOFF
	mov x0, x13
	mov x2, x19
	bl core::slice::index::slice_index_fail
LBB5_46:
Lloh16:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.16@PAGE
Lloh17:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.16@PAGEOFF
	mov x1, x23
	ldr x2, [sp, #80]
	bl core::slice::index::slice_index_fail
LBB5_47:
Lloh18:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.5@PAGE
Lloh19:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.5@PAGEOFF
	mov x0, x12
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB5_48:
Lloh20:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.11@PAGE
Lloh21:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.11@PAGEOFF
	mov x0, x12
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB5_49:
Lloh22:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.4@PAGE
Lloh23:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.4@PAGEOFF
	mov x0, x8
	mov x1, x22
	bl core::panicking::panic_bounds_check
LBB5_50:
Lloh24:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.7@PAGE
Lloh25:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.7@PAGEOFF
	mov x0, x8
	mov x1, x22
	bl core::panicking::panic_bounds_check
LBB5_51:
Lloh26:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.9@PAGE
Lloh27:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.9@PAGEOFF
	mov x0, x10
	mov x1, x22
	bl core::panicking::panic_bounds_check
LBB5_52:
Lloh28:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.8@PAGE
Lloh29:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.8@PAGEOFF
	mov x0, x9
	mov x1, x22
	bl core::panicking::panic_bounds_check
LBB5_53:
Lloh30:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.10@PAGE
Lloh31:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.10@PAGEOFF
	mov x0, x11
	mov x1, x22
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh30, Lloh31
