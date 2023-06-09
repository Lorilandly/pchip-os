CROSSCOMPILE?=riscv64-unknown-elf-
OBJCOPY=${CROSSCOMPILE}objcopy # TODO: use objcopy from rust toolchain to avoid need to compile riscv tools

# change this directory to match path on your machine
BRAM_DIR=/home/loriland/Documents/pchip_fpga/riscv_code/bram_init/

.PHONY: cp_bram
cp_bram: gen_bram 
	cp conform_15_0.mem conform_31_16.mem conform_47_32.mem conform_63_48.mem $(BRAM_DIR)

gen_bram: pchip_os.hex
		./bram_mem_15_0 $^ > ./conform_15_0.mem
		./bram_mem_31_16 $^ > ./conform_31_16.mem
		./bram_mem_47_32 $^ > ./conform_47_32.mem
		./bram_mem_63_48 $^ > ./conform_63_48.mem

target/riscv64gc-unknown-none-elf/release/pchip_os: FORCE
	cargo b --release

%.bin: target/riscv64gc-unknown-none-elf/release/pchip_os
	$(OBJCOPY) -S -R .comment -R .note.gnu.build-id -O binary $^ $@
	
%.hex: %.bin
	hexdump -v -e '/4 "%08X" "\n"' $^ > $@

FORCE: ;

clean::
	cargo clean
	rm -f *.mem *.bin *.hex