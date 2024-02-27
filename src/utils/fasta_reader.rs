use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::BufRead;

pub fn char_to_num(char_of_interest: char) -> u8 {
    return match char_of_interest {
        'A' | 'a' => 0,
        'C' | 'c' => 1,
        'G' | 'g' => 2,
        'T' | 't' => 3,
        _ => 4
    }
}

pub fn num_to_char(nuc_num: u8) -> &'static str {
    return match nuc_num {
        0 => "A",
        1 => "C",
        2 => "G",
        3 => "T",
        _ => "N",
    }
}

pub fn sequence_array_to_string(input_array: &Vec<u8>) -> String {
    let mut return_string = String::new();
    for num in input_array {
        return_string += num_to_char(*num);
    }
    return_string
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename);
    Ok(io::BufReader::new(file.unwrap()).lines())
}

pub fn read_fasta(fasta_path: &str) -> HashMap<String, Vec<u8>> {
    println!("Reading fasta: {}", fasta_path);

    let mut fasta_map: HashMap<String, Vec<u8>> = HashMap::new();
    let mut current_key = String::new();

    if let Ok(lines) = read_lines(fasta_path) {
        let mut temp_seq: Vec<u8> = vec![];
        lines.for_each(|line| match line {
            Ok(l) => {
                if l.starts_with('>') {
                    if !current_key.is_empty() {
                        fasta_map.entry(current_key.clone()).or_insert(temp_seq.clone());
                    }
                    current_key = String::from(l.strip_prefix('>').unwrap());
                    temp_seq = vec![];
                } else {
                    for char in l.chars() {
                        temp_seq.push(char_to_num(char));
                    }
                }
            },
            Err(_) => (),
        });
        // Need to pick up the last one
        fasta_map.entry(current_key.clone()).or_insert(temp_seq.clone());
    }
    fasta_map
}

