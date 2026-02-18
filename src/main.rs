use libloading::{Library, Symbol};
use once_cell::sync::OnceCell;
use std::ffi::{c_char, CString};
use std::io::{self, Write};

static DLL: OnceCell<Library> = OnceCell::new();

fn load_dll() -> &'static Library {
    DLL.get_or_init(|| {
        unsafe { Library::new("yuki.dll").expect("Failed to load dll") }
    })
}

type InitializeFn = extern "C" fn() -> bool;
type IsAttachedFn = extern "C" fn() -> u8;
type ExecuteFn = extern "C" fn(*const c_char);

pub fn initialize() -> bool {
    let func: Symbol<InitializeFn> = unsafe { load_dll().get(b"initialize").unwrap() };
    func()
}

pub fn is_attached() -> u8 {
    let func: Symbol<IsAttachedFn> = unsafe { load_dll().get(b"isAttached").unwrap() };
    func()
}

pub fn execute(script: &str) {
    let c_string = CString::new(script).expect("Failed to create C string");
    let func: Symbol<ExecuteFn> = unsafe { load_dll().get(b"execute").unwrap() };
    func(c_string.as_ptr());
}

fn main() {
    println!("Initializing Masha Injector... (´｡• ᵕ •理｡`) ♡");
    if initialize() {
        println!("Masha Injector Initialized!");
    } else {
        println!("Failed to initialize Masha... (T_T)");
        return;
    }

    loop {
        if is_attached() != 0 {
            println!("\n[Status: Attached! Ready for Onii-chan's scripts!]");
        } else {
            println!("\n[Status: Not attached... Please attach to the game first!]");
        }

        print!("Enter Lua script (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut script = String::new();
        io::stdin().read_line(&mut script).expect("Failed to read line");
        let script = script.trim();

        if script.eq_ignore_ascii_case("exit") {
            println!("Bye bye, Onii-chan! ( *^^)o∀*∀o(^^*) ");
            break;
        }

        if !script.is_empty() {
            execute(script);
            println!("Script sent to Masha!");
        }
    }
}
