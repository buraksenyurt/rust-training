/*
   Makrolar meta programlamanın temel unsurlarındadır. Meta programlama kod yazan kodlar
   oluşturulmasını benimseyen bir konsepttir. Makroları kullanarak kod tekrarını azaltan
   kurguları işletebiliriz.

   println! write! gibi sonu ! ile biten enstrümanlar aslında birer makrodur. Makroları
   Declarative ve Procedural olmak üzere iki ana kategoriye ayırabiliriz.

   Bu projedeki test metotlarında declarative macro örnekleri yer almaktadır.
   Aynı zamanda wtime isimli bir procedural macro kullanım örneği de mevcuttur.
*/
use proc_macros::work_time_effort;

mod model;
mod samples;
mod with_macro;
mod without_macro;

#[work_time_effort]
fn find_total() {
    let mut total = 0;
    for i in 0..100 {
        total += i;
    }
    println!("Result: {}", total);
}

fn main() {
    let total = find_total();
}
