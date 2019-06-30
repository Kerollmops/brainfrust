# brainfrust

A translation tool from brainfuck to Rust

## Hello World Example

```brainfuck
# hello-world.b
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

```bash
cargo build --release
./run.sh hello-world
```

```rust
// hello-world.rs

use std::io::{Read, Write};

const MEMORY_LEN: usize = 30_000;

fn main() {
    let mut memory = [0u8; MEMORY_LEN];
    let mut ptr = 0;
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    while memory[ptr] != 0 {
        ptr = (ptr + 1) % MEMORY_LEN;
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        ptr = (ptr + 1) % MEMORY_LEN;
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        ptr = (ptr + 1) % MEMORY_LEN;
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        memory[ptr] = memory[ptr].wrapping_add(1);
        ptr = (ptr + 1) % MEMORY_LEN;
        memory[ptr] = memory[ptr].wrapping_add(1);
        ptr = ptr.wrapping_sub(1) % MEMORY_LEN;
        ptr = ptr.wrapping_sub(1) % MEMORY_LEN;
        ptr = ptr.wrapping_sub(1) % MEMORY_LEN;
        ptr = ptr.wrapping_sub(1) % MEMORY_LEN;
        memory[ptr] = memory[ptr].wrapping_sub(1);
    }
    ptr = (ptr + 1) % MEMORY_LEN;
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    ptr = (ptr + 1) % MEMORY_LEN;
    memory[ptr] = memory[ptr].wrapping_add(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    stdout.write_all(&[memory[ptr]]).unwrap();
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    ptr = (ptr + 1) % MEMORY_LEN;
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    ptr = ptr.wrapping_sub(1) % MEMORY_LEN;
    ptr = ptr.wrapping_sub(1) % MEMORY_LEN;
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    ptr = (ptr + 1) % MEMORY_LEN;
    stdout.write_all(&[memory[ptr]]).unwrap();
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    memory[ptr] = memory[ptr].wrapping_add(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    memory[ptr] = memory[ptr].wrapping_sub(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    ptr = (ptr + 1) % MEMORY_LEN;
    memory[ptr] = memory[ptr].wrapping_add(1);
    stdout.write_all(&[memory[ptr]]).unwrap();
    ptr = (ptr + 1) % MEMORY_LEN;
    stdout.write_all(&[memory[ptr]]).unwrap();
}
```
