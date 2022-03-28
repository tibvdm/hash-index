//! K-mer module

/// K-mer represented as an u64
pub struct Kmer(u64);

impl Kmer {
    /// Create a new k-mer
    pub fn new(kmer: u64) -> Kmer {
        Kmer(kmer)
    }
}

impl From<&str> for Kmer {
    /// Create a k-mer from a string
    fn from(s: &str) -> Self {
        let mut u64_repr: u64 = 0;

        for (i, c) in s.chars().enumerate() {
            u64_repr |= match c {
                'A' => 0,
                'C' => 1,
                'D' => 2,
                'E' => 3,
                'F' => 4,
                'G' => 5,
                'H' => 6,
                'I' => 7,
                'K' => 8,
                'L' => 9,
                'M' => 10,
                'N' => 11,
                'P' => 12,
                'Q' => 13,
                'R' => 14,
                'S' => 15,
                'T' => 16,
                'V' => 17,
                'W' => 18,
                'Y' => 19,
                _   => 20 // TODO: error handle this
            } << (i * 5)
        }

        println!("{}", u64_repr);

        Kmer(u64_repr)
    }
}

impl From<Kmer> for String {
    /// Create a string from a k-mer
    fn from(kmer: Kmer) -> Self {
        let mut string: String = "".to_string();

        for i in 0 .. 9 {
            string.push(
                match kmer.0 >> (i * 5) & 0x1F {
                0  => 'A',
                1  => 'C',
                2  => 'D',
                3  => 'E',
                4  => 'F',
                5  => 'G',
                6  => 'H',
                7  => 'I',
                8  => 'K',
                9  => 'L',
                10 => 'M',
                11 => 'N',
                12 => 'P',
                13 => 'Q',
                14 => 'R',
                15 => 'S',
                16 => 'T',
                17 => 'V',
                18 => 'W',
                19 => 'Y',
                _  => ' ' // TODO: error handle this
                }
            );
        }

        string
    }
}
