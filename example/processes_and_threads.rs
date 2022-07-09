///
///
/// threads(processes(env).status)
///
///
use std::env;
use std::io;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    let process1 = thread::spawn(move || {
        println!("thread 1");
        thread::sleep(Duration::from_secs(30));
    });

    let process2 = thread::spawn(move || {
        println!("thread 2");
        Command::new("xlogo")
            .status()
            .expect("Failed to execute command");
    });

    let process3 = thread::spawn(move || {
        Command::new("./threads")
            .status()
            .expect("Failed to execute command");
    });

    let process4 = thread::spawn(move || {
        Command::new("echo")
            .arg("Hello world!")
            .status()
            .expect("command failed to start");
    });

    {
        let status = Command::new("/bin/cat")
            .arg("file.txt")
            .status()
            .expect("failed to execute process");
        println!("process finished with: {status}");

        let mut cmd = Command::new("ls");
        let out = cmd.current_dir("/bin").output().unwrap();
        // io::stdout().write_all(&out.stdout).unwrap();
    }

    {
        println!("main thread");
        thread::sleep(Duration::from_secs(30));
    }

    process1.join();
    process2.join();
    process3.join();
    process4.join();
}
