
# prot

## Note
**_THIS CODE IS POORLY STRUCTURED BY RUST STANDARDS_**
Hence I am reworking it in near future.

Following Rust principles,
my main goals in using Rust is for code to be modular,
to deal with unexpected scenarios and usability.

- To start using errors instead of panic where appropriate
- Using traits or struct types more (and impl)
- Build up towards a more modular code or library (crate) to refer,
i.e. where a lot of challenges use similar structure.

## About
Return the protein coded by a string of codons at the first AUG only.
- Solution for Rosalind's [Translating RNA into Protein](https://rosalind.info/problems/prot/).
- Uses clap, serde and csv.

The included `human_codon_usage_table.csv` is modified from
[the coding biologist - codon translation tables](https://thecodingbiologist.com/posts/codon-translation-tables).
I am not affiliated, do not own or do not license the material.

# Usage

```
prot <RNA_STRING>
```

- <RNA_STRING>: String of RNA

# Comments

## [2025-10-01]

Regarding data handling, I'm introducing serde and csv.
While it's quite easy to parse a two column csv file,
*csv* and *serde* allows us scale to huge multi-column datasets.

If I'm understanding it correctly, incl_str! macro,
is not the most efficient way to store our data,
given every time we run the program, *csv* and *serde* has to process it into our hashmap.
An unused crate phf appears to be a solution to this, acting as a static hashmap (compiles at run-time),
though it doesn't type it into Rust for you.
