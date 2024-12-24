use cli_utils;

fn main() {
    println!("Hello, Welcome");
    println!("This is CLI-Utils by swami.");
    println!("\n");

    cli_utils::run_cli().expect("Can't get pwd at this time! try again later!");
}
