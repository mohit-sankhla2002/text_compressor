extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::fast());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    let input_len = input.get_ref().metadata().unwrap().len() as f64;
    let target_len = output.metadata().unwrap().len() as f64;
    println!(
        "Source len: {:?}", input_len
        
    );
    println!("Target len: {:?}", target_len);
    let compression_percentage: f64 = (target_len / input_len) * (100 as f64);
    println!("Compression Percentage: {compression_percentage}%");
    println!("Elapsed: {:?}", start.elapsed());
}