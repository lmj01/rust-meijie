use std::env;

fn main() {
    let name = env::args().skip(1).next();
    println!("Hello from an examples");
    println!("hello: {}!", name.unwrap_or("world".into()));
}