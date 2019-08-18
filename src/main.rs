fn main() {
    say_hello("foo");
}

fn say_hello(name: &str) {
    println!("Hello, {name}!", name = name);
}
