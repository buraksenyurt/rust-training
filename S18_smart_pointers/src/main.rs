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

use std::fmt::{Display, Formatter};
use crate::boxing::*;

fn main() {
    simple_boxing();
}

fn simple_boxing() {
    // Örnek bir Boxing işlemi
    let value = 23; // Normalde stack' de saklanır
    let boxed_value = Box::new(value); // Şimdi heap'e alındı ama boxed_value hala stack'te zira adres göstermekte
    println!("Boxed value is {}", boxed_value);

    let identity = ("John Smith", 23, true); // tuple veriyi stack'ta saklar
    let boxed_identity = Box::new(identity); // Şimdi heap' te
    println!("Boxed identity is {:?}", boxed_identity);

    //recursive_data_model_with_error();
    recursive_sample();
}

