## The Interrupt Stack Frame:
For exception and interrupt handlers, the CPU performs the following steps when an interrupt occurs

0. Saving the old stack pointer - The CPU reads the stack pointer (rsp) and stack segment (ss) register values and remembers them in an internal buffer.
1. Aligning the stack pointer - An interrupt can occur at any instruction, so the stack pointer can have any value, too. However, some CPU instructions (e.g., some SSE instructions) require that the stack pointer be aligned on a 16-byte boundary, so the CPU performs such an alignment right after the interrupt.
2. Switching stacks (in some cases) - A stack switch occurs when the CPU privilege level changes, for example, when a CPU exception occurs in a user-mode program. It is also possible to configure stack switches for specific interrupts using the so-called Interrupt Stack Table (described in the next post).
3. Pushing the old stack pointer - The CPU pushes the rsp and ss values from step 0 to the stack. This makes it possible to restore the original stack pointer when returning from an interrupt handler.
4. Pushing and updating the RFLAGS register - The RFLAGS register contains various control and status bits. On interrupt entry, the CPU changes some bits and pushes the old value.
5. Pushing the instruction pointer - Before jumping to the interrupt handler function, the CPU pushes the instruction pointer (rip) and the code segment (cs). This is comparable to the return address push of a normal function call.
6. Pushing an error code (for some exceptions) - For some specific exceptions, such as page faults, the CPU pushes an error code, which describes the cause of the exception.
7. Invoking the interrupt handler - The CPU reads the address and the segment descriptor of the interrupt handler function from the corresponding field in the IDT. It then invokes this handler by loading the values into the rip and cs registers.

![exception stack frame](images/exception-stack-frame.svg)

In order for the CPU to use our new interrupt descriptor table, we need to load it using the lidt instruction.

## Double fault:
 A double fault is a special exception that occurs when the CPU fails to invoke an exception handler. For example, it occurs when a page fault is triggered but there is no page fault handler registered in the Interrupt Descriptor Table (IDT).

When exception handler not added to IDT and there is no double fault handler, the below happen;

1. The CPU tries to write to 0xdeadbeef, which causes a page fault.
2. The CPU looks at the corresponding entry in the IDT and sees that no handler function is specified. Thus, it can’t call the page fault handler and a double fault occurs.
3. The CPU looks at the IDT entry of the double fault handler, but this entry does not specify a handler function either. Thus, a triple fault occurs.
4.A triple fault is fatal. QEMU reacts to it like most real hardware and issues a system reset.

## Kernel Stack Overflow
A guard page is a special memory page at the bottom of a stack that makes it possible to detect stack overflows. The page is not mapped to any physical frame, so accessing it causes a page fault instead of silently corrupting other memory. The bootloader sets up a guard page for our kernel stack, so a stack overflow causes a page fault.


> When a page fault occurs, the CPU looks up the page fault handler in the IDT and tries to push the interrupt stack frame onto the stack. However, the current stack pointer still points to the non-present guard page. Thus, a second page fault occurs, which causes a double fault (according to the above table).
>
> So the CPU tries to call the double fault handler now. However, on a double fault, the CPU tries to push the exception stack frame, too. The stack pointer still points to the guard page, so a third page fault occurs, which causes a triple fault and a system reboot. So our current double fault handler can’t avoid a triple fault in this case.

#### Switching Stack
The x86_64 architecture has a solution to the kernel stack overflow problem.

The x86_64 architecture is able to switch to a predefined, known-good stack when an exception occurs. This switch happens at hardware level, so it can be performed before the CPU pushes the exception stack frame.

The switching mechanism is implemented as an Interrupt Stack Table (IST). The IST is a table of 7 pointers to known-good stacks.

#### Task State Segment
The Interrupt Stack Table (IST) is part of an old legacy structure called Task State Segment (TSS). The TSS used to hold various pieces of information (e.g., processor register state, I/O port permissions, Inner-level stack pointers, previous TSS link) about a task in 32-bit mode and was, for example, used for hardware context switching. However, hardware context switching is no longer supported in 64-bit mode and the format of the TSS has changed completely.

On x86_64, the TSS no longer holds any task-specific information at all. Instead, it holds two stack tables (the IST is one of them). The only common field between the 32-bit and 64-bit TSS is the pointer to the I/O port permissions bitmap.

| Field |	Type | Description |
| ---- | ---- | ---- |
| (reserved)	| u32 |
| Privilege Stack Table	| [u64; 3] | used by the CPU when privilege level changes. E.g if an exception occurs while the CPU is in user mode.
| (reserved) |	u64 |
| Interrupt Stack Table	| [u64; 7] |
| (reserved)	| u64 |
| (reserved)	| u16 |
| I/O Map Base Address 	| u16 |
