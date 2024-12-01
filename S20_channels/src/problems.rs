/*
   Her ne kadar rust dili thread-safe bir ortam sağlamak için bazı kuralları devreye alsa da
   deadlock veya mutex poisoning gibi durumlardan kaçılamayabilir. Aşağıdaki örnek kodlarda
   bu durumlar ele alınmaktadır.

   Kilit mekanizmalarının hatalı kullanımları deadlock oluşmasına sebep olabilir.
   Diğer yandan bir kilit söz konusu iken bulunulan thread'de panic oluşması da sorun yaratır ve
   bu durum Thread Poisoning olarak adlandırılır.
*/
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn deadlock_case() {
    let number = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for i in 0..10 {
        let number = Arc::clone(&number);
        let handle = thread::spawn(move || {
            println!("For counter is {}", i);
            let mut num = number.lock().unwrap();
            /*
               Aşağıdaki satırda number birkez daha kilitlenmeye çalışılır.
               Bu thread'lerin birbirini beklemesine sebep olacak bir deadlock oluşması anlamına gelir.
               Dolayısıyla program bir türlü sonlanmaz.
            */
            let mut another_num = number.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        println!("Joining handle");
        handle.join().unwrap();
    }

    println!("{:?}", number.lock().unwrap());
}

struct Account {
    owner: String,
    balance: f32,
}
/*
   Aşağıdaki senaryoda da bir deadlock durumu oluşmaktadır.
   Hesap bilgileri Mutex'ler ile korunu ve arka arkaya açılan iki thread bu mutex'ler ile
   işlem yapmaya çalışır. Thread'ler içerisindeki kilit açma sıralarında karşılıklı Mutex'lerin
   ele alınmaya çalışılıyor olması bunun en büyük nedenidir.
*/
pub fn deadlock_case_banking() {
    let my_account = Arc::new(Mutex::new(Account {
        owner: "John Doe".to_string(),
        balance: 100.0,
    }));
    let other_account = Arc::new(Mutex::new(Account {
        owner: "Merry Jane".to_string(),
        balance: 200.0,
    }));

    let my_account_clone = Arc::clone(&my_account);
    let other_account_clone = Arc::clone(&other_account);

    let handle1 = thread::spawn(move || {
        let mut source_account = my_account_clone.lock().unwrap();
        println!("Thread 1: Locked by source account");
        thread::sleep(std::time::Duration::from_secs(1));
        let mut target_account = other_account_clone.lock().unwrap();
        println!("Thread 1: Locked by target account");

        source_account.balance -= 50.0;
        target_account.balance += 50.0;
    });

    let my_account_clone = Arc::clone(&my_account);
    let other_account_clone = Arc::clone(&other_account);

    let handle2 = thread::spawn(move || {
        let mut acc2 = other_account_clone.lock().unwrap();
        println!("Thread 2: Locked by target account");
        thread::sleep(std::time::Duration::from_secs(1));
        let mut acc1 = my_account_clone.lock().unwrap();
        println!("Thread 2: Locked by source account");

        acc2.balance -= 25.0;
        acc1.balance += 25.0;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!(
        "Final balances: My Account : {}, Other Account: {}",
        my_account.lock().unwrap().balance,
        other_account.lock().unwrap().balance
    );
}

/*
   Bu fonksiyonda thread poisoning durumu ele alınmaktadır.
   thread açıldığında Mutex'ten bir kilit alınır ancak hemen arkasından kasıtlı olarak panic
   durumu oluşturulur. Bu nokta thread'in zehirlendiği kısımdır.

   Bu nedenle guard değişkeni unwrap_or_else metodu ile ele alınmıştır. Panic oluşması halinde
   into_iter ile kurtarılması sağlanabilir.
*/
pub fn poisoning_case() {
    let first_lock = Arc::new(Mutex::new(100));
    let second_lock = Arc::clone(&first_lock);

    let _ = thread::spawn(move || {
        let mut inner_guard = second_lock.lock().unwrap();
        *inner_guard += 1;
        panic!("Thread is poisoning...");
    })
    .join();

    let mut guard = first_lock
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    *guard += 10;
    println!("Thread guard: {:?}", guard);
}

/*
   Aşağıdaki senaryoda da Thread Poisoning durumu söz konusudur ancak biraz daha gerçekçi bir
   durum ele alınmaktadır. Birden fazla thread'in bir log dosyasına kayıt attığını düşünelim.
   Disk dolabilir farklı bir I/O hatası olabilir ve yazan thread içinden panic üretimi mümkün
   hale gelebilir. Böyle bir durumda thread zehirlenmesi meydana gelir.
   Aşağıdaki örnekte ilk thread içerisinde kasıtlı olarak bir panic durumu oluşturulur.
   Uygulamanın ilk versiyonunda bu durum diğer thread tarafından ele alınmaz ve program istem dışı
   şekilde sonlanır.
   Ancak ikinci uygulamada unwrap_or_else ile mutex'in zehirli olup olmama durumuna göre bir kontrol
   söz konusudur.
*/
pub fn poisoning_case_logging() {
    let log_file = Arc::new(Mutex::new(
        File::create("system.log").expect("Unable to create log file"),
    ));
    let log_file_clone = Arc::clone(&log_file);

    let handle = thread::spawn(move || {
        let mut file = log_file_clone.lock().unwrap();
        writeln!(file, "Thread 1: Writing the system health status").unwrap();
        panic!("Errors occurred while writing to the log file!");
    });

    // let log_file_clone = Arc::clone(&log_file);
    // let handle_2 = thread::spawn(move || {
    //     let mut file = log_file_clone.lock().unwrap();
    //     thread::sleep(std::time::Duration::from_secs(3));
    //     writeln!(file, "Thread 2: Attempting to write").unwrap();
    // });

    // Üstteki durum değerlendirildikten sonra aşağıdaki kod parçası incelenir
    let log_file_clone = Arc::clone(&log_file);
    let recovery_handle = thread::spawn(move || {
        let mut file = log_file_clone
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        thread::sleep(std::time::Duration::from_secs(3));
        writeln!(file, "Thread 2: Recovering from poisoned state").unwrap();
    });

    let _ = handle.join();
    // let _ = handle_2.join();
    let _ = recovery_handle.join();

    println!("Log file operations completed");
}
