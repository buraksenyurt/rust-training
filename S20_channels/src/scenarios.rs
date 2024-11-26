use rand::Rng;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/*
    Bu senaryoda sistemdeki n sayıda dosyanın n thread ile işlenmesi söz konusudur.
    Her thread dosyayı işlediğinde ve işlemi bitirdiğinde kanala bilgi bırakır.
    En sonunda tüm bu bilgiler receiver üzerinden toplanır.
 */
pub fn process_reports() {
    let (transmitter, receiver) = channel();

    let reports = [
        "salary.json",
        "invoices.json",
        "summary.json",
        "personnel.json",
    ];

    for report in reports {
        let transmitter = transmitter.clone();
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let sleep_time = rng.gen_range(2..=5);
            transmitter
                .send(format!("Processing '{}' report...", report))
                .unwrap();

            // Rapor dosyalarının işlendiği bazı business'lar çağırdığımızı düşünelim

            thread::sleep(Duration::from_secs(sleep_time));

            transmitter
                .send(format!(
                    "Finished processing '{}' in {} seconds",
                    report, sleep_time
                ))
                .unwrap();
        });
    }

    drop(transmitter);
    println!("Started the processing reports");
    for result in receiver {
        println!("Status {}", result);
    }
    println!("Completed the processing reports");
}
