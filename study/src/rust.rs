fn main(){
    //IO
    {
        //Input
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    }
    {
        //Output
        panic!("This function never returns!");
    }

    //Variables
    {
        //Int i8 isize u8 usize
        let mut a:i32 = 32;
        // f8 - float
        let _ = 42; // nothing
        let _x = 42; // unused
    }

    type Meters = u32;
    type Kilograms = u32;

    let m: Meters = 3;
    let k: Kilograms = 3;




{
        //&str as an array and a slice
        let string = "Hello there."; // string: &str
        //String as a vector
        let mut s = "Hello".to_string(); // mut s: String
        s.push_str(", world.");

        .bytes() //12313
        .chars() //'*5ts
        .graphemes() //i̙̮͚̦c͚̉
    }

    //Tuples
    let mut x = (1, 2, 3);
    let y = (2, 2, 4);
    
    //Structs
    struct Person{
        fname: String,
        sname: &'static str,
        age: i32,
    };
    let andrew = Person{
        fname:"Andrew".to_string(),
        sname:"Ivanov",
        age:22
    };
    println!("{} {} {}", andrew.fname, andrew.sname, andrew.age);

    struct Inches(i32);
    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);

    //Enums
    enum Direction{
        UP,
        DOWN,
        RIGHT,
        LEFT,
    }
    let user = Direction::UP;
    
    //Arrays
    let a = [1, 2, 3]; // a: [i32; 3]
    a.len();
    
    //Vectors
    let v:Vec<i32> = vec![1, 2, 3]; // v: Vec<i32>
    v.push(4);
    v.get(0); // get 0s element of vector

    //Slice
    let slice = &v[1..4];

    //If
    if x == y {
        println!("yes");
    } else {
        println!("no");
    }
    
    //Match
    match user{
        Direction::UP => println!("UP"),
        Direction::DOWN => println!("DOWN"),
        Direction::LEFT => println!("LEFT"),
        Direction::RIGHT => println!("RIGHT"),
        _ => println!("Error"),
    }

    //For like in python
    for x in 0..10{
        println!("{}", x);
    }

    //While
    let mut wl = false;
    while !wl {
        if 1 == 2{
            continue;
            //it would be skip 1 loop
        }
        println!("wl loop");
        wl = !wl;
    }
    loop{
        println!("while true loop!");
        break;
    }

    //Functions
    fn name(a:i32, b:String) -> String {
        return "kek".to_string();
    }
    fn diverges() -> ! {
        panic!("This function never returns!");
    }
   
}

/*rust

split_whitespace()
split()
chars()
s.push('a
s.push_str('a
Concatenation a + &b
format!(
let slice = &string[5..12]; 
.starts_with("c")
.pop(value) //remove value
.trim() // remove spaces start and end
str.matches("abc")





let mut a:Vec<i32> = vec![1,1,1,1];
Vec::new()
a[0]
.push()
.pop()  //del a value
.contains() //boot a contain
.remove() //by index
.len() // length of vect
.capacity()
.iter().position(|&e| e == value).unwrap();
for i in vec.iter(){}
for x in my_vec.iter_mut(){*x *= 3;}
let slice:&[i32] = &my_vec[2..4];


    */


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
let my_cube = CubeYes {
    width: 12,
    heigth: 12,
    color: "123".to_string(),
};
fn some(s:CubeYes) -> CubeYes { return s; }
my_cube.info();
CubeYes::hello("world");




//struct tuple

struct test(i32, String, i32);
    let a = test(32, "12".to_string(), 321);
    a.0;

fn somefn(lang: String) -> Option<bool> {
    if lang == "Rust" {
        return Some(true);
    } else {
        return None;

    }



enum TypeOfMovement {
    Flying,
    Steps,
    Jumps,
    Crawling,
    Swimming,

}
trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

struct Dog {
   code:i32,
   name:String,
   level: Option<String>,
   go:TypeOfMovement, 
}
impl Dog {
    pub fn kok(){}
}
impl Animal for Dog {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}




fn file_found(i:bool) -> Result<i32,bool> {
   if i { // if true
      Ok(200) // return Ok(200)
   } else { // if false
      Err(false) // return Err(false)
   }
}

if file_found(true).is_ok(){}


// <T> type
// structs 
// func
// trait
// enum
// vec
// collections

fn main(){
   println!("- Passing a string literal"); 
   concatenate(" Rust ", " Programming "); 
   println!("- Passing an integer"); 
   concatenate(10 as i32, 1 as i32);
   
}
use std::fmt::Display;
fn concatenate<T:Display>(t:T, s:T){
   let result = format!("{}{}", t , s);
   println!("{}", result);
}


mod r {
  fn print_statement(){
    println!("Hi, this a function of module r");
  }
}
// main function
fn main() {
  // invoke a module 'r'
   r::print_statement();
}


// declare a module
mod outer_module {
  // function within outer module
  fn my_private_function() {
    println!("Hi, I got into the private function of outer module");
  }
  // declare a nested module
  pub mod inner_module {
    // function within nested module
    pub fn my_public_function() {
      println!("Hi, I got into the public function of inner module");
      println!("I'll invoke private function of outer module");
      super::my_private_function();
    }
  }
}


let a = 2;



// all statics data types in rust stored in stack memory
// All primitive data types that have a fixed size are stored in stack memory.
// All other data types are stored in heap memory.
// When the size of data is not known at compile time rather it is known at the run time, it goes in a portion of program memory called heap memory.

// owership

// 1. Each value has a variable binding called its owner.               {a=1,b=1}
// 2. There can only be one owner at a time.                            {a,-b- =3} {a=3, b=3}
// 3. When an owner goes out of scope, it does not remain accessible.   {{a=3} drop(a)}

// 4. When a value is copied for stack, the new owner is the same as the old owner. {stack f(s){copy(stack)}}
// copy value type is for primitive types          

// 5. When a value is moved for heap, its owner is no longer accessible. {heap f(h){drop(heap)}}
// move value type is for non-primitive types
// let mut a = String::from("Rust"); // define a String and save in 'a'
// let b = a.clone(); // b clones a



// Rules of Borrowing
// 1. Borrowing {one mut var } || { many shared var }
// 2. References must always be valid. {f(a) b != &a}

// 2. Borrowing can only occur in a function that borrows the value.


// Memory lifetime
// When we know the lifetime of a variable by just looking at the program code
// When we can’t say anything about the lifetime of a variable

// when we dont know the lifetime of a variable in function
// we can use 'static lifetime
// 'static lifetime is a lifetime that lasts the entire program.
// 'static lifetime is the default lifetime for all types.

fn get_course<'a, 'b> (c1: &'a Course, c2: &'b Course) -> &<'a, 'b> Course {}
fn a() -> <'_> {
    &var
}

// 'static lifetime for entire prog life





// Rules for Elision
// 1. Each input parameter gets its own lifetime. If the lifetime is not specified, then the lifetime of each parameter is different.
// 2. If there is only one input parameter, its lifetime is assigned to all the elided output lifetimes
// 3. If there are multiple input lifetimes, one of them is &self or &mut self, the lifetime of self is assigned to all elided output lifetimes.


// Rust has a concept of ownership.
// Ownership is the ability to control the lifetime of a value.
// The value can be owned by you, by the compiler, or by the operating system.
// The ownership model is similar to that of C++.
// The ownership model is based on reference counting.
// When a variable is created, it is assigned a unique reference count of 1.
// When a variable is assigned another variable, the previous variable is destroyed and the new variable is assigned a unique reference count of 1.
// When a variable is destroyed, its reference count is decremented.
// When the reference count of a variable reaches 0, the variable is destroyed.
// When a variable is assigned to another variable, the previous variable is destroyed and the new variable is assigned a unique reference count of 1








// closure fn
// fn murfn oncefn

fn foobar<F>(x: i32, y: i32, is_greater: F)
    where F: Fn(i32, i32) -> bool
{
    let (greater, smaller) = if is_greater(x, y) {
        (x, y)
    } else {
        (y, x)
    };
    println!("{} is greater than {}", greater, smaller);
}
 
fn main() {
    foobar(32, 64, |x, y| x > y);
}



///////////////////
fn countdown<F>(count: usize, tick: F)
    where F: Fn(usize)
{
    for i in (1..=count).rev() {
        tick(i);
    }
}
 
fn main() {
    countdown(3, |i| println!("tick {}...", i));
}

// output:
// tick 3...
// tick 2...
// tick 1...
