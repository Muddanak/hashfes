use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use clap::Parser;
use md5::Md5;
use sha2::{Sha256, Digest};

#[derive(Parser)]
#[command(author = "Mudd", version, long_about = None)]
#[command(about = "A multi-hash file lister written in Rust")]
#[command(arg_required_else_help = true, propagate_version = true)]
struct Args {
    ///Filename to analyze
    filename: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    //Check that the user actually has a filename provided
    if !args.filename.is_empty() {
        //Open the file for the BufReader to pull data from
        let mut reader = BufReader::new(File::open(args.filename)?);

        //Create the buffer
        let mut buffer = Vec::new();

        //Read the entire file into the buffer
        let bufsize = reader.read_to_end(&mut buffer).unwrap();

        //Print out how many lines were read
        println!("Read {} bytes", bufsize);

        //Create the hashers for SHA-256 and MD5
        let mut sha256hasher = Sha256::new();
        let mut md5hasher = Md5::new();

        //Set the data and then digest for the MD5
        md5hasher.update(&buffer);
        println!("MD5:\t{:X}", md5hasher.finalize());

        //Set the data and then digest for SHA-256
        sha256hasher.update(&buffer);
        println!("SHA256:\t{:X}", sha256hasher.finalize());

    }

    Ok(())
}
