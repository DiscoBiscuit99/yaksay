extern crate structopt;
extern crate colored;
extern crate failure;
extern crate exitfailure;

use structopt::StructOpt;
use colored::*;
use exitfailure::ExitFailure;
use std::io::{self, Read};

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Mooh!")]
    /// What will the yak say?
    message: String,
    #[structopt(short = "b", long = "bored")]
    /// Spawn a bored yak instead of the default live one.
    bored: bool,
    #[structopt(short = "s", long = "surprised")]
    /// Spawn a surprised yak instead of the default happy one.
    surprised: bool,
    #[structopt(short = "d", long = "dead")]
    /// Spawn a dead yak instead of the deafult happy one.
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load custom ascii from the specified file.
    yakfile: Option<std::path::PathBuf>,
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument.
    stdin: bool,
}

fn main() -> Result<(), exitfailure::ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();

    // Check for stdin.
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
        message.pop();
    } else {
        message = options.message;
    }

    // Print an error message if the yak neighs.
    if message.to_lowercase() == "neigh" {
        let err = "A yak doesn't neigh...";
        eprintln!("\n {}", err.bright_red());
    }
    
    let mut in_multiple_states = false;
    if options.dead && options.bored {
        in_multiple_states = true;
    } else if options.dead && options.surprised {
        in_multiple_states = true;
    } else if options.bored && options.surprised {
        in_multiple_states = true;
    }

    let surprised_yak = r#"
     \
      \          _.-````'-,_
       \     ,-'`           `'-.,_
     /)     (\                   '``-.
    ( ( .,-') )                      ``
     \ '   (_/                        !!
      |       /)           '          !!!
      `\    o'            '     !    !!!!
        !      _/! , !   !  ! !  !   !!!
         \Y,   |!!!  !  ! !!  !! !!!!!!!
           `!!! !!!! !!  )!!!!!!!!!!!!!
            !!  ! ! \( \(  !!!|/!  |/!
        mic & dwb  /_(/_(    /_(  /_(    bison yakified by ejm
    "#;

    // Check for different states.
    let yak: &str;
    if options.dead {
        yak = r#"
     \
      \          _.-````'-,_
       \     ,-'`           `'-.,_
     /)     (\                   '``-.
    ( ( .,-') )                      ``
     \ '   (_/                        !!
      |       /)           '          !!!
      `\    x'            '     !    !!!!
        !      _/! , !   !  ! !  !   !!!
         \Y,   |!!!  !  ! !!  !! !!!!!!!
           `!!! !!!! !!  )!!!!!!!!!!!!!
            !!  ! ! \( \(  !!!|/!  |/!
        mic & dwb  /_(/_(    /_(  /_(    bison yakified by ejm
        "#;    
    } else if options.bored {
        yak = r#"
     \
      \          _.-````'-,_
       \     ,-'`           `'-.,_
     /)     (\                   '``-.
    ( ( .,-') )                      ``
     \ '   (_/                        !!
      |       /)           '          !!!
      `\    -'            '     !    !!!!
        !      _/! , !   !  ! !  !   !!!
         \Y,   |!!!  !  ! !!  !! !!!!!!!
           `!!! !!!! !!  )!!!!!!!!!!!!!
            !!  ! ! \( \(  !!!|/!  |/!
        mic & dwb  /_(/_(    /_(  /_(    bison yakified by ejm
        "#;    
    } else if options.surprised {
        yak = surprised_yak;
    } else {
        yak = r#"
     \
      \          _.-````'-,_
       \     ,-'`           `'-.,_
     /)     (\                   '``-.
    ( ( .,-') )                      ``
     \ '   (_/                        !!
      |       /)           '          !!!
      `\    ^'            '     !    !!!!
        !      _/! , !   !  ! !  !   !!!
         \Y,   |!!!  !  ! !!  !! !!!!!!!
           `!!! !!!! !!  )!!!!!!!!!!!!!
            !!  ! ! \( \(  !!!|/!  |/!
        mic & dwb  /_(/_(    /_(  /_(    bison yakified by ejm
        "#;
    }

    // Make a string with dashes the length of the message.
    let dashes: String = message.chars().map(|x| match x {
        _ => "-",
    }).collect();

    // Print the message and the yak depending on the `-f` flag.
    match &options.yakfile {
        Some (path) => {
            let ascii = match std::fs::read_to_string(path) {
                Ok(file_content) => {
                    // Return the content with prepended `new line`.
                    let mut fc: String = "\n".to_owned();
                    let bs: &str = &file_content;
                    fc.push_str(bs);
                    fc
                },
                Err(_e) => {
                    // Report `file not found` error to the user (assuming it's a `file not found` error).
                    let err = "File not found:";
                    eprintln!("\n {} {:?}", err.bright_red(), path);

                    // Print the surprised yak and format it correctly.
                    let ascii = String::from(surprised_yak);
                    let ascii = ascii.trim_start_matches("\n");
                    String::from(ascii)
                },
            };

            println!("\n +-{}-+", &dashes);
            println!(" | {} |", message.bright_yellow());
            println!(" +-{}-+", dashes);
            print!("{}", ascii);
        },
        None => {
            if in_multiple_states {
                // Inform the user of the quantum collapse.
                let err = "Assuming the yak is quantum, it collapsed to one state...";
                eprintln!("\n {}", err.bright_red());
            }

            println!("\n +-{}-+", &dashes);
            println!(" | {} |", message.bright_yellow());
            print!(" +-{}-+", dashes);
            print!("{}", yak);
        }
    }

    Ok(())
}

