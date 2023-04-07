use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use clap::{Parser};
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
    //if let Some(filename) = args.filename {
    if args.filename != "" {
        let mut reader = BufReader::new(File::open(args.filename)?);
        let mut buffer = Vec::new();

        let bufsize = reader.read_to_end(&mut buffer).unwrap();
        println!("Read {} bytes", bufsize);

        let mut sha256hasher = Sha256::new();
        let md5hasher = md5::compute(&buffer);
        let mut ripemdhash = ripemd::Ripemd256::new();
        ripemdhash.update(&buffer);
        sha256hasher.update(&buffer);
        println!("SHA256:\t{:X}", sha256hasher.finalize());
        println!("RIPEMD:\t{:X}", ripemdhash.finalize());
        println!("MD5:\t{:X}", md5hasher);
    }

    Ok(())
}
