fn dna_strand(dna: &str) -> String {
    return dna
        .chars()
        .map(|x| match x {
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            'A' => 'T',
            _ => ' ',
        })
        .collect::<String>();
}

fn xo(string: &'static str) -> bool {
    let mut x = 0;
    let mut o = 0;
    for i in string.chars() {
        match i {
            'x' | 'X' => x += 1,
            'o' | 'O' => o += 1,
            _ => x = x,
        }
    }
    if x - o == 0 {
        return true;
    }
    return false;
}

fn main() {
    println!("{}", dna_strand("AAATTTCCCGGG"));
}
