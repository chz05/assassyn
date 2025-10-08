# Goal

Make Ramulator2 C wrapper usage in Rust unit-test platform-independent.

# Action Items

1. Read [simulator.md](../python/assassyn/codegen/simulator/simulator.md) to understand how to create `libloading` for Linux and MacOS.
2. Current [Rust unit test](../tools/rust-sim-runtime/tests/test_ramulator2.rs) and
   [Rust wrapper implementation](../tools/rust-sim-runtime/src/ramulator2.rs) only tests for Linux.
   - [The test document](../tools/rust-sim-runtime/tests/test_ramulator2.md) is updated, while the implementation is lagging.
   - Add macros to support multiple OS.
   - `cargo test` to make sure it compiles.
   - Stage and commit with `--no-verify`.
3. Add this `cargo test` to `pre-commit`.
4. Add this `cargo test` to [workflow](../.github/workflows/test.yaml) right before "Python Frontend Test".
   - Stage and commit

# Checklist

- [ ] Add the existing rust wrapper usage test.