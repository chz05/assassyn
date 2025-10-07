use libloading::Library;
use std::ffi::c_void;
use std::env;

mod ramulator2;
use ramulator2::{MemoryInterface, Request, RequestCallback};

// Callback function to handle request completion
extern "C" fn request_callback(req: *mut Request, ctx: *mut c_void) {
    unsafe {
        let cycle = *(ctx as *const i32);
        let request = &*req;
        println!(
            "Cycle {}: Request completed: {} the data is: {}",
            cycle + 3 + (request.depart - request.arrive) as i32,
            request.addr,
            request.addr - 1
        );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get ASSASSYN_HOME environment variable
    let home = env::var("ASSASSYN_HOME").unwrap_or_else(|_| {
        env::current_dir().unwrap().to_string_lossy().to_string()
    });
    let config_path = format!("{}/tools/c-ramulator2-wrapper/configs/example_config.yaml", home);
    
    // Check if config file exists
    if !std::path::Path::new(&config_path).exists() {
        eprintln!("Error: Config file not found at {}", config_path);
        eprintln!("ASSASSYN_HOME: {}", home);
        return Err("Config file not found".into());
    }

    // Load libramulator.so first (dependency for libwrapper.so)
    let lib_ramulator_path = format!("{}/3rd-party/ramulator2/libramulator.so", home);
    let _ramulator_lib = unsafe { Library::new(&lib_ramulator_path)? };
    
    // Load the wrapper library
    let lib_path = format!("{}/tools/c-ramulator2-wrapper/build/lib/libwrapper.so", home);
    let lib = unsafe { Library::new(&lib_path)? };

    // Create memory interface
    let memory = unsafe { MemoryInterface::new(lib.into())? };

    // Initialize with config
    unsafe {
        memory.init(&config_path);
    }

    let mut is_write = false;
    let mut v = 0i32; // counter

    println!("Starting Rust Ramulator2 test (should match C++ test.cpp output)...");

    for i in 0..200 {
        let plused = v + 1;
        let we = v & 1;
        let re = !we;
        let raddr = (v & 0xFF) as i64;
        let waddr = (plused & 0xFF) as i64;
        let addr = if is_write { waddr } else { raddr };

        // Create context for callback
        let cycle_context = Box::new(i);
        let ctx_ptr = Box::into_raw(cycle_context) as *mut c_void;

        // Send request
        let ok = unsafe {
            memory.send_request(addr, is_write, request_callback, ctx_ptr)
        };

        // Print write request status
        if is_write {
            println!(
                "Cycle {}: Write request sent for address {}, success or not (true or false){}",
                i + 2,
                addr,
                ok
            );
        }

        // Toggle write/read
        is_write = !is_write;

        // Advance simulation
        unsafe {
            memory.frontend_tick();
            memory.memory_tick();
        }

        v = plused;
    }

    // Finish simulation
    unsafe {
        memory.finish();
    }

    println!("Rust Ramulator2 test completed successfully!");
    Ok(())
}
