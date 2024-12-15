mod api_sample;
mod iot_sample;
mod metric_sample;
/*
   Asenkron programlamada async, await, future ve task gibi kavramlar öne çıkar.
   async kullanımı thread kullanımı ile benzerlik gösterir ancak async ile çalışılan yapılar
   thread'lere nazaran daha az kaynak tüketir.

   Burada future:Future trait'ine bakmakta yarar vardır.
   Future trait asenkron çalışan bir operasyon için ileri zamanlı hesaplama desteği sağlar.
   Basit bir polling mantığı ile başlatılan asenkron işin tamamlanıp tamamlanmadığı kontrol
   edilir. Eğeri iş tamamlanmışsa hesaplama sonucu çekilebilir.

    use std::future;
    future::Future::poll();
*/

#[tokio::main]
async fn main() {
    println!("Starting...");

    api_sample::run().await;
    iot_sample::run().await;
    metric_sample::run().await;

    println!("Done!");
}
