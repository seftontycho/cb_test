#ifndef CB_TEST_HPP
#define CB_TEST_HPP

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace ffi {

struct Sender;

extern "C" {

void flush(Sender *sender);

Sender *init(int channel_size);

void run(Sender *sender, void *cb_data, void (*cb)(void*, int), int data);

} // extern "C"

} // namespace ffi

#endif // CB_TEST_HPP
