use rand::thread_rng;
use crate::utils::fasta_reader::read_fasta;
use crate::utils::mutate::mutate_fasta;
use crate::utils::make_reads::generate_reads;

use crate::utils::fastq_writer::write_fastq;

pub mod utils;

fn main() {
    let file = "data/ecoli.fa";
    let my_file = read_fasta(file);
    let mutated_fasta = mutate_fasta(&my_file);

    let read_length: usize = 150; // default for testing.
    let coverage: usize = 10; // default for testing
    let mut rng = thread_rng();
    let strict_read_length: Option<bool> = Option::from(true);

    for (_, sequence) in mutated_fasta {
        // defined as a set of read sequences that should cover the mutated sequence `coverage` number of times
        let data_set = generate_reads(
            &sequence,
            read_length,
            coverage,
            &mut rng,
            strict_read_length,
        );

    write_fastq(
        "C:\\Users\\jallen17\\code\\neat_projects\\test.fastq",
        data_set,
    ).unwrap();

    }
}
