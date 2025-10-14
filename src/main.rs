use anyhow::{bail, Result};
use clap::Parser;
use std::collections::BTreeMap;
// Local library
mod hashkey;
use crate::hashkey::CodonKey;

const ABOUT_MESSAGE: &str =
    "Return the protein coded by a string of RNA codons at the first AUG only.";

const START_CODON: &str = "AUG";

#[derive(Parser)]
#[command(version, about = ABOUT_MESSAGE, long_about = None)]
struct Cli {
    /// String of either rna
    rna: String,
}

struct RNA<'a> {
    string: &'a str,
}

struct Prot {
    string: String,
}

/// Check if single-line literal only contains CGAU
fn build_rna(string: &str) -> Result<RNA> {
    for base in string.as_bytes() {
        if let b'A' | b'C' | b'G' | b'U' = base {
        } else {
            bail!("Non CGAU char {} found!", base)
        }
    }
    Ok(RNA { string })
}

impl RNA<'_> {
    /// Try converting RNA into protein.
    /// If valid, this will trim to the first AUG and end at first stop codon.
    pub fn try_into_prot(&self, key: CodonKey) -> Result<Prot> {
        let codons = self.try_into_codons()?;
        let mut prot = String::new();
        for codon in &codons {
            let result = key.get(*codon);
            match result {
                Some('*') => return Ok(Prot { string: prot }),
                None => bail!("Codon {} read does not match key.", codon),
                Some(amino_acid) => prot.push(*amino_acid),
            }
        }
        bail!(
            "All codons were valid, but no STOP codon was found, ending at {}.",
            codons.get(codons.len()).unwrap()
        );
    }

    fn try_into_codons(&self) -> Result<Vec<&str>> {
        let mut codons: Vec<&str> = Vec::new();

        let rna = if self.string.starts_with(START_CODON) {
            &self.string
        } else {
            if let Some((trim_start, _)) = self.string.split_once(START_CODON) {
                self.string.trim_start_matches(trim_start)
            } else {
                bail!("No start codon {} found", START_CODON)
            }
        };

        for i in 0..&rna.len() / 3 {
            codons.push(&rna[i * 3..=i * 3 + 2])
        }
        Ok(codons)
    }
}

fn main() -> Result<()> {
    // Generate key
    let csv_file = include_str!("../data/human_codon_usage_table.csv");
    let mut key: CodonKey = BTreeMap::new();
    hashkey::populate(&mut key, csv_file);

    let args = Cli::parse();
    let rna = build_rna(&args.rna)?;
    let prot = rna.try_into_prot(key)?;
    println!("{}", prot.string);

    Ok(())
}
