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
	adrp x1, l_anon.7ad164de683c838adae784d9fc1e3405.0@PAGE
Lloh1:
	add x1, x1, l_anon.7ad164de683c838adae784d9fc1e3405.0@PAGEOFF
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
sgemm_arch_asm::sgemm_arch:
Lfunc_begin4:
	.cfi_startproc
	sub sp, sp, #352
	.cfi_def_cfa_offset 352
	stp d9, d8, [sp, #240]
	stp x28, x27, [sp, #256]
	stp x26, x25, [sp, #272]
	stp x24, x23, [sp, #288]
	stp x22, x21, [sp, #304]
	stp x20, x19, [sp, #320]
	stp x29, x30, [sp, #336]
	add x29, sp, #336
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
	str x0, [sp, #32]
	mov w8, #512
	stp x8, x8, [sp, #136]
	mov w8, #262144
	stp x1, x8, [sp, #152]
	cmp x1, #64, lsl #12
	b.ne LBB4_37
	mov w8, #512
	stp x8, x8, [sp, #136]
	mov w8, #262144
	stp x3, x8, [sp, #152]
	cmp x3, #64, lsl #12
	b.ne LBB4_37
	mov w8, #512
	stp x8, x8, [sp, #136]
	mov w8, #262144
	stp x5, x8, [sp, #152]
	cmp x5, #64, lsl #12
	b.ne LBB4_38
	mov x16, #0
	mov x17, #0
	mov x13, #0
	str x4, [sp, #48]
	add x0, x4, #32
	mov w1, #64
	mov w4, #448
	mov w7, #512
	mov w3, #52429
	movk w3, #15820, lsl #16
	dup.4s v0, w3
	mov w5, #64
LBB4_4:
	cmp x16, #448
	csel x8, x16, x4, lo
	add x8, x8, x1
	str x8, [sp, #80]
	lsr x8, x8, #2
	subs x8, x8, #1
	csel x9, xzr, x8, lo
	cmp x5, #512
	csel x8, x5, x7, lo
	add x8, x8, x17
	cmp x8, #1
	csinc x10, x8, xzr, hi
	lsl x11, x13, #6
	cmp x11, #448
	csel x8, x11, x4, lo
	add x12, x8, #64
	lsl x14, x13, #15
	lsl x8, x12, #9
	sub x19, x12, x11
	stp x7, x19, [sp, #136]
	lsl x15, x19, #9
	sub x8, x8, x14
	cmp x15, x8
	stp x8, x15, [sp, #152]
	b.ne LBB4_38
	stp x5, x1, [sp, #88]
	stp x17, x16, [sp, #104]
	cmp x12, x11
	b.ne LBB4_7
LBB4_6:
	add x13, x13, #1
	ldp x5, x1, [sp, #88]
	add x5, x5, #64
	ldp x17, x16, [sp, #104]
	sub x17, x17, #64
	add x0, x0, #32, lsl #12
	add x2, x2, #32, lsl #12
	add x16, x16, #64
	sub x1, x1, #64
	cmp x13, #8
	b.ne LBB4_4
	b LBB4_34
LBB4_7:
	mov x11, #0
	add x9, x9, #1
	str x9, [sp, #40]
	mov x12, x0
	ldr x9, [sp, #48]
	add x9, x9, x14, lsl #2
	str x9, [sp, #72]
LBB4_8:
	lsl x9, x11, #9
	add x11, x11, #1
	lsl x1, x11, #9
	cmp x1, x9
	ccmp x1, x8, #2, hs
	b.hi LBB4_35
	mov x9, x12
	mov w14, #512
LBB4_10:
	ldp q1, q2, [x9, #-32]
	ldp q3, q4, [x9]
	fmul.4s v1, v1, v0
	fmul.4s v2, v2, v0
	fmul.4s v3, v3, v0
	fmul.4s v4, v4, v0
	stp q1, q2, [x9, #-32]
	stp q3, q4, [x9], #64
	subs x14, x14, #16
	b.ne LBB4_10
	add x12, x12, #2048
	cmp x11, x10
	b.ne LBB4_8
	mov x5, #0
	mov x22, #0
	lsr x9, x19, #2
	subs x9, x9, #1
	csel x9, xzr, x9, lo
	str x9, [sp, #24]
	mov x24, x2
	mov w11, #64
	ldr x6, [sp, #32]
	stp x0, x2, [sp, #56]
	b LBB4_14
LBB4_13:
	add x22, x22, #1
	add x6, x6, #32, lsl #12
	add x5, x5, #64
	ldp x11, x24, [sp, #120]
	sub x11, x11, #64
	add x24, x24, #256
	cmp x22, #8
	ldp x0, x2, [sp, #56]
	mov w4, #448
	mov w7, #512
	b.eq LBB4_6
LBB4_14:
	cmp x5, #448
	csel x12, x5, x4, lo
	lsl x27, x22, #6
	cmp x27, #448
	csel x9, x27, x4, lo
	add x10, x9, #64
	lsl x9, x10, #9
	sub x9, x9, x22, lsl #15
	sub x14, x10, x27
	stp x7, x14, [sp, #136]
	lsl x14, x14, #9
	stp x9, x14, [sp, #152]
	cmp x14, x9
	b.ne LBB4_37
	cmp x10, x27
	stp x11, x24, [sp, #120]
	mov x10, #0
	b.ne LBB4_20
	ldr x9, [sp, #40]
	ldr x12, [sp, #80]
	b LBB4_18
LBB4_17:
	add x10, x10, #2048
	sub x12, x12, #4
	subs x9, x9, #1
	b.eq LBB4_13
LBB4_18:
	cbz x12, LBB4_17
	add x14, x10, #2047
	cmp x14, x8
	b.lo LBB4_17
	b LBB4_36
LBB4_20:
	add x28, x12, x11
	mov x1, x5
	mov w20, #4
	ldr x12, [sp, #24]
	b LBB4_22
LBB4_21:
	mov x10, x20
	add x20, x20, #4
	sub x12, x0, #1
	add x1, x1, #2048
	add x24, x24, #2, lsl #12
	cbz x0, LBB4_13
LBB4_22:
	mov x0, x12
	cmp x19, x10
	b.eq LBB4_21
	lsl x10, x10, #9
	orr x12, x10, #0x7ff
	cmp x12, x8
	b.hs LBB4_36
	mov x16, #0
	ldr x11, [sp, #72]
	add x23, x11, x10, lsl #2
	add x21, x23, #1, lsl #12
	add x4, x23, #2048
	mov w11, #6144
	add x17, x23, x11
	orr x11, x10, #0x200
	str x11, [sp, #16]
	orr x11, x10, #0x400
	str x11, [sp, #8]
	orr x11, x10, #0x600
	str x11, [sp]
	mov w7, #15
	mov x2, x6
	mov w14, #1
LBB4_25:
	mov x15, #0
	lsl x12, x16, #6
	add x25, x23, x12
	ldp q7, q4, [x25, #32]
	mov x16, x14
	ldp q21, q22, [x25]
	add x14, x4, x12
	add x25, x21, x12
	ldp q16, q5, [x14, #32]
	ldp q19, q20, [x14]
	ldp q6, q3, [x25, #32]
	ldp q17, q18, [x25]
	add x25, x17, x12
	ldp q2, q1, [x25, #32]
	mov x30, x24
	mov x14, x7
	ldp q23, q24, [x25]
	mov x26, x2
LBB4_26:
	cmp x14, x9
	b.hs LBB4_33
	add x25, x26, #1, lsl #12
	prfm pldl1keep, [x25]
	add x25, x1, x15
	cmp x25, x8
	b.hs LBB4_42
	add x25, x25, #512
	cmp x25, x8
	b.hs LBB4_41
	add x25, x1, x15
	add x11, x25, #1024
	cmp x11, x8
	b.hs LBB4_40
	add x11, x25, #1536
	cmp x11, x8
	b.hs LBB4_39
	ldp q26, q25, [x26, #32]
	ldr s27, [x30]
	ldr s28, [x30, #2048]
	ldr s29, [x30, #4096]
	fmov s30, w3
	fmul s27, s27, s30
	fmul s28, s28, s30
	ldr s31, [x30, #6144]
	fmul s29, s29, s30
	fmul s30, s31, s30
	fmla.4s v4, v25, v27[0]
	fmla.4s v7, v26, v27[0]
	ldp q8, q31, [x26]
	fmla.4s v22, v31, v27[0]
	fmla.4s v21, v8, v27[0]
	fmla.4s v5, v25, v28[0]
	fmla.4s v16, v26, v28[0]
	fmla.4s v20, v31, v28[0]
	fmla.4s v19, v8, v28[0]
	fmla.4s v3, v25, v29[0]
	fmla.4s v6, v26, v29[0]
	fmla.4s v18, v31, v29[0]
	fmla.4s v17, v8, v29[0]
	fmla.4s v1, v25, v30[0]
	fmla.4s v2, v26, v30[0]
	fmla.4s v24, v31, v30[0]
	add x15, x15, #1
	fmla.4s v23, v8, v30[0]
	add x26, x26, #2048
	add x14, x14, #512
	add x30, x30, #4
	cmp x28, x15
	b.ne LBB4_26
	add x11, x23, x12
	stp q21, q22, [x11]
	stp q7, q4, [x11, #32]
	add x11, x4, x12
	stp q19, q20, [x11]
	stp q16, q5, [x11, #32]
	add x11, x21, x12
	stp q17, q18, [x11]
	stp q6, q3, [x11, #32]
	add x11, x17, x12
	stp q23, q24, [x11]
	add x2, x2, #64
	add x7, x7, #16
	stp q2, q1, [x11, #32]
	cmp x16, #32
	cinc x14, x16, lo
	b.lo LBB4_25
	b LBB4_21
LBB4_33:
	sub x0, x14, #15
Lloh2:
	adrp x3, l_anon.7ad164de683c838adae784d9fc1e3405.6@PAGE
Lloh3:
	add x3, x3, l_anon.7ad164de683c838adae784d9fc1e3405.6@PAGEOFF
	add x1, x14, #1
	mov x2, x9
	bl core::slice::index::slice_index_fail
LBB4_34:
	.cfi_def_cfa wsp, 352
	ldp x29, x30, [sp, #336]
	ldp x20, x19, [sp, #320]
	ldp x22, x21, [sp, #304]
	ldp x24, x23, [sp, #288]
	ldp x26, x25, [sp, #272]
	ldp x28, x27, [sp, #256]
	ldp d9, d8, [sp, #240]
	add sp, sp, #352
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
LBB4_35:
	.cfi_restore_state
Lloh4:
	adrp x3, l_anon.7ad164de683c838adae784d9fc1e3405.12@PAGE
Lloh5:
	add x3, x3, l_anon.7ad164de683c838adae784d9fc1e3405.12@PAGEOFF
	mov x0, x9
	mov x2, x8
	bl core::slice::index::slice_index_fail
LBB4_36:
Lloh6:
	adrp x3, l_anon.7ad164de683c838adae784d9fc1e3405.7@PAGE
Lloh7:
	add x3, x3, l_anon.7ad164de683c838adae784d9fc1e3405.7@PAGEOFF
	add x1, x10, #2048
	mov x0, x10
	mov x2, x8
	bl core::slice::index::slice_index_fail
LBB4_37:
Lloh8:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh9:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	add x9, sp, #152
	stp x9, x8, [sp, #168]
	add x9, sp, #136
	stp x9, x8, [sp, #184]
	add x9, sp, #144
	stp x9, x8, [sp, #200]
	add x9, sp, #160
	stp x9, x8, [sp, #216]
Lloh10:
	adrp x3, l_anon.7ad164de683c838adae784d9fc1e3405.8@PAGE
Lloh11:
	add x3, x3, l_anon.7ad164de683c838adae784d9fc1e3405.8@PAGEOFF
Lloh12:
	adrp x5, l_anon.7ad164de683c838adae784d9fc1e3405.10@PAGE
Lloh13:
	add x5, x5, l_anon.7ad164de683c838adae784d9fc1e3405.10@PAGEOFF
	add x1, sp, #160
	add x2, sp, #152
	add x4, sp, #168
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
LBB4_38:
Lloh14:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh15:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	add x9, sp, #152
	stp x9, x8, [sp, #168]
	add x9, sp, #136
	stp x9, x8, [sp, #184]
	add x9, sp, #144
	stp x9, x8, [sp, #200]
	add x9, sp, #160
	stp x9, x8, [sp, #216]
Lloh16:
	adrp x3, l_anon.7ad164de683c838adae784d9fc1e3405.8@PAGE
Lloh17:
	add x3, x3, l_anon.7ad164de683c838adae784d9fc1e3405.8@PAGEOFF
Lloh18:
	adrp x5, l_anon.7ad164de683c838adae784d9fc1e3405.11@PAGE
Lloh19:
	add x5, x5, l_anon.7ad164de683c838adae784d9fc1e3405.11@PAGEOFF
	add x1, sp, #160
	add x2, sp, #152
	add x4, sp, #168
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
LBB4_39:
	ldr x9, [sp]
	add x9, x9, x27
	cmp x8, x9
	csel x0, x8, x9, hi
Lloh20:
	adrp x2, l_anon.7ad164de683c838adae784d9fc1e3405.5@PAGE
Lloh21:
	add x2, x2, l_anon.7ad164de683c838adae784d9fc1e3405.5@PAGEOFF
	mov x1, x8
	bl core::panicking::panic_bounds_check
LBB4_40:
	ldr x9, [sp, #8]
	add x9, x9, x27
	cmp x8, x9
	csel x0, x8, x9, hi
Lloh22:
	adrp x2, l_anon.7ad164de683c838adae784d9fc1e3405.4@PAGE
Lloh23:
	add x2, x2, l_anon.7ad164de683c838adae784d9fc1e3405.4@PAGEOFF
	mov x1, x8
	bl core::panicking::panic_bounds_check
LBB4_41:
	ldr x9, [sp, #16]
	add x9, x9, x27
	cmp x8, x9
	csel x0, x8, x9, hi
Lloh24:
	adrp x2, l_anon.7ad164de683c838adae784d9fc1e3405.3@PAGE
Lloh25:
	add x2, x2, l_anon.7ad164de683c838adae784d9fc1e3405.3@PAGEOFF
	mov x1, x8
	bl core::panicking::panic_bounds_check
LBB4_42:
	add x9, x10, x27
	cmp x8, x9
	csel x0, x8, x9, hi
Lloh26:
	adrp x2, l_anon.7ad164de683c838adae784d9fc1e3405.2@PAGE
Lloh27:
	add x2, x2, l_anon.7ad164de683c838adae784d9fc1e3405.2@PAGEOFF
	mov x1, x8
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGot	Lloh8, Lloh9
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpLdrGot	Lloh14, Lloh15
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh26, Lloh27
