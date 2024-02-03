
## Documentation
This is still (mostly) a work in progress.<br/>

To properly make use of this project it is recommended to read the included documentation pdf (Niisc.pdf). Both a compiler and an assembler are included, furthermore a gui program is also provided to better view and understand the instructions at a binary level.<br/>
The save file for the logic simulator / game "turing complete" is included, the verilog save is included as well.<br/>

## Compiler and Assembler
The repository includes toolchain to be used in junction with the logic simulation. Both are a work in progress.
#### 1) Compiler
The compiler will *compile* C-Like code to assembly, after which it can be assembled using the provided assembler.

#### 2) Assembler
The assembler can be used to assemble code produced by the compiler or to assemble assembly code written by the user, the assembly language is heavily influenced by x86 assembly. The repository includes some sample programs to start understading the provided assembly in an concrete manner.

## Acknowledgments

A thank you goes to the following people and resources that were consulted in the making of this instruction set architecture.

#### 1) Consulted individuals

@endershadow<br/>
@FluffyKittenMika<br/>
@gelthor<br/>
@mayge<br/>
@megaing<br/>
@suncat


#### 2) Logic simulator
Turing Complete
https://store.steampowered.com/app/1444480/Turing_Complete/

#### 3) Compiler and assembler
https://austinhenley.com/blog/teenytinycompiler1.html<br/>
https://austinhenley.com/blog/teenytinycompiler2.html<br/>
https://austinhenley.com/blog/teenytinycompiler3.html

#### 4) Book(s)
Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jeffrey D. Ullman. 2006. Compilers: Principles, Techniques, and Tools (2nd Edition). Addison-Wesley Longman Publishing Co., Inc., USA.

