.section __TEXT,__text,regular,pure_instructions
	.p2align	2
sgemm_asm::sgemm_direct:
Lfunc_begin4:
	.cfi_startproc
	sub sp, sp, #304
	.cfi_def_cfa_offset 304
	stp d9, d8, [sp, #192]
	stp x28, x27, [sp, #208]
	stp x26, x25, [sp, #224]
	stp x24, x23, [sp, #240]
	stp x22, x21, [sp, #256]
	stp x20, x19, [sp, #272]
	stp x29, x30, [sp, #288]
	add x29, sp, #288
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
	mov w8, #512
	stp x8, x8, [sp, #104]
	str x1, [sp, #40]
	mov w8, #262144
	str x8, [sp, #72]
	cmp x1, #64, lsl #12
	b.ne LBB4_12
	mov w8, #512
	stp x8, x8, [sp, #104]
	str x3, [sp, #40]
	mov w8, #262144
	str x8, [sp, #72]
	cmp x3, #64, lsl #12
	b.ne LBB4_12
	mov w8, #512
	stp x8, x8, [sp, #104]
	str x5, [sp, #40]
	mov w8, #262144
	str x8, [sp, #72]
	cmp x5, #64, lsl #12
	b.ne LBB4_13
	mov x19, x4
	mov x20, x2
	mov x21, x0
	mov x23, #0
	add x24, x4, #32
	mov w8, #52429
	movk w8, #15820, lsl #16
	dup.4s v4, w8
Lloh2:
	adrp x9, lCPI4_0@PAGE
Lloh3:
	ldr q5, [x9, lCPI4_0@PAGEOFF]
	mov w25, #64
	fmov s8, w8
	stp q4, q5, [sp]
LBB4_4:
	mov x8, #0
	lsl x9, x23, #15
	mov x10, x24
	add x26, x19, x23, lsl #17
LBB4_5:
	mov x11, x10
	mov w12, #512
LBB4_6:
	ldp q0, q1, [x11, #-32]
	ldp q2, q3, [x11]
	fmul.4s v0, v0, v4
	fmul.4s v1, v1, v4
	fmul.4s v2, v2, v4
	fmul.4s v3, v3, v4
	stp q0, q1, [x11, #-32]
	stp q2, q3, [x11], #64
	subs x12, x12, #16
	b.ne LBB4_6
	add x8, x8, #1
	add x10, x10, #2048
	cmp x8, #64
	b.ne LBB4_5
	mov x22, #0
	add x27, x20, x9, lsl #2
	mov x28, x21
LBB4_9:
	str x28, [sp, #40]
	stur q5, [sp, #48]
	stp x25, x27, [sp, #64]
	stur q5, [sp, #80]
	str x25, [sp, #96]
	str x26, [sp, #120]
	stur q5, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov x5, #0
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #4
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #8
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #12
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #16
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #20
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #24
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #28
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #32
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #36
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #40
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #44
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #48
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #52
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #56
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	str x26, [sp, #120]
	ldr q0, [sp, #16]
	stur q0, [sp, #128]
	str x25, [sp, #144]
	add x0, sp, #40
	add x1, sp, #72
	add x2, sp, #120
	fmov d0, d8
	mov x3, x22
	mov w4, #64
	mov w5, #60
	mov w6, #512
	mov w7, #512
	bl lak::l3::gemm::nn_microkernel::skernel_mrnr
	ldr q5, [sp, #16]
	add x22, x22, #64
	add x28, x28, #32, lsl #12
	cmp x22, #512
	b.ne LBB4_9
	add x23, x23, #1
	add x24, x24, #32, lsl #12
	cmp x23, #8
	ldr q4, [sp]
	b.ne LBB4_4
	.cfi_def_cfa wsp, 304
	ldp x29, x30, [sp, #288]
	ldp x20, x19, [sp, #272]
	ldp x22, x21, [sp, #256]
	ldp x24, x23, [sp, #240]
	ldp x26, x25, [sp, #224]
	ldp x28, x27, [sp, #208]
	ldp d9, d8, [sp, #192]
	add sp, sp, #304
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
LBB4_12:
	.cfi_restore_state
Lloh4:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh5:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	add x9, sp, #40
	stp x9, x8, [sp, #120]
	add x9, sp, #104
	stp x9, x8, [sp, #136]
	add x9, sp, #112
	stp x9, x8, [sp, #152]
	add x9, sp, #72
	stp x9, x8, [sp, #168]
Lloh6:
	adrp x3, l_anon.b9ccbc4d3ffe3db7f020f58e849ab456.1@PAGE
Lloh7:
	add x3, x3, l_anon.b9ccbc4d3ffe3db7f020f58e849ab456.1@PAGEOFF
Lloh8:
	adrp x5, l_anon.b9ccbc4d3ffe3db7f020f58e849ab456.3@PAGE
Lloh9:
	add x5, x5, l_anon.b9ccbc4d3ffe3db7f020f58e849ab456.3@PAGEOFF
	add x1, sp, #72
	add x2, sp, #40
	add x4, sp, #120
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
LBB4_13:
Lloh10:
	adrp x8, <usize as core::fmt::Display>::fmt@GOTPAGE
Lloh11:
	ldr x8, [x8, <usize as core::fmt::Display>::fmt@GOTPAGEOFF]
	add x9, sp, #40
	stp x9, x8, [sp, #120]
	add x9, sp, #104
	stp x9, x8, [sp, #136]
	add x9, sp, #112
	stp x9, x8, [sp, #152]
	add x9, sp, #72
	stp x9, x8, [sp, #168]
Lloh12:
	adrp x3, l_anon.b9ccbc4d3ffe3db7f020f58e849ab456.1@PAGE
Lloh13:
	add x3, x3, l_anon.b9ccbc4d3ffe3db7f020f58e849ab456.1@PAGEOFF
Lloh14:
	adrp x5, l_anon.b9ccbc4d3ffe3db7f020f58e849ab456.4@PAGE
Lloh15:
	add x5, x5, l_anon.b9ccbc4d3ffe3db7f020f58e849ab456.4@PAGEOFF
	add x1, sp, #72
	add x2, sp, #40
	add x4, sp, #120
	mov w0, #0
	bl core::panicking::assert_failed::<usize, usize>
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpLdrGot	Lloh4, Lloh5
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpLdrGot	Lloh10, Lloh11
