use std::fmt::Result;

fn lol (x: &str){
    println!("{}",x);
    let h:u32  = "3".parse().unwrap();
}

const CONSTANT_PI:f32 = 3.14; //GLOBAL!
                
fn main() {
    println!("Hello, world!");
    
    {
        //1 Basics
        println!("\n\t1 Basics");
        let x: &str = "Local var"; //LOCAL!
        println!("Hello, {}!\n {} is global.", x, CONSTANT_PI, );
    }

    {
        //2 Arrays
        println!("\n\t2 Array");
        let mut array: [i32; 4] = [0, 1, 2, 3];
        println!("All array: {:?}", array);
        println!("3rf item of array: {}", array[2]);
        println!("Array length: {}", array.len());
        // Slice of array
        let slice_array: &[i32]=&array[1..2];
    }

    {
        //3 Tuples
        println!("\n\t3 Tuples");
        let mut tuple: (&str, i32, bool) = ("Alex", 19, true);
        println!("{:?}", tuple);
        println!("{} is {} y.o.", tuple.0, tuple.1);
        let (x,y,z) = tuple;
    }

    {
        //4 
        println!("\n\t4 none");
        
    }

    {
        //5 
        println!("\n\t5 none");
        
    }

    {
        //6 
        println!("\n\t6 none");
        
    }

}
