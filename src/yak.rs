extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
pub struct Options {
    #[structopt(default_value = "Mooh!")]
    /// What will the yak say?
    pub message: String,
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument.
    pub stdin: bool,
    #[structopt(short = "c", long = "color", default_value = "yellow")]
    /// How to color the input.
    color: String,
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
}

// Print an error message if the yak neighs.
pub fn check_for_neighs(message: &String) {
    if message.to_lowercase() == "neigh" {
        let err = "A yak doesn't neigh...";
        eprintln!("\n {}", err.bright_red());
    }
}

// Convert the message to a ColoredString.
pub fn get_colored_message(message: &String, options: &Options) -> colored::ColoredString {
    match &options.color.to_lowercase()[..] {
        "red" => message.bright_red(),
        "green" => message.bright_green(),
        "yellow" => message.bright_yellow(),
        "blue" => message.bright_blue(),
        "magenta" => message.bright_magenta(),
        "cyan" => message.bright_cyan(),
        "white" => message.bright_white(),
        _ => {
            // Inform the user that the color doesn't exist.
            let err = "No such color, default (yellow) is used...";
            eprintln!("\n {}", err.bright_red());
            message.bright_yellow()
        }
    }
}

// Get dashes equal to the length of the message.
pub fn get_dashes(message: String) -> String {
    message.chars().map(|x| match x {
        _ => "-",
    }).collect()
}

// Print the message and the yak depending on the `-f` flag.
pub fn print_message_and_ascii(options: Options, message: colored::ColoredString, dashes: String) {
    let (yak, surprised_yak) = get_yaks(&options);

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
            println!(" | {} |", message);
            println!(" +-{}-+", dashes);
            print!("{}", ascii);
        },
        None => {
            if in_multiple_states(&options) {
                // Inform the user of the quantum collapse.
                let err = "Assuming the yak is quantum, it collapsed to one state...";
                eprintln!("\n {}", err.bright_red());
            }

            println!("\n +-{}-+", &dashes);
            println!(" | {} |", message);
            print!(" +-{}-+", dashes);
            print!("{}", yak);
        }
    }
}

// Check for multiple states.
fn in_multiple_states(options: &Options) -> bool {
    if options.dead && options.bored {
        true
    } else if options.dead && options.surprised {
        true
    } else if options.bored && options.surprised {
        true
    } else {
        false
    }
}

// Return the relevant yaks.
fn get_yaks(options: &Options) -> (String, &str) {
    // Get the eyes.
    let eyes = if options.dead {
        "x"
    } else if options.bored {
        "-"
    } else if options.surprised {
        "o"
    } else {
        "^"
    };

    // The yak with chosen eyes.
    let yak = format!(r#"
     \
      \          _.-````'-,_
       \     ,-'`           `'-.,_
     /)     (\                   '``-.
    ( ( .,-') )                      ``
     \ '   (_/                        !!
      |       /)           '          !!!
      `\    {}'            '     !    !!!!
        !      _/! , !   !  ! !  !   !!!
         \Y,   |!!!  !  ! !!  !! !!!!!!!
           `!!! !!!! !!  )!!!!!!!!!!!!!
            !!  ! ! \( \(  !!!|/!  |/!
        mic & dwb  /_(/_(    /_(  /_(    bison yakified by ejm"#, eyes);

    // The surprised yak.
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
        mic & dwb  /_(/_(    /_(  /_(    bison yakified by ejm"#;

    // Return the two.
    (yak, surprised_yak)
}
