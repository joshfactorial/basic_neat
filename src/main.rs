use crate::utils::fasta_reader::{num_to_char, read_fasta, sequence_array_to_string};
use crate::utils::mutate::mutate_fasta;

pub mod utils;

fn main() {
    let file = "data/H1N1.fa";
    let my_file = read_fasta(file);
    let mutated_fasta = mutate_fasta(&my_file);
    let first_key = my_file.keys().next().unwrap();
    let seq1 = my_file.get(first_key).unwrap().clone();
    let seq2 = mutated_fasta.get(first_key).unwrap().clone();
    println!("Original record {}: {}", first_key, sequence_array_to_string(&seq1));
    println!("Mutated record {}: {}", first_key, sequence_array_to_string(&seq2));
    let mut combine = String::new();
    for index in 0..seq1.len() {
        if seq1[index] == seq2[index] {
            combine += num_to_char(seq1[index].clone())
        } else {
            combine += &format!("[{}/{}]", num_to_char(seq1[index].clone()), num_to_char(seq2[index].clone()))
        }
    }
    println!("Mutation seq: {}", combine)
}
