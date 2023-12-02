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
