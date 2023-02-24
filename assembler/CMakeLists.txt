cmake_minimum_required(VERSION 3.24)
project(niisc_assembler)

set(CMAKE_CXX_STANDARD 14)

add_executable(niisc_assembler
        lexer.cpp
        main.cpp lexer.h)
