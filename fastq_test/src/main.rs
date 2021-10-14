use fastq_test::Reader;
use std::fs::File;

fn main(){
    let mut f = File::open("rna_1.fq").unwrap();
    let mut reader = Reader::new(f);

    let mut nb_reads = 0;
    let mut nb_bases = 0;

    for result in reader.records() {
        let record = result.expect("Error during fastq record parsing");

        nb_reads += 1;
        nb_bases += record.seq().len();
    }

    println!("Number of reads: {}", nb_reads);
    println!("Number of bases: {}", nb_bases);
}
