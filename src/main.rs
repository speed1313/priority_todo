use clap::Parser;
use std::io;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    method: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    match &*args.method{
        "add" => add(),
        "complete" => complete(),
        "show" => show(),
        _ => panic!()
    };
}


fn add()->io::Result<()>{
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    fs::write("./todolist.txt",&buffer)?;
    println!("{}",buffer);
    Ok(())

}

fn complete()->io::Result<()>{
    Ok(())
}

fn show()->io::Result<()>{
    Ok(())
}
