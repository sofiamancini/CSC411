
target/inspect/rum:	file format mach-o arm64

Disassembly of section __TEXT,__text:

000000010000489c <__ZN3rum2vm14VirtualMachine13unmap_segment17h829a5e138342ecc0E>:
10000489c: a9bd57f6    	stp	x22, x21, [sp, #-0x30]!
1000048a0: a9014ff4    	stp	x20, x19, [sp, #0x10]
1000048a4: a9027bfd    	stp	x29, x30, [sp, #0x20]
1000048a8: 910083fd    	add	x29, sp, #0x20
1000048ac: 2a0103f3    	mov	w19, w1
1000048b0: f9400801    	ldr	x1, [x0, #0x10]
1000048b4: eb13003f    	cmp	x1, x19
1000048b8: 540004c9    	b.ls	0x100004950 <__ZN3rum2vm14VirtualMachine13unmap_segment17h829a5e138342ecc0E+0xb4>
1000048bc: aa0003f4    	mov	x20, x0
1000048c0: f9400408    	ldr	x8, [x0, #0x8]
1000048c4: 52800309    	mov	w9, #0x18               ; =24
1000048c8: 9ba92275    	umaddl	x21, w19, w9, x8
1000048cc: f94002a8    	ldr	x8, [x21]
1000048d0: d2f00016    	mov	x22, #-0x8000000000000000 ; =-9223372036854775808
1000048d4: eb16011f    	cmp	x8, x22
1000048d8: fa401904    	ccmp	x8, #0x0, #0x4, ne
1000048dc: 54000241    	b.ne	0x100004924 <__ZN3rum2vm14VirtualMachine13unmap_segment17h829a5e138342ecc0E+0x88>
1000048e0: f90002b6    	str	x22, [x21]
1000048e4: aa1403e0    	mov	x0, x20
1000048e8: f8418c08    	ldr	x8, [x0, #0x18]!
1000048ec: f9400815    	ldr	x21, [x0, #0x10]
1000048f0: eb0802bf    	cmp	x21, x8
1000048f4: 54000081    	b.ne	0x100004904 <__ZN3rum2vm14VirtualMachine13unmap_segment17h829a5e138342ecc0E+0x68>
1000048f8: 90000241    	adrp	x1, 0x10004c000 <dyld_stub_binder+0x10004c000>
1000048fc: 9102a021    	add	x1, x1, #0xa8
100004900: 94000375    	bl	0x1000056d4 <__ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h7a5af934f1dcf825E>
100004904: f9401288    	ldr	x8, [x20, #0x20]
100004908: f8357913    	str	x19, [x8, x21, lsl #3]
10000490c: 910006a8    	add	x8, x21, #0x1
100004910: f9001688    	str	x8, [x20, #0x28]
100004914: a9427bfd    	ldp	x29, x30, [sp, #0x20]
100004918: a9414ff4    	ldp	x20, x19, [sp, #0x10]
10000491c: a8c357f6    	ldp	x22, x21, [sp], #0x30
100004920: d65f03c0    	ret
100004924: f94006a0    	ldr	x0, [x21, #0x8]
100004928: d37ef501    	lsl	x1, x8, #2
10000492c: 52800082    	mov	w2, #0x4                ; =4
100004930: 9400047b    	bl	0x100005b1c <___rust_dealloc>
100004934: f90002b6    	str	x22, [x21]
100004938: aa1403e0    	mov	x0, x20
10000493c: f8418c08    	ldr	x8, [x0, #0x18]!
100004940: f9400815    	ldr	x21, [x0, #0x10]
100004944: eb0802bf    	cmp	x21, x8
100004948: 54fffd80    	b.eq	0x1000048f8 <__ZN3rum2vm14VirtualMachine13unmap_segment17h829a5e138342ecc0E+0x5c>
10000494c: 17ffffee    	b	0x100004904 <__ZN3rum2vm14VirtualMachine13unmap_segment17h829a5e138342ecc0E+0x68>
100004950: 90000242    	adrp	x2, 0x10004c000 <dyld_stub_binder+0x10004c000>
100004954: 91024042    	add	x2, x2, #0x90
100004958: aa1303e0    	mov	x0, x19
10000495c: 9400e034    	bl	0x10003ca2c <__ZN4core9panicking18panic_bounds_check17h4d54449b19f7f1d3E>
