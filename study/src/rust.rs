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
    }
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
