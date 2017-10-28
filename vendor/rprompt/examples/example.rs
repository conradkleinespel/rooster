extern crate rprompt;

fn main() {
    // Prompt for a reply on STDOUT
    let reply = rprompt::prompt_reply_stdout("What's your name? ").unwrap();
    println!("Your reply is {}", reply);

    // Prompt for a reply on STDERR
    let reply = rprompt::prompt_reply_stderr("What is the capital of Rust crates? ").unwrap();
    println!("Your reply is {}", reply);

    // Read a reply without prompt
    let reply = rprompt::read_reply().unwrap();
    println!("Your reply is {}", reply);
}
