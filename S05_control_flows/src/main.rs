use rand::Rng;

fn main() {
    let mut rnd = rand::thread_rng();
    let some_number = rnd.gen_range(1..1_000);

    println!("{}", some_number);

    if some_number % 2 == 0 {
        println!("{} is even.", some_number);
    } else if some_number % 3 == 0 {
        println!("{} is odd", some_number);
    }

    loop {
        let number = rnd.gen_range(1..101);
        println!("{}", number);
        if number % 23 == 0 {
            println!("I have got you {}", number);
            break;
        } else {
            continue;
        }
    }

    let mut try_count = 0;
    while try_count < 100 {
        let number = rnd.gen_range(1..101);
        if number % 23 == 0 {
            println!("I found the number {} in {} try", number, try_count);
            break;
        }
        try_count += 1;
    }

    let data = get_random_numbers(10);
    for (index, value) in data.iter().enumerate() {
        println!("{index}\t {value}");
    }

    check_exam_score(rnd.gen_range(0..=100));
}

fn get_random_numbers(max_limit: u8) -> Vec<i32> {
    let mut rnd = rand::thread_rng();
    let mut numbers = Vec::new();
    for _ in 0..max_limit {
        let n = rnd.gen_range(1..101);
        if numbers.contains(&n) {
            continue;
        }
        numbers.push(n);
    }
    numbers
}

fn check_exam_score(point: i32) {
    if point == 0 {
        println!("Blank paper! Fails");
    } else if point > 70 {
        println!("{point} is enough for pass.");
    } else if point < 50 {
        println!("{point} is not enough for pass.");
    } else if point > 50 && point < 70 {
        println!("{point} is greater than 50 but less than 70. Come in September!");
    }
}
