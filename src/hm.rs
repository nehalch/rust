//! # hello
//!  hm
//!  ``` rust
//!  fn main() {
//!     println!("Hello, world!");
//!  }
//!  ```

/// HEADER
/// # 1ok
pub fn main() {
    println!("Hello, world!");
    super::hello();
}

pub mod greetings {
    //! # hello function
    pub fn hello() {
        println!("Hello, world!");
    }

    /// # hello name function
    pub fn hello_name(name: &str) {
        println!("Hello, {}!", name);
    }

    /// # good day function
    pub fn good_day(name: &str) {
        println!("Good day, {}!", name);
    }
}
