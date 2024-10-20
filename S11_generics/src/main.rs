/*
   Generic türler ile tipler üzerinden genel soyutlamalar yapmak mümkün olur.
   Aynı fonksiyonu veya veri yapısını farklı türler için ayrı ayrı tasarlamak yerine
   generic alt yapısından yararlanılabilir.

   Generic kavramı C# ve Go gibi dillerde de yaygın olarak kullanılır.

*/
use std::fmt::{Debug, Display};
use std::ops::Add;

fn main() {
    log_any(3.14f32);
    log_any("TCP connection established");
    log_any(State::InProgress);
    log_any(false);

    let game_point = Point::<i32>::new(10, 20, 10);
    println!("{}", game_point.info());

    let vehicle_position: Point<f32> = Point::new(5.5, 3.14, 2.20);
    println!("Vehicle Position {}", vehicle_position.info());

    let vehicle_new_position: Point<f32> = vehicle_position.add(Point::new(1.0, 1.0, 2.0));
    println!("New position after move {}", vehicle_new_position.info());
}

#[derive(Debug)]
enum State {
    InProgress,
    Done,
    Error,
}

/*
   Örnek bir generic fonksiyon.
   parametre olarak gelen object değişkeni T türünden olabilir.
   T için herhangibir kısıt belirtilmediğinden herhangibir türü alabilir.

   Tabii burada henüz işlenmeyen iki trait kullanımı söz konusu.
   log_any fonksiyonu içerisindeki println! metodunda {:?} formasyonu kullanılmıştır.
   Bu formasyon ilgili türün Debug Trait'ini uygulamış olduğunu varsasyar.
   Buna göre gelen T türünün println! fonksiyonunda {:?} formasyonunda kullanılabilmesi
   için T'nin Debug trait'ini uygulamak zorunda olduğu belirtilmelidir.
   T:Debug bu amaçla yazılmıştır.
*/
fn log_any<T: Debug>(object: T) {
    println!("Logged value ise '{:?}'", object);
}

/*
   Kendi generic veri modellerimizi de tanımlayabiliriz.
   Yaygın kullanılan örneklerden birisi koordinat bilgisi veya kompleks sayılardır.

   Aşağıdaki 3D koordinat sistemi için bir veri modeli tanımı söz konusudur.

   Sadece T kullanmak veri yapısını bozabilir. Nitekim her tür buraya gelebilir.
   Bu nedenle generic constraint kullanmak mantıklıdır.
   Mesela Point yapısının sadece i32, u8, i16, f32 gibi sayısal türler ile çalışması için
   T'nin farklı bir şekilde kısıtlanması gerekir.

   Bu kısım belki eğitim için biraz zorlayıcı olabilir.

   Copy ve Add traitleri sayısal ifadeler için kullanılan davranışları tanımlar.
   Ayrıca T türünün {:?} formasyonu ile kullanılabilmesi için Debug trait'inin uygulanması gerekir.
*/
struct Point<T: Copy + Debug + Add<Output = T>> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy + Debug + Add<Output = T>> Point<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Point { x, y, z }
    }
    fn info(&self) -> String {
        format!("({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
