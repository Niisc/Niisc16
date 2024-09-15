section .text
_start:
IMM 1
MOV EX, CX 
mov ax, EX 
IMM 0
MOV EX COMMA CX 
mov bx, EX 
mov io, ax
loop:
add ax, bx
mov bx, ax
mov ax, dx
mov io, ax
IMM loop
MOV EX COMMA CX 
IMM 1
jnz EX, CX 
