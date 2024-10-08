# writes the values of a fibonacci sequence  
        section .text      # _start defines where the program starts
_start:

   mov   ax, 1    # latest number

   mov   bx, 0    # previous number
   mov   io, ax   # "prints" ax
    
loop:
   add   ax, bx   # = dx
   mov   bx, ax   # set previous number to current
   mov   ax, dx   # result of previous operation saved in dx
   mov   io, ax
   jnz   loop, 1
   # hlt            # halt execution