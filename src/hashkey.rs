use csv::Reader;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
struct CodonResult {
    codon: String,
    amino_acid: char,
}

/// Type for for converting Codons to Amino Acids (a unit of a protein).
pub type CodonKey = BTreeMap<String, char>;

/// Function to build to hashkey based on our csv_file.
pub fn populate(key: &mut CodonKey, csv_file: &str) {
    let mut rdr_key = Reader::from_reader(csv_file.as_bytes());
    for result in rdr_key.deserialize() {
        // Crash the runtime program, allowing Serde to handle err.
        let unwrap: CodonResult = result.unwrap();
        key.insert(unwrap.codon, unwrap.amino_acid);
    }
}
