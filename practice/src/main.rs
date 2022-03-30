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
struct CubeYes {
    width: i32,
    heigth: i32,
    color: String,
}
impl CubeYes {
    fn info(&self) {
        println!("{0} {1} {2}", self.width, self.heigth, self.color);
    }
    fn hello(name: &str) {
        println!("Hello {}!", name);
    }
}

fn somefn(lang: String) -> Option<bool> {
    if lang == "Rust" {
        return Some(true);
    } else {
        return None;
    }
}

fn main() {
    let input = std::io::stdin();

    let mut b = String::new();
    input.read_line(&mut b);

    let c = b.trim();
    let d = c.parse::<i32>().unwrap();

    println!("{}", c);
    println!("{}", b);

    print!("{}", random_number(123));
}

use rand::Rng;
fn random_number(size: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..size);
}
