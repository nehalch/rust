fn grow(mut nums: Vec<i32>) -> i32 {
    nums.iter().product()
}

fn main() {
    println!("{}", grow(svec![1, 2, 3, 4, 5]));
}
