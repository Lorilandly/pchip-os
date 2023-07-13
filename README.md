# PCHIP OS
An Operating system built for the Pchip.

## Building
### Running on qemu
```bash
cargo run
```
This will launch a serial port on your machine, and you will need to use a serial port console to connect to it. 

If you wish to try the OS directly on your terminal, edit `.cargo/config.toml`: change `pty` to `stdio` on line 7.

### Running on Pchip
The OS will run on the Pchip, but be aware that due to the various bug on the Pchip, the OS will not run correctly as of now.

#### Option 1: Run the OS from BRAM

ue to the size limit of BRAM (4k), PCHIP OS doesn't fit in BRAM anymore. If you wish to follow these steps, make sure you delete some code so that it is smaller than 4kb.

1. Change the linker script.

    In `.cargo/config.toml`, change the `rustflags` to use the pchip linker file: 
    
    ```rustflags = ['-Clink-arg=-Tsrc/lds/pchip.lds', '-Ctarget-feature=-c']```
    
2. Change the driver

    In `src/uart.rs`, change all instances of `virt_uart` to `axi_uart_lite`

3. generate the `.mem` files

    ```bash
    make gen_bram
    ```

    This will create 4 `.mem` files under the project root. These files can then be written into BRAM of the Bitstream using the Vivado tools. This is the same as the `sd.c` project. There should be a `Makefile` that does the rest of the conversions. Please ask Suezuku-San for details. 
        
#### Option 2: Running the OS via Bootloader

Warning: I have not tried this as a bootloader isn't available on pchip yet.
    
Make sure your bootloader puts the code at address `0x8000_0000`. If so, you should be able to keep using the `virt.ld` linker file. Otherwise you will need to write your own linker file. You will still need to change the UART driver as described above.

## Using PCHIP OS
After the OS is booted, the following message is shown.
```
Hello world!
File status: false
Please select an Operation:
    h. Show this help message
    0. Trigger Breakpoint
```
After uploading a file/executable, the complete list will be shown
```
Please select an Operation:
    h. Show this help message
    0. Trigger Breakpoint
    1. Recieve a file
    2. Execute
    3. Show file in plain text
    4. Show file in hex
    5. Print value at memory address
```

### Uploading files
This is tested with `minicom` on Linux.

1. Press `1` to initiate Xmodem Receiver on PCHIP OS. You should NOT press any keys on your keyboard, as Xmodem will be listening for a Xmodem Sender on UART.
2. To initiate Xmodem Sending in `minicom`, press `ctrl-a` and then `s`. Select `Xmodem` in the pop up window, and then choose the file you want to send. The transfer will start shortly.
3. After the file is sent, PCHIP OS will show the checksum of the received file. To verify the cksum on your host machine, you can execute `cksum [FILENAME]` in your terminal. If the checksum is correct, enter `yes` and the transfer will be complete.

### Compile program to run on Pchip OS

Compile the program with:

- Base address at 0x9000_0000.
- Convert to binary format (elf is default)
- Call syscall 0 to exit the program

C example:

1. Add a system call at the end of the program. The system call will exit from the program and return control to the OS. 
    Code for the system call:
    ```c
    asm volatile ("li a0, 0; ecall;");
    ```
2. Compile the program with base address of `0x90000000` and convert to binary. 

    Example Makefile:
    ```makefile
    CROSSCOMPILE?=riscv64-unknown-elf-
    CC=${CROSSCOMPILE}gcc
    LD=${CROSSCOMPILE}ld
    OBJCOPY=${CROSSCOMPILE}objcopy
    CFLAGS=-ffreestanding
    LDFLAGS=-Ttext=0x90000000 -nostdlib

    main.bin: main.elf
	    $(OBJCOPY) -S -O binary $^ $@

    main.elf: main.o
	    $(LD) $(LDFLAGS) -o $@ $^

    %.o: %.c
	    $(CC) $(CFLAGS) -c -o $@ $<

    clean::
	    rm *.o *.elf *.bin
    ```

That's it! Compile your program and upload the .bin file to run in PCHIP OS!
