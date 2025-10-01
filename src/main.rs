use clap::Parser;
use csv::Reader;
use serde::Deserialize;
use std::cmp;
use std::collections::BTreeMap;

const ABOUT_MESSAGE: &str =
    "Return the protein coded by a string of RNA codons at the first AUG only.";
const START_CODON: &str = "AUG";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// String of RNA
    rna_string: String,
}

#[derive(Debug, Deserialize)]
struct CodonResult {
    codon: String,
    amino_acid: char,
}

type CodonKey = BTreeMap<String, char>;

// Function to check if singleline literal only contains CGAU
// Copied from HAMM, replacing 84 (T) with 85 (U)
fn proof_rna(strand: &str) -> bool {
    for base in strand.as_bytes() {
        if let 67 | 71 | 65 | 85 = base {
        } else {
            return false;
        }
    }
    return true;
}

fn rna_trim_to_aug(rna: &str) -> &str {
    if rna.starts_with(START_CODON) {
        return rna;
    }
    let trim_start = rna.split_once(START_CODON).unwrap().0;
    return rna.trim_start_matches(trim_start);
}

fn rna_to_codons(rna_raw: &str) -> Vec<&str> {
    let rna = rna_trim_to_aug(rna_raw);
    let bases = rna.chars();
    let codons: Vec<&str> = Vec::new();

    for _ in 0..&bases.count() / 3 {
        let base = || bases.next().expect("Invalid unwrap in RNA to codons.");
        let codon = formatln!("{}{}{}", base(), base(), base());
        codons.push(codon);
    }
    codons
}

fn codon_to_prot(codon: &str, key: CodonKey) -> char {
    *key.get(codon).expect("Unmatched codon {codon} against key")
}

fn main() {
    // This fancy macro allows us to include a UTF-8 encoded file as string at compile compile time.
    // For Windows users compiling, you may need to tweak the path, i.e. '\'
    let csv_key = include_str!("../data/human_codon_usage_table.csv");

    // Code to convert our table into a BTreeMap or Hashmap for key usage.
    let mut rdr_key = Reader::from_reader(csv_key.as_bytes());
    let mut key: CodonKey = BTreeMap::new();
    for result in rdr_key.deserialize() {
        let unwrap: CodonResult = result.expect("Unexpected type mismatch!");
        key.insert(unwrap.codon, unwrap.amino_acid);
    }

    let args = Cli::parse();
    let mut prot = String::new();
    let rna = args.rna_string;

    if !proof_rna(&rna) {
        panic!("RNA contains non-viable char!");
    }
}
