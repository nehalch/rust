mod hm;
use hm::greetings::*;
use rand::Rng;
use std::fs::File;

/// Just some structure for cube
struct CubeYes {
    width: i32,
    height: i32,
    color: String,
}
impl CubeYes {
    fn info(&self) {
        println!("{0} {1} {2}", self.width, self.height, self.color);
    }

    /// # hello method works good
    fn hello(name: &str) {
        println!("Hello {}!", name);
    }
}

/// ## Very hard function that create function that counts integral of some math func
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

/// ## Some DNA practice function
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

/// ## xo-xo-xo
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

/// some_fn
fn some_fn(lang: String) -> Option<bool> {
    if lang == "Rust" {
        return Some(true);
    } else {
        return None;
    }
}

/// ## Function that return random number
fn random_number(size: i32) -> i32 {
    let mut rng = rand::thread_rng();

    hm::main();
    hello();
    return rng.gen_range(0..size);
}

/// ### What the hell it is?
fn hell<'a, T>(name: &'a mut T) -> &'a T {
    return name;
}

fn main() {
    // CubeYes
    let cube1: CubeYes = CubeYes {
        width: 11,
        height: 11,
        color: "red".to_string(),
    };
    cube1.info();
    CubeYes::hello("111");

    // Integral
    let x2 = integral_fn_of(&|x| x * x * x);
    x2(12, 122, 0.03);

    // DNA
    println!("DNA: {}", dna_strand("AATTGGAACC"));

    // xo-xo
    xo(&"XoxoXOOxoOX");

    // some fn
    some_fn("rust".to_string());

    // random number function
    print!("random number: {}", random_number(123));
    print!("random number: {}", random_number(123));

    // Hell
    let mut string_var = String::from("Hello");
    let asdf = hell(&mut string_var);
    println!("{}", asdf);

    // --- SHIT ---

    println!("What is your name?");
    let input = std::io::stdin();
    let mut b = String::new();
    input.read_line(&mut b);
    let c = b.trim();
    let d = c.parse::<i32>().unwrap();
    println!("{}", c);
    println!("{}", b);
    let f = File::open("foo.txt").unwrap();
    println!("{:?}", f.metadata().unwrap());
    //fs::write("foo.txt", b"Lorem ipsum");
}
