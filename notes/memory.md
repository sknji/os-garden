There are two different approaches for communicating between the CPU and peripheral hardware on x86, memory-mapped I/O and port-mapped I/O.

- port mapped I/O - port-mapped I/O uses a separate I/O bus for communication. Each connected peripheral has one or more port numbers. To communicate with such an I/O port, there are special CPU instructions called in and out, which take a port number and a data byte (there are also variations of these commands that allow sending a u16 or u32).

- memory mapped I/O - I/O devices use the same address bus as memory, meaning that CPU can refer to memory or the I/O device based on the value of the address. This approach requires isolation in the address space: that is, addresses reserved for I/O should not be available to physical memory.


On x86, the hardware supports two different approaches to memory protection: segmentation and paging.

#### Segmentation
Segmentation was already introduced in 1978, originally to increase the amount of addressable memory. The situation back then was that CPUs only used 16-bit addresses, which limited the amount of addressable memory to 64 KiB. To make more than these 64 KiB accessible, additional segment registers were introduced, each containing an offset address. The CPU automatically added this offset on each memory access, so that up to 1 MiB of memory was accessible.

The segment register is chosen automatically by the CPU depending on the kind of memory access: For fetching instructions, the code segment CS is used, and for stack operations (push/pop), the stack segment SS is used. Other instructions use the data segment DS or the extra segment ES. Later, two additional segment registers, FS and GS, were added, which can be used freely.

The fragmentation problem is one of the reasons that segmentation is no longer used by most systems. In fact, segmentation is not even supported in 64-bit mode on x86 anymore. Instead, paging is used, which completely avoids the fragmentation problem.

#### Virtual Memory
The idea behind virtual memory is to abstract away the memory addresses from the underlying physical storage device. Instead of directly accessing the storage device, a translation step is performed first. For segmentation, the translation step is to add the offset address of the active segment. Imagine a program accessing memory address 0x1234000 in a segment with an offset of 0x1111000: The address that is really accessed is 0x2345000.

To differentiate the two address types, addresses before the translation are called virtual, and addresses after the translation are called physical. One important difference between these two kinds of addresses is that physical addresses are unique and always refer to the same distinct memory location. Virtual addresses, on the other hand, depend on the translation function. It is entirely possible that two different virtual addresses refer to the same physical address. Also, identical virtual addresses can refer to different physical addresses when they use different translation functions.

#### Paging
The idea is to divide both the virtual and physical memory space into small, fixed-size blocks. The blocks of the virtual memory space are called pages, and the blocks of the physical address space are called frames. Each page can be individually mapped to a frame, which makes it possible to split larger memory regions across non-continuous physical frames.
