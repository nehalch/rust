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

enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum Mood {
    Happy,
    Good,
    Sad,
    Angry,
    None,
}
struct Day {
    data: String,
    name: String,
    weekday: WeekDay,
    day_type: TypeOfDay,
    mood: Mood,
}
impl Day {
    pub fn is_workday(&self) -> bool {
        match self.day_type {
            TypeOfDay::Work => true,
            _ => false,
        }
    }
}

/////////////////////////////
///
///
///
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

/////////////////////////////////////////////////

fn main() {
    let x = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let y = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;
    println!("{} {}", x.run(12, 14), y.run(123, 65));
    let tommorow = Day {
        data: "String".to_string(),
        name: "String".to_string(),
        weekday: WeekDay::Friday,
        day_type: TypeOfDay::Study,
        mood: Mood::Good,
    };
    println!("{}", tommorow.is_workday());
}

fn somefn(lang: String) -> Option<bool> {
    if lang == "Rust" {
        return Some(true);
    } else {
        return None;
    }
}
