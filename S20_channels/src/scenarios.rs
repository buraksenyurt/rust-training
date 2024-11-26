use rand::Rng;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

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

/*
   Aşağıdaki örneklerde rust'ın standart mpsc modeli ile tokio karşılaştırılmakta.
   Standart kütüphanedeki mpsc'in esasında gerçek anlamda bir asenkronluk sunmadığı belirtilmekte.
   Mesaj gönderimleri senkron olsa bile receiver tarafında bloklanma söz konusu olabilir ve
   burası aslında senkron çalışır. Bu nedenle mpsc çoklu dinleme yapılan senaryolar için pek
   uygun değil.
*/

pub fn do_with_standard() {
    let (transmitter, receiver) = channel();

    for i in 1..=5 {
        let tx_clone = transmitter.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));
            tx_clone.send(format!("Task {} completed", i)).unwrap();
        });
    }

    drop(transmitter);

    println!("Waiting for all tasks...");

    // Aşağıdaki döngü main thread üzerine çalışıp buradaki akışı bloklamaya neden olacak
    for i in 0..10 {
        thread::sleep(Duration::from_secs(1));
        println!("Main task is working...Counting {}", i);
    }

    while let Ok(message) = receiver.recv() {
        println!("{}", message);
    }

    println!("All tasks completed!");
}

pub async fn do_with_tokio() {
    let (transmitter, mut receiver) = mpsc::channel(10);

    for i in 1..=5 {
        let tx_clone = transmitter.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
            tx_clone
                .send(format!("Task {} completed", i))
                .await
                .unwrap();
        });
    }

    drop(transmitter);

    println!("Waiting for all tasks...");

    /*
       Standart mpsc örneğinden farklı olarak burada ana thread bloklanmadan
       döngünün asenkron olarak çalıştırılması sağlanır.
    */
    tokio::spawn(async {
        for i in 0..10 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Main task is working...Counting {}", i);
        }
    });

    while let Some(message) = receiver.recv().await {
        println!("{}", message);
    }

    println!("All tasks completed!");
}
