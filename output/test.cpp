#include <iostream>
#include <vector>
#include "./libs/cb_test.hpp"

// Define a static function that takes a lambda function as an argument
static void cbWrapper(int value, void (*cb)(int)) {
    cb(value);
}

int main() {
    std::vector<int> vector = {};

    // Define a lambda function
    auto cb = [&](int value) {
        vector.push_back(value);
    };

    // Capture the lambda function in a function pointer and pass it to ffi::inc
    void (*cbPtr)(int) = [](int value) { cb(value); };

    for (int i = 0; i < 10; i++) {
        std::cout << "Calling add with " << i << std::endl;
        ffi::inc(i, cbPtr);
    }

    return 0;
}
