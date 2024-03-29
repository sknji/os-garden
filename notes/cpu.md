CPU exceptions occur in various erroneous situations, for example, when accessing an invalid memory address or when dividing by zero. To react to them, we have to set up an Interrupt Descriptor Table (IDT) that provides handler functions.

An exception signals that something is wrong with the current instruction. For example, the CPU issues an exception if the current instruction tries to divide by 0. When an exception occurs, the CPU interrupts its current work and immediately calls a specific exception handler function, depending on the exception type.

On x86, there are about 20 different CPU exception types.

The most important are:

  1. Page Fault: A page fault occurs on illegal memory accesses. For example, if the current instruction tries to read from an unmapped page or tries to write to a read-only page.
  2. Invalid Opcode: This exception occurs when the current instruction is invalid, for example, when we try to use new SSE instructions on an old CPU that does not support them.
  3. General Protection Fault: This is the exception with the broadest range of causes. It occurs on various kinds of access violations, such as trying to execute a privileged instruction in user-level code or writing reserved fields in configuration registers.
  4. Double Fault: When an exception occurs, the CPU tries to call the corresponding handler function. If another exception occurs while calling the exception handler, the CPU raises a double fault exception. This exception also occurs when there is no handler function registered for an exception.
  5. Triple Fault: If an exception occurs while the CPU tries to call the double fault handler function, it issues a fatal triple fault. We can’t catch or handle a triple fault. Most processors react by resetting themselves and rebooting the operating system.


When an exception occurs, the CPU roughly does the following:
- Push some registers on the stack, including the instruction pointer and the RFLAGS register. (We will use these values later in this post.)
- Read the corresponding entry from the Interrupt Descriptor Table (IDT). For example, the CPU reads the 14th entry when a page fault occurs.
- Check if the entry is present and, if not, raise a double fault.
- Disable hardware interrupts if the entry is an interrupt gate (bit 40 not set).
- Load the specified GDT selector into the CS (code segment).
- Jump to the specified handler function.

Preserving all Registers:
Exceptions can occur on any instruction. In most cases, we don’t even know at compile time if the generated code will cause an exception. For example, the compiler can’t know if an instruction causes a stack overflow or a page fault.

Since we don’t know when an exception occurs, we can’t backup any registers before. This means we can’t use a calling convention that relies on caller-saved registers for exception handlers. Instead, we need a calling convention that preserves all registers. The x86-interrupt calling convention is such a calling convention, so it guarantees that all register values are restored to their original values on function return.

Global Descriptor Table (GDT)
is a binary data structure specific to the IA-32 and x86-64 architectures. It contains entries telling the CPU about memory segments


Links:
https://wiki.osdev.org/Exceptions
https://en.wikipedia.org/wiki/Calling_convention
x86-interrupt https://github.com/rust-lang/rust/pull/39832 https://internals.rust-lang.org/t/pre-rfc-interrupt-calling-conventions/16182 https://internals.rust-lang.org/t/pre-rfc-interrupt-calling-conventions/16182/12
