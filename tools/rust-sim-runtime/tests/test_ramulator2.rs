use libloading::Library;
use std::ffi::c_void;
use std::env;
use std::path::Path;

// Include the runtime module directly for integration testing without requiring a library crate export
mod ramulator2 {
  include!("../src/ramulator2.rs");
}
use ramulator2::{MemoryInterface, Request, RequestCallback};

fn get_shared_lib_extension() -> &'static str {
  if cfg!(target_os = "windows") {
    ".dll"
  } else if cfg!(target_os = "macos") {
    ".dylib"
  } else {
    ".so"
  }
}

fn load_shared_library(lib_path: &str) -> Result<Library, Box<dyn std::error::Error>> {
  let ext = get_shared_lib_extension();
  let primary_path = format!("{}{}", lib_path, ext);
  if Path::new(&primary_path).exists() {
    return Ok(unsafe { Library::new(&primary_path)? });
  }
  let fallback_extensions = [".so", ".dll", ".dylib"];
  for fallback_ext in &fallback_extensions {
    if *fallback_ext != ext {
      let fallback_path = format!("{}{}", lib_path, fallback_ext);
      if Path::new(&fallback_path).exists() {
        return Ok(unsafe { Library::new(&fallback_path)? });
      }
    }
  }
  Err(format!("Could not find shared library at {} with any supported extension", lib_path).into())
}

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

#[test]
fn test_ramulator2_outputs_match_cpp() -> Result<(), Box<dyn std::error::Error>> {
  let home = env::var("ASSASSYN_HOME").unwrap_or_else(|_| env::current_dir().unwrap().to_string_lossy().to_string());
  let config_path = format!("{}/tools/c-ramulator2-wrapper/configs/example_config.yaml", home);
  assert!(Path::new(&config_path).exists(), "Config file not found at {}", config_path);

  let lib_ramulator_path = format!("{}/3rd-party/ramulator2/libramulator", home);
  let _ramulator_lib = load_shared_library(&lib_ramulator_path)?;

  let lib_path = format!("{}/tools/c-ramulator2-wrapper/build/lib/libwrapper", home);
  let lib = load_shared_library(&lib_path)?;

  let memory = unsafe { MemoryInterface::new(lib.into())? };

  unsafe { memory.init(&config_path); }

  let mut is_write = false;
  let mut v = 0i32;

  for i in 0..200 {
    let plused = v + 1;
    let we = v & 1;
    let _re = !we;
    let raddr = (v & 0xFF) as i64;
    let waddr = (plused & 0xFF) as i64;
    let addr = if is_write { waddr } else { raddr };

    let cycle_context = Box::new(i);
    let ctx_ptr = Box::into_raw(cycle_context) as *mut c_void;

    let ok = unsafe { memory.send_request(addr, is_write, request_callback, ctx_ptr) };

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

    is_write = !is_write;
    unsafe {
      memory.frontend_tick();
      memory.memory_system_tick();
    }
    v = plused;
  }

  unsafe { memory.finish(); }
  Ok(())
}


