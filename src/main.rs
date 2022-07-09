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
fn integral_fn_of<F, A, B, C>(f: &'static F) -> impl Fn(A, B, C) -> f64 + 'static
where
    A: Into<f64>,
    A: From<i32>,
    B: Into<f64>,
    B: From<i32>,
    C: Into<f64>,
    C: From<i32>,
    F: Fn(f64) -> f64 + 'static,
{
    move |from: A, to: B, step: C| {
        let mut from: f64 = from.into();
        let to: f64 = to.into();
        let step: f64 = step.into();

        let mut result: f64 = 0.0;

        println!(
            "Start counting integral\nfrom {} to {} with step {} \n....",
            from, to, step
        );

        while from <= to {
            result += f(from) * step;
            from += step;
        }
        println!("Answer is {} squares", result);
        result
    }
}

fn main() {
    println!("What is your name?");

    let input = std::io::stdin();

    let mut b = String::new();
    input.read_line(&mut b);

    let c = b.trim();
    let d = c.parse::<i32>().unwrap();

    println!("{}", c);
    println!("{}", b);

    let mut qwer = String::from("Hello");
    fn hell<'a, T>(name: &'a mut T) -> &'a T {
        return name;
    }
    let asdf = hell(&mut qwer);
    println!("{}", asdf);

    print!("{}", random_number(123));
    print!("{}", random_number(123));
    print!("{}", random_number(123));

    use std::fs;
    use std::fs::File;

    let mut f = File::open("foo.txt").unwrap();
    println!("{:?}", f.metadata().unwrap());
    //fs::write("foo.txt", b"Lorem ipsum");
}
mod hm;
use hm::greatings::*;
use rand::Rng;
fn random_number(size: i32) -> i32 {
    let mut rng = rand::thread_rng();

    hm::main();
    hello();
    return rng.gen_range(0..size);
}
