use core::arch::global_asm;

// Include the assembly files from here
global_asm!(include_str!("asm/boot.S"));
global_asm!(include_str!("asm/trap.S"));
