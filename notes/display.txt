Display:

The easiest way to print text to the screen at this stage is the VGA text buffer. It is a special memory area mapped to the VGA hardware that contains the contents displayed on screen. It normally consists of 25 lines that each contain 80 character cells. Each character cell displays an ASCII character with some foreground and background colors.

The buffer is located at address 0xb8000 and that each character cell consists of an ASCII byte and a color byte.

Each array entry describes a single screen character through the following format:

Bit(s)	Value
0-7	    ASCII code point
8-11  	Foreground color
12-14	  Background color
15	    Blink


The first byte represents the character that should be printed in the ASCII encoding.

The second byte defines how the character is displayed. The first four bits define the foreground color, the next three bits the background color, and the last bit whether the character should blink.

The VGA text buffer is accessible via memory-mapped I/O to the address 0xb8000. This means that reads and writes to that address don’t access the RAM but directly access the text buffer on the VGA hardware. This means we can read and write it through normal memory operations to that address.

https://web.stanford.edu/class/cs140/projects/pintos/specs/freevga/vga/vgamem.htm#manip

https://en.wikipedia.org/wiki/VGA_text_mode

https://wiki.osdev.org/Printing_To_Screen
