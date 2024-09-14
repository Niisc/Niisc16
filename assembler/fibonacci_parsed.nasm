section .text
_start:
hi:
right:
IMM 1
MOV EX COMMA CX 
mov ax, EX 
IMM 0
MOV EX COMMA CX 
mov bx, EX 
IMM 7
MOV EX COMMA CX 
mov cx, EX 
mov io, ax
loop:
add ax, bx
mov bx, ax
mov ax, dx
random:
IMM 1
MOV EX COMMA CX
sub cx, EX 
mov cx, dx
mov ax, io
IMM loop
MOV EX COMMA CX 
jnz EX, dx
last:
or_not: