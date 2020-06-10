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

fn main() -> Result<(), ExitFailure> {
    let options = Options::from_args();
    let mut message = String::new();

    // Check for stdin.
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }

    // Print an error message if the yak neighs.
    if message.to_lowercase() == "neigh" {
        let err = "A yak doesn't neigh...";
        eprintln!("\n {}", err.bright_red());
    }

    // Fail if both surprised and dead.
    //if options.dead && options.bored {
        //let err = "A yak can't be both dead and bored...";
        //eprintln!("\n {}", err.bright_red());
    //} else if options.dead && options.surprised {
        //let err = "A yak can't be both dead and surprised...";
        //eprintln!("\n {}", err.bright_red());
    //} else if options.bored && options.surprised {
        //let err = "A yak can't be both bored and surprised...";
        //eprintln!("\n {}", err.bright_red());
    //}
    
    // Inform the user of the quantum collapse.
    let err = "Assuming the yak is quantum, it collapsed to one state...";
    if options.dead && options.bored {
        eprintln!("\n {}", err.bright_red());
    } else if options.dead && options.surprised {
        eprintln!("\n {}", err.bright_red());
    } else if options.bored && options.surprised {
        eprintln!("\n {}", err.bright_red());
    }

    // Check for different states.
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
    } else if options.bored {
        yak = r#"
    \
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
        yak = r#"
    \
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

    // Make a string with dashes the length of the message.
    let dashes: String = message.chars().map(|x| match x {
        _ => "-",
    }).collect();

    // Print the message and the yak depending on the `-f` flag.
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

