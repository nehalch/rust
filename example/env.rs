use std::env;

fn main() {
    for argument in env::args_os() {
        println!("{argument:?}");
    }
    println!(
        "The current directory is {}",
        env::current_dir().unwrap().display()
    );
    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
}
