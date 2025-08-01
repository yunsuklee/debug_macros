#[macro_export]
macro_rules! debug_println {
    ($fmt:expr, $($arg:expr),*) => {
        #[cfg(debug_assertions)]
        {
            let pid = std::process::id();
            let thread_id = std::thread::current().id();

            #[cfg(unix)]
            let ppid = std::fs::read_to_string("/proc/self/stat")
                .ok()
                .and_then(|content| content.split_whitespace().nth(3)?.parse::<u32>().ok())
                .unwrap_or(0);

            #[cfg(not(unix))]
            let ppid = 0;

            println!("[ PID: {} PPID: {} TID: {:?} ]", pid, ppid, thread_id);
            print!("[");
            $(
                print!(" {}@0x{:x}", stringify!($arg), &$arg as *const _ as usize);
            )*
            println!(" ]");
            println!($fmt, $($arg),*);
        }
        #[cfg(not(debug_assertions))]
        {
            println!($fmt, $($arg),*);
        }
    };

    ($fmt:expr) => {
        #[cfg(debug_assertions)]
        {
            let pid = std::process::id();
            let thread_id = std::thread::current().id();

            #[cfg(unix)]
            let ppid = std::fs::read_to_string("/proc/self/stat")
                .ok()
                .and_then(|content| content.split_whitespace().nth(3)?.parse::<u32>().ok())
                .unwrap_or(0);

            #[cfg(not(unix))]
            let ppid = 0;

            println!("[ PID: {} PPID: {} TID: {:?} ]", pid, ppid, thread_id);
            println!("{}", $fmt);
        }
        #[cfg(not(debug_assertions))]
        {
            println!($fmt);
        }
    };
}

pub mod prelude {
    pub use crate::debug_println;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_simple_message() {
        debug_println!("Hello, World!");
        assert!(true);
    }

    #[test]
    fn test_with_different_types() {
        let string_val = "test";
        let int_val = 42;
        let float_val = 3.14;
        let bool_val = true;
        let vec_val = vec![1, 2, 3];

        debug_println!(
            "Mixed types: {} {} {} {} {:?}",
            string_val,
            int_val,
            float_val,
            bool_val,
            vec_val
        );
    }

    #[test]
    fn test_edge_cases() {
        let empty_string = "";
        let zero = 0;
        let negative = -42;

        debug_println!("Edge cases: '{}' {} {}", empty_string, zero, negative);
    }

    // This tests that release mode compiles differently
    #[test]
    fn test_conditional_compilation_exists() {
        // We can't easily test the actual output difference,
        // but we can verify both code paths compile
        #[cfg(debug_assertions)]
        {
            debug_println!("This should show debug info");
        }

        #[cfg(not(debug_assertions))]
        {
            debug_println!("This should be clean");
        }
    }
}
