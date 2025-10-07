use libloading::Library;
use std::ffi::c_void;
use std::env;
use std::path::Path;

mod ramulator2;
use ramulator2::{MemoryInterface, Request, RequestCallback};

/// Get the appropriate shared library extension for the current OS
fn get_shared_lib_extension() -> &'static str {
    if cfg!(target_os = "windows") {
        ".dll"
    } else if cfg!(target_os = "macos") {
        ".dylib"
    } else {
        ".so" // Linux and other Unix-like systems
    }
}

/// Load a shared library with fallback for different extensions
fn load_shared_library(lib_path: &str) -> Result<Library, Box<dyn std::error::Error>> {
    let ext = get_shared_lib_extension();
    let primary_path = format!("{}{}", lib_path, ext);
    
    // Try the primary extension first
    if Path::new(&primary_path).exists() {
        return Ok(unsafe { Library::new(&primary_path)? });
    }
    
    // Fallback: try other common extensions
    let fallback_extensions = [".so", ".dll", ".dylib"];
    for fallback_ext in &fallback_extensions {
        if *fallback_ext != ext {
            let fallback_path = format!("{}{}", lib_path, fallback_ext);
            if Path::new(&fallback_path).exists() {
                return Ok(unsafe { Library::new(&fallback_path)? });
            }
        }
    }
    
    // If no library found, return an error
    Err(format!("Could not find shared library at {} with any supported extension", lib_path).into())
}

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

    // Load libramulator first (dependency for libwrapper)
    let lib_ramulator_path = format!("{}/3rd-party/ramulator2/libramulator", home);
    let _ramulator_lib = load_shared_library(&lib_ramulator_path)?;
    
    // Load the wrapper library
    let lib_path = format!("{}/tools/c-ramulator2-wrapper/build/lib/libwrapper", home);
    let lib = load_shared_library(&lib_path)?;

    // Create memory interface
    let memory = unsafe { MemoryInterface::new(lib.into())? };

    // Initialize with config
    unsafe {
        memory.init(&config_path);
    }

    let mut is_write = false;
    let mut v = 0i32; // counter

    // Align output strictly with C++ test.cpp

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
        let ok = unsafe { memory.send_request(addr, is_write, request_callback, ctx_ptr) };

        // Print write request status
        if is_write {
            println!(
                "Cycle {}: Write request sent for address {}, success or not (true or false){}",
                i + 2,
                addr,
                ok
            );
            use std::io::Write;
            std::io::stdout().flush().ok();
        }

        // Toggle write/read
        is_write = !is_write;

        // Advance simulation
        unsafe {
            memory.frontend_tick();
            memory.memory_system_tick();
        }

        v = plused;
    }

    // Finish simulation
    unsafe {
        memory.finish();
    }

    // No trailing summary line to keep output identical to C++
    Ok(())
}
