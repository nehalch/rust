use std::thread;
use std::time::Duration;

// Thread function
fn thread_func() {
    for i in 1..8 {
        println!("\t{} spawned thread! ", i);
        thread::sleep(Duration::from_secs(3));
    }
}

fn main() {
    // Threads
    let thread_1 = thread::spawn(move || thread_func());
    let thread_2 = thread::spawn(move || thread_func());
    let thread_3 = thread::spawn(move || thread_func());

    // main function
    for i in 1..5 {
        println!("  {} number main thread!", i);
        thread::sleep(Duration::from_secs(3));
    }

    thread_1.join();
    thread_2.join();
    thread_3.join();

    println!("After Thread");
}
