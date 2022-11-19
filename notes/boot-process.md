The Boot Process
- When you turn on a computer;
  - it begins executing firmware(BIOS/UEFI) code that is stored in motherboard ROM. This firmware performs a power-on self-test, detects available RAM, and pre-initializes the CPU and hardware routines.
  - Afterwards, it looks for a bootable disk and starts booting the operating system kernel. Control is transferred to the bootloader which is a 512-byte portion of executable code stored at the disk beginning. hat location is the first sector of the disk (cylinder 0, head 0, sector 0) and it takes 512 bytes. To make sure that the "disk is bootable", the BIOS checks that bytes 511 and 512 of the alleged boot sector are bytes 0xAA55  (beware of endianness, x86 is little-endian). Most bootloaders are larger than 512 bytes, so bootloaders are commonly split into a small first stage, which fits into 512 bytes, and a second stage, which is subsequently loaded by the first stage.


On x86, there are two firmware standards: the “Basic Input/Output System“ (BIOS) and the newer “Unified Extensible Firmware Interface” (UEFI). The BIOS standard is old and outdated, but simple and well-supported on any x86 machine since the 1980s. UEFI, in contrast, is more modern and has much more features

Real mode:
also called real address mode, is an operating mode of all x86-compatible CPUs. The mode gets its name from the fact that addresses in real mode always correspond to real locations in memory. Real mode is characterized by a 20-bit segmented memory address space (giving 1 MB of addressable memory) and unlimited direct software access to all addressable memory, I/O addresses and peripheral hardware. Real mode provides no support for memory protection, multitasking, or code privilege levels.
Protected mode:
also called protected virtual address mode,[1] is an operational mode of x86-compatible central processing units (CPUs). It allows system software to use features such as virtual memory, paging and safe multi-tasking designed to increase an operating system's control over application software. When a processor that supports x86 protected mode is powered on, it begins executing instructions in real mode, in order to maintain backward compatibility with earlier x86 processors.[4] Protected mode may only be entered after the system software sets up one descriptor table and enables the Protection Enable (PE) bit in the control register 0 (CR0)
Long mode:
long mode is the mode where a 64-bit operating system can access 64-bit instructions and registers. 64-bit programs are run in a sub-mode called 64-bit mode, while 32-bit programs and 16-bit protected mode programs are executed in a sub-mode called compatibility mode.


bootloader:
- The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode, where 64-bit registers and the complete main memory are available.
- Bootloader also query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.


Multiboot standard(GNU GRUB):
- The Free Software Foundation created an open bootloader standard called Multiboot in 1995. The standard defines an interface between the bootloader and the operating system, so that any Multiboot-compliant bootloader can load any Multiboot-compliant operating system. The reference implementation is GNU GRUB.

- To make a kernel Multiboot compliant, one just needs to insert a so-called Multiboot header at the beginning of the kernel file.
