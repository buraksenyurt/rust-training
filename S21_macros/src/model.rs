/*
   Programda aşağıdaki gibi entity nesnelerimiz olduğunu düşünelim.
   Bu türler için basit CRUD operasyonlarına ihtiyacımız olacaktır.
   Yeni modeller ekledikçe benzer metotları ilave etmemiz gerekir.
   Buradaki yeniden kullanılabilir kodu makrolar ile oluşturabiliriz.
*/
pub struct Player {
    pub id: i32,
    pub name: String,
    pub rating: f32,
}

// impl Player {
//     fn create(id: i32, name: String, rating: f32) -> Self {
//         Player { id, name, rating }
//     }
// }

pub struct Game {
    pub id: i32,
    pub title: String,
    pub studio: String,
    pub year: u8,
}

// impl Game {
//     fn create(id: i32, title: String, studio: String, year: u8) -> Self {
//         Game {
//             id,
//             title,
//             studio,
//             year,
//         }
//     }
// }
