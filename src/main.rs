use clap::Parser;
use csv::Reader;
use serde::Deserialize;
use std::collections::BTreeMap;

const ABOUT_MESSAGE: &str =
    "Return the protein coded by a string of RNA codons at the first AUG only.";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// String of RNA
    rna: String,
}

#[derive(Debug, Deserialize)]
struct CodonResult {
    codon: String,
    amino_acid: char,
}

type CodonKey = BTreeMap<String, char>;

fn main() {
    let csv_key = include_str!("../data/human_codon_usage_table.csv");

    // Hash-key
    let mut rdr_key = Reader::from_reader(csv_key.as_bytes());
    let mut key: CodonKey = BTreeMap::new();
    for result in rdr_key.deserialize() {
        let unwrap: CodonResult = result.expect("Unexpected type mismatch!");
        key.insert(unwrap.codon, unwrap.amino_acid);
    }

    // Parsing input with $(cat test.txt) as arg
    let args = Cli::parse();
    let rna = args.rna;

    // Processing
    let mut codons: Vec<&str> = Vec::new();
    for i in 0..&rna.len() / 3 {
        codons.push(&rna[i * 3..=i * 3 + 2])
    }
    let mut prot = String::new();
    for codon in codons {
        match *key.get(codon).expect("Unmatched codon to key {codon}") {
            '*' => (),
            aa => prot.push(aa),
        }
    }
    println!("{}", prot);
}
