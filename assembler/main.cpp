#include <iostream>
#include <fstream>


int main(void) {
    
    std::ifstream ifs;
    ifs.open("/Users/nico/Documents/Code stuff/Niisc16/assembler/main.nasm", std::ifstream::in);
    std::string str ( (std::istreambuf_iterator<char>(ifs) ),(std::istreambuf_iterator<char>()));


}
