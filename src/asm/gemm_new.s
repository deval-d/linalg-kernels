	.globl	<f32 as lak::traits::GemmDispatch>::gemm
	.p2align	2
<f32 as lak::traits::GemmDispatch>::gemm:
Lfunc_begin4:
	.cfi_startproc
	stp d9, d8, [sp, #-112]!
	.cfi_def_cfa_offset 112
	stp x28, x27, [sp, #16]
	stp x26, x25, [sp, #32]
	stp x24, x23, [sp, #48]
	stp x22, x21, [sp, #64]
	stp x20, x19, [sp, #80]
	stp x29, x30, [sp, #96]
	add x29, sp, #96
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
	sub sp, sp, #1040
	str xzr, [sp]
	tbnz w0, #0, LBB4_162
	cbnz w1, LBB4_162
	mov x10, x2
	ldr x23, [x2, #24]
	ldp x8, x1, [x3, #16]
	stp x23, x8, [x29, #-216]
	cmp x23, x8
	b.ne LBB4_166
	lsr x8, x1, #6
	tst x1, #0x3f
	cinc x5, x8, ne
	lsr x8, x23, #6
	tst x23, #0x3f
	cinc x6, x8, ne
	cbz x1, LBB4_144
	mov x7, #0
	mov x19, #0
	ldr x2, [x4, #16]
	lsl x21, x2, #2
	ldp x27, x25, [x10, #8]
	lsr x12, x25, #6
	and x8, x25, #0xffffffffffffffc0
	str x8, [sp, #872]
	lsl x13, x25, #2
	lsl x9, x25, #1
	lsr x22, x25, #4
	sub x24, x21, #4
	lsr x8, x24, #2
	add x26, x8, #1
	str x9, [sp, #504]
	add x8, x9, x25
	str x8, [sp, #232]
	lsl x8, x8, #2
	ldr x28, [x10]
	add x30, x28, x8
	lsl x9, x25, #3
	add x10, x28, x9
	str x10, [sp, #296]
	lsl x10, x12, #8
	mov w11, #32
	str x12, [sp, #600]
	bfi x11, x12, #8, #56
	ldp x15, x14, [x4]
	ldp x16, x12, [x3]
	str x12, [sp, #512]
	add x12, x28, x11
	add x11, x15, x11
	str x11, [sp, #680]
	add x11, x15, x10
	str x11, [sp, #672]
	add x10, x28, x10
	stp x10, x12, [sp, #272]
	mov w10, #12
	madd x10, x23, x10, x16
	str x10, [sp, #632]
	lsl x10, x22, #6
	and x11, x10, #0x7fffffffffffffc0
	add x12, x15, x13
	add x12, x12, x11
	str x12, [sp, #664]
	add x9, x15, x9
	add x9, x9, x11
	str x9, [sp, #656]
	add x8, x11, x8
	add x9, x15, x11
	str x9, [sp, #648]
	add x9, x28, x10
	str x9, [sp, #264]
	add x8, x15, x8
	str x8, [sp, #640]
	and x10, x25, #0x3f
	and x20, x25, #0xf
	and x8, x25, #0x7ffffffffffffff0
	str x8, [sp, #800]
	lsl x8, x23, #2
	and x9, x26, #0x7ffffffffffffff0
	str x9, [sp, #808]
	lsl x9, x9, #2
	stp x10, x9, [sp, #376]
	and x9, x26, #0xc
	str x9, [sp, #536]
	and x9, x26, #0x7ffffffffffffffc
	lsl x10, x9, #2
	str x10, [sp, #744]
	and x10, x25, #0x30
	str x10, [sp, #368]
	and x10, x25, #0xc
	str x10, [sp, #728]
	and x10, x25, #0x3c
	lsl x11, x2, #8
	str x11, [sp, #464]
	lsl x11, x2, #4
	str x11, [sp, #440]
	add x11, x15, #32
	str x11, [sp, #696]
	str x9, [sp, #752]
	neg x9, x9
	str x9, [sp, #736]
	lsl x9, x25, #8
	str x9, [sp, #128]
	lsl x9, x25, #4
	str x9, [sp, #768]
	stur x13, [x29, #-224]
	add x9, x28, x13
	str x9, [sp, #256]
	mov x13, x14
	lsl x9, x23, #1
	str x10, [sp, #488]
	neg x10, x10
	str x10, [sp, #480]
	str x9, [sp, #248]
	add x9, x9, x23
	str x9, [sp, #240]
	lsl x9, x23, #8
	stp x8, x9, [sp, #448]
	lsl x9, x23, #4
	str x9, [sp, #432]
	add x8, x16, x8
	str x8, [sp, #624]
	lsl x11, x25, #6
	mov w14, #64
	str x16, [sp, #616]
	stp x11, x15, [sp, #392]
	str x15, [sp, #688]
	str x16, [sp, #288]
	add x8, x16, x23, lsl #3
	str x8, [sp, #608]
LBB4_5:
	cmp x1, x14
	csel x8, x1, x14, lo
	add x8, x8, x7
	cmp x8, #1
	csinc x9, x8, xzr, hi
	lsl x10, x19, #6
	add x8, x10, #64
	cmp x1, x8
	csel x8, x1, x8, lo
	mul x15, x10, x2
	mul x12, x8, x2
	subs x0, x12, x15
	ccmp x12, x13, #2, hs
	b.hi LBB4_158
	sub x17, x8, x10
	stp x2, x17, [x29, #-200]
	mul x11, x17, x2
	stp x0, x11, [x29, #-184]
	cmp x11, x0
	b.ne LBB4_163
	str x14, [sp, #552]
	mul x14, x10, x23
	mul x16, x8, x23
	subs x12, x16, x14
	ldr x11, [sp, #512]
	ccmp x16, x11, #2, hs
	b.hi LBB4_159
	stp x23, x17, [x29, #-200]
	mul x11, x17, x23
	stp x12, x11, [x29, #-184]
	cmp x11, x12
	b.ne LBB4_164
	cmp x8, x10
	b.ne LBB4_14
	ldr x11, [sp, #392]
	cbz x23, LBB4_143
	mov x15, #0
	mov w8, #64
	mov w9, #1
LBB4_12:
	cmp x23, x8
	csel x10, x23, x8, lo
	mul x16, x10, x25
	cmp x16, x15
	ccmp x16, x27, #2, hs
	b.hi LBB4_151
	add x15, x15, x11
	add x8, x8, #64
	cmp x9, x6
	add x9, x9, #1
	b.lo LBB4_12
	b LBB4_143
LBB4_14:
	ldr x10, [sp, #400]
	add x10, x10, x15, lsl #2
	str x10, [sp, #864]
	str x0, [sp, #784]
	str x17, [sp, #592]
	cbz x2, LBB4_32
	mov x3, x13
	mov x13, #0
	ldr x10, [sp, #688]
	ldr x11, [sp, #696]
	mov w16, #1
	b LBB4_17
LBB4_16:
	add x16, x13, #1
	add x11, x11, x21
	add x10, x10, x21
	cmp x13, x9
	b.eq LBB4_31
LBB4_17:
	mul x15, x13, x2
	mov x13, x16
	mul x16, x16, x2
	cmp x16, x0
	ccmp x16, x15, #0, ls
	b.lo LBB4_152
	ldr x16, [sp, #864]
	add x15, x16, x15, lsl #2
	mov x16, x15
	cmp x24, #12
	b.lo LBB4_29
	cmp x24, #60
	b.hs LBB4_21
	mov x0, #0
	b LBB4_25
LBB4_21:
	mov x16, x11
	ldr x17, [sp, #808]
LBB4_22:
	ldp q2, q3, [x16, #-32]
	ldp q4, q5, [x16]
	fmul.4s v2, v2, v1[0]
	fmul.4s v3, v3, v1[0]
	fmul.4s v4, v4, v1[0]
	fmul.4s v5, v5, v1[0]
	stp q2, q3, [x16, #-32]
	stp q4, q5, [x16], #64
	subs x17, x17, #16
	b.ne LBB4_22
	ldr x16, [sp, #808]
	cmp x26, x16
	b.eq LBB4_16
	ldr x0, [sp, #808]
	ldr x16, [sp, #536]
	cbz x16, LBB4_28
LBB4_25:
	ldr x16, [sp, #744]
	add x16, x15, x16
	ldr x17, [sp, #736]
	add x17, x17, x0
	add x0, x10, x0, lsl #2
LBB4_26:
	ldr q2, [x0]
	fmul.4s v2, v2, v1[0]
	str q2, [x0], #16
	adds x17, x17, #4
	b.ne LBB4_26
	ldr x17, [sp, #752]
	cmp x26, x17
	ldr x0, [sp, #784]
	b.eq LBB4_16
	b LBB4_29
LBB4_28:
	ldr x16, [sp, #384]
	add x16, x15, x16
	ldr x0, [sp, #784]
LBB4_29:
	add x15, x15, x21
LBB4_30:
	ldr s2, [x16]
	fmul s2, s1, s2
	str s2, [x16], #4
	cmp x16, x15
	b.ne LBB4_30
	b LBB4_16
LBB4_31:
	mov x13, x3
	ldr x17, [sp, #592]
LBB4_32:
	cbz x23, LBB4_143
	mov x10, #0
	mov x15, #0
	lsr x9, x17, #2
	tst x8, #0x3
	cinc x8, x9, ne
	str x8, [sp, #224]
	ldr x0, [sp, #616]
	ldr x8, [sp, #624]
	stp x23, x8, [sp, #336]
	ldr x8, [sp, #248]
	str x8, [sp, #328]
	ldr x8, [sp, #608]
	str x8, [sp, #320]
	ldr x8, [sp, #632]
	str x8, [sp, #312]
	ldr x8, [sp, #240]
	str x8, [sp, #304]
	ldp x11, x8, [sp, #264]
	str x8, [sp, #184]
	ldp x8, x9, [sp, #280]
	stp x11, x8, [sp, #168]
	mov w16, #64
	str x28, [sp, #472]
	ldr x11, [sp, #256]
	ldr x8, [sp, #296]
	stp x8, x11, [sp, #208]
	str x30, [sp, #200]
	mov w8, #1
	add x9, x9, x14, lsl #2
	str x9, [sp, #760]
	stp x5, x1, [sp, #80]
	stp x7, x6, [sp, #64]
	stp x24, x19, [sp, #48]
	str x21, [sp, #352]
	str x22, [sp, #776]
	stp x27, x26, [sp, #32]
	stp x30, x28, [sp, #16]
	str x13, [sp, #8]
LBB4_34:
	lsl x3, x15, #6
	stp x16, x8, [sp, #112]
	cmp x23, x16
	csel x8, x23, x16, lo
	add x9, x3, #64
	cmp x23, x9
	csel x11, x23, x9, lo
	mul x15, x3, x25
	mul x16, x11, x25
	subs x14, x16, x15
	ccmp x16, x27, #2, hs
	b.hi LBB4_151
	mov x15, #0
	str x10, [sp, #104]
	add x8, x8, x10
	stur x8, [x29, #-256]
	lsr x8, x8, #2
	lsl x9, x8, #2
	str x9, [sp, #144]
	ldur x9, [x29, #-224]
	mul x9, x9, x8
	str x9, [sp, #136]
	ldr x9, [sp, #768]
	ldr x10, [sp, #472]
	madd x8, x9, x8, x10
	str x8, [sp, #160]
	ldr x8, [sp, #800]
	cmp x8, x25
	cset w9, eq
	ldr x8, [sp, #872]
	cmp x8, x25
	cset w8, ne
	subs x27, x11, x3
	lsr x10, x27, #2
	str x10, [sp, #712]
	and x10, x27, #0xfffffffffffffffc
	stp x10, x11, [sp, #408]
	orr x10, x10, #0x1
	str x10, [sp, #152]
	csel w8, wzr, w8, eq
	str w8, [sp, #364]
	csinc w8, w9, wzr, ne
	str w8, [sp, #360]
	ldr x21, [sp, #640]
	ldr x11, [sp, #656]
	ldr x9, [sp, #664]
	ldr x22, [sp, #648]
	str x0, [sp, #96]
	stur x0, [x29, #-240]
	ldr x8, [sp, #344]
	stur x8, [x29, #-248]
	ldr x8, [sp, #336]
	stur x8, [x29, #-232]
	ldp x28, x0, [sp, #320]
	ldp x13, x30, [sp, #304]
	ldr x8, [sp, #672]
	str x8, [sp, #584]
	ldr x8, [sp, #680]
	str x8, [sp, #576]
	ldr x8, [sp, #696]
	str x8, [sp, #568]
	ldr x8, [sp, #688]
	str x8, [sp, #560]
	mov x10, x3
	ldr x1, [sp, #224]
	mov x26, x3
	str x3, [sp, #720]
	str x27, [sp, #848]
	b LBB4_37
LBB4_36:
	ldp x15, x8, [sp, #440]
	add x10, x10, x8
	ldr x16, [sp, #560]
	add x16, x16, x15
	str x16, [sp, #560]
	ldr x16, [sp, #568]
	add x16, x16, x15
	str x16, [sp, #568]
	ldr x16, [sp, #576]
	add x16, x16, x15
	str x16, [sp, #576]
	ldr x16, [sp, #584]
	add x16, x16, x15
	str x16, [sp, #584]
	add x13, x13, x8
	ldr x15, [sp, #432]
	add x30, x30, x15
	add x28, x28, x15
	add x0, x0, x8
	ldur x16, [x29, #-232]
	add x16, x16, x8
	ldur x8, [x29, #-248]
	add x8, x8, x15
	stur x8, [x29, #-248]
	ldur x8, [x29, #-240]
	add x8, x8, x15
	stp x8, x16, [x29, #-240]
	ldr x8, [sp, #768]
	add x22, x22, x8
	add x9, x9, x8
	add x11, x11, x8
	add x21, x21, x8
	ldr x15, [sp, #520]
	ldr x1, [sp, #528]
	sub x1, x1, #1
	ldr x17, [sp, #592]
	cbz x1, LBB4_142
LBB4_37:
	add x3, x15, #4
	sub x8, x17, x15
	cmp x8, #3
	str x1, [sp, #528]
	str x3, [sp, #520]
	b.ls LBB4_54
	mul x17, x15, x25
	mul x16, x3, x25
	cmp x16, x17
	ldr x8, [sp, #784]
	ccmp x16, x8, #2, hs
	b.hi LBB4_165
	ldur x8, [x29, #-224]
	ldr x16, [sp, #504]
	cmp x16, x8
	b.hi LBB4_167
	tbnz x25, #63, LBB4_168
	cmp x25, #16
	ldr x3, [sp, #776]
	b.lo LBB4_56
	mov x8, #0
	mov x16, #0
	ldr x1, [sp, #864]
	add x1, x1, x17, lsl #2
	ldr x17, [sp, #504]
	add x19, x1, x17, lsl #2
	ldur x17, [x29, #-224]
	add x4, x1, x17
	str x4, [sp, #856]
	add x17, x19, x17
	str x17, [sp, #840]
	ldr x4, [sp, #472]
	mov w6, #1
	b LBB4_44
LBB4_43:
	add x16, x1, x27
	stp q19, q20, [x16]
	stp q5, q4, [x16, #32]
	stp q25, q22, [x26]
	stp q16, q17, [x26, #32]
	stp q24, q21, [x24]
	stp q6, q7, [x24, #32]
	stp q23, q18, [x7, #32]
	add x8, x8, #16
	add x4, x4, #64
	mov x16, x6
	stp q3, q2, [x7]
	ldr x3, [sp, #776]
	cmp x6, x3
	cinc x6, x6, lo
	b.hs LBB4_56
LBB4_44:
	cmp x16, x3
	b.hs LBB4_182
	add x17, x1, x16, lsl #6
	ldp q5, q4, [x17, #32]
	ldp q19, q20, [x17]
	lsl x27, x16, #6
	ldr x16, [sp, #856]
	add x26, x16, x27
	add x24, x19, x27
	ldp q16, q17, [x26, #32]
	ldp q25, q22, [x26]
	ldp q6, q7, [x24, #32]
	ldp q24, q21, [x24]
	ldr x16, [sp, #840]
	add x7, x16, x27
	ldp q23, q18, [x7, #32]
	ldp q3, q2, [x7]
	ldr x16, [sp, #848]
	cbz x16, LBB4_43
	mov x3, #0
	mov x5, x4
	mov x16, x8
LBB4_47:
	add x17, x16, #16
	cmn x16, #17
	b.hi LBB4_145
	cmp x17, x14
	b.hi LBB4_145
	add x17, x10, x3
	cmp x17, x12
	b.hs LBB4_174
	ldur x17, [x29, #-232]
	add x17, x17, x3
	cmp x17, x12
	b.hs LBB4_176
	add x17, x0, x3
	cmp x17, x12
	b.hs LBB4_177
	add x17, x13, x3
	cmp x17, x12
	b.hs LBB4_175
	ldp q27, q26, [x5, #32]
	ldur x17, [x29, #-240]
	ldr s28, [x17, x3, lsl #2]
	ldur x17, [x29, #-248]
	ldr s29, [x17, x3, lsl #2]
	ldr s30, [x28, x3, lsl #2]
	fmul s28, s0, s28
	fmul s29, s0, s29
	ldr s31, [x30, x3, lsl #2]
	fmul s30, s0, s30
	fmul s31, s0, s31
	fmla.4s v5, v27, v28[0]
	ldp q9, q8, [x5]
	fmla.4s v20, v8, v28[0]
	fmla.4s v19, v9, v28[0]
	fmla.4s v4, v26, v28[0]
	fmla.4s v16, v27, v29[0]
	fmla.4s v22, v8, v29[0]
	fmla.4s v25, v9, v29[0]
	fmla.4s v17, v26, v29[0]
	fmla.4s v7, v26, v30[0]
	fmla.4s v6, v27, v30[0]
	fmla.4s v21, v8, v30[0]
	fmla.4s v24, v9, v30[0]
	fmla.4s v18, v26, v31[0]
	fmla.4s v23, v27, v31[0]
	fmla.4s v2, v8, v31[0]
	fmla.4s v3, v9, v31[0]
	add x3, x3, #1
	add x16, x16, x25
	ldur x17, [x29, #-224]
	add x5, x5, x17
	ldur x17, [x29, #-256]
	cmp x17, x3
	b.ne LBB4_47
	b LBB4_43
LBB4_54:
	cmp x17, x15
	b.ls LBB4_36
	orr x8, x15, #0x1
	ldr x6, [sp, #584]
	ldr x16, [sp, #576]
	str x16, [sp, #544]
	ldr x1, [sp, #568]
	ldr x16, [sp, #560]
	str x16, [sp, #704]
	str x23, [sp, #192]
	b LBB4_90
LBB4_56:
	ldr x26, [sp, #720]
	ldr x27, [sp, #848]
	ldr w8, [sp, #360]
	tbnz w8, #0, LBB4_36
	mov x8, #0
	mul x1, x15, x23
	orr x16, x15, #0x1
	mul x19, x16, x23
	orr x16, x15, #0x2
	mul x7, x16, x23
	orr x17, x15, #0x3
	ldr x15, [sp, #168]
	mov w16, #1
	mul x24, x17, x23
	b LBB4_59
LBB4_58:
	add x16, x8, #1
	ldur x17, [x29, #-224]
	add x15, x15, x17
	cmp x8, x27
	b.eq LBB4_36
LBB4_59:
	mov x3, x8
	mov x8, x16
	ldr x16, [sp, #800]
	madd x16, x3, x25, x16
	mul x17, x8, x25
	cmp x17, x14
	ccmp x17, x16, #0, ls
	b.lo LBB4_155
	add x6, x3, x26
	add x3, x6, x1
	cmp x3, x12
	b.hs LBB4_179
	add x4, x6, x19
	cmp x4, x12
	b.hs LBB4_178
	add x5, x6, x7
	cmp x5, x12
	b.hs LBB4_181
	add x6, x6, x24
	cmp x6, x12
	b.hs LBB4_180
	cmp x17, x16
	b.eq LBB4_58
	ldr x16, [sp, #760]
	ldr s5, [x16, x3, lsl #2]
	ldr s4, [x16, x4, lsl #2]
	ldr s3, [x16, x5, lsl #2]
	ldr s2, [x16, x6, lsl #2]
	fmul s5, s0, s5
	cmp x20, #4
	b.hs LBB4_67
	mov x16, #0
	ldr x4, [sp, #728]
	b LBB4_70
LBB4_67:
	mov x16, x15
	mov x17, x22
	ldr x4, [sp, #728]
	mov x3, x4
LBB4_68:
	ldr q6, [x16], #16
	ldr q7, [x17]
	fmla.4s v7, v6, v5[0]
	str q7, [x17], #16
	subs x3, x3, #4
	b.ne LBB4_68
	mov x16, x4
	cmp x20, x4
	b.eq LBB4_71
LBB4_70:
	ldr s6, [x15, x16, lsl #2]
	ldr s7, [x22, x16, lsl #2]
	fmadd s6, s5, s6, s7
	str s6, [x22, x16, lsl #2]
	add x16, x16, #1
	cmp x20, x16
	b.ne LBB4_70
LBB4_71:
	fmul s4, s0, s4
	cmp x20, #4
	b.hs LBB4_73
	mov x16, #0
	b LBB4_76
LBB4_73:
	mov x16, #0
	mov x17, x4
LBB4_74:
	ldr q5, [x15, x16]
	ldr q6, [x9, x16]
	fmla.4s v6, v5, v4[0]
	str q6, [x9, x16]
	add x16, x16, #16
	subs x17, x17, #4
	b.ne LBB4_74
	mov x16, x4
	cmp x20, x4
	b.eq LBB4_77
LBB4_76:
	ldr s5, [x15, x16, lsl #2]
	ldr s6, [x9, x16, lsl #2]
	fmadd s5, s4, s5, s6
	str s5, [x9, x16, lsl #2]
	add x16, x16, #1
	cmp x20, x16
	b.ne LBB4_76
LBB4_77:
	fmul s3, s0, s3
	cmp x20, #4
	b.hs LBB4_79
	mov x16, #0
	b LBB4_82
LBB4_79:
	mov x16, #0
	mov x17, x4
LBB4_80:
	ldr q4, [x15, x16]
	ldr q5, [x11, x16]
	fmla.4s v5, v4, v3[0]
	str q5, [x11, x16]
	add x16, x16, #16
	subs x17, x17, #4
	b.ne LBB4_80
	mov x16, x4
	cmp x20, x4
	b.eq LBB4_83
LBB4_82:
	ldr s4, [x15, x16, lsl #2]
	ldr s5, [x11, x16, lsl #2]
	fmadd s4, s3, s4, s5
	str s4, [x11, x16, lsl #2]
	add x16, x16, #1
	cmp x20, x16
	b.ne LBB4_82
LBB4_83:
	fmul s2, s0, s2
	cmp x20, #4
	b.hs LBB4_85
	mov x16, #0
	b LBB4_88
LBB4_85:
	mov x16, #0
	mov x17, x4
LBB4_86:
	ldr q3, [x15, x16]
	ldr q4, [x21, x16]
	fmla.4s v4, v3, v2[0]
	str q4, [x21, x16]
	add x16, x16, #16
	subs x17, x17, #4
	b.ne LBB4_86
	mov x16, x4
	cmp x20, x4
	b.eq LBB4_58
LBB4_88:
	ldr s3, [x15, x16, lsl #2]
	ldr s4, [x21, x16, lsl #2]
	fmadd s3, s2, s3, s4
	str s3, [x21, x16, lsl #2]
	add x16, x16, #1
	cmp x20, x16
	b.ne LBB4_88
	b LBB4_58
LBB4_89:
	ldr x17, [sp, #424]
	add x8, x17, #1
	ldr x15, [sp, #352]
	ldr x16, [sp, #704]
	add x16, x16, x15
	str x16, [sp, #704]
	ldr x1, [sp, #496]
	add x1, x1, x15
	ldr x16, [sp, #544]
	add x16, x16, x15
	str x16, [sp, #544]
	add x6, x6, x15
	mov x15, x17
	ldr x16, [sp, #592]
	cmp x17, x16
	b.hs LBB4_36
LBB4_90:
	mov x3, x8
	mul x16, x15, x2
	mul x17, x8, x2
	ldr x8, [sp, #784]
	cmp x17, x8
	ccmp x17, x16, #0, ls
	b.lo LBB4_160
	mul x8, x15, x23
	add x16, x8, x26
	ldr x15, [sp, #416]
	add x17, x8, x15
	cmp x17, x16
	ccmp x17, x12, #2, hs
	b.hi LBB4_161
	str x3, [sp, #424]
	str x1, [sp, #496]
	ldr x8, [sp, #760]
	add x8, x8, x16, lsl #2
	str x8, [sp, #856]
	cmp x27, #4
	b.lo LBB4_111
	mov x7, #0
	mov x16, #0
	ldr x8, [sp, #472]
	str x8, [sp, #840]
	mov x1, x25
	ldr x8, [sp, #216]
	str x8, [sp, #832]
	ldr x23, [sp, #504]
	ldr x8, [sp, #208]
	str x8, [sp, #824]
	ldr x15, [sp, #232]
	ldr x8, [sp, #200]
	str x8, [sp, #816]
	mov w8, #1
	b LBB4_95
LBB4_94:
	ldr x16, [sp, #768]
	ldr x8, [sp, #816]
	add x8, x8, x16
	str x8, [sp, #816]
	ldur x8, [x29, #-224]
	add x15, x15, x8
	ldr x17, [sp, #824]
	add x17, x17, x16
	str x17, [sp, #824]
	add x23, x23, x8
	ldr x17, [sp, #832]
	add x17, x17, x16
	str x17, [sp, #832]
	add x1, x1, x8
	ldr x17, [sp, #840]
	add x17, x17, x16
	str x17, [sp, #840]
	add x7, x7, x8
	ldr x8, [sp, #712]
	ldr x16, [sp, #792]
	cmp x16, x8
	cinc x8, x16, lo
	ldr x26, [sp, #720]
	ldr x27, [sp, #848]
	b.hs LBB4_111
LBB4_95:
	lsl x17, x16, #2
	cmp x17, x27
	b.hs LBB4_183
	orr x3, x17, #0x1
	cmp x3, x27
	b.hs LBB4_185
	orr x4, x17, #0x2
	cmp x4, x27
	b.hs LBB4_186
	str x8, [sp, #792]
	orr x5, x17, #0x3
	cmp x5, x27
	b.hs LBB4_184
	cmp x25, #64
	b.lo LBB4_94
	mov x16, #0
	ldr x8, [sp, #856]
	ldr s2, [x8, x17, lsl #2]
	fmul s2, s0, s2
	ldr s3, [x8, x3, lsl #2]
	fmul s3, s0, s3
	ldr s4, [x8, x4, lsl #2]
	fmul s4, s0, s4
	ldr s5, [x8, x5, lsl #2]
	ldr x3, [sp, #840]
	ldr x5, [sp, #832]
	fmul s5, s0, s5
	ldr x24, [sp, #824]
	ldr x8, [sp, #816]
	ldr x26, [sp, #704]
	ldr x27, [sp, #600]
LBB4_101:
	add x17, x7, x16
	add x4, x17, #64
	cmn x17, #65
	b.hi LBB4_146
	cmp x4, x14
	b.hi LBB4_146
	add x19, x1, x16
	add x17, x1, x16
	add x4, x17, #64
	cmn x19, #65
	b.hi LBB4_147
	cmp x4, x14
	b.hi LBB4_147
	add x19, x23, x16
	add x17, x23, x16
	add x4, x17, #64
	cmn x19, #65
	b.hi LBB4_148
	cmp x4, x14
	b.hi LBB4_148
	add x19, x15, x16
	add x17, x15, x16
	add x4, x17, #64
	cmn x19, #65
	b.hi LBB4_149
	cmp x4, x14
	b.hi LBB4_149
	add x17, x16, #63
	cmp x17, x2
	b.hs LBB4_150
	ldp q31, q30, [x3, #224]
	ldp q29, q28, [x3, #192]
	ldp q27, q26, [x3, #160]
	ldp q25, q24, [x3, #128]
	ldp q23, q22, [x3, #96]
	ldp q21, q20, [x3, #64]
	ldp q19, q18, [x3, #32]
	ldp q16, q17, [x3]
	ldp q6, q7, [x26]
	fmla.4s v7, v17, v2[0]
	fmla.4s v6, v16, v2[0]
	ldp q17, q16, [x26, #32]
	fmla.4s v17, v19, v2[0]
	fmla.4s v16, v18, v2[0]
	ldp q19, q18, [x26, #64]
	fmla.4s v19, v21, v2[0]
	fmla.4s v18, v20, v2[0]
	ldp q21, q20, [x26, #96]
	fmla.4s v21, v23, v2[0]
	fmla.4s v20, v22, v2[0]
	ldp q23, q22, [x26, #128]
	fmla.4s v23, v25, v2[0]
	fmla.4s v22, v24, v2[0]
	ldp q25, q24, [x26, #160]
	fmla.4s v25, v27, v2[0]
	fmla.4s v24, v26, v2[0]
	ldp q27, q26, [x26, #192]
	fmla.4s v27, v29, v2[0]
	fmla.4s v26, v28, v2[0]
	ldp q29, q28, [x26, #224]
	fmla.4s v29, v31, v2[0]
	fmla.4s v28, v30, v2[0]
	ldp q31, q30, [x5, #224]
	fmla.4s v28, v30, v3[0]
	fmla.4s v29, v31, v3[0]
	ldp q31, q30, [x5, #192]
	fmla.4s v26, v30, v3[0]
	fmla.4s v27, v31, v3[0]
	ldp q31, q30, [x5, #160]
	fmla.4s v24, v30, v3[0]
	fmla.4s v25, v31, v3[0]
	ldp q31, q30, [x5, #128]
	fmla.4s v22, v30, v3[0]
	fmla.4s v23, v31, v3[0]
	ldp q31, q30, [x5, #96]
	fmla.4s v20, v30, v3[0]
	fmla.4s v21, v31, v3[0]
	ldp q31, q30, [x5, #64]
	fmla.4s v18, v30, v3[0]
	fmla.4s v19, v31, v3[0]
	ldp q31, q30, [x5, #32]
	fmla.4s v16, v30, v3[0]
	fmla.4s v17, v31, v3[0]
	ldp q31, q30, [x5], #256
	fmla.4s v6, v31, v3[0]
	fmla.4s v7, v30, v3[0]
	ldp q31, q30, [x24]
	fmla.4s v7, v30, v4[0]
	fmla.4s v6, v31, v4[0]
	ldp q31, q30, [x24, #32]
	fmla.4s v17, v31, v4[0]
	fmla.4s v16, v30, v4[0]
	ldp q31, q30, [x24, #64]
	fmla.4s v19, v31, v4[0]
	fmla.4s v18, v30, v4[0]
	ldp q31, q30, [x24, #96]
	fmla.4s v21, v31, v4[0]
	fmla.4s v20, v30, v4[0]
	ldp q31, q30, [x24, #128]
	fmla.4s v23, v31, v4[0]
	fmla.4s v22, v30, v4[0]
	ldp q31, q30, [x24, #160]
	fmla.4s v25, v31, v4[0]
	fmla.4s v24, v30, v4[0]
	ldp q31, q30, [x24, #192]
	fmla.4s v27, v31, v4[0]
	fmla.4s v26, v30, v4[0]
	ldp q31, q30, [x24, #224]
	fmla.4s v29, v31, v4[0]
	fmla.4s v28, v30, v4[0]
	ldp q30, q31, [x8, #224]
	fmla.4s v28, v31, v5[0]
	fmla.4s v29, v30, v5[0]
	ldp q30, q31, [x8, #192]
	fmla.4s v26, v31, v5[0]
	fmla.4s v27, v30, v5[0]
	ldp q30, q31, [x8, #160]
	fmla.4s v24, v31, v5[0]
	fmla.4s v25, v30, v5[0]
	ldp q30, q31, [x8, #128]
	fmla.4s v22, v31, v5[0]
	fmla.4s v23, v30, v5[0]
	ldp q30, q31, [x8, #96]
	fmla.4s v20, v31, v5[0]
	fmla.4s v21, v30, v5[0]
	ldp q30, q31, [x8, #64]
	fmla.4s v18, v31, v5[0]
	fmla.4s v19, v30, v5[0]
	ldp q30, q31, [x8, #32]
	fmla.4s v16, v31, v5[0]
	fmla.4s v17, v30, v5[0]
	ldp q31, q30, [x8], #256
	fmla.4s v6, v31, v5[0]
	fmla.4s v7, v30, v5[0]
	stp q6, q7, [x26]
	stp q17, q16, [x26, #32]
	stp q19, q18, [x26, #64]
	stp q21, q20, [x26, #96]
	stp q23, q22, [x26, #128]
	stp q25, q24, [x26, #160]
	stp q27, q26, [x26, #192]
	stp q29, q28, [x26, #224]
	add x16, x16, #64
	add x26, x26, #256
	add x24, x24, #256
	add x3, x3, #256
	subs x27, x27, #1
	b.ne LBB4_101
	b LBB4_94
LBB4_111:
	ldr x8, [sp, #408]
	cmp x8, x27
	ldp x7, x5, [sp, #368]
	ldr x3, [sp, #496]
	b.eq LBB4_124
	ldr x8, [sp, #872]
	cmp x8, x2
	b.hi LBB4_169
	ldp x16, x8, [sp, #152]
	ldr x17, [sp, #408]
	ldr x15, [sp, #872]
	cbz x15, LBB4_120
LBB4_114:
	mov x15, x16
	mul x16, x17, x25
	ldr x1, [sp, #872]
	adds x4, x16, x1
	b.hs LBB4_157
	cmp x4, x14
	b.hi LBB4_157
	cmp x17, x27
	b.hs LBB4_188
	ldr x16, [sp, #856]
	ldr s2, [x16, x17, lsl #2]
	fmul s2, s0, s2
	mov x16, x3
	mov x17, x8
	ldr x1, [sp, #872]
LBB4_118:
	ldp q3, q4, [x17]
	ldp q5, q6, [x17, #32]
	ldp q7, q16, [x16, #-32]
	ldp q17, q18, [x16]
	fmla.4s v7, v3, v2[0]
	fmla.4s v16, v4, v2[0]
	fmla.4s v17, v5, v2[0]
	fmla.4s v18, v6, v2[0]
	stp q7, q16, [x16, #-32]
	stp q17, q18, [x16], #64
	add x17, x17, #64
	subs x1, x1, #16
	b.ne LBB4_118
	add x16, x15, #1
	ldur x17, [x29, #-224]
	add x8, x8, x17
	mov x17, x15
	cmp x15, x27
	b.lo LBB4_114
	b LBB4_124
LBB4_120:
	ldp x16, x8, [sp, #136]
LBB4_121:
	cmp x16, x14
	b.hi LBB4_156
	cmp x8, x27
	b.hs LBB4_187
	add x8, x8, #1
	add x16, x16, x25
	cmp x8, x27
	b.lo LBB4_121
LBB4_124:
	ldr x23, [sp, #192]
	ldr w8, [sp, #364]
	cbz w8, LBB4_89
	cmp x25, x2
	b.hi LBB4_172
	mov x8, #0
	ldp x16, x15, [sp, #176]
	mov w17, #1
	b LBB4_128
LBB4_127:
	add x17, x8, #1
	ldur x1, [x29, #-224]
	add x16, x16, x1
	add x15, x15, x1
	cmp x8, x27
	b.eq LBB4_89
LBB4_128:
	mov x1, x8
	mov x8, x17
	ldr x17, [sp, #872]
	madd x17, x1, x25, x17
	mul x3, x8, x25
	cmp x3, x14
	ccmp x3, x17, #0, ls
	b.lo LBB4_153
	cmp x3, x17
	b.eq LBB4_127
	ldr x17, [sp, #856]
	ldr s2, [x17, x1, lsl #2]
	fmul s2, s0, s2
	cmp x5, #4
	b.hs LBB4_132
	mov x17, #0
	b LBB4_141
LBB4_132:
	cmp x5, #16
	b.hs LBB4_134
	mov x1, #0
	b LBB4_138
LBB4_134:
	ldr x17, [sp, #544]
	mov x1, x16
	mov x3, x7
LBB4_135:
	ldp q3, q4, [x1, #-32]
	ldp q5, q6, [x1], #64
	ldp q7, q16, [x17, #-32]
	ldp q17, q18, [x17]
	fmla.4s v7, v3, v2[0]
	fmla.4s v16, v4, v2[0]
	fmla.4s v17, v5, v2[0]
	fmla.4s v18, v6, v2[0]
	stp q7, q16, [x17, #-32]
	stp q17, q18, [x17], #64
	subs x3, x3, #16
	b.ne LBB4_135
	cmp x5, x7
	b.eq LBB4_127
	mov x1, x7
	mov x17, x7
	ldr x3, [sp, #728]
	cbz x3, LBB4_141
LBB4_138:
	ldr x17, [sp, #480]
	add x17, x17, x1
	lsl x3, x1, #2
	add x1, x6, x3
	add x3, x15, x3
LBB4_139:
	ldr q3, [x3], #16
	ldr q4, [x1]
	fmla.4s v4, v3, v2[0]
	str q4, [x1], #16
	adds x17, x17, #4
	b.ne LBB4_139
	ldr x1, [sp, #488]
	mov x17, x1
	cmp x5, x1
	b.eq LBB4_127
LBB4_141:
	ldr s3, [x15, x17, lsl #2]
	ldr s4, [x6, x17, lsl #2]
	fmadd s3, s2, s3, s4
	str s3, [x6, x17, lsl #2]
	add x17, x17, #1
	cmp x5, x17
	b.ne LBB4_141
	b LBB4_127
LBB4_142:
	ldp x10, x16, [sp, #104]
	sub x10, x10, #64
	ldr x8, [sp, #128]
	ldr x9, [sp, #200]
	add x9, x9, x8
	str x9, [sp, #200]
	ldr x9, [sp, #208]
	add x9, x9, x8
	str x9, [sp, #208]
	ldr x9, [sp, #216]
	add x9, x9, x8
	str x9, [sp, #216]
	ldr x9, [sp, #472]
	add x9, x9, x8
	str x9, [sp, #472]
	add x16, x16, #64
	ldr x9, [sp, #176]
	add x9, x9, x8
	str x9, [sp, #176]
	ldr x9, [sp, #184]
	add x9, x9, x8
	str x9, [sp, #184]
	ldr x9, [sp, #304]
	add x11, x9, #64
	ldr x9, [sp, #312]
	add x9, x9, #256
	stp x11, x9, [sp, #304]
	ldr x9, [sp, #320]
	add x11, x9, #256
	ldr x9, [sp, #328]
	add x9, x9, #64
	stp x11, x9, [sp, #320]
	ldr x9, [sp, #336]
	add x11, x9, #64
	ldp x9, x21, [sp, #344]
	add x9, x9, #256
	stp x11, x9, [sp, #336]
	ldp x1, x0, [sp, #88]
	add x0, x0, #256
	ldr x9, [sp, #168]
	add x9, x9, x8
	str x9, [sp, #168]
	ldp x6, x5, [sp, #72]
	ldr x15, [sp, #120]
	cmp x15, x6
	cinc x8, x15, lo
	ldp x19, x7, [sp, #56]
	ldr x22, [sp, #776]
	ldp x26, x24, [sp, #40]
	ldp x28, x27, [sp, #24]
	ldp x13, x30, [sp, #8]
	b.lo LBB4_34
LBB4_143:
	add x19, x19, #1
	ldr x14, [sp, #552]
	add x14, x14, #64
	sub x7, x7, #64
	ldp x10, x9, [sp, #456]
	ldr x8, [sp, #696]
	add x8, x8, x9
	str x8, [sp, #696]
	ldr x8, [sp, #688]
	add x8, x8, x9
	str x8, [sp, #688]
	ldr x8, [sp, #680]
	add x8, x8, x9
	str x8, [sp, #680]
	ldr x8, [sp, #672]
	add x8, x8, x9
	str x8, [sp, #672]
	ldr x8, [sp, #632]
	add x8, x8, x10
	str x8, [sp, #632]
	ldr x8, [sp, #608]
	add x8, x8, x10
	str x8, [sp, #608]
	ldr x8, [sp, #624]
	add x8, x8, x10
	str x8, [sp, #624]
	ldr x8, [sp, #616]
	add x8, x8, x10
	str x8, [sp, #616]
	ldr x8, [sp, #648]
	add x8, x8, x9
	str x8, [sp, #648]
	ldr x8, [sp, #664]
	add x8, x8, x9
	str x8, [sp, #664]
	ldr x8, [sp, #656]
	add x8, x8, x9
	str x8, [sp, #656]
	ldr x8, [sp, #640]
	add x8, x8, x9
	str x8, [sp, #640]
	cmp x19, x5
	b.ne LBB4_5
LBB4_144:
	add sp, sp, #1040
	.cfi_def_cfa wsp, 112
	ldp x29, x30, [sp, #96]
	ldp x20, x19, [sp, #80]
	ldp x22, x21, [sp, #64]
	ldp x24, x23, [sp, #48]
	ldp x26, x25, [sp, #32]
	ldp x28, x27, [sp, #16]
	ldp d9, d8, [sp], #112
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
LBB4_145:
	.cfi_restore_state
Lloh28:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.48@PAGE
Lloh29:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.48@PAGEOFF
	mov x0, x16
	mov x1, x17
	mov x2, x14
	bl core::slice::index::slice_index_fail
LBB4_146:
Lloh30:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.14@PAGE
Lloh31:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.14@PAGEOFF
	mov x0, x17
	mov x1, x4
	mov x2, x14
	bl core::slice::index::slice_index_fail
LBB4_147:
Lloh32:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.13@PAGE
Lloh33:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.13@PAGEOFF
	mov x0, x17
	mov x1, x4
	mov x2, x14
	bl core::slice::index::slice_index_fail
LBB4_148:
Lloh34:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.12@PAGE
Lloh35:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.12@PAGEOFF
	mov x0, x17
	mov x1, x4
	mov x2, x14
	bl core::slice::index::slice_index_fail
LBB4_149:
Lloh36:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.11@PAGE
Lloh37:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.11@PAGEOFF
	mov x0, x17
	mov x1, x4
	mov x2, x14
	bl core::slice::index::slice_index_fail
LBB4_150:
Lloh38:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.10@PAGE
Lloh39:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.10@PAGEOFF
	add x1, x16, #64
	mov x0, x16
	bl core::slice::index::slice_index_fail
LBB4_151:
Lloh40:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.20@PAGE
Lloh41:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.20@PAGEOFF
	mov x0, x15
	mov x1, x16
	mov x2, x27
	bl core::slice::index::slice_index_fail
LBB4_152:
Lloh42:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.23@PAGE
Lloh43:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.23@PAGEOFF
	mov x2, x0
	mov x0, x15
	mov x1, x16
	bl core::slice::index::slice_index_fail
LBB4_153:
	str x17, [sp, #872]
	mov x25, x3
LBB4_154:
Lloh44:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.2@PAGE
Lloh45:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.2@PAGEOFF
	ldr x0, [sp, #872]
	mov x1, x25
	mov x2, x14
	bl core::slice::index::slice_index_fail
LBB4_155:
Lloh46:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.42@PAGE
Lloh47:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.42@PAGEOFF
	mov x0, x16
	mov x1, x17
	mov x2, x14
	bl core::slice::index::slice_index_fail
LBB4_156:
	mov x4, x16
LBB4_157:
Lloh48:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.5@PAGE
Lloh49:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.5@PAGEOFF
	mov x0, x16
	mov x1, x4
	mov x2, x14
	bl core::slice::index::slice_index_fail
LBB4_158:
Lloh50:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.21@PAGE
Lloh51:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.21@PAGEOFF
	mov x0, x15
	mov x1, x12
	mov x2, x13
	bl core::slice::index::slice_index_fail
LBB4_159:
Lloh52:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.20@PAGE
Lloh53:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.20@PAGEOFF
	mov x0, x14
	mov x1, x16
	ldr x2, [sp, #512]
	bl core::slice::index::slice_index_fail
LBB4_160:
Lloh54:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.23@PAGE
Lloh55:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.23@PAGEOFF
	mov x0, x16
	mov x1, x17
	ldr x2, [sp, #784]
	bl core::slice::index::slice_index_fail
LBB4_161:
Lloh56:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.35@PAGE
Lloh57:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.35@PAGEOFF
	mov x0, x16
	mov x1, x17
	mov x2, x12
	bl core::slice::index::slice_index_fail
LBB4_162:
Lloh58:
	adrp x0, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.70@PAGE
Lloh59:
	add x0, x0, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.70@PAGEOFF
Lloh60:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.72@PAGE
Lloh61:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.72@PAGEOFF
	mov w1, #15
	bl core::panicking::panic
LBB4_163:
Lloh62:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh63:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	sub x9, x29, #184
	stp x9, x8, [x29, #-168]
	sub x9, x29, #200
	stp x9, x8, [x29, #-152]
	sub x9, x29, #192
	stp x9, x8, [x29, #-136]
	sub x9, x29, #176
	stp x9, x8, [x29, #-120]
Lloh64:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.17@PAGE
Lloh65:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.17@PAGEOFF
Lloh66:
	adrp x5, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.22@PAGE
Lloh67:
	add x5, x5, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.22@PAGEOFF
	sub x1, x29, #176
	sub x2, x29, #184
	sub x4, x29, #168
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
LBB4_164:
Lloh68:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh69:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	sub x9, x29, #184
	stp x9, x8, [x29, #-168]
	sub x9, x29, #200
	stp x9, x8, [x29, #-152]
	sub x9, x29, #192
	stp x9, x8, [x29, #-136]
	sub x9, x29, #176
	stp x9, x8, [x29, #-120]
Lloh70:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.17@PAGE
Lloh71:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.17@PAGEOFF
Lloh72:
	adrp x5, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.19@PAGE
Lloh73:
	add x5, x5, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.19@PAGEOFF
	sub x1, x29, #176
	sub x2, x29, #184
	sub x4, x29, #168
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
LBB4_165:
Lloh74:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.49@PAGE
Lloh75:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.49@PAGEOFF
	mov x0, x17
	mov x1, x16
	ldr x2, [sp, #784]
	bl core::slice::index::slice_index_fail
LBB4_166:
Lloh76:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.65@PAGE
Lloh77:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.65@PAGEOFF
Lloh78:
	adrp x5, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.67@PAGE
Lloh79:
	add x5, x5, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.67@PAGEOFF
	sub x1, x29, #216
	sub x2, x29, #208
	mov w0, #0
	mov w4, #69
	bl core::panicking::assert_failed::<usize, usize>
LBB4_167:
Lloh80:
	adrp x0, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.16@PAGE
Lloh81:
	add x0, x0, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.16@PAGEOFF
Lloh82:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.36@PAGE
Lloh83:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.36@PAGEOFF
	mov w1, #19
	bl core::panicking::panic_fmt
LBB4_168:
Lloh84:
	adrp x0, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.16@PAGE
Lloh85:
	add x0, x0, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.16@PAGEOFF
Lloh86:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.37@PAGE
Lloh87:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.37@PAGEOFF
	mov w1, #19
	bl core::panicking::panic_fmt
LBB4_169:
	ldr x8, [sp, #408]
	mul x16, x8, x25
	ldr x8, [sp, #872]
	adds x4, x16, x8
	b.hs LBB4_157
	cmp x4, x14
	b.hi LBB4_157
Lloh88:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.4@PAGE
Lloh89:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.4@PAGEOFF
	mov x0, #0
	ldr x1, [sp, #872]
	bl core::slice::index::slice_index_fail
LBB4_172:
	cmp x25, x14
	b.hi LBB4_154
Lloh90:
	adrp x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.1@PAGE
Lloh91:
	add x3, x3, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.1@PAGEOFF
	ldr x0, [sp, #872]
	mov x1, x25
	bl core::slice::index::slice_index_fail
LBB4_174:
	cmp x12, x10
	csel x0, x12, x10, hi
Lloh92:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.44@PAGE
Lloh93:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.44@PAGEOFF
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB4_175:
Lloh94:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.47@PAGE
Lloh95:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.47@PAGEOFF
	mov x0, x17
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB4_176:
Lloh96:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.45@PAGE
Lloh97:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.45@PAGEOFF
	mov x0, x17
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB4_177:
Lloh98:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.46@PAGE
Lloh99:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.46@PAGEOFF
	mov x0, x17
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB4_178:
Lloh100:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.39@PAGE
Lloh101:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.39@PAGEOFF
	mov x0, x4
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB4_179:
Lloh102:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.38@PAGE
Lloh103:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.38@PAGEOFF
	mov x0, x3
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB4_180:
Lloh104:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.41@PAGE
Lloh105:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.41@PAGEOFF
	mov x0, x6
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB4_181:
Lloh106:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.40@PAGE
Lloh107:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.40@PAGEOFF
	mov x0, x5
	mov x1, x12
	bl core::panicking::panic_bounds_check
LBB4_182:
Lloh108:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.43@PAGE
Lloh109:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.43@PAGEOFF
	mov x0, x16
	ldr x1, [sp, #776]
	bl core::panicking::panic_bounds_check
LBB4_183:
Lloh110:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.6@PAGE
Lloh111:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.6@PAGEOFF
	mov x0, x17
	mov x1, x27
	bl core::panicking::panic_bounds_check
LBB4_184:
Lloh112:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.9@PAGE
Lloh113:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.9@PAGEOFF
	mov x0, x5
	ldr x1, [sp, #848]
	bl core::panicking::panic_bounds_check
LBB4_185:
Lloh114:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.7@PAGE
Lloh115:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.7@PAGEOFF
	mov x0, x3
	mov x1, x27
	bl core::panicking::panic_bounds_check
LBB4_186:
Lloh116:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.8@PAGE
Lloh117:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.8@PAGEOFF
	mov x0, x4
	mov x1, x27
	bl core::panicking::panic_bounds_check
LBB4_187:
	mov x17, x27
LBB4_188:
Lloh118:
	adrp x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.3@PAGE
Lloh119:
	add x2, x2, l_anon.0704fd6fbbc155aa5451c8c4c9b7014d.3@PAGEOFF
	mov x0, x17
	ldr x1, [sp, #848]
	bl core::panicking::panic_bounds_check
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh38, Lloh39
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh50, Lloh51
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh58, Lloh59
	.loh AdrpAdd	Lloh66, Lloh67
	.loh AdrpAdd	Lloh64, Lloh65
	.loh AdrpLdrGot	Lloh62, Lloh63
	.loh AdrpAdd	Lloh72, Lloh73
	.loh AdrpAdd	Lloh70, Lloh71
	.loh AdrpLdrGot	Lloh68, Lloh69
	.loh AdrpAdd	Lloh74, Lloh75
	.loh AdrpAdd	Lloh78, Lloh79
	.loh AdrpAdd	Lloh76, Lloh77
	.loh AdrpAdd	Lloh82, Lloh83
	.loh AdrpAdd	Lloh80, Lloh81
	.loh AdrpAdd	Lloh86, Lloh87
	.loh AdrpAdd	Lloh84, Lloh85
	.loh AdrpAdd	Lloh88, Lloh89
	.loh AdrpAdd	Lloh90, Lloh91
	.loh AdrpAdd	Lloh92, Lloh93
	.loh AdrpAdd	Lloh94, Lloh95
	.loh AdrpAdd	Lloh96, Lloh97
	.loh AdrpAdd	Lloh98, Lloh99
	.loh AdrpAdd	Lloh100, Lloh101
	.loh AdrpAdd	Lloh102, Lloh103
	.loh AdrpAdd	Lloh104, Lloh105
	.loh AdrpAdd	Lloh106, Lloh107
	.loh AdrpAdd	Lloh108, Lloh109
	.loh AdrpAdd	Lloh110, Lloh111
	.loh AdrpAdd	Lloh112, Lloh113
	.loh AdrpAdd	Lloh114, Lloh115
	.loh AdrpAdd	Lloh116, Lloh117
	.loh AdrpAdd	Lloh118, Lloh119
