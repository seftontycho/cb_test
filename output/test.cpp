#include <iostream>
#include <vector>
#include "./libs/cb_test.hpp"

int main()
{
    std::vector<int> vector = {};

    // Capture the lambda function in a function pointer and pass it to ffi::inc
    void (*cbPtr)(int) = [&](int value) {
        vector.push_back(value);
    };

    for (int i = 0; i < 10; i++) {
        std::cout << "Calling add with " << i << std::endl;
        ffi::inc(i, cbPtr);
    }

    return 0;
}
