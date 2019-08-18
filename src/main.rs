use log::{info};
use simple_logger;

fn main() {
    simple_logger::init().unwrap();
    say_hello("foo");
}

fn say_hello(name: &str) {
    info!("Hello, {name}!", name = name);
}
