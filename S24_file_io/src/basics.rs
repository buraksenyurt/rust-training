use crate::model::Game;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::{fs, io};

fn append_game(game: &Game) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("games.dat")?;
    writeln!(file, "{}", game)?;
    Ok(())
}

fn read(path: &str) -> io::Result<String> {
    // if !fs::metadata(path).is_ok() {
    //     panic!("File does not exist: {}", path);
    // }
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn run() -> io::Result<()> {
    if fs::metadata("games.dat").is_ok() {
        fs::remove_file("games.dat")?;
    }
    let games = vec![
        Game::new(1, "Starcraft", 1996, "Strategy", 8.7),
        Game::new(2, "Generals Zero Hour", 2004, "Strategy", 8.3),
        Game::new(3, "Prince of Persia", 1983, "RPG", 9.2),
    ];
    games.iter().for_each(|g| append_game(g).unwrap());

    let content = read("games.dat")?;
    println!("Popular Games List\n{}", content);
    Ok(())
}
