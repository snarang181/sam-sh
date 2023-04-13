mod utils;

fn main() {
    let immutable_prompt = "sam_sh> ";
    print!("{}", immutable_prompt);
    utils::flush_buffer();
    let mut user_command = String::new();
    loop {
        std::io::stdin().read_line(&mut user_command).expect("Failed to read line");
        if user_command.trim() == "exit" {
            break;
        }
        println!("You entered: {}", user_command);
        user_command.clear();
        print!("{}", immutable_prompt);
        utils::flush_buffer();
    }
}
