fn variables() {
    // int
    let x: i8 = 1; // Default is 32 bits
    let y: u8 = 100;
    let z: f32 = 100.1;
    println!("{} {} {}", x, y, z);

    // bool
    let is_true: bool = true;
    if is_true {
        println!("{}", is_true)
    } else {
        println!("{}", "is false")
    }

    // string
    let hello: &str = "jehjeh";
    println!("{}", hello);

    let world: String = String::from("world");
    println!("{}", world);

    let char1: Option<char> = world.chars().nth(0);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("{}", "Index 0 character not found"),
    }
}

fn looping() {
    // loop
    for i in 0..10 {
        print!("{}", i)
    }

    println!();
    let name: String = String::from("my name is aryan");
    println!("{}", get_first_word(name));
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

fn main() {
    variables();
    looping();
}
