use clap::{Arg, App};
use std::io::{self, Read};

fn escape_data_str(input: &str) -> String {
    input.replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
}

fn escape_prop_str(input: &str) -> String {
    escape_data_str(input).replace(":", "%3A").replace(",", "%2C")
}

fn main() -> io::Result<()> {
    let matches = App::new("echo-output")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("name").takes_value(true).required(true).index(1))
        .get_matches();
    let name = matches.value_of("name").unwrap();
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    println!("::set-output name={}::{}", escape_prop_str(name), escape_data_str(&buffer));
    Ok(())
}
