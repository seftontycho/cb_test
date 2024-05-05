#include <iostream>
#include <vector>
#include <mutex>
#include "./libs/cb_test.hpp"

// Because the callback must be thread safe we need to provide a mutex to protect the data.
struct cb_data
{
    std::vector<int> ints;
    std::mutex mut;
};
// The callback function, this will be called after the task has been completed.
// The first arg will be the user_data provided to the task.
// The second arg will be result of the task.
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

    // Initialize the state with a buffer size of 4 (which implies 4 threads)
    auto state = ffi::init(4);

    for (int i = 0; i < 10; i++)
    {
        std::cout << "Calling add with " << i << std::endl;

        // Add a new task to the queue (we provide the state, the callback's first arg, the callback, and the input value)
        // If there is room in the buffer this will return immediately, otherwise it will block until there is room.
        // It can be assumed that when this function returns the input (in this case i) has been copied.
        // Therefore it is safe to drop/modify.
        ffi::run(state, &cb_data, cb, i);
    }

    // Flush the queue, this will block until all tasks have been completed.
    // It also ensures that the callback has been called for all tasks.
    // It also drops the state, so it is not safe to use it after this point.
    // If this is not called the task scheduler thread will be left running (not good).
    ffi:flush(state);

    for (int i = 0; i < cb_data.ints.size(); i++)
    {
        std::cout << "Vector[" << i << "] = " << cb_data.ints[i] << std::endl;
    }

    return 0;
}
