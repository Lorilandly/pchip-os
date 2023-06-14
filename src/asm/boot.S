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

3:
  wfi
  j 3b