use argster::command;

struct App;

#[command]
impl App {
    /// This is a hello command
    /// # Args
    /// --number -n The number of times to print the output
    fn hello(input: String, number: Option<usize>, reee: usize) {
        _ = reee;
        for _ in 0..number.unwrap_or(1) {
            println!("Hello {input}");
        }
    }
}

fn main() {
    App::main();
}
