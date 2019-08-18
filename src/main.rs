extern crate log;
extern crate simple_logger;
extern crate clap;

use log::{info, debug};
use clap::{Arg, App, ArgMatches};
use std::env;


fn main() {
    simple_logger::init().unwrap();

    let args = parse_cmd_line_args();

    info!("Verbose: {:?}", args.is_present("v"));

    say_hello("foo");
}

fn say_hello(name: &str) {
    info!("Hello, {name}!", name = name);
}

// see https://github.com/clap-rs/clap#quick-example
fn parse_cmd_line_args<'a>() -> ArgMatches<'a> {
    debug!("{:?}", env::args_os());
    let matches = App::new("Rustlang Demo")
        .version("1.0")
        .author("Jim Barritt <jimb@thoughtworks.com")
        .about("Shows some basic application tasks in rust, including a little api server")
        .arg(Arg::with_name("v")
            .short("v")
            .long("verbose")
            .help("Sets the level of verbosity"))
        .get_matches();

    return matches;

}
