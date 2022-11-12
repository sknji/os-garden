he x86-interrupt calling convention is a powerful abstraction that hides almost all of the messy details of the exception handling process.



1. Retrieving the arguments: Most calling conventions expect that the arguments are passed in registers. This is not possible for exception handlers since we must not overwrite any register values before backing them up on the stack. Instead, the x86-interrupt calling convention is aware that the arguments already lie on the stack at a specific offset.
2. Returning using iretq: Since the interrupt stack frame completely differs from stack frames of normal function calls, we can’t return from handler functions through the normal ret instruction. So instead, the iretq instruction must be used.
3. Handling the error code: The error code, which is pushed for some exceptions, makes things much more complex. It changes the stack alignment (see the next point) and needs to be popped off the stack before returning. The x86-interrupt calling convention handles all that complexity. However, it doesn’t know which handler function is used for which exception, so it needs to deduce that information from the number of function arguments. That means the programmer is still responsible for using the correct function type for each exception. 
4. Aligning the stack: Some instructions (especially SSE instructions) require a 16-byte stack alignment. The CPU ensures this alignment whenever an exception occurs, but for some exceptions it destroys it again later when it pushes an error code. The x86-interrupt calling convention takes care of this by realigning the stack in this case.


Links
https://github.com/rust-lang/rfcs/blob/master/text/1201-naked-fns.md
