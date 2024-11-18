use std::thread;
use std::time::Duration;

pub fn start_a_simple_thread() {
    let handle = thread::spawn(move || {
        println!("Starting thread...");
        thread::sleep(Duration::new(3, 0));
        println!("Thread stopped...");
    });
    // handle.join().unwrap();
    /*
       İlk önce bu olmadan deneyelim.
       Eğer kullanmazsa main thread buradaki işlemin tamamlanmasını beklemeden yoluna devam eder.
    */
}

// pub fn move_keyword_error() {
//     let student_points = vec![30.50, 49.90, 65.55, 90.00, 81.00];
//     /*
//        Aşağıdaki durum biraz daha dikkatli ele alınmalı.
//        main thread içinde tanımlı student_points vektörünü yeni başlatılan bir thread içinde
//        ele almak istiyoruz. println! ile doğrudan yazdırmak istediğimizde eğer move keyword
//        kullanılmamışsa şu hatayı alırız;
//
//        error[E0373]: closure may outlive the current function
//        , but it borrows `student_points`, which is owned by the current function
//
//        Ancak student_points içeriğini örneğin bir döngüde kullanır ve elemanları tek tek
//        f32 olarak ele alırsak move anahtarını kullanmasak bile problem olmaz.
//
//        Bunu açıklamaya çalışalım;
//
//        vektör türü ile ifade edilen veriler bilindiği üzere heap üzerinde durmakta.
//        main thread içinde başlatılan ikinci thread'de bu vektörün sahipliği(ownership) alınmak isteniyor.
//        Ancak ikinci thread ki closure ile bir kod bloğu açıyor,
//        main thread'den daha uzun süre çalışabilir ve bu memory safe bir davranış değildir.
//        Bu yüzden rust move kullanılmasını ve bu sayede vektör sahipliğinin closure'a taşınmasını istiyor.
//
//        Esasında Rust thread safe bir ortam sağlamaya çalışıyor.
//        Birden fazla thread üzerinde aynı bellek adresine işaret eden bir yapı varsa,
//        Rust buna izin vermez zira bu data race durumunun oluşmasına yol açabilir.
//
//        move kullanıldığında closure, student_points vektörünün tamamının sahipliğini alır.
//        Vektörün sahipliği artık ana thread'e değil, closure bloğuna aittir.
//        Rust thread’ler arası sahiplik sorununu bu şekilde çözer.
//
//        O zaman tek tek içeriği dolaştığımızda move kullanılmaması neden bir probleme sebebiyet
//        vermiyor? Bu durumda closure, vektörün kendisini değil elemanlarını kullanmış oluyor.
//        Zira bu örnekte vektör elemanları f32 türündendir dolayısıyla kopyalanarak taşınabilirler.
//        closure bu kopyalar üzerinde çalışır. Dolayısıyla orjinal vektörün sahiplenilmesi veya
//        borç olara alınması söz konusu değildir.
//     */
//     let handle = thread::spawn(|| {
//         println!("Thread is starting...");
//         println!("{:?}", student_points); // Bu kullanımla aşağıdaki farklı
//                                           // for point in student_points {
//                                           //     println!("Processing for point {}", point);
//                                           // }
//         println!("Thread completed");
//     });
//     handle.join().unwrap();
// }

pub fn move_keyword_success() {
    let student_points = vec![30.50, 49.90, 65.55, 90.00, 81.00];
    let handle = thread::spawn(move || {
        println!("Thread is starting...");
        println!("{:?}", student_points);
        println!("Thread completed");
    });
    handle.join().unwrap();
}

fn calc_factorial(n: u64) -> u64 {
    (1..=n).product()
}
pub fn multiple_threads_sample() {
    let numbers = vec![10, 3, 5, 13, 8, 9, 1, 2, 17, 11, 7, 6];
    let threads_count = 4;
    let mut handles = vec![];
    let chunk_size = numbers.len() / threads_count;

    for i in 0..threads_count {
        let chunk = numbers[i * chunk_size..(i + 1) * chunk_size].to_vec();
        handles.push(thread::spawn(move || {
            let mut results = vec![];
            for num in chunk {
                println!("Thread {} processing for {}", i, num);
                results.push((num, calc_factorial(num)));
            }
            results
        }));
    }

    let mut final_results = vec![];

    for handle in handles {
        final_results.push(handle.join().unwrap());
    }

    println!("{:?}", final_results);
}
