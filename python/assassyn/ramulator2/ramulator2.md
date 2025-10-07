# PyRamulator - Python Wrapper for Ramulator2

PyRamulator is a Python wrapper that provides a high-level interface to interact with the Ramulator2 memory simulator. It uses two shared libraries to bridge Python and C++ functionality:

- **`libwrapper.so`**: Custom C++ wrapper library (`CRamualator2Wrapper`) that provides C-compatible functions
- **`libramulator.so`**: Core Ramulator2 library containing the memory simulation engine

## Overview

The PyRamulator class encapsulates memory simulation functionality, allowing Python applications to:
- Initialize memory systems with configuration files
- Send read/write requests to simulated memory
- Advance simulation time through ticking mechanisms
- Handle request completion callbacks

## PyRamulator Class Methods

### `__init__(config_path: str)`

Initializes a new PyRamulator instance with the specified configuration file.

**Parameters:**
- `config_path` (str): Path to the YAML configuration file (e.g., `example_config.yaml`)

**Raises:**
- `RuntimeError`: If the CRamualator2Wrapper instance cannot be created

**Example:**
```python
from assassyn.ramulator2 import PyRamulator
sim = PyRamulator("/path/to/config.yaml")
```

### `get_memory_tCK() -> float`

Returns the memory clock period (tCK) in nanoseconds.

**Returns:**
- `float`: Memory clock period in nanoseconds

**Example:**
```python
clock_period = sim.get_memory_tCK()
print(f"Memory clock period: {clock_period} ns")
```

### `send_request(addr: int, is_write: bool, callback, ctx) -> bool`

Sends a memory request to the simulated memory system.

**Parameters:**
- `addr` (int): Memory address for the request
- `is_write` (bool): `True` for write request, `False` for read request
- `callback`: Python function to call when request completes (for read requests)
- `ctx`: Context object passed to the callback function

**Returns:**
- `bool`: `True` if request was successfully enqueued, `False` otherwise

**Raises:**
- `ValueError`: If callback is `None`

**Example:**
```python
def request_callback(req, cycle):
    print(f"Request completed at cycle {cycle}: addr={req.addr}")

success = sim.send_request(0x1000, False, request_callback, 42)
```

### `frontend_tick()`

Advances the frontend simulation by one clock cycle. This processes incoming requests and manages the request queue.

**Example:**
```python
sim.frontend_tick()
```

### `memory_system_tick()`

Advances the memory system simulation by one clock cycle. This processes memory operations and updates timing.

**Example:**
```python
sim.memory_system_tick()
```

### `finish()`

Finalizes the simulation and collects statistics. Should be called when simulation is complete.

**Example:**
```python
sim.finish()
```

### `__del__()`

Destructor that automatically cleans up the underlying C++ wrapper instance when the Python object is garbage collected.

## Request Structure

The `Request` class represents a memory request with the following key fields:

- `addr` (int64): Memory address
- `arrive` (int64): Cycle when request arrived
- `depart` (int64): Cycle when request completed
- `command` (int): Memory command type
- `is_stat_updated` (bool): Whether statistics were updated

## Usage Pattern

A typical simulation loop follows this pattern:

```python
from assassyn.ramulator2 import PyRamulator, Request

# Initialize simulator
sim = PyRamulator("config.yaml")

# Simulation loop
for cycle in range(num_cycles):
    # Send requests
    if should_send_request:
        sim.send_request(address, is_write, callback, context)
    
    # Advance simulation
    sim.frontend_tick()
    sim.memory_system_tick()

# Clean up
sim.finish()
```

## Dependencies

The PyRamulator module requires:
- `libwrapper.so`: Built from `tools/c-ramulator2-wrapper/CRamualator2Wrapper.cpp`
- `libramulator.so`: Core Ramulator2 library from `3rd-party/ramulator2/`
- Python `ctypes` module for C library interfacing

## Building Requirements

Before using PyRamulator, ensure the wrapper library is built:

```bash
cd tools/c-ramulator2-wrapper
mkdir -p build
cd build
cmake ..
make
```

This creates `libwrapper.so` in the `build/lib/` directory, which PyRamulator loads at runtime.

## Cross-Validation Suite

PyRamulator is part of a comprehensive cross-validation suite that ensures consistency across different language implementations:

- **C++ Test**: `tools/c-ramulator2-wrapper/test.cpp`
- **Python Test**: `python/unit-tests/test_ramulator2.py`
- **Rust Test**: `tools/rust-sim-runtime/src/test_ramulator2.rs`

All implementations must produce **identical output** when given the same:
- Configuration file
- Request sequence
- Simulation parameters

This validates that the language bindings correctly interface with the core `libramulator.so` library and maintain behavioral consistency across different programming languages.
