  .align 2
trap_entry:
  csrw mscratch, sp
  addi sp, sp, -248
  sd a0, 0(sp)
  sd a1, 8(sp)
  sd a2, 16(sp)
  sd a3, 24(sp)
  sd a4, 32(sp)
  sd a5, 40(sp)
  sd a6, 48(sp)
  sd a7, 56(sp)
  sd t0, 64(sp)
  sd t1, 72(sp)
  sd t2, 80(sp)
  sd t3, 88(sp)
  sd t4, 96(sp)
  sd t5, 104(sp)
  sd t6, 112(sp)
  sd s0, 120(sp)
  sd s1, 128(sp)
  sd s2, 136(sp)
  sd s3, 144(sp)
  sd s4, 152(sp)
  sd s5, 160(sp)
  sd s6, 168(sp)
  sd s7, 176(sp)
  sd s8, 184(sp)
  sd s9, 192(sp)
  sd s10,200(sp)
  sd s11,208(sp)
  sd ra, 216(sp)
  sd gp, 224(sp)
  sd tp, 232(sp)
  csrr t0, mscratch
  sd t0, 240(sp)
  mv a0, sp
  call trap_handle
  csrw mepc, a0
  ld a0, 0(sp)
  ld a1, 8(sp)
  ld a2, 16(sp)
  ld a3, 24(sp)
  ld a4, 32(sp)
  ld a5, 40(sp)
  ld a6, 48(sp)
  ld a7, 56(sp)
  ld t0, 64(sp)
  ld t1, 72(sp)
  ld t2, 80(sp)
  ld t3, 88(sp)
  ld t4, 96(sp)
  ld t5, 104(sp)
  ld t6, 112(sp)
  ld s0, 120(sp)
  ld s1, 128(sp)
  ld s2, 136(sp)
  ld s3, 144(sp)
  ld s4, 152(sp)
  ld s5, 160(sp)
  ld s6, 168(sp)
  ld s7, 176(sp)
  ld s8, 184(sp)
  ld s9, 192(sp)
  ld s10,200(sp)
  ld s11,208(sp)
  ld ra, 216(sp)
  ld gp, 224(sp)
  ld tp, 232(sp)
  ld sp, 240(sp)
  mret

.global syscall
syscall:
  mv a7, a6
  ecall
  ret
