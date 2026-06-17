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
Lfunc_end5:
	.cfi_endproc

	.p2align	2
asm_compare::gemv1_panel_faxpy_local:
Lfunc_begin6:
	.cfi_startproc
	sub sp, sp, #288
	.cfi_def_cfa_offset 288
	stp d9, d8, [sp, #176]
	stp x28, x27, [sp, #192]
	stp x26, x25, [sp, #208]
	stp x24, x23, [sp, #224]
	stp x22, x21, [sp, #240]
	stp x20, x19, [sp, #256]
	stp x29, x30, [sp, #272]
	add x29, sp, #272
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
	stp x2, x5, [sp, #24]
	str x4, [sp, #16]
	stp x0, x0, [sp, #72]
	mul x8, x0, x0
	stp x2, x8, [sp, #88]
	cmp x8, x2
	b.ne LBB6_25
	mov x23, x3
	mov x24, x1
	mov x25, x0
	fmov d8, d0
	cbz x6, LBB6_15
	lsl x8, x6, #2
	sub x11, x8, #4
	ldr x10, [sp, #32]
	cmp x11, #12
	b.lo LBB6_13
	lsr x9, x11, #2
	add x9, x9, #1
	cmp x11, #60
	b.hs LBB6_5
	mov x11, #0
	b LBB6_9
LBB6_5:
	and x11, x9, #0x7ffffffffffffff0
	ldr x10, [sp, #32]
	add x10, x10, #32
	mov x12, x11
LBB6_6:
	ldp q0, q2, [x10, #-32]
	ldp q3, q4, [x10]
	fmul.4s v0, v0, v1[0]
	fmul.4s v2, v2, v1[0]
	fmul.4s v3, v3, v1[0]
	fmul.4s v4, v4, v1[0]
	stp q0, q2, [x10, #-32]
	stp q3, q4, [x10], #64
	subs x12, x12, #16
	b.ne LBB6_6
	cmp x9, x11
	b.eq LBB6_15
	tst x9, #0xc
	b.eq LBB6_12
LBB6_9:
	and x12, x9, #0x7ffffffffffffffc
	ldr x14, [sp, #32]
	add x10, x14, x12, lsl #2
	sub x13, x11, x12
	add x11, x14, x11, lsl #2
LBB6_10:
	ldr q0, [x11]
	fmul.4s v0, v0, v1[0]
	str q0, [x11], #16
	adds x13, x13, #4
	b.ne LBB6_10
	cmp x9, x12
	b.ne LBB6_13
	b LBB6_15
LBB6_12:
	ldr x9, [sp, #32]
	add x10, x9, x11, lsl #2
LBB6_13:
	ldr x9, [sp, #32]
	add x8, x9, x8
LBB6_14:
	ldr s0, [x10]
	fmul s0, s1, s0
	str s0, [x10], #4
	cmp x10, x8
	b.ne LBB6_14
LBB6_15:
	str x6, [sp, #8]
	cbz x25, LBB6_22
	mov x27, #0
	mov x26, #0
	mov x28, #0
	mov x0, #0
	lsr x8, x25, #7
	tst x25, #0x7f
	cinc x19, x8, ne
	lsl x8, x25, #9
	str x8, [sp]
	lsl x22, x25, #7
	mov w20, #128
LBB6_17:
	add x21, x0, #128
	cmp x25, x20
	csel x9, x25, x20, lo
	cmp x25, x21
	csel x1, x25, x21, lo
	mul x8, x1, x25
	cmp x8, x26
	ldr x10, [sp, #24]
	ccmp x8, x10, #2, hs
	b.hi LBB6_23
	add x2, x9, x28
	madd x8, x25, x9, x27
	stp x25, x2, [sp, #72]
	mul x9, x2, x25
	stp x8, x9, [sp, #88]
	cmp x8, x9
	b.ne LBB6_25
	stp x24, x8, [sp, #40]
	stp x25, x2, [sp, #56]
	cmp x1, x0
	b.lo LBB6_24
	ldr x8, [sp, #16]
	cmp x1, x8
	b.hi LBB6_24
	sub x19, x19, #1
	add x0, sp, #40
	fmov d0, d8
	mov x1, x23
	ldr x3, [sp, #32]
	ldr x4, [sp, #8]
	bl asm_compare::faxpy_struct_f32_local
	add x23, x23, #512
	add x20, x20, #128
	sub x28, x28, #128
	ldr x8, [sp]
	add x24, x24, x8
	add x26, x26, x22
	sub x27, x27, x22
	mov x0, x21
	cbnz x19, LBB6_17
LBB6_22:
	.cfi_def_cfa wsp, 288
	ldp x29, x30, [sp, #272]
	ldp x20, x19, [sp, #256]
	ldp x22, x21, [sp, #240]
	ldp x24, x23, [sp, #224]
	ldp x26, x25, [sp, #208]
	ldp x28, x27, [sp, #192]
	ldp d9, d8, [sp, #176]
	add sp, sp, #288
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
LBB6_23:
	.cfi_restore_state
Lloh32:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.42@PAGE
Lloh33:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.42@PAGEOFF
	mov x0, x26
	mov x1, x8
	ldr x2, [sp, #24]
	bl core::slice::index::slice_index_fail
LBB6_24:
Lloh34:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.39@PAGE
Lloh35:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.39@PAGEOFF
	ldr x2, [sp, #16]
	bl core::slice::index::slice_index_fail
LBB6_25:
Lloh36:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh37:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	add x9, sp, #88
	stp x9, x8, [sp, #104]
	add x9, sp, #72
	stp x9, x8, [sp, #120]
	add x9, sp, #80
	stp x9, x8, [sp, #136]
	add x9, sp, #96
	stp x9, x8, [sp, #152]
Lloh38:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.40@PAGE
Lloh39:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.40@PAGEOFF
Lloh40:
	adrp x5, l_anon.6efade69f4e207c800bc74b0e469f61c.41@PAGE
Lloh41:
	add x5, x5, l_anon.6efade69f4e207c800bc74b0e469f61c.41@PAGEOFF
	add x1, sp, #96
	add x2, sp, #88
	add x4, sp, #104
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpLdrGot	Lloh36, Lloh37
Lfunc_end6:
	.cfi_endproc

	.p2align	2
asm_compare::saxpyf_contiguous_local:
Lfunc_begin7:
	.cfi_startproc
	sub sp, sp, #128
	.cfi_def_cfa_offset 128
	stp x28, x27, [sp, #32]
	stp x26, x25, [sp, #48]
	stp x24, x23, [sp, #64]
	stp x22, x21, [sp, #80]
	stp x20, x19, [sp, #96]
	stp x29, x30, [sp, #112]
	add x29, sp, #112
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
	.cfi_remember_state
	cbz x0, LBB7_67
	mov x8, x2
	mov x9, #0
	ldr x2, [x29, #16]
	lsl x17, x3, #2
	mov w10, #12
	madd x11, x6, x10, x4
	lsl x20, x6, #4
	lsl x21, x6, #2
	add x10, x4, x21
	stp x10, x11, [sp, #16]
	mov w23, #256
	add x10, x4, x6, lsl #3
	str x10, [sp, #8]
	b LBB7_3
LBB7_2:
	cmp x9, x0
	b.hs LBB7_67
LBB7_3:
	mov x10, x9
	sub x11, x0, x9
	cmp x11, #256
	csel x26, x11, x23, lo
	adds x9, x26, x9
	b.hs LBB7_73
	cmp x9, x2
	b.hi LBB7_73
	cmp x11, #32
	b.lo LBB7_18
	lsr x11, x26, #5
	add x27, x7, x10, lsl #2
	cmp x1, #4
	b.hs LBB7_28
	mov x28, #0
LBB7_8:
	cmp x28, x1
	b.hs LBB7_18
	lsl x15, x11, #7
	cmp x28, x3
	csel x12, x28, x3, hi
	madd x13, x6, x28, x10
	add x16, x4, x13, lsl #2
	b LBB7_11
LBB7_10:
	add x28, x28, #1
	add x16, x16, x21
	cmp x28, x1
	b.eq LBB7_18
LBB7_11:
	cmp x28, x12
	b.eq LBB7_81
	ldr s0, [x8, x28, lsl #2]
	fcmp s0, #0.0
	b.eq LBB7_10
	madd x13, x28, x6, x10
	adds x14, x13, x26
	b.hs LBB7_68
	cmp x14, x5
	b.hi LBB7_68
	mov x13, x15
	mov x14, x16
	mov x19, x27
LBB7_16:
	cbz x13, LBB7_75
	ldp q1, q2, [x19, #96]
	ldp q3, q4, [x19, #64]
	ldp q5, q6, [x19, #32]
	ldp q7, q16, [x19]
	ldp q17, q18, [x14, #96]
	ldp q19, q20, [x14, #64]
	ldp q21, q22, [x14, #32]
	ldp q23, q24, [x14], #128
	fmla.4s v16, v24, v0[0]
	fmla.4s v7, v23, v0[0]
	fmla.4s v6, v22, v0[0]
	fmla.4s v5, v21, v0[0]
	fmla.4s v4, v20, v0[0]
	fmla.4s v3, v19, v0[0]
	fmla.4s v2, v18, v0[0]
	fmla.4s v1, v17, v0[0]
	stp q1, q2, [x19, #96]
	stp q3, q4, [x19, #64]
	stp q5, q6, [x19, #32]
	stp q7, q16, [x19], #128
	subs x13, x13, #128
	b.ne LBB7_16
	b LBB7_10
LBB7_18:
	and x12, x26, #0x1f
	cbz x12, LBB7_2
	and x11, x26, #0x1e0
	add x13, x11, x10
	cbz x3, LBB7_60
	mov x14, #0
	mov x15, x13
	b LBB7_22
LBB7_21:
	str s0, [x7, x10, lsl #2]
	add x15, x15, #1
	cmp x14, x12
	b.eq LBB7_2
LBB7_22:
	add x10, x13, x14
	cmp x10, x2
	b.hs LBB7_80
	add x14, x14, #1
	ldr s0, [x7, x10, lsl #2]
	mov x16, x17
	mov x11, x15
	mov x19, x8
	b LBB7_25
LBB7_24:
	add x11, x11, x6
	subs x16, x16, #4
	b.eq LBB7_21
LBB7_25:
	ldr s1, [x19], #4
	fcmp s1, #0.0
	b.eq LBB7_24
	cmp x11, x5
	b.hs LBB7_74
	ldr s2, [x4, x11, lsl #2]
	fmul s1, s1, s2
	fadd s0, s0, s1
	b LBB7_24
LBB7_28:
	mov x12, #0
	lsl x30, x11, #7
	lsl x13, x10, #2
	ldr x14, [sp, #24]
	add x24, x14, x13
	ldp x15, x14, [sp, #8]
	add x19, x15, x13
	add x22, x14, x13
	add x25, x4, x13
	mov w13, #4
	b LBB7_30
LBB7_29:
	add x13, x28, #4
	add x24, x24, x20
	add x19, x19, x20
	add x22, x22, x20
	add x25, x25, x20
	mov x12, x28
	cmp x13, x1
	b.hi LBB7_8
LBB7_30:
	cmp x12, x3
	b.hs LBB7_83
	orr x15, x12, #0x1
	cmp x15, x3
	b.hs LBB7_82
	orr x14, x12, #0x2
	cmp x14, x3
	b.hs LBB7_85
	mov x28, x13
	orr x13, x12, #0x3
	cmp x13, x3
	b.hs LBB7_84
	ldr s0, [x8, x12, lsl #2]
	ldr s1, [x8, x15, lsl #2]
	ldr s2, [x8, x14, lsl #2]
	ldr s3, [x8, x13, lsl #2]
	fcmp s0, #0.0
	b.ne LBB7_38
	fcmp s1, #0.0
	b.ne LBB7_38
	fcmp s2, #0.0
	b.ne LBB7_38
	fcmp s3, #0.0
	b.eq LBB7_29
LBB7_38:
	madd x12, x12, x6, x10
	adds x16, x12, x26
	b.hs LBB7_69
	cmp x16, x5
	b.hi LBB7_69
	madd x12, x15, x6, x10
	adds x15, x12, x26
	b.hs LBB7_70
	cmp x15, x5
	b.hi LBB7_70
	madd x12, x14, x6, x10
	adds x14, x12, x26
	b.hs LBB7_71
	cmp x14, x5
	b.hi LBB7_71
	madd x12, x13, x6, x10
	adds x13, x12, x26
	b.hs LBB7_72
	cmp x13, x5
	b.hi LBB7_72
	mov x12, #0
	mov x13, x30
	b LBB7_49
LBB7_47:
	add x15, x24, x12, lsl #7
	ldp q21, q20, [x15, #96]
	ldp q22, q23, [x15]
	ldp q25, q24, [x15, #32]
	fmla.4s v17, v25, v3[0]
	fmla.4s v19, v23, v3[0]
	ldp q25, q23, [x15, #64]
	fmla.4s v18, v22, v3[0]
	fmla.4s v16, v24, v3[0]
	fmla.4s v7, v25, v3[0]
	fmla.4s v6, v23, v3[0]
	fmla.4s v5, v21, v3[0]
	fmla.4s v4, v20, v3[0]
LBB7_48:
	stp q18, q19, [x14]
	stp q17, q16, [x14, #32]
	stp q7, q6, [x14, #64]
	add x12, x12, #1
	stp q5, q4, [x14, #96]
	subs x13, x13, #128
	b.eq LBB7_29
LBB7_49:
	add x14, x27, x12, lsl #7
	ldp q5, q4, [x14, #96]
	ldp q7, q6, [x14, #64]
	ldp q17, q16, [x14, #32]
	fcmp s0, #0.0
	ldp q18, q19, [x14]
	b.eq LBB7_52
	cmp x12, x11
	b.hs LBB7_78
	add x15, x25, x12, lsl #7
	ldp q21, q20, [x15, #96]
	ldp q22, q23, [x15]
	ldp q25, q24, [x15, #32]
	fmla.4s v17, v25, v0[0]
	fmla.4s v19, v23, v0[0]
	ldp q25, q23, [x15, #64]
	fmla.4s v18, v22, v0[0]
	fmla.4s v16, v24, v0[0]
	fmla.4s v7, v25, v0[0]
	fmla.4s v6, v23, v0[0]
	fmla.4s v5, v21, v0[0]
	fmla.4s v4, v20, v0[0]
LBB7_52:
	fcmp s1, #0.0
	b.eq LBB7_55
	cmp x12, x11
	b.hs LBB7_77
	add x15, x22, x12, lsl #7
	ldp q21, q20, [x15, #96]
	ldp q22, q23, [x15]
	ldp q25, q24, [x15, #32]
	fmla.4s v17, v25, v1[0]
	fmla.4s v19, v23, v1[0]
	ldp q25, q23, [x15, #64]
	fmla.4s v18, v22, v1[0]
	fmla.4s v16, v24, v1[0]
	fmla.4s v7, v25, v1[0]
	fmla.4s v6, v23, v1[0]
	fmla.4s v5, v21, v1[0]
	fmla.4s v4, v20, v1[0]
LBB7_55:
	fcmp s2, #0.0
	b.eq LBB7_58
	cmp x12, x11
	b.hs LBB7_76
	add x15, x19, x12, lsl #7
	ldp q21, q20, [x15, #96]
	ldp q22, q23, [x15]
	ldp q25, q24, [x15, #32]
	fmla.4s v17, v25, v2[0]
	fmla.4s v19, v23, v2[0]
	ldp q25, q23, [x15, #64]
	fmla.4s v18, v22, v2[0]
	fmla.4s v16, v24, v2[0]
	fmla.4s v7, v25, v2[0]
	fmla.4s v6, v23, v2[0]
	fmla.4s v5, v21, v2[0]
	fmla.4s v4, v20, v2[0]
LBB7_58:
	fcmp s3, #0.0
	b.eq LBB7_48
	cmp x12, x11
	b.lo LBB7_47
	b LBB7_79
LBB7_60:
	cmp x2, x13
	csel x13, x2, x13, hi
	add x14, x10, x11
	sub x13, x13, x14
	sub x14, x12, #1
	cmp x13, x14
	csel x14, x13, x14, lo
	add x13, x14, #1
	cmp x13, #5
	b.hs LBB7_62
	mov x13, #0
	b LBB7_64
LBB7_62:
	ands x15, x13, #0x3
	mov w16, #4
	csel x15, x16, x15, eq
	sub x13, x13, x15
	mvn x14, x14
	add x14, x14, x15
LBB7_63:
	adds x14, x14, #4
	b.ne LBB7_63
LBB7_64:
	sub x12, x13, x12
	add x10, x13, x10
	add x10, x10, x11
LBB7_65:
	cmp x10, x2
	b.hs LBB7_80
	add x10, x10, #1
	adds x12, x12, #1
	b.lo LBB7_65
	b LBB7_2
LBB7_67:
	.cfi_def_cfa wsp, 128
	ldp x29, x30, [sp, #112]
	ldp x20, x19, [sp, #96]
	ldp x22, x21, [sp, #80]
	ldp x24, x23, [sp, #64]
	ldp x26, x25, [sp, #48]
	ldp x28, x27, [sp, #32]
	add sp, sp, #128
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
	ret
LBB7_68:
	.cfi_restore_state
Lloh42:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.21@PAGE
Lloh43:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.21@PAGEOFF
	mov x0, x13
	mov x1, x14
	mov x2, x5
	bl core::slice::index::slice_index_fail
LBB7_69:
Lloh44:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.33@PAGE
Lloh45:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.33@PAGEOFF
	mov x0, x12
	mov x1, x16
	mov x2, x5
	bl core::slice::index::slice_index_fail
LBB7_70:
Lloh46:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.32@PAGE
Lloh47:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.32@PAGEOFF
	mov x0, x12
	mov x1, x15
	mov x2, x5
	bl core::slice::index::slice_index_fail
LBB7_71:
Lloh48:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.31@PAGE
Lloh49:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.31@PAGEOFF
	mov x0, x12
	mov x1, x14
	mov x2, x5
	bl core::slice::index::slice_index_fail
LBB7_72:
Lloh50:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.30@PAGE
Lloh51:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.30@PAGEOFF
	mov x0, x12
	mov x1, x13
	mov x2, x5
	bl core::slice::index::slice_index_fail
LBB7_73:
Lloh52:
	adrp x3, l_anon.6efade69f4e207c800bc74b0e469f61c.34@PAGE
Lloh53:
	add x3, x3, l_anon.6efade69f4e207c800bc74b0e469f61c.34@PAGEOFF
	mov x0, x10
	mov x1, x9
	bl core::slice::index::slice_index_fail
LBB7_74:
Lloh54:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.18@PAGE
Lloh55:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.18@PAGEOFF
	mov x0, x11
	mov x1, x5
	bl core::panicking::panic_bounds_check
LBB7_75:
Lloh56:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.20@PAGE
Lloh57:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.20@PAGEOFF
	mov x0, x11
	mov x1, x11
	bl core::panicking::panic_bounds_check
LBB7_76:
Lloh58:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.28@PAGE
Lloh59:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.28@PAGEOFF
	mov x0, x12
	mov x1, x11
	bl core::panicking::panic_bounds_check
LBB7_77:
Lloh60:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.27@PAGE
Lloh61:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.27@PAGEOFF
	mov x0, x12
	mov x1, x11
	bl core::panicking::panic_bounds_check
LBB7_78:
Lloh62:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.26@PAGE
Lloh63:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.26@PAGEOFF
	mov x0, x12
	mov x1, x11
	bl core::panicking::panic_bounds_check
LBB7_79:
Lloh64:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.29@PAGE
Lloh65:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.29@PAGEOFF
	mov x0, x12
	mov x1, x11
	bl core::panicking::panic_bounds_check
LBB7_80:
Lloh66:
	adrp x8, l_anon.6efade69f4e207c800bc74b0e469f61c.17@PAGE
Lloh67:
	add x8, x8, l_anon.6efade69f4e207c800bc74b0e469f61c.17@PAGEOFF
	mov x0, x10
	mov x1, x2
	mov x2, x8
	bl core::panicking::panic_bounds_check
LBB7_81:
Lloh68:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.19@PAGE
Lloh69:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.19@PAGEOFF
	mov x0, x12
	mov x1, x3
	bl core::panicking::panic_bounds_check
LBB7_82:
Lloh70:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.23@PAGE
Lloh71:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.23@PAGEOFF
	mov x0, x15
	mov x1, x3
	bl core::panicking::panic_bounds_check
LBB7_83:
Lloh72:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.22@PAGE
Lloh73:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.22@PAGEOFF
	mov x0, x12
	mov x1, x3
	bl core::panicking::panic_bounds_check
LBB7_84:
Lloh74:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.25@PAGE
Lloh75:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.25@PAGEOFF
	mov x0, x13
	mov x1, x3
	bl core::panicking::panic_bounds_check
LBB7_85:
Lloh76:
	adrp x2, l_anon.6efade69f4e207c800bc74b0e469f61c.24@PAGE
Lloh77:
	add x2, x2, l_anon.6efade69f4e207c800bc74b0e469f61c.24@PAGEOFF
	mov x0, x14
	mov x1, x3
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh58, Lloh59
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh62, Lloh63
	.loh AdrpAdd	Lloh64, Lloh65
	.loh AdrpAdd	Lloh66, Lloh67
	.loh AdrpAdd	Lloh68, Lloh69
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpAdd	Lloh74, Lloh75
	.loh AdrpAdd	Lloh76, Lloh77
