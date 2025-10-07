This directory contains a Python script to compare the outputs of three Ramulator2 wrappers (C++, Rust, Python) and ensure they produce identical results.

Targets compared:
- C++: `ASSASSYN_HOME/tools/c-ramulator2-wrapper/test.cpp` (binary: `ASSASSYN_HOME/tools/c-ramulator2-wrapper/build/bin/test`)
- Rust: `ASSASSYN_HOME/tools/rust-sim-runtime/src/test_ramulator2.rs`
- Python: `ASSASSYN_HOME/python/unit-tests/test_ramulator2.py`

Script:
- `compare_ramulator2_outputs.py` runs each target, captures stdout, and emits diffs if they differ.

Prerequisites:
- `ASSASSYN_HOME` set to the repo root (defaults to current working directory if not set)
- CMake + Make for the C++ wrapper
- Rust toolchain (Cargo) for the Rust runtime
- Python 3

The script will attempt to build C++ and Rust targets if needed.

Usage:
```bash
python tools/ramulator2_crosslang_consistency/compare_ramulator2_outputs.py
```

Options:
- `--skip <cpp|rust|python>`: Skip a target (can be specified multiple times)
- `--debug`: Print commands, environments, and extra diagnostics (e.g., `ldd` for the C++ binary)

Behavior:
- Returns exit code 0 if all outputs match
- Returns non-zero and prints unified diffs if any mismatch or command failure occurs

Troubleshooting:
- If the C++ binary fails due to config path, ensure the executable exists at `ASSASSYN_HOME/tools/c-ramulator2-wrapper/build/bin/test`.
- If shared libraries are missing, the script sets `LD_LIBRARY_PATH` for the C++ run to include:
  - `ASSASSYN_HOME/tools/c-ramulator2-wrapper/build/lib`
  - `ASSASSYN_HOME/3rd-party/ramulator2`
- Run with `--debug` and share the `[ERROR]` and `[DEBUG]` output to diagnose further.

Expected result:
- All three outputs must be identical.