use std::io::{stdin};
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
                program => { 
                    /*
                        * Handling the case where piping is involved
                        * There are two possible cases:
                        * 1. There is another command piped behind this one
                        * 2. There are no more commands piped behind this one
                        * In case of number 1, we prepare to send output to the next command
                        * And in case of number 2, we send output to shell stdout
                    */
                    let std_input = prev_command.map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));
                    let std_output;
                    if user_commands.peek().is_some() { 
                        /*
                            * There are more commands
                            * Redirect output to next command
                        */
                        std_output = Stdio::piped();
                    } else { 
                        // Redirect output to shell stdout
                        std_output = Stdio::inherit();
                    }
                    /*
                        * Prepare to spawn the command with arguments of args
                        * Stdinput is std_input, and stdout is std_output
                        * Spawn is used to create a new process and execute the command
                    */
                    let curr_output = Command::new(program)
                        .args(args)
                        .stdin(std_input)
                        .stdout(std_output)
                        .spawn();
                    /*
                        * Check if the command was successfully spawned
                        * If it was, set prev_command to curr_output
                    */
                    match curr_output { 
                        Ok(curr_output) => { prev_command = Some(curr_output); },
                        Err(_) => { 
                            prev_command = None;
                            eprintln!("Error: Command not found");
                            utils::flush_buffer();
                        }
                        // _ => { 
                        //     print!("Error: Command not found");
                        //     utils::flush_buffer();
                        // }
                    }

                },
                // _ => { 
                //         print!("Default case");
                //         utils::flush_buffer();
                //     }
            }
        }
        // Wait until all commands have finished executing
        if let Some(mut last_cmd) = prev_command { 
            last_cmd.wait().unwrap();
        }
    }
}