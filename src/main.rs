use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;
use std::ffi::{c_char, CString};
use std::io::{self, Write};

static DLL: OnceCell = OnceCell::new();

fn load_dll() -> &'static Library {
    DLL.get_or_init(|| {
        unsafe { Library::new("yuki.dll").expect("Failed to load dll") }
    })
}

type InitializeFn = extern "C" fn() -> bool;
type IsAttachedFn = extern "C" fn() -> u8;
type ExecuteFn = extern "C" fn(*const c_char);

pub fn initialize() -> bool {
    let func: Symbol = unsafe { load_dll().get(b"initialize").unwrap() };
    func()
}

pub fn is_attached() -> u8 {
    let func: Symbol = unsafe { load_dll().get(b"isAttached").unwrap() };
    func()
}

pub fn execute(script: &str) {
    let c_string = CString::new(script).expect("Failed to create C string");
    let func: Symbol = unsafe { load_dll().get(b"execute").unwrap() };
    func(c_string.as_ptr());
}

fn main() {
    initialize();

    //Free to call execute(script) hereafter
}
