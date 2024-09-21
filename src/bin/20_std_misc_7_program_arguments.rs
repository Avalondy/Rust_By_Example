#![allow(dead_code)]

// The command line arguments can be accessed using std::env::args, which returns
// an iterator that yields a String for each argument:

use std::env;

fn test() {
    let args = env::args().collect::<Vec<String>>();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}

mod argument_parsing {
    use std::env;

    fn increase(number: i32) {
        println!("{}", number + 1);
    }

    fn decrease(number: i32) {
        println!("{}", number - 1);
    }

    fn help() {
        println!(
            "usage:
    match_args <string>
        Check whether given string is the answer.
    match_args {{increase|decrease}} <integer>
        Increase or decrease given integer by one."
        );
    }

    pub fn test() {
        let args: Vec<String> = env::args().collect();

        match args.len() {
            // no arguments passed
            1 => {
                println!("My name is 'match_args'. Try passing some arguments!");
            }
            // one argument passed
            2 => match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _ => println!("This is not the answer."),
            },
            // one command and one argument passed
            3 => {
                let cmd = &args[1];
                let num = &args[2];
                // parse the number
                let number: i32 = match num.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        eprintln!("error: second argument not an integer");
                        help();
                        return;
                    }
                };
                // parse the command
                match &cmd[..] {
                    "increase" => increase(number),
                    "decrease" => decrease(number),
                    _ => {
                        eprintln!("error: invalid command");
                        help();
                    }
                }
            }
            // all the other cases
            _ => {
                // show a help message
                help();
            }
        }
    }
}

fn main() {
    // test();
    argument_parsing::test();
}
