use chrono::Utc;
use crossterm::event::{read, Event, KeyCode};
use rand::prelude::IndexedRandom;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
/*
    Aşağıdaki fonksiyonun amacı basitçe büyük miktarda veri içeren bir log dosyası üretmektir.
    Senaryo gereği messages isimli vektörden rastgele içerikler çekip parametre olarak
    verilen dosya içerisine yazar.
*/
fn generate_random_file(file_path: &str, line_numbers: usize) -> io::Result<()> {
    let messages = [
        "[INFO] Incoming service request for \\games HTTP Get",
        "[ERROR] Security breach detected!",
        "[INFO] User logged in.",
        "[WARNING] Disk space running low.",
        "[ERROR] Connection to database failed.",
        "[WARNING] Unexpected message 'Why so serious'.",
        "[ERROR] Configuration file not found.",
        "[INFO] Resilience strategy has been installed",
        "[INFO] User profile updated.",
        "[ERROR] Failed to load module.",
    ];

    let mut rng = rand::rng();
    let mut file = File::create(file_path)?;

    for _ in 0..line_numbers {
        let message = messages.choose(&mut rng).unwrap();
        writeln!(file, "{}: {}", Utc::now(), message)?;
    }

    Ok(())
}

/*
    Fiziki I/O işlemleri disk veya usb gibi ortamları kullandıklarından büyük boyutlu dosyalarla
    çalışmak problem olabilir. Bu durumda veriyi memory'de buffer'layarak okumak daha iyi bir
    performans sergiler. Aşağıdaki örnekte dosya içeriği satır bazlı okunurken BufReader türünden
    yararlanılır.
*/
fn read_lines(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    // let reader = BufReader::new(file);
    // Buffer açarken istersek kapasiteyi de belirtebiliriz. Örnekte 8 Kb'lık bloklar kullanılıyor
    let reader = BufReader::with_capacity(8 * 1024, file);

    for (idx, line) in reader.lines().enumerate() {
        println!("{}- {}", idx + 1, line?);
    }

    Ok(())
}

/*
    Aşağıdaki fonksiyon dosya içeriğini sayfa sayfa okumak için kullanılır.
    Dosya içeriği yine bir buffer üzerinden ele alınır ancak page_size değerine göre
    bölümlenerek işlenir. Terminalden basılan tuşu algılamak içinse crossterm crate'inden
    yararlanılmaktadır.
*/
fn read_lines_with_pause(file_path: &str, page_size: usize) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::with_capacity(8 * 1024, file);

    let mut line_count = 0;

    for (idx, line) in reader.lines().enumerate() {
        println!("{}- {}", idx + 1, line?);
        line_count += 1;

        if line_count % page_size == 0 {
            println!("Press SPACE for continue, or press E for exit...");

            loop {
                if let Event::Key(event) = read()? {
                    match event.code {
                        KeyCode::Char(' ') => break,
                        KeyCode::Char('e') | KeyCode::Char('E') => return Ok(()),
                        _ => {
                            println!(
                                "Invalid input. Press SPACE for continue, or press E for exit... "
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn run() -> io::Result<()> {
    let file_path = "sys.logs";
    let line_numbers = 1000;

    generate_random_file(file_path, line_numbers)?;
    println!("'{file_path}' with {line_numbers} lines has been generated.");

    // read_lines(file_path)?;
    read_lines_with_pause(file_path, 50)?;
    Ok(())
}
