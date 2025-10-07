# Rust Ramulator2 Test

This Rust test (`test_ramulator2.rs`) mirrors the functionality of the C++ test (`test.cpp`) in the `c-ramulator2-wrapper` directory. It serves as a cross-validation tool to ensure that both Rust and C++ implementations produce identical results when using the same underlying `libramulator.so` library.

## Purpose

This test validates that:
1. Rust bindings correctly interface with the `CRamualator2Wrapper` C++ wrapper
2. Memory simulation behavior is consistent across different language implementations
3. The same request sequence produces identical output in both Rust and C++

## Prerequisites

1. **Build the C++ wrapper library:**
   ```bash
   cd /root/assassyn/tools/c-ramulator2-wrapper
   mkdir -p build
   cd build
   cmake ..
   make
   ```

2. **Ensure the config file exists:**
   ```bash
   ls /root/assassyn/tools/c-ramulator2-wrapper/configs/example_config.yaml
   ```

## Running the Test

### With ASSASSYN_HOME environment variable:
If you are in docker, you automatically have the ASSASSYN_HOME environment variable
```bash
cd /root/assassyn/tools/rust-sim-runtime
cargo run --bin test_ramulator2
```
if you do not have the ASSASSYN_HOME, you need to set the ASSASSYN_HOME as you assassyn project directory.

## Expected Output

The test should produce output identical to the C++ test, including:
- Write request status messages
- Request completion callbacks with cycle timing
- Same address patterns and timing calculations

## Test Logic

The test follows the same pattern as `test.cpp`:
1. Initialize memory interface with config file
2. Run 200 simulation cycles
3. Alternate between read and write requests
4. Use address patterns: `raddr = v & 0xFF`, `waddr = (v+1) & 0xFF`
5. Print write request status and read completion callbacks
6. Advance simulation with `frontend_tick()` and `memory_tick()`

## Cross-Validation

This test is part of a comprehensive validation suite that includes:
- **C++ Test**: `tools/c-ramulator2-wrapper/test.cpp`
- **Python Test**: `python/unit-tests/test_ramulator2.py`
- **Rust Test**: `tools/rust-sim-runtime/src/test_ramulator2.rs`

All tests must produce identical output when given the same configuration and request sequence.
