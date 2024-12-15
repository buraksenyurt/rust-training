/*
   Makrolar meta programlamanın temel unsurlarındadır. Meta programlama kod yazan kodlar
   oluşturulmasını benimseyen bir konsepttir. Makroları kullanarak kod tekrarını azaltan
   kurguları işletebiliriz.

   println! write! gibi sonu ! ile biten enstrümanlar aslında birer makrodur. Makroları
   Declarative ve Procedural olmak üzere iki ana kategoriye ayırabiliriz.

   Bu projedeki test metotlarında declarative macro örnekleri yer almaktadır.
   Aynı zamanda wtime isimli bir procedural macro kullanım örneği de mevcuttur.
*/
use log::info;
use proc_macros::{log_execution, work_time_effort, work_time_effort_log};

mod model;
mod samples;
mod with_macro;
mod without_macro;

#[work_time_effort]
fn find_total() {
    let mut total = 0;
    for i in 0..1000 {
        total += i;
    }
    println!("Result: {}", total);
}

/*
   Attribute türevli procedural makrolara parametre de geçilebilir.
   Argümanlar, macrodaki ilk parametredeki TokenStream üzerinden yakalanır.
*/
#[work_time_effort_log("info")]
fn find_another_total() {
    let mut total = 0;
    for i in 0..1_000 {
        total += i;
    }
    println!("Result: {}", total);
}

struct Logger;

impl Logger {
    pub fn log(&self, msg: &str) {
        info!("{}", msg);
    }
}

#[log_execution(Logger)]
fn sum() {
    let mut total = 0;
    for i in 0..1_000 {
        total += i;
    }
    println!("Result: {}", total);
}

/*
   Log çıktılarını görmek için uygulamayı linux platformunda

       RUST_LOG=info cargo run

   ile,
   Windows platformunda Powershell' de ise

       set RUST_LOG=info && cargo run

   ile çalıştırabiliriz.
*/
fn main() {
    env_logger::init();

    let _total = find_total();
    let _another_total = find_another_total();
    let _sum = sum();
}
