/*
    Rust projelerinde organizasyonu Crate, Module ve Package kavramları ile sağlarız.

    Crate'ler projeler arasında metotların iş kurallarının paylaşımının sağlanması için kullanılır.
    Module türleri ise içinde bulunduğu library veya executable için ortak fonksiyonellikler sağlar.

    crates.io, rust ile yazılmış crate'lerin resmi olarak toplandığı yerdir.

    Bir library'deki modül ve içeriğinin organizasyonel olarak görünümünü öğrenmek için
    cargo-modules aracı kullanılabilir.

    Mesela bulunduğumuz workspace için şöyle bir komut işletilebilir ve S14_modules içeriği
    gözlemlenebilir
    cargo modules structure --package S14_modules

    pub(self) tanımını private erişim belirleyici olarak düşünebiliriz.
*/
mod borrow_system;
mod membership;
mod repository;

// mod library {
//     // pub struct Book {
//     //     pub title: String,
//     //     pub author: String,
//     //     pub can_be_borrowed: bool,
//     // }
//
//     // mod book_repository {
//     //     pub fn add() {}
//     //     pub fn remove() {}
//     //     pub fn list() {}
//     // }
//
//     // mod borrow_system {
//     //     pub fn borrow() {}
//     //     pub fn giving_back() {}
//     //     pub fn is_borrowable() {}
//     // }
//
//     // mod member_management {
//     //     pub fn subscribe() {}
//     //     pub fn unsubscribe() {}
//     //     fn list() {}
//     // }
// }

pub struct Book {
    pub title: String,
    pub author: String,
    pub can_be_borrowed: bool,
}
