# Cross-Language Ramulator2 Validation

This document describes the `compare_ramulator2_outputs.py` script, which serves as a cross-validation invoker for Ramulator2 memory simulator implementations across different programming languages.

## Overview

The script validates that three different Ramulator2 wrapper implementations produce identical outputs when given the same configuration and request sequence:

- **C++ Implementation**: `tools/c-ramulator2-wrapper/test.cpp` (executable: `build/bin/test`)
- **Rust Implementation**: `tools/rust-sim-runtime/tests/test_ramulator2.rs` (via `cargo test`)
- **Python Implementation**: `python/unit-tests/test_ramulator2.py`

## Purpose

This cross-validation ensures:
1. **Behavioral Consistency**: All language bindings produce identical simulation results
2. **API Correctness**: Wrapper functions correctly interface with the core `libramulator` library
3. **Cross-Platform Compatibility**: Shared library loading works across different operating systems
4. **Regression Detection**: Changes to any implementation don't break cross-language consistency

## Prerequisites

- **Repository Root**: `ASSASSYN_HOME` environment variable set to the repository root (defaults to current working directory)
- **C++ Build Tools**: CMake and Make for building the C++ wrapper
- **Rust Toolchain**: Cargo for building and running Rust tests
- **Python 3**: For running the Python implementation
- **Shared Libraries**: The script automatically builds missing C++ and Rust artifacts

## Usage

### Basic Usage
```bash
python python/unit-tests/compare_ramulator2_outputs.py
```

### Command Line Options

#### `--skip <language>`
Skip running a specific language implementation. Can be specified multiple times.

```bash
# Skip only C++
python python/unit-tests/compare_ramulator2_outputs.py --skip cpp

# Skip both C++ and Rust
python python/unit-tests/compare_ramulator2_outputs.py --skip cpp --skip rust
```

#### `--debug`
Enable verbose debugging output including:
- Command execution details
- Environment variables
- C++ binary library dependencies (`ldd` output)
- Build process information

```bash
python python/unit-tests/compare_ramulator2_outputs.py --debug
```

#### `--show-outputs`
Display raw outputs from all language implementations before comparison and filtering.

```bash
python python/unit-tests/compare_ramulator2_outputs.py --show-outputs
```

### Combined Options
```bash
# Debug with output inspection
python python/unit-tests/compare_ramulator2_outputs.py --debug --show-outputs

# Skip C++, show outputs
python python/unit-tests/compare_ramulator2_outputs.py --skip cpp --show-outputs
```

## Behavior

### Output Processing
The script performs several normalization steps to ensure fair comparison:

1. **Cargo Test Harness Filtering**: Removes Rust-specific output:
   - `running 1 test`
   - `test result: ok...`
   - Progress indicators (`.`)
   - Empty lines

2. **Whitespace Normalization**: Removes all blank lines to eliminate formatting differences in statistics sections

3. **Trailing Whitespace**: Strips trailing whitespace from all outputs

### Return Codes
- **0**: All outputs are identical across implementations
- **1**: Outputs differ between implementations (shows unified diff)
- **2**: Command execution failed (shows error details)

### Automatic Building
The script automatically builds missing dependencies:
- **C++**: Runs `cmake ..` and `make -j` in `tools/c-ramulator2-wrapper/build/`
- **Rust**: Runs `cargo build --quiet` in `tools/rust-sim-runtime/`

## Implementation Details

### Environment Setup
- Sets `ASSASSYN_HOME` for all child processes
- Configures `LD_LIBRARY_PATH` for C++ binary to find shared libraries:
  - `tools/c-ramulator2-wrapper/build/lib`
  - `3rd-party/ramulator2`

### Cross-Platform Support
The script handles different operating systems automatically:
- **Linux**: Uses `.so` shared libraries
- **Windows**: Uses `.dll` shared libraries  
- **macOS**: Uses `.dylib` shared libraries

### Error Handling
- **Build Failures**: Clear error messages with build output
- **Missing Executables**: Helpful suggestions for manual building
- **Library Dependencies**: `ldd` diagnostics for C++ binary issues
- **Command Failures**: Full stdout/stderr output for debugging

## Example Output

### Successful Run
```bash
$ python python/unit-tests/compare_ramulator2_outputs.py
All outputs are identical across implementations.
```

### Failed Run with Differences
```bash
$ python python/unit-tests/compare_ramulator2_outputs.py
[DIFF] cpp vs rust:
--- cpp
+++ rust
@@ -1,3 +1,2 @@
 Cycle 3: Write request sent for address 2, success or not (true or false)true
 Cycle 9: Request completed: 2 the data is: 1
-Cycle 5: Write request sent for address 4, success or not (true or false)true
+Cycle 5: Write request sent for address 4, success or not (true or false)false
```

### Debug Output
```bash
$ python python/unit-tests/compare_ramulator2_outputs.py --debug
[DEBUG] Building C++ in /path/to/tools/c-ramulator2-wrapper/build
[DEBUG] cpp cwd=/path/to/tools/c-ramulator2-wrapper/build/bin
[DEBUG] cpp cmd=/path/to/tools/c-ramulator2-wrapper/build/bin/test
[DEBUG] cpp LD_LIBRARY_PATH=/path/to/tools/c-ramulator2-wrapper/build/lib:/path/to/3rd-party/ramulator2
[DEBUG] rust cwd=/path/to/tools/rust-sim-runtime
[DEBUG] rust cmd=cargo test --quiet --test test_ramulator2 -- --nocapture
[DEBUG] python cwd=/path/to/assassyn
[DEBUG] python cmd=python -u /path/to/python/unit-tests/test_ramulator2.py
```

## Troubleshooting

### Common Issues

#### C++ Binary Not Found
```
[ERROR] cpp: Non-zero exit (255). Stderr: Config file ../../configs/example_config.yaml does not exist!
```
**Solution**: Ensure the C++ binary is built and run from the correct directory.

#### Missing Shared Libraries
```
[ERROR] cpp: Non-zero exit (255). Stderr: libwrapper.so: cannot open shared object file
```
**Solution**: The script sets `LD_LIBRARY_PATH` automatically, but verify the libraries exist:
```bash
ls tools/c-ramulator2-wrapper/build/lib/
ls 3rd-party/ramulator2/
```

#### Rust Test Harness Noise
If you see "running 1 test" or "test result" in comparisons, the filtering may need adjustment.

#### Python Import Errors
```
ModuleNotFoundError: No module named 'assassyn'
```
**Solution**: Run from the repository root or set `PYTHONPATH`:
```bash
PYTHONPATH=/path/to/assassyn/python python python/unit-tests/compare_ramulator2_outputs.py
```

### Debugging Steps

1. **Use `--debug`** to see command execution details
2. **Use `--show-outputs`** to inspect raw outputs before filtering
3. **Check individual implementations** by using `--skip` to isolate issues
4. **Verify environment** with `echo $ASSASSYN_HOME` and `ldd` for C++ binary

## Integration

This script is designed to be integrated into:
- **CI/CD Pipelines**: Automated cross-language validation
- **Development Workflows**: Pre-commit hooks or manual testing
- **Regression Testing**: Ensuring changes don't break consistency

## Related Files

- `python/unit-tests/test_ramulator2.py`: Python implementation
- `tools/c-ramulator2-wrapper/test.cpp`: C++ implementation  
- `tools/rust-sim-runtime/tests/test_ramulator2.rs`: Rust implementation
- `tools/ramulator2_crosslang_consistency/README.md`: Original documentation
