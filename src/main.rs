use log::{info};
use simple_logger;

fn main() {
    say_hello("foo");
}

fn say_hello(name: &str) {
    simple_logger::init().unwrap();
    info!("Hello, {name}!", name = name);
}
