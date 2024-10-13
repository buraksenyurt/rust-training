fn main() {
    print_greetings();
    let name = "Can Cloud Van Dam";
    greet(name);
    println!("4 + 5 = {}", sum(4, 5));

    let numbers = vec![1, 4, 6, 2, 8, 10, 3, 7, 27, 9, 144, 1024, 5];
    let evens = get_evens(&numbers);
    println!("{:?}", evens);
    let odds = get_odds(&numbers);
    println!("{:?}", odds);

    let (x, y) = move_position(10.0, 5.0, 1.0);
    println!("{:?}:{:?}", x, y);

    let names = vec!["caN", "clOUD", "vAn", "DaM"];
    print_all_to_upper(names);
}

fn print_greetings() {
    println!("Welcome to my new world!");
}

fn greet(name: &str) {
    println!("Hello {}! How are you?", name);
}

fn print_all_to_upper(names: Vec<&str>) {
    for name in names {
        println!("{}", name.to_uppercase());
    }
}

fn sum(value_1: i32, value_2: i32) -> i32 {
    value_1 + value_2
}

fn double_it(value: i32) -> i32 {
    value * 2
}

fn move_position(mut x: f32, mut y: f32, acceleration: f32) -> (f32, f32) {
    x += acceleration;
    y += acceleration;
    (x, y)
}

fn get_evens(numbers: &Vec<i32>) -> Vec<i32> {
    let mut evens: Vec<i32> = Vec::new();
    for number in numbers {
        if *number % 2 == 0 {
            evens.push(*number);
        }
    }
    evens
}

fn get_odds(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .filter(|&&i| i % 3 == 0)
        .cloned()
        .collect::<Vec<i32>>()
}
