use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
// pub fn run_with_error() {
//     let data = vec![
//         "Service Red: Task A",
//         "Service Blue: Task B",
//         "Service Green: Task C",
//         "Service Alpha: Task D",
//     ];
//
//     let mut handles = vec![];
//
//     /*
//        Bu kullanımda thread'ler data isimli vektör içeriğini ortaklaşa kullanmak ister.
//        Ancak burada data içeriği closure tarafından sahiplenilir ve doğal olarak diğer
//        bir thread'in erişmesi mümkün olmaz. Çözüm için Atomcially Reference Counted (Arc)
//        smart pointer'ı kullanılabilir.
//     */
//     for i in 0..2 {
//         let handle = thread::spawn(move || {
//             for task in &data {
//                 println!("Thread '{}' is processing '{}'", i, task);
//             }
//         });
//
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
// }

/*
   Arc kullanılarak thread'lerin veriye güvenli bir şekilde erişmesi sağlanabilir
   ancak burada veri tutarlılığının bozulma ihtimali de olabilir.
*/
// pub fn run_inconsistent() {
//     let data = Arc::new(vec![0; 10]);
//     let mut handles = vec![];
//
//     for i in 0..4 {
//         /*
//            Thread'ler için verinin referansını Arc ile klonluyoruz.
//            Böylece thread'ler ilgili veriye erişebilirler.
//         */
//
//         let data_clone = Arc::clone(&data);
//
//         let handle = thread::spawn(move || {
//             for j in 0..data_clone.len() {
//                 /*
//                    Aşağıda ise veri üzerinde bir güncelleme yapmaktayız.
//                    Bu güncelleme işlemi derleme zamanında
//
//                    error[E0596]: cannot borrow data in an `Arc` as mutable
//
//                    hatasının oluşmasına sebep olur. Zira Arc ile veriye eş zamanlı erişmek
//                    mümkünken güvenli bir şekilde değiştirmek söz konusu değildir.
//
//                    istisna olarak unsafe bir yaklaşım ile bu durum ihlal edilebilir.
//                    Ancak ideal olanı Mutex kullanmak ve threadler arasındaki ortak veriyi
//                    değiştirirken güvenli bir erişim sağlamaktır. Birisi veriyi değiştirirken
//                    başka thread o veriyi değiştiremez.
//
//                    Böylece thread'ler veriyi ortaklaşa okuyabilir ve güvenli şekilde değiştirebilir.
//                 */
//                 data_clone[j] += 2;
//                 println!("Thread {} updating index {}", i, j);
//             }
//         });
//
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     println!("{:?}", *data);
// }

/*
   Bu metot data'ya ortaklaşa erişen ve onu güvenli şekilde değiştiren
   çoklu thread kullanımını örneklemektedir.
*/
pub fn run_safely() {
    // Datayı mutex ile güvenlik altına alırken, Arc ile de thread safe referans sayımı yapıyoruz.
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        // Her thread için kullanılacak smart pointer nesnesini oluşturuyoruz
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // Veriyi kullanmak için thread içerisinde önce kilidi açmamız lazım
            let mut num = data_clone.lock().unwrap();
            *num += 1;
        }); // Lock burada serbest bırakılır

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", *data.lock().unwrap());
}

/*
   Mutex veriyi kullandırırken kilit mekanizmasını işletir.
   Bunu görmek için aşağıdaki örneğe bakabiliriz.
*/
pub fn run_mutex() {
    // Ortak olarak kullanılacak veri
    let data = Arc::new(Mutex::new(0));

    let data_clone_one = Arc::clone(&data);
    let t1 = thread::spawn(move || {
        let mut num = data_clone_one.lock().unwrap();
        *num += 3;
        println!("Thread 1 has locked the data.");
        thread::sleep(Duration::from_secs(3)); // Kasıtlı olarak bekletme yapıyoruz
        println!("After 3 seconds...\nThread 1 is unlocking the data.");
    });

    let data_clone_two = Arc::clone(&data);
    let t2 = thread::spawn(move || {
        println!("Thread 2 is trying to lock the data.");
        let mut num = data_clone_two.lock().unwrap();
        *num += 5;
        println!("Thread 2 has locked and updated the data.");
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!("Final value: {}", *data.lock().unwrap());
}
