use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprint!("Usage: git-config <name> <email>");
        std::process::exit(1);
    }

    let name = &args[1];
    let email = &args[2];

    let name_status = Command::new("git")
        .args(["config", "--global", "user.name", name])
        .status()
        .expect("Failed to run git for user.name");

    if !name_status.success() {
        eprintln!("Failed to set git user.name");
        std::process::exit(1);
    }

    let email_status = Command::new("git")
        .args(["config", "--global", "user.email", email])
        .status()
        .expect("Failed to run git for user.email");

    if !email_status.success() {
        eprintln!("Failed to set git user.email");
        std::process::exit(1)
    }

    println!("Git config updated succesfully.")
}
