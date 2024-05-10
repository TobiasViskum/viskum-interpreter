// Assembler program to print "Hello World!"
// to stdout.
//
// X0-X7 - parameters to linux function services
// X16 - linux function number
//
.global _main
.align 2

// Setup the parameters to print hello world
// and then call Linux to do it.

_main:
        adr X1, hellome
        mov X2, #10
        bl _printf
        
        adr X1, helloworld
        mov X2, #13
        bl _printf

        b _terminate

// Setup the parameters to exit the program
// and then call Linux to do it.

        

_printf:
        mov X0, #1 // 1 = StdOut
        mov X16, #4 // MacOS write system call
        svc 0 // Call MacOS
        ret // Return to caller

_terminate:
        mov X0, #0 // Use 0 return code
        mov X16, #1 // Service command code 1 terminates this program
        svc 0 // Call MacOS

helloworld: .ascii  "Hello World!\n"
hellome: .ascii  "Hello Me!\n"

// To compile, link and execute
// as -o ./asm/out.o ./asm/out.s
// ld ./asm/out.o -o ./asm/executable
// ./asm/executable
// as -o ./asm/out.o ./asm/out.s && ld ./asm/out.o -o ./asm/executable && ./asm/executable

// Links:
// https://opensource.apple.com/source/xnu/xnu-1504.3.12/bsd/kern/syscalls.master
// https://github.com/Haruno19/m1-assembly?tab=readme-ov-file
// https://developer.arm.com/documentation/
// https://www.youtube.com/watch?v=vhyettT7sdA

// Tutorial with chapters:
// https://github.com/below/HelloSilicon/tree/main

