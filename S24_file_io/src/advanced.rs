/*
    Bu seferki senaryoda bir log dosyası içerisinde yer alan bilgileri thread sayısına göre
    parçalayıp her blok içerisindeki Error loglarının
    kanal(channel) üzerinden bir dinleyiciye(receiver) gönderilmesini sağlıyoruz.

    Yani bir dosya içeriğini
*/
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn analyze_logs(file_path: &str, num_threads: usize) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let (tx, rx) = mpsc::channel();
    let lines = Arc::new(Mutex::new(reader.lines()));

    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let tx = tx.clone();
        let lines = Arc::clone(&lines);

        let handle = thread::spawn(move || {
            while let Ok(mut guard) = lines.lock() {
                if let Some(Ok(line)) = guard.next() {
                    drop(guard);
                    if line.contains("ERROR") {
                        tx.send(line).unwrap();
                    }
                } else {
                    break;
                }
            }
        });

        handles.push(handle);
    }

    drop(tx);
    let mut error_count = 0;
    for received in rx {
        error_count += 1;
        println!("Founded error log: {}", received);
    }
    println!("TOTAL ERROR COUNT: {}", error_count);

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
pub fn run() -> io::Result<()> {
    analyze_logs("sys.logs", 4)?;
    Ok(())
}
