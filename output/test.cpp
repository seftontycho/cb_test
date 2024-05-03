#include <iostream>
#include <vector>
#include <mutex>
#include "./libs/cb_test.hpp"

struct cb_data
{
    std::vector<int> ints;
    std::mutex mut;
};

void cb(void *user_data, int value)
{
    cb_data &data = *reinterpret_cast<cb_data *>(user_data);
    auto guard = std::lock_guard<std::mutex>(data.mut);
    data.ints.push_back(value);

    std::cout << "Callback called with " << value << std::endl;
    std::cout << std::flush;
}

int main()
{
    cb_data cb_data;

    auto state = ffi::init(4);

    for (int i = 0; i < 10; i++)
    {
        std::cout << "Calling add with " << i << std::endl;
        ffi::run(state, &cb_data, cb, i);
    }

    ffi:flush(state);

    for (int i = 0; i < cb_data.ints.size(); i++)
    {
        std::cout << "Vector[" << i << "] = " << cb_data.ints[i] << std::endl;
    }

    return 0;
}
