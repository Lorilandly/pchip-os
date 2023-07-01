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
  sd ra, 120(sp)
  sd sp, 128(sp)
  sd gp, 136(sp)
  sd tp, 144(sp)
  sd s0, 152(sp)
  sd s1, 160(sp)
  sd s2, 168(sp)
  sd s3, 176(sp)
  sd s4, 184(sp)
  sd s5, 192(sp)
  sd s6, 200(sp)
  sd s7, 208(sp)
  sd s8, 216(sp)
  sd s9, 224(sp)
  sd s10, 232(sp)
  sd s11, 240(sp)
  mv a0, sp
  csrr a1, mscratch
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
  ld ra, 120(sp)
  ld sp, 128(sp)
  ld gp, 136(sp)
  ld tp, 144(sp)
  ld s0, 152(sp)
  ld s1, 160(sp)
  ld s2, 168(sp)
  ld s3, 176(sp)
  ld s4, 184(sp)
  ld s5, 192(sp)
  ld s6, 200(sp)
  ld s7, 208(sp)
  ld s8, 216(sp)
  ld s9, 224(sp)
  ld s10, 232(sp)
  ld s11, 240(sp)
  addi sp, sp, 248
  mret

.global syscall
syscall:
  mv a7, a6
  ecall
  ret
