use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

mod utils;

fn main() { 
    let immutable_shell_prompt = "sam_sh>"; 
    // Loop indefinitely, until the user types "exit" or "quit" to quit the program
    loop { 
        print!("{}", immutable_shell_prompt);
        utils::flush_buffer();
        /*
            * Read the user's input; make user_input so that read_line() can write to it
            * Since user enters on the terminal, we read from stdin()
            * read_line() will keep reading until it encounters a newline character (CR or LF)
            * read_line() will return an io::Result, which is an enum that can be either Ok or Err
            * If read_line() returns an Err, we use unwrap() to crash the program and print the error
        */
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap(); 

        user_input = user_input.trim().to_string();

        /*
            * Now, we have to parse the user's input
            * Store iterator of strings in user_commands 
            * Make user_commands peekable; peekable on the iterator will allow us to "peek" on the next element without processing it
            * This peekable property will help check if we are on the last command and form our loop 
            * for processing the user's commands
        */
        let mut user_commands = user_input.split(" | ").peekable();
        let mut prev_command = None;

        while let Some(command) = user_commands.next() { 
            /*
                * Now break each command into the bash built-in command and its arguments
                * Split the entered command into an iterator of strings
                * The first string is the command, and the rest are the arguments
                * We use unwrap() to crash the program if the user enters an empty command
                * command_parts.next() rturns the first element of the iterator, which is the command
                * command_parts returns the rest of the elements of the iterator, which are the arguments
            */ 
            let mut command_parts = command.trim().split_whitespace();
            let mut program = command_parts.next().unwrap();
            program = program.trim();
            let args = command_parts;

            match program { 
                "exit" | "quit" => return,

                _ => { 
                        print!("Default case"),
                        utils::flush_buffer();
                    }
            }

            
        }
    
    }
}