use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use std::env;

fn main() {
    let duration = Duration::new(2, 5);
    let arg_list: Vec<String> = env::args().skip(1).collect();
    let command = arg_list.join(" ");
    let shell = env::var("SHELL").unwrap_or(String::from("shell"));

    loop {
        println!(
            "Every {}s: {} -c \"{}\"",
            duration.as_secs(),
            shell,
            command
        );
        Command::new(&shell)
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Uh oh!");
        println!("");
        sleep(duration);
    }
}
