.option norvc

.section .text.init

.globl _start
_start:
.option push
.option norelax
  la  gp, _global_pointer
.option pop
  // Set stack pointer
  csrr t0, mhartid // get hartid (cpu id)
  slli  t0, t0, 12
  la    sp, _stack_end
  sub   sp, sp, t0

  bnez t0, 3f // if not hart 0, go here
  
  // interrupt goes here
  la t0, trap_entry
  csrw mtvec, t0


  // Load data section
  la t0, _data_lma
  la t1, _data_start
  beq t0, t1, 2f
  la t2, _data_end
  bgeu t1, t2, 2f
1:
  ld t3, 0(t0)
  sd t3, 0(t1)
  addi t0, t0, 8
  addi t1, t1, 8
  bltu t1, t2, 1b
2:

  // Zero BSS section
  la 		t0, _bss_start
	la		t1, _bss_end
	bgeu	t0, t1, 2f
1:
	sd		zero, (t0)
	addi	t0, t0, 8
	bltu	t0, t1, 1b
2:

  call main

  li t0, 0x80000000
  csrr a0, mhartid
  jr t0

  .align 2
trap_entry:
  addi sp, sp, -128
  sd a0, 8(sp)
  sd a1, 16(sp)
  sd a2, 24(sp)
  sd a3, 32(sp)
  sd a4, 40(sp)
  sd a5, 48(sp)
  sd a6, 56(sp)
  sd a7, 64(sp)
  sd t0, 72(sp)
  sd t1, 80(sp)
  sd t2, 88(sp)
  sd t3, 96(sp)
  sd t4, 104(sp)
  sd t5, 112(sp)
  sd ra, 120(sp)
  csrw mscratch, t6
  call trap_handle
  csrw mepc, a0
  csrr t6, mscratch
  ld a0, 8(sp)
  ld a1, 16(sp)
  ld a2, 24(sp)
  ld a3, 32(sp)
  ld a4, 40(sp)
  ld a5, 48(sp)
  ld a6, 56(sp)
  ld a7, 64(sp)
  ld t0, 72(sp)
  ld t1, 80(sp)
  ld t2, 88(sp)
  ld t3, 96(sp)
  ld t4, 104(sp)
  ld t5, 112(sp)
  ld ra, 120(sp)
  addi sp, sp, 128
  mret

3:
  wfi
  j 3b