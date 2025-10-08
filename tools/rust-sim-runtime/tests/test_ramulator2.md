# Test Ramulator2 Rust Wrapper

This case tests the wrapped [Ramulator2 methods](../../c-ramulator2-wrapper/).
This test case aims at creating a equivalance with a [C++ version Ramulator2 C wrapper use](../../c-ramulator2-wrapper/test.cpp)
as described in its [corresponding doc](../../c-ramulator2-wrapper/test.md).

## Multi-platform Support

As discussed in [Rust simulator generation](../../../python/assassyn/codegen/simulator/simulator.md),
Linux and MacOS has different behaviors on dynamic objects that links other dynamic objects,
and MacOS has to manually specify `RTLD_LAZY | RTLD_GLOBAL` flag.
Thus a platform-related macro shall be imposed.
