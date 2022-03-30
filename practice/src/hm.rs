//! # hello
//!  hm
//!  ``` rust
//!  fn main() {
//!     println!("Hello, world!");
//!  }
//!  ```

/// froepkf
/// # 1ok
pub fn main() {
    println!("Hello, world!");
}

pub mod greatings {
    pub fn hello() {
        println!("Hello, world!");
    }
    pub fn hello_name(name: &str) {
        println!("Hello, {}!", name);
    }
    pub fn good_day(name: &str) {
        println!("Good day, {}!", name);
    }
}
