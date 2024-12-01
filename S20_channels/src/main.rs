/*
   Thread'ler arasında veri aktarımı yapmak için kanallardan(channels) yararlanılabilir.
   Kanallar ownership ve borrowing kuralları ile uyumlu olacak şekilde thread-safe ve verimli
   bir haberleşme imkanı sunar.

   Genelde multi-producer, single-consumer modelinde işlenirler. Yani birden fazla veri üreten
   thread(producers) söz konusu iken bu üretilen verileri dinleyen ya da kullananan sadece tek bir
   tüketetici(consumer) thread vardır.

   Kanallar içerisinde sadece belirlenen veri türleri kullanılır bu da tip güvenliğinin
   sağlanması anlamına gelir. Burada yine defacto olarak ifade edebileceğim tokio kütüphanesi
   sayesinden kanalların asenkron çalıştırılması da sağlanabilir.
*/
use crate::basic::*;
use crate::problems::*;
use crate::scenarios::*;

mod basic;
mod problems;
mod scenarios;
fn main() {
    // hello_channels();
    // multi_producer();
    // multi_producer_2();
    // process_reports();
    // deadlock_case();
    // deadlock_case_banking();
    // poisoning_case();
    poisoning_case_logging();
}

// #[tokio::main]
// async fn main() {
//     println!("Standard mpsc scenario");
//     do_with_standard();
//     println!("async scenario with tokio");
//     do_with_tokio().await
// }
