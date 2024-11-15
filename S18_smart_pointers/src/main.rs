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
*/
mod boxing;

use crate::boxing::*;

fn main() {
    simple_boxing();
    //recursive_data_model_with_error();
    recursive_sample();
}

