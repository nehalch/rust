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

struct Point {
    x: i32,
    y: i32,
}
fn test(_point1: Point, _point2: Point) -> f32 {
    // Write code here!
    return ((_point1.x + _point2.x) * *2 - (_point1.y + _point2.y) * *2) * *0.5;
}

fn main() {
    println!("{}", dna_strand("AAATTTCCCGGG"));

    let my_cube = CubeYes {
        width: 12,
        heigth: 12,
        color: "Blue".to_string(),
    };
    my_cube.info();
    CubeYes::hello("world");
    struct test(i32, String, i32);
    impl test {
        fn hz(&self) {
            print!("{}", self.0);
        }
    }

    let a = test(32, "12".to_string(), 321);
    a.0;
    a.hz();
}
