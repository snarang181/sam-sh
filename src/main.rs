mod utils;

fn main() {
    let immutable_prompt = "sam_sh> ";
    print!("{}", immutable_prompt);
    utils::flush_buffer();
    let mut user_command = String::new();
    loop {
        match std::io::stdin().read_line(&mut user_command) {
            Ok(_) => {},
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
        if user_command.trim() == "exit" {
            break;
        }
        println!("You entered: {}", user_command);
        user_command.clear();
        print!("{}", immutable_prompt);
        utils::flush_buffer();
    }
}
