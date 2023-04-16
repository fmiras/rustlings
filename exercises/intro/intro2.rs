// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Hello {}!", args[1]);
}
