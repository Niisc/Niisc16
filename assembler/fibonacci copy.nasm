# writes the first 8 values of a fibonacci sequence  
        section .text      # _start defines where the program starts
_start:

   mov   ax, 1    # latest number

   mov   bx, 0    # previous number
   mov   cx, 7    # number of numbers to print
   mov   io, ax   # "prints" ax
    
loop:
   add   ax, bx   # = dx
   mov   bx, ax   # set previous number to current
   mov   ax, dx   # result of previous operation saved in dx
   sub   cx, 1    # remove one
   mov   cx, dx

   mov   io, ax
   jnz   loop, dx
   # hlt            # halt execution