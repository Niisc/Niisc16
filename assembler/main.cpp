#include <iostream>
#include <fstream>

int add(int a, int b);

int main(void) {
    
    std::ifstream ifs;
    ifs.open("./../Inputs/10.txt", std::ifstream::in);
    std::string str ( (std::istreambuf_iterator<char>(ifs) ),(std::istreambuf_iterator<char>()));


}
