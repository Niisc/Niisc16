section .data
    fib_sequence resb 16        # Reserve 16 bytes for Fibonacci sequence (8 numbers, 2 bytes each)
var:		
	.byte 64 	                # Declare a byte, referred to as location var, containing the value 64.
	.byte 10 	                # Declare a byte with no label, containing the value 10. Its location is var + 1.
x:		
	.short 42 	                # Declare a 2-byte value initialized to 42, referred to as location x.
y:		
	.long 30000     	        # Declare a 4-byte value, referred to as location y, initialized to 30000.

section .text
global _start

_start:
    mov   ax, 1                 # latest number
    mov   bx, 0                 # previous number
    mov   cx, 8                 # number of numbers to calculate (8 values)

    mov   [fib_sequence], bx    # store the first number (0)
    add   bx, 2                 # move to next memory location (2 bytes per number)
    mov   [bx], ax              # store the second number (1)

    sub   cx, 2                 # we've already stored the first two numbers (0 and 1)
    jmp   fib_loop

fib_loop:
    add   ax, bx                # ax = ax + bx
    mov   dx, bx                # temporarily store bx in dx
    mov   bx, ax                # bx = ax (update previous number)
    mov   ax, dx                # ax = dx (update latest number)
    
    add   bx, 2                 # move to next memory location (2 bytes per number)
    mov   [bx], ax              # store the next Fibonacci number

    loop  fib_loop              # decrement cx and loop if cx != 0

    hlt                         # halt execution
