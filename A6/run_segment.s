
target/inspect/rum:	file format mach-o arm64

Disassembly of section __TEXT,__text:

0000000100004960 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E>:
100004960: d10303ff    	sub	sp, sp, #0xc0
100004964: a9066ffc    	stp	x28, x27, [sp, #0x60]
100004968: a90767fa    	stp	x26, x25, [sp, #0x70]
10000496c: a9085ff8    	stp	x24, x23, [sp, #0x80]
100004970: a90957f6    	stp	x22, x21, [sp, #0x90]
100004974: a90a4ff4    	stp	x20, x19, [sp, #0xa0]
100004978: a90b7bfd    	stp	x29, x30, [sp, #0xb0]
10000497c: 9102c3fd    	add	x29, sp, #0xb0
100004980: f9400801    	ldr	x1, [x0, #0x10]
100004984: b40024e1    	cbz	x1, 0x100004e20 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x4c0>
100004988: aa0003f3    	mov	x19, x0
10000498c: 91012018    	add	x24, x0, #0x48
100004990: d2f00019    	mov	x25, #-0x8000000000000000 ; =-9223372036854775808
100004994: d00001da    	adrp	x26, 0x10003e000 <GCC_except_table1055+0x18>
100004998: 911b435a    	add	x26, x26, #0x6d0
10000499c: 52800317    	mov	w23, #0x18              ; =24
1000049a0: 910073fb    	add	x27, sp, #0x1c
1000049a4: 900001bc    	adrp	x28, 0x100038000 <__ZN4core3fmt9Formatter3pad17h7e01c7b66e02bd16E+0x338>
1000049a8: 911d939c    	add	x28, x28, #0x764
1000049ac: 52800088    	mov	w8, #0x4                ; =4
1000049b0: f90007e8    	str	x8, [sp, #0x8]
1000049b4: 1400000a    	b	0x1000049dc <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x7c>
1000049b8: d3462109    	ubfx	x9, x8, #6, #3
1000049bc: d343150a    	ubfx	x10, x8, #3, #3
1000049c0: b86a7b0a    	ldr	w10, [x24, x10, lsl #2]
1000049c4: 92400908    	and	x8, x8, #0x7
1000049c8: b8687b08    	ldr	w8, [x24, x8, lsl #2]
1000049cc: 1b0a7d08    	mul	w8, w8, w10
1000049d0: b8297b08    	str	w8, [x24, x9, lsl #2]
1000049d4: f9400a61    	ldr	x1, [x19, #0x10]
1000049d8: b4002241    	cbz	x1, 0x100004e20 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x4c0>
1000049dc: f9400669    	ldr	x9, [x19, #0x8]
1000049e0: f9400128    	ldr	x8, [x9]
1000049e4: eb19011f    	cmp	x8, x25
1000049e8: 54002100    	b.eq	0x100004e08 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x4a8>
1000049ec: f940366a    	ldr	x10, [x19, #0x68]
1000049f0: f9400928    	ldr	x8, [x9, #0x10]
1000049f4: eb08015f    	cmp	x10, x8
1000049f8: 54001f82    	b.hs	0x100004de8 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x488>
1000049fc: f9400528    	ldr	x8, [x9, #0x8]
100004a00: b86a7908    	ldr	w8, [x8, x10, lsl #2]
100004a04: 9100054a    	add	x10, x10, #0x1
100004a08: f900366a    	str	x10, [x19, #0x68]
100004a0c: d35c7d0a    	ubfx	x10, x8, #28, #4
100004a10: b9001bea    	str	w10, [sp, #0x18]
100004a14: 7100355f    	cmp	w10, #0xd
100004a18: 540025a8    	b.hi	0x100004ecc <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x56c>
100004a1c: 2a0a03ea    	mov	w10, w10
100004a20: 10fffccb    	adr	x11, 0x1000049b8 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x58>
100004a24: 786a7b4c    	ldrh	w12, [x26, x10, lsl #1]
100004a28: 8b0c096b    	add	x11, x11, x12, lsl #2
100004a2c: d61f0160    	br	x11
100004a30: 92400909    	and	x9, x8, #0x7
100004a34: b8697b09    	ldr	w9, [x24, x9, lsl #2]
100004a38: 34fffce9    	cbz	w9, 0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004a3c: 53031509    	ubfx	w9, w8, #3, #3
100004a40: 53062108    	ubfx	w8, w8, #6, #3
100004a44: b8695b09    	ldr	w9, [x24, w9, uxtw #2]
100004a48: b8285b09    	str	w9, [x24, w8, uxtw #2]
100004a4c: 17ffffe2    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004a50: 12000914    	and	w20, w8, #0x7
100004a54: 390143ff    	strb	wzr, [sp, #0x50]
100004a58: 9400706e    	bl	0x100020c10 <__ZN3std2io5stdio5stdin17h721148d9aa8b02f2E>
100004a5c: f90013e0    	str	x0, [sp, #0x20]
100004a60: 910083e0    	add	x0, sp, #0x20
100004a64: 910143e1    	add	x1, sp, #0x50
100004a68: 52800022    	mov	w2, #0x1                ; =1
100004a6c: 94007078    	bl	0x100020c4c <__ZN55_$LT$std..io..stdio..Stdin$u20$as$u20$std..io..Read$GT$10read_exact17h6a2df4b973c5df92E>
100004a70: b4001780    	cbz	x0, 0x100004d60 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x400>
100004a74: aa0003f5    	mov	x21, x0
100004a78: 12800008    	mov	w8, #-0x1               ; =-1
100004a7c: b8345b08    	str	w8, [x24, w20, uxtw #2]
100004a80: 92400408    	and	x8, x0, #0x3
100004a84: f100051f    	cmp	x8, #0x1
100004a88: 54fffa61    	b.ne	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004a8c: f85ffeb6    	ldr	x22, [x21, #-0x1]!
100004a90: f94006b4    	ldr	x20, [x21, #0x8]
100004a94: f9400288    	ldr	x8, [x20]
100004a98: b4000068    	cbz	x8, 0x100004aa4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x144>
100004a9c: aa1603e0    	mov	x0, x22
100004aa0: d63f0100    	blr	x8
100004aa4: f9400681    	ldr	x1, [x20, #0x8]
100004aa8: b4000081    	cbz	x1, 0x100004ab8 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x158>
100004aac: f9400a82    	ldr	x2, [x20, #0x10]
100004ab0: aa1603e0    	mov	x0, x22
100004ab4: 9400041a    	bl	0x100005b1c <___rust_dealloc>
100004ab8: aa1503e0    	mov	x0, x21
100004abc: 52800301    	mov	w1, #0x18               ; =24
100004ac0: 52800102    	mov	w2, #0x8                ; =8
100004ac4: 94000416    	bl	0x100005b1c <___rust_dealloc>
100004ac8: 17ffffc3    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004acc: 12000908    	and	w8, w8, #0x7
100004ad0: d37e7d08    	ubfiz	x8, x8, #2, #32
100004ad4: 38686b08    	ldrb	w8, [x24, x8]
100004ad8: b9001fe8    	str	w8, [sp, #0x1c]
100004adc: a90573fb    	stp	x27, x28, [sp, #0x50]
100004ae0: d00001c8    	adrp	x8, 0x10003e000 <GCC_except_table1055+0x18>
100004ae4: 91204109    	add	x9, x8, #0x810
100004ae8: 52800028    	mov	w8, #0x1                ; =1
100004aec: a90223e9    	stp	x9, x8, [sp, #0x20]
100004af0: a903ffe8    	stp	x8, xzr, [sp, #0x38]
100004af4: 910143e9    	add	x9, sp, #0x50
100004af8: f9001be9    	str	x9, [sp, #0x30]
100004afc: 910083e0    	add	x0, sp, #0x20
100004b00: 940073cf    	bl	0x100021a3c <__ZN3std2io5stdio6_print17h9989baa6d5472142E>
100004b04: 940070f1    	bl	0x100020ec8 <__ZN3std2io5stdio6stdout17h6c3c59aabf03c7ddE>
100004b08: f9002be0    	str	x0, [sp, #0x50]
100004b0c: 910143e0    	add	x0, sp, #0x50
100004b10: 940070fd    	bl	0x100020f04 <__ZN57_$LT$std..io..stdio..Stdout$u20$as$u20$std..io..Write$GT$5flush17h97bb46b7907f6bbeE>
100004b14: b4fff600    	cbz	x0, 0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004b18: 140000e2    	b	0x100004ea0 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x540>
100004b1c: d346210a    	ubfx	x10, x8, #6, #3
100004b20: b86a7b00    	ldr	w0, [x24, x10, lsl #2]
100004b24: eb00003f    	cmp	x1, x0
100004b28: 54001b09    	b.ls	0x100004e88 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x528>
100004b2c: 9bb72409    	umaddl	x9, w0, w23, x9
100004b30: f940012a    	ldr	x10, [x9]
100004b34: eb19015f    	cmp	x10, x25
100004b38: 54001840    	b.eq	0x100004e40 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x4e0>
100004b3c: d343150a    	ubfx	x10, x8, #3, #3
100004b40: b86a7b00    	ldr	w0, [x24, x10, lsl #2]
100004b44: f9400921    	ldr	x1, [x9, #0x10]
100004b48: eb00003f    	cmp	x1, x0
100004b4c: 54001989    	b.ls	0x100004e7c <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x51c>
100004b50: 92400908    	and	x8, x8, #0x7
100004b54: b8687b08    	ldr	w8, [x24, x8, lsl #2]
100004b58: f9400529    	ldr	x9, [x9, #0x8]
100004b5c: b8207928    	str	w8, [x9, x0, lsl #2]
100004b60: 17ffff9d    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004b64: d3462109    	ubfx	x9, x8, #6, #3
100004b68: d343150a    	ubfx	x10, x8, #3, #3
100004b6c: b86a7b0a    	ldr	w10, [x24, x10, lsl #2]
100004b70: 92400908    	and	x8, x8, #0x7
100004b74: b8687b08    	ldr	w8, [x24, x8, lsl #2]
100004b78: 0b0a0108    	add	w8, w8, w10
100004b7c: 17ffff95    	b	0x1000049d0 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x70>
100004b80: d343150a    	ubfx	x10, x8, #3, #3
100004b84: b86a7b00    	ldr	w0, [x24, x10, lsl #2]
100004b88: eb00003f    	cmp	x1, x0
100004b8c: 54001669    	b.ls	0x100004e58 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x4f8>
100004b90: 9bb72409    	umaddl	x9, w0, w23, x9
100004b94: f940012a    	ldr	x10, [x9]
100004b98: eb19015f    	cmp	x10, x25
100004b9c: 54001640    	b.eq	0x100004e64 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x504>
100004ba0: 9240090a    	and	x10, x8, #0x7
100004ba4: b86a7b00    	ldr	w0, [x24, x10, lsl #2]
100004ba8: f9400921    	ldr	x1, [x9, #0x10]
100004bac: eb00003f    	cmp	x1, x0
100004bb0: 54001429    	b.ls	0x100004e34 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x4d4>
100004bb4: d3462108    	ubfx	x8, x8, #6, #3
100004bb8: f9400529    	ldr	x9, [x9, #0x8]
100004bbc: b8607929    	ldr	w9, [x9, x0, lsl #2]
100004bc0: b8287b09    	str	w9, [x24, x8, lsl #2]
100004bc4: 17ffff84    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004bc8: 53031514    	ubfx	w20, w8, #3, #3
100004bcc: 12000908    	and	w8, w8, #0x7
100004bd0: b8685b01    	ldr	w1, [x24, w8, uxtw #2]
100004bd4: aa1303e0    	mov	x0, x19
100004bd8: 97fffece    	bl	0x100004710 <__ZN3rum2vm14VirtualMachine11map_segment17h19aafd444a88f054E>
100004bdc: b8345b00    	str	w0, [x24, w20, uxtw #2]
100004be0: 17ffff7d    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004be4: 92400909    	and	x9, x8, #0x7
100004be8: b8697b09    	ldr	w9, [x24, x9, lsl #2]
100004bec: 34001549    	cbz	w9, 0x100004e94 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x534>
100004bf0: 5303150a    	ubfx	w10, w8, #3, #3
100004bf4: b86a5b0a    	ldr	w10, [x24, w10, uxtw #2]
100004bf8: 53062108    	ubfx	w8, w8, #6, #3
100004bfc: 1ac90949    	udiv	w9, w10, w9
100004c00: b8285b09    	str	w9, [x24, w8, uxtw #2]
100004c04: 17ffff74    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004c08: d3462109    	ubfx	x9, x8, #6, #3
100004c0c: 1200090a    	and	w10, w8, #0x7
100004c10: 53031508    	ubfx	w8, w8, #3, #3
100004c14: b8685b08    	ldr	w8, [x24, w8, uxtw #2]
100004c18: b86a5b0a    	ldr	w10, [x24, w10, uxtw #2]
100004c1c: 0a080148    	and	w8, w10, w8
100004c20: 2a2803e8    	mvn	w8, w8
100004c24: 17ffff6b    	b	0x1000049d0 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x70>
100004c28: 12006109    	and	w9, w8, #0x1ffffff
100004c2c: 53196d08    	ubfx	w8, w8, #25, #3
100004c30: b8285b09    	str	w9, [x24, w8, uxtw #2]
100004c34: 17ffff68    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004c38: 12000908    	and	w8, w8, #0x7
100004c3c: b8685b01    	ldr	w1, [x24, w8, uxtw #2]
100004c40: aa1303e0    	mov	x0, x19
100004c44: 97ffff16    	bl	0x10000489c <__ZN3rum2vm14VirtualMachine13unmap_segment17h829a5e138342ecc0E>
100004c48: 17ffff63    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004c4c: 1200090a    	and	w10, w8, #0x7
100004c50: 53031508    	ubfx	w8, w8, #3, #3
100004c54: b8685b00    	ldr	w0, [x24, w8, uxtw #2]
100004c58: 340007e0    	cbz	w0, 0x100004d54 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x3f4>
100004c5c: eb00003f    	cmp	x1, x0
100004c60: 54001549    	b.ls	0x100004f08 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x5a8>
100004c64: 9bb7241b    	umaddl	x27, w0, w23, x9
100004c68: f9400368    	ldr	x8, [x27]
100004c6c: eb19011f    	cmp	x8, x25
100004c70: 540015e0    	b.eq	0x100004f2c <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x5cc>
100004c74: aa1c03f4    	mov	x20, x28
100004c78: f9402268    	ldr	x8, [x19, #0x40]
100004c7c: b90017ea    	str	w10, [sp, #0x14]
100004c80: b4000108    	cbz	x8, 0x100004ca0 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x340>
100004c84: d1000508    	sub	x8, x8, #0x1
100004c88: f9002268    	str	x8, [x19, #0x40]
100004c8c: f9401e69    	ldr	x9, [x19, #0x38]
100004c90: 9b172508    	madd	x8, x8, x23, x9
100004c94: f940011c    	ldr	x28, [x8]
100004c98: eb19039f    	cmp	x28, x25
100004c9c: 54000681    	b.ne	0x100004d6c <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x40c>
100004ca0: f9400b7c    	ldr	x28, [x27, #0x10]
100004ca4: d37ef796    	lsl	x22, x28, #2
100004ca8: d37eff88    	lsr	x8, x28, #62
100004cac: f100011f    	cmp	x8, #0x0
100004cb0: b27ef3e8    	mov	x8, #0x7ffffffffffffffc ; =9223372036854775804
100004cb4: fa4802c2    	ccmp	x22, x8, #0x2, eq
100004cb8: 54001508    	b.hi	0x100004f58 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x5f8>
100004cbc: b4000836    	cbz	x22, 0x100004dc0 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x460>
100004cc0: 90000268    	adrp	x8, 0x100050000 <dyld_stub_binder+0x100050000>
100004cc4: 910ea508    	add	x8, x8, #0x3a9
100004cc8: 3940011f    	ldrb	wzr, [x8]
100004ccc: aa1603e0    	mov	x0, x22
100004cd0: 52800081    	mov	w1, #0x4                ; =4
100004cd4: 94000391    	bl	0x100005b18 <___rust_alloc>
100004cd8: b4001440    	cbz	x0, 0x100004f60 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x600>
100004cdc: aa0003f5    	mov	x21, x0
100004ce0: f90003e0    	str	x0, [sp]
100004ce4: a90203fc    	stp	x28, x0, [sp, #0x20]
100004ce8: f9001bff    	str	xzr, [sp, #0x30]
100004cec: a940db77    	ldp	x23, x22, [x27, #0x8]
100004cf0: eb16039f    	cmp	x28, x22
100004cf4: 54000483    	b.lo	0x100004d84 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x424>
100004cf8: d280001b    	mov	x27, #0x0               ; =0
100004cfc: d37ef6c2    	lsl	x2, x22, #2
100004d00: 8b1b0aa0    	add	x0, x21, x27, lsl #2
100004d04: aa1703e1    	mov	x1, x23
100004d08: 9400e078    	bl	0x10003cee8 <dyld_stub_binder+0x10003cee8>
100004d0c: f9400a68    	ldr	x8, [x19, #0x10]
100004d10: b4001028    	cbz	x8, 0x100004f14 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x5b4>
100004d14: f9400677    	ldr	x23, [x19, #0x8]
100004d18: f94002e8    	ldr	x8, [x23]
100004d1c: eb19011f    	cmp	x8, x25
100004d20: 540000c0    	b.eq	0x100004d38 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x3d8>
100004d24: b40000a8    	cbz	x8, 0x100004d38 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x3d8>
100004d28: f94006e0    	ldr	x0, [x23, #0x8]
100004d2c: d37ef501    	lsl	x1, x8, #2
100004d30: 52800082    	mov	w2, #0x4                ; =4
100004d34: 9400037a    	bl	0x100005b1c <___rust_dealloc>
100004d38: 8b160368    	add	x8, x27, x22
100004d3c: a90056fc    	stp	x28, x21, [x23]
100004d40: f9000ae8    	str	x8, [x23, #0x10]
100004d44: 52800317    	mov	w23, #0x18              ; =24
100004d48: 910073fb    	add	x27, sp, #0x1c
100004d4c: aa1403fc    	mov	x28, x20
100004d50: b94017ea    	ldr	w10, [sp, #0x14]
100004d54: b86a5b08    	ldr	w8, [x24, w10, uxtw #2]
100004d58: f9003668    	str	x8, [x19, #0x68]
100004d5c: 17ffff1e    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004d60: 394143e8    	ldrb	w8, [sp, #0x50]
100004d64: b8345b08    	str	w8, [x24, w20, uxtw #2]
100004d68: 17ffff1b    	b	0x1000049d4 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x74>
100004d6c: f9400515    	ldr	x21, [x8, #0x8]
100004d70: a90257fc    	stp	x28, x21, [sp, #0x20]
100004d74: f9001bff    	str	xzr, [sp, #0x30]
100004d78: a940db77    	ldp	x23, x22, [x27, #0x8]
100004d7c: eb16039f    	cmp	x28, x22
100004d80: 54fffbc2    	b.hs	0x100004cf8 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x398>
100004d84: 910083e0    	add	x0, sp, #0x20
100004d88: d2800001    	mov	x1, #0x0                ; =0
100004d8c: aa1603e2    	mov	x2, x22
100004d90: 52800083    	mov	w3, #0x4                ; =4
100004d94: 52800084    	mov	w4, #0x4                ; =4
100004d98: 9400db0a    	bl	0x10003b9c0 <__ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17hf464de7356e78eb4E>
100004d9c: a942eff5    	ldp	x21, x27, [sp, #0x28]
100004da0: f94013fc    	ldr	x28, [sp, #0x20]
100004da4: d37ef6c2    	lsl	x2, x22, #2
100004da8: 8b1b0aa0    	add	x0, x21, x27, lsl #2
100004dac: aa1703e1    	mov	x1, x23
100004db0: 9400e04e    	bl	0x10003cee8 <dyld_stub_binder+0x10003cee8>
100004db4: f9400a68    	ldr	x8, [x19, #0x10]
100004db8: b5fffae8    	cbnz	x8, 0x100004d14 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x3b4>
100004dbc: 14000056    	b	0x100004f14 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x5b4>
100004dc0: d280001c    	mov	x28, #0x0               ; =0
100004dc4: 52800095    	mov	w21, #0x4               ; =4
100004dc8: 52800088    	mov	w8, #0x4                ; =4
100004dcc: f90003e8    	str	x8, [sp]
100004dd0: a90257fc    	stp	x28, x21, [sp, #0x20]
100004dd4: f9001bff    	str	xzr, [sp, #0x30]
100004dd8: a940db77    	ldp	x23, x22, [x27, #0x8]
100004ddc: eb16039f    	cmp	x28, x22
100004de0: 54fff8c2    	b.hs	0x100004cf8 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x398>
100004de4: 17ffffe8    	b	0x100004d84 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x424>
100004de8: a94b7bfd    	ldp	x29, x30, [sp, #0xb0]
100004dec: a94a4ff4    	ldp	x20, x19, [sp, #0xa0]
100004df0: a94957f6    	ldp	x22, x21, [sp, #0x90]
100004df4: a9485ff8    	ldp	x24, x23, [sp, #0x80]
100004df8: a94767fa    	ldp	x26, x25, [sp, #0x70]
100004dfc: a9466ffc    	ldp	x28, x27, [sp, #0x60]
100004e00: 910303ff    	add	sp, sp, #0xc0
100004e04: d65f03c0    	ret
100004e08: d00001c0    	adrp	x0, 0x10003e000 <GCC_except_table1055+0x18>
100004e0c: 911c8000    	add	x0, x0, #0x720
100004e10: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e14: 9103e042    	add	x2, x2, #0xf8
100004e18: 52800221    	mov	w1, #0x11               ; =17
100004e1c: 9400deb0    	bl	0x10003c8dc <__ZN4core6option13expect_failed17h1c87388c17a818f4E>
100004e20: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e24: 91038042    	add	x2, x2, #0xe0
100004e28: d2800000    	mov	x0, #0x0                ; =0
100004e2c: d2800001    	mov	x1, #0x0                ; =0
100004e30: 9400deff    	bl	0x10003ca2c <__ZN4core9panicking18panic_bounds_check17h4d54449b19f7f1d3E>
100004e34: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e38: 91050042    	add	x2, x2, #0x140
100004e3c: 9400defc    	bl	0x10003ca2c <__ZN4core9panicking18panic_bounds_check17h4d54449b19f7f1d3E>
100004e40: d00001c0    	adrp	x0, 0x10003e000 <GCC_except_table1055+0x18>
100004e44: 911cc400    	add	x0, x0, #0x731
100004e48: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e4c: 9105c042    	add	x2, x2, #0x170
100004e50: 528001e1    	mov	w1, #0xf                ; =15
100004e54: 9400dea2    	bl	0x10003c8dc <__ZN4core6option13expect_failed17h1c87388c17a818f4E>
100004e58: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e5c: 91044042    	add	x2, x2, #0x110
100004e60: 9400def3    	bl	0x10003ca2c <__ZN4core9panicking18panic_bounds_check17h4d54449b19f7f1d3E>
100004e64: d00001c0    	adrp	x0, 0x10003e000 <GCC_except_table1055+0x18>
100004e68: 911cc400    	add	x0, x0, #0x731
100004e6c: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e70: 9104a042    	add	x2, x2, #0x128
100004e74: 528001e1    	mov	w1, #0xf                ; =15
100004e78: 9400de99    	bl	0x10003c8dc <__ZN4core6option13expect_failed17h1c87388c17a818f4E>
100004e7c: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e80: 91062042    	add	x2, x2, #0x188
100004e84: 9400deea    	bl	0x10003ca2c <__ZN4core9panicking18panic_bounds_check17h4d54449b19f7f1d3E>
100004e88: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e8c: 91056042    	add	x2, x2, #0x158
100004e90: 9400dee7    	bl	0x10003ca2c <__ZN4core9panicking18panic_bounds_check17h4d54449b19f7f1d3E>
100004e94: 90000240    	adrp	x0, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004e98: 91068000    	add	x0, x0, #0x1a0
100004e9c: 9400df84    	bl	0x10003ccac <__ZN4core9panicking11panic_const23panic_const_div_by_zero17h7d3ef343e11e6945E>
100004ea0: f90013e0    	str	x0, [sp, #0x20]
100004ea4: d00001c0    	adrp	x0, 0x10003e000 <GCC_except_table1055+0x18>
100004ea8: 911bb000    	add	x0, x0, #0x6ec
100004eac: 90000243    	adrp	x3, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004eb0: 91004063    	add	x3, x3, #0x10
100004eb4: 90000244    	adrp	x4, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004eb8: 9106e084    	add	x4, x4, #0x1b8
100004ebc: 910083e2    	add	x2, sp, #0x20
100004ec0: 52800561    	mov	w1, #0x2b               ; =43
100004ec4: 9400df3c    	bl	0x10003cbb4 <__ZN4core6result13unwrap_failed17h47e2afd53f1d6de6E>
100004ec8: 14000018    	b	0x100004f28 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x5c8>
100004ecc: f00001a8    	adrp	x8, 0x10003b000 <__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$3fmt17hbdb2109e8331f94aE+0x74>
100004ed0: 91122108    	add	x8, x8, #0x488
100004ed4: 910063e9    	add	x9, sp, #0x18
100004ed8: a90523e9    	stp	x9, x8, [sp, #0x50]
100004edc: 90000248    	adrp	x8, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004ee0: 91034108    	add	x8, x8, #0xd0
100004ee4: 52800029    	mov	w9, #0x1                ; =1
100004ee8: a90227e8    	stp	x8, x9, [sp, #0x20]
100004eec: a903ffe9    	stp	x9, xzr, [sp, #0x38]
100004ef0: 910143e8    	add	x8, sp, #0x50
100004ef4: f9001be8    	str	x8, [sp, #0x30]
100004ef8: 90000241    	adrp	x1, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004efc: 9108c021    	add	x1, x1, #0x230
100004f00: 910083e0    	add	x0, sp, #0x20
100004f04: 9400de88    	bl	0x10003c924 <__ZN4core9panicking9panic_fmt17ha0f8363f677e0181E>
100004f08: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004f0c: 91074042    	add	x2, x2, #0x1d0
100004f10: 9400dec7    	bl	0x10003ca2c <__ZN4core9panicking18panic_bounds_check17h4d54449b19f7f1d3E>
100004f14: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004f18: 91080042    	add	x2, x2, #0x200
100004f1c: d2800000    	mov	x0, #0x0                ; =0
100004f20: d2800001    	mov	x1, #0x0                ; =0
100004f24: 9400dec2    	bl	0x10003ca2c <__ZN4core9panicking18panic_bounds_check17h4d54449b19f7f1d3E>
100004f28: d4200020    	brk	#0x1
100004f2c: 90000248    	adrp	x8, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004f30: 91030108    	add	x8, x8, #0xc0
100004f34: 52800029    	mov	w9, #0x1                ; =1
100004f38: a90227e8    	stp	x8, x9, [sp, #0x20]
100004f3c: 52800108    	mov	w8, #0x8                ; =8
100004f40: a903ffff    	stp	xzr, xzr, [sp, #0x38]
100004f44: f9001be8    	str	x8, [sp, #0x30]
100004f48: 90000241    	adrp	x1, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004f4c: 91086021    	add	x1, x1, #0x218
100004f50: 910083e0    	add	x0, sp, #0x20
100004f54: 9400de74    	bl	0x10003c924 <__ZN4core9panicking9panic_fmt17ha0f8363f677e0181E>
100004f58: f90007ff    	str	xzr, [sp, #0x8]
100004f5c: f94003f6    	ldr	x22, [sp]
100004f60: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004f64: 9107a042    	add	x2, x2, #0x1e8
100004f68: f94007e0    	ldr	x0, [sp, #0x8]
100004f6c: aa1603e1    	mov	x1, x22
100004f70: 9400de37    	bl	0x10003c84c <__ZN5alloc7raw_vec12handle_error17h601d29026320c7bcE>
100004f74: aa0003f3    	mov	x19, x0
100004f78: f94013e8    	ldr	x8, [sp, #0x20]
100004f7c: b4000428    	cbz	x8, 0x100005000 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x6a0>
100004f80: f94017e0    	ldr	x0, [sp, #0x28]
100004f84: d37ef501    	lsl	x1, x8, #2
100004f88: 52800082    	mov	w2, #0x4                ; =4
100004f8c: 940002e4    	bl	0x100005b1c <___rust_dealloc>
100004f90: aa1303e0    	mov	x0, x19
100004f94: 9400df81    	bl	0x10003cd98 <dyld_stub_binder+0x10003cd98>
100004f98: aa0003f3    	mov	x19, x0
100004f9c: f9400681    	ldr	x1, [x20, #0x8]
100004fa0: b4000081    	cbz	x1, 0x100004fb0 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x650>
100004fa4: f9400a82    	ldr	x2, [x20, #0x10]
100004fa8: aa1603e0    	mov	x0, x22
100004fac: 940002dc    	bl	0x100005b1c <___rust_dealloc>
100004fb0: aa1503e0    	mov	x0, x21
100004fb4: 52800301    	mov	w1, #0x18               ; =24
100004fb8: 52800102    	mov	w2, #0x8                ; =8
100004fbc: 940002d8    	bl	0x100005b1c <___rust_dealloc>
100004fc0: aa1303e0    	mov	x0, x19
100004fc4: 9400df75    	bl	0x10003cd98 <dyld_stub_binder+0x10003cd98>
100004fc8: aa0003f3    	mov	x19, x0
100004fcc: b2410388    	orr	x8, x28, #0x8000000000000000
100004fd0: d2f00009    	mov	x9, #-0x8000000000000000 ; =-9223372036854775808
100004fd4: eb09011f    	cmp	x8, x9
100004fd8: 54000140    	b.eq	0x100005000 <__ZN3rum2vm14VirtualMachine3run17h7b66852acc16a513E+0x6a0>
100004fdc: d37ef781    	lsl	x1, x28, #2
100004fe0: aa1503e0    	mov	x0, x21
100004fe4: 52800082    	mov	w2, #0x4                ; =4
100004fe8: 940002cd    	bl	0x100005b1c <___rust_dealloc>
100004fec: aa1303e0    	mov	x0, x19
100004ff0: 9400df6a    	bl	0x10003cd98 <dyld_stub_binder+0x10003cd98>
100004ff4: aa0003f3    	mov	x19, x0
100004ff8: 910083e0    	add	x0, sp, #0x20
100004ffc: 97fffd0d    	bl	0x100004430 <__ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hae97c4af285e5b21E>
100005000: aa1303e0    	mov	x0, x19
100005004: 9400df65    	bl	0x10003cd98 <dyld_stub_binder+0x10003cd98>
100005008: 9400dea3    	bl	0x10003ca94 <__ZN4core9panicking16panic_in_cleanup17h08fe812c23ada610E>
