extern crate structopt;
extern crate colored;
extern crate failure;
extern crate exitfailure;

use structopt::StructOpt;
use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;
use std::io::{self, Read};

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Mooh!")]
    /// What will the yak say?
    message: String,
    #[structopt(short = "d", long = "dead")]
    /// Spawn a dead yak instead of the deafult live one.
    dead: bool,
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load custom ascii from the specified file.
    yakfile: Option<std::path::PathBuf>,
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument.
    stdin: bool,
}

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();

    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    if message.to_lowercase() == "neigh" {
        eprintln!("\nA yak doesn't neigh...");
    }

    let yak: &str;

    if options.dead {
        yak = r#"
    \
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
    } else {
        yak = r#"
    \
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

    let dashes: String = message.chars().map(|x| match x {
        _ => "-",
    }).collect();

    match &options.yakfile {
        Some (path) => {
            let ascii = std::fs::read_to_string(path)
                .with_context(|_| format!("Could not read file {:?}", path))?;

            println!("\n +-{}-+", &dashes);
            println!(" | {} |", message.bright_yellow());
            println!(" +-{}-+", dashes);
            println!("\n{}", ascii);
        },
        None => {
            println!("\n +-{}-+", &dashes);
            println!(" | {} |", message.bright_yellow());
            print!(" +-{}-+", dashes);
            println!("{}", yak);
        }
    }

    Ok(())
}

