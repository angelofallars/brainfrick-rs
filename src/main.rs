use std::fs::File;
use std::io::Read;

use clap::Parser;

mod interpreter;
mod utils;

#[cfg(test)]
mod test;

#[derive(Parser, Debug)]
#[clap(name = "brainfrick-rs")]
#[clap(author = "Angelo Fallaria <ba.fallaria@gmail.com")]
#[clap(version = "1.0")]
#[clap(about = "My own implementation of the Brainfuck programming language in Rust")]
struct Args {
    #[clap(value_parser)]
    file_name: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result = interpreter::interpret_contents(contents);

    if let Err(err) = result {
        println!("Error found while parsing the file: {:#?}", err);
    }

    Ok(())
}
