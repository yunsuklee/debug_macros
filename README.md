# debug_macros

Debug-enhanced macros for Rust applications.

## Overview

This crate provides debugging-enhanced versions of common Rust macros.

## Features

- **Conditional compilation**: Different behavior in debug vs release builds
- **Clean release output**: Standard macros behavior in release builds

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
debug_macros = "0.1.0"
```

Then use the macro in your code:

```rust
use debug_macros::prelude::*;

fn main() {
    let name = "World";
    let count = 42;
    
    // In debug builds: shows PID, TID, and memory addresses
    // In release builds: clean output like regular println!
    debug_println!("Hello, {}! Count: {}", name, count);
}
```

## Debug vs Release Output

### Debug Build
```
[PID: 12345 TID: ThreadId(1)]
0x7fff5fbff6e8 -> "World"
0x7fff5fbff6ec -> 42
```

### Release Build
```
Hello, World! Count: 42
```

## API

### `debug_println!`

Enhanced version of `println!` with debug information.

**Syntax:**
- `debug_println!(format_string, args...)` - Format string with arguments
- `debug_println!(message)` - Simple message without arguments

**Debug mode behavior:**
- Prints process ID and thread ID
- Shows memory addresses and values of all arguments

**Release mode behavior:**
- Functions exactly like standard `println!`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Repository

[https://github.com/yunsuklee/debug_macros](https://github.com/yunsuklee/debug_macros)
