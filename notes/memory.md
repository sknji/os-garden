There are two different approaches for communicating between the CPU and peripheral hardware on x86, memory-mapped I/O and port-mapped I/O.

port mapped I/O - port-mapped I/O uses a separate I/O bus for communication. Each connected peripheral has one or more port numbers. To communicate with such an I/O port, there are special CPU instructions called in and out, which take a port number and a data byte (there are also variations of these commands that allow sending a u16 or u32).

memory mapped I/O - I/O devices use the same address bus as memory, meaning that CPU can refer to memory or the I/O device based on the value of the address. This approach requires isolation in the address space: that is, addresses reserved for I/O should not be available to physical memory.
