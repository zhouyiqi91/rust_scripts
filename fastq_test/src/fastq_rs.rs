use std::io::stdin;
use fastq::Parser;
use std::fs::File;
use fastq::{parse_path, Record};

fn main() {

    let filename = "rna_1.fq.gz";
    let path = Some(filename);  
    let mut total: usize = 0;

    parse_path(path, |mut parser| {
        let stopped = parser.each(|record| {
            total += 1;
            true
        }).expect("Invalid fastq file");
    }).expect("Invalid compression");
    println!("total reads {}", total);
}

