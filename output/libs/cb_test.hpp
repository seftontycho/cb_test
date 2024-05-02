#ifndef CB_TEST_HPP
#define CB_TEST_HPP

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace ffi {

extern "C" {

void inc(int a, void (*cb)(int));

} // extern "C"

} // namespace ffi

#endif // CB_TEST_HPP
