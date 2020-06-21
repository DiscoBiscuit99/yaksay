extern crate structopt;
extern crate failure;

use structopt::StructOpt;
use std::io::{self, Read};

mod yak;

fn main() {
    let options = yak::Options::from_args();
    let mut message = String::new();

    // Check for stdin.
    if options.stdin {
        io::stdin().read_to_string(&mut message).unwrap();
        message.pop();
    } else {
        message = options.message.clone();
    }

    yak::check_for_neighs(&message);

    let colored_message = yak::get_colored_message(&message, &options); 
    let dashes = yak::get_dashes_and_width(&options, &colored_message);

    yak::print_message_and_ascii(options, colored_message, dashes);
}
