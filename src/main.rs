use std::io::Write;
use std::process;
use std::io;

fn main() {
    let city = input("Enter a city: ").to_owned();
    
    let output = process::Command::new("curl").arg(String::from("wttr.in/").to_owned()+city.as_str().trim())
    .output().expect("Could not run command.");

    io::stdout().write_all(&output.stdout).unwrap()
}

fn input(s:&str) -> String {
    let mut val: String = String::new();

    eprint!("{}", s);

    io::stdin().read_line(&mut val).expect("Could not read input.");

    return val;
}
