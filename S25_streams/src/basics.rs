use std::io::{self, BufRead, Write};

fn write_to_file() -> io::Result<()> {
    let mut input = String::new();
    println!("Please enter some text:");

    io::stdin().read_line(&mut input)?;
    println!("Your text is: {}", input.trim());

    Ok(())
}

fn sum() -> io::Result<i32> {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please enter the first number:");
    io::stdin().read_line(&mut input1)?;

    println!("Second number:");
    io::stdin().read_line(&mut input2)?;

    let x: i32 = input1.trim().parse().expect("Please enter a number!");
    let y: i32 = input2.trim().parse().expect("Please enter a number!");

    Ok(x + y)
}

fn read() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("Please enter some text (Ctrl+Z for exit):");
    for line in reader.lines() {
        let line = line?;
        println!("Input: {}", line);
    }

    Ok(())
}

pub fn run() -> io::Result<()> {
    write_to_file()?;

    let total = sum()?;
    println!("Total: {}", total);

    read()?;

    Ok(())
}
