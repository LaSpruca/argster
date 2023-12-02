# Argster

[![Crates.io](https://img.shields.io/crates/v/argster)](https://crates.io/crates/argster)

A simple command line parser to invoke functions with the required arguments

---

[![Argster example](https://asciinema.org/a/RWWvftRNrYWhJC8dFZV4f9lcI.svg)](https://asciinema.org/a/RWWvftRNrYWhJC8dFZV4f9lcI)

# Example

```rs
use argster::command;

struct App;

#[command]
impl App {
    /// A hello command
    /// # Args
    /// input The name to greet
    /// --number -n The number of times to greet them
    fn hello(input: String, times: Option<u32>) {
        for _ in 0..times.unwrap_or(1) {
            println!("Hello {input}");
        }
    }
}

fn main() {
    App::main();
}
```

# Command syntax

```
    -<c>
    -<c><value>
    -<c> <value>
    --<name>
    --<name> <value>
```

Inspired by: [This tsoding video](https://www.youtube.com/watch?v=LQ2rX5B0DUA)
