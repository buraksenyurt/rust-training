/*
   Pointer
       - Bellek üzerindeki bir veri içeriğini işaret eden adres bilgisini taşıyan
           değişken olarak düşünülebilir.
       - Farkında olmadan şu ana kadar kullandığımız bir pointer vardır (&)
           ve datayı referans etmekte kullanılır.

   Smart Pointer
       - Pointer adreslerine ek metadata bilgileri veya kabiliyetler içerirler.
       - Rust diline özel bir kavram değildir esasında C++ orijinlidir.
       - Referanslar veriyi işaret ederken Smart Pointer’ lar genellikle sahipliğini de alır
           (Sürpriz! String ve Vec<T> türleri smart pointer olarak da geçer zira belli bir bellek
           adresindeki verinin sahipliğini alırlar ve onu manipüle etmemize izin verirler)
       - Deref ve Drop trait’lerini implemente eden struct türleri olarak tasarlanabilirler
           (Yani kendi Smart Pointer modellerimizi tasarlayabiliriz)

    Hangisi ne zaman?

        Box ve RefCell birden fazla sahipliği tek bir veri üzerinde sağlarken,
        Rc aynı veri üzerinden birden fazla sahiplik sunar.

        Box immutable veya mutable ödünç alma (borrowing) için derleme zamanında kontrol sağlar.
        Rc sadece immutable borrowing için derleme zamanında kontrol sağlar.
        RefCell immutable veya mutable ödünç alma için runtime'da kontrol sağlar.
*/
mod boxing;
mod rc_scenario;

use crate::boxing::*;
use crate::rc_scenario::*;

fn main() {
    simple_boxing();
    //recursive_data_model_with_error();
    recursive_sample();
    hello_rc();
    // run_rc_with_error();
    // run_rc_with_error_2();

    /*
        Alttaki senaryoda aslında farklı kullanım şekilleri de söz konusu olabilir.

        - Sadece bir vektör üzerinde çalışma yapıyorsak RefCell<Vec<Player>> kullanımı yeterlidir.
        - Vektörün paylaşımı söz konusu ise Rc<RefCell<Vec<Player>>> daha uygun olabilir.
        - Hem vektörü hem de içindeki elemanları paylaşmamız gerekirse Rc<Vec<RefCell<Player>>>
            daha iyi bir çözüm olabilir.

        Tabii şunu uutmamamak lazım ki hem Rc hem de RefCell kullanımı nedeniyle
        runtime maliyeti daha yüksektir (Zir burada referans sayımı ve mutasyon kontrolleri yapılır)
     */
    run_rc();
}
