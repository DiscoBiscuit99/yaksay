# Yaksay &ndash; Cowsay enhanced, maybe

To install the program, run `cargo install yaksay` in your terminal. If you haven't changed your cargo install directory, it will be installed to the .cargo/bin folder.

![Yaksay default preview](yaksay-preview-default.png)

Running the command `yaksay` in the terminal will spawn the default yak saying "Mooh!" as seen above. The program can of course also be run with a custom input string, like `yaksay "Hello, World!"`. This can be seen below. (Be aware that it might be required to prepend a backslash to exclamation marks like so '\!').

![Yaksay custom string preview](yaksay-preview-custom_string.png)

It is also possible to pass custom ascii to the program to be output instead of the default yak. This can be done with command `yaksay -f path/to/file.*` (the asterics denoting that file type doesn't matter). 

To print helpful information, run the program with the `-h` flag. An extensive table of commands and related behavior can be found below.

## Command - Behavior table

| Flags and options                                | Behavior                                                                                                                        |
|--------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|
| `<message>`                                      | Message from the yak (optional, default: "Mooh!")                                                                               |
| `-h`, `--help`                                   | Prints helpful information described in this table                                                                              |
| `-d`, `--dead`                                   | Spawns a dead cow instead of the default live one.                                                                              |
| `-b`, `--bored`                                  | Spawns a bored cow instead of the default happy one.                                                                            |
| `-s`, `--surprised`                              | Spawns a surprised cow instead of the default happy one.                                                                        |
| `-f <path/to/file.*>`, `--file <path/to/file.*>` | Prints custom ascii from file. The program reads the whole file to a string, so nothing but the ascii art should be in the file |
| `-i`, `--stdin`                                  | Passes text from STDIN to the program (could be used like: `echo "Hello, World!" \| yaksay -i`)                                 |
