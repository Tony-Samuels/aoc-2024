use std::{env::args, fs::File, io::Read};

fn main() {
    let mut data = Vec::new();

    for arg in args().skip(1) {
        let mut file = File::open(&arg).unwrap();
        file.read_to_end(&mut data).unwrap();
    }

    let mut freq = [0_usize; 256];
    for c in data {
        freq[c as usize] += 1;
    }

    let max_freq = *freq.iter().max().unwrap();

    print!("struct Aoc3;impl HeuristicFrequencyRank for Aoc3{{fn rank(&self, byte: u8) -> u8 {{const TABLE: [u8; 256] = [");
    for freq in freq {
        print!("{},", (freq * 255) / max_freq);
    }
    println!("];TABLE[byte as usize]}}}}");
}
