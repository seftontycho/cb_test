#include <iostream>
#include "cb_test.hpp"


void cb (int a) {
    std::cout << "Callback called with " << a << std::endl;
}

int main() {
    
    for (int i = 0; i < 10; i++) {
        std::cout << "Calling add with " << i << std::endl;
        ffi::add(i, cb);
    }
}