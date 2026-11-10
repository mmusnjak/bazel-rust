extern crate hello_lib;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// The greeting to use.
    #[arg(short = 'g', long, default_value = "Hello")]
    greeting: String,

    /// The thing to greet.
    #[arg(short = 't', long, default_value = "world")]
    thing: String,
}

fn main() {
    let args = Args::parse();
    let greeter = hello_lib::Greeter::new(&args.greeting);
    greeter.greet(&args.thing);
}
