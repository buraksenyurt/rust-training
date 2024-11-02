# Rust Training

Rust ile ilgili temelden orta seviyeye eğitim vermek istesem hangi konuları hangi örneklerle ele alırdım sorusuna cevap aradığım repodur.

Buradaki konular ile ilgili birde Youtube serimiz var. ["Birlikte Rust Öğrenelim"](https://www.youtube.com/playlist?list=PLY-17mI_rla4zcAQtUsolk6G5bfbQAdYZ)

## Konular

- [ ] S00 - Hello World
- [ ] S01 - Variables, Mut and Scalar Types
- [ ] S02 - Vectors and Slices
- [ ] S03 - String and &str
- [ ] S04 - Functions
- [ ] S05 - Control Flows
- [ ] S06 - Ownership and Move
- [ ] S07 - Structs
- [ ] S08 - Lifetimes
- [ ] S09 - Enums
- [ ] S10 - Pattern Matching
- [ ] S11 - Generics
- [ ] S12 - Traits
- [ ] S13 - Built-In Traits
- [ ] S14 - Modules
- [ ] S15 - 
- [ ] S99 - Questions

## Yardımcılar

Rust dilinde isimlendirme standartları da kod okunurluğu ve genel uyumluluk açısından önemlidir. Aşağıdaki isimlendirme önerilerine ait bilgilerin yer aldığı bir tablo bulunmaktadır.

| Kategori               | İsimlendirme Standardı                   |
|------------------------|------------------------------------------|
| Constants              | SCREAMING_SNAKE_CASE                     |
| Conversion constructors| from_some_other_type                     |
| Crates                 | unclear                                  |
| Enum variants          | UpperCamelCase                           |
| Features               | unclear but see C-FEATURE                |
| Functions              | snake_case                               |
| General constructors   | new / init                               |
| Lifetimes              | 'a, 'de, 'src                            |
| Local variables        | snake_case                               |
| Macros                 | snake_case!                              |
| Methods                | snake_case                               |
| Modules                | snake_case                               |
| Statics                | SCREAMING_SNAKE_CASE                     |
| Traits                 | UpperCamelCase                           |
| Type parameters        | T, K gibi olabilir                       |
| Types                  | UpperCamelCase                           |

S03 bölümünde string slice (&str) ve string literal karmaşası için ek bilgi.

| Özellik      | String Literal (`&'static str`)                                 | String Slice (`&str`)                                                                              |
|--------------|-----------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| **Lifetime** | `'static` ömre sahiptir ve programın tamamı boyunca geçerlidir. | Herhangi bir ömür ile kullanılabilir (örneğin bir fonksiyonun yerel değişkenini referans edebilir) |
| **Memory**   | Bellekte statik olarak saklanır, sabit yer kaplar.              | Dinamik `String` veya string literalinin bir dilimini(parçasını) referans edebilir.                |
| **Resource** | Sabit ve değiştirilmez stringlerdir.                            | Bir `String` değişkenin ya da başka bir string'in bir parçası olabilir.                            |
| **Usage**    | Kaynak kodda doğrudan yazılan string değerleri ifade eder.      | Bir `String`'den veya string literalinden belirli bir kısmı almak için kullanılır.                 |

S06 Ownership bölümünde Copy ve Clone trait'lerinden bahsedilmektedir. Burada Copy ve Clone arasındaki farkı da bilmek gerekir. Bu kavramlar bellekte veri çoğaltma işlemleriyle ilgilidir, ancak işleyişleri ve kullanım alanları farklıdır.

- **Copy**
  - Copy özelliği, i32, f64, bool, char gibi küçük ve sabit boyutlu türlerde kullanılır.
  - Bir türün Copy davranışına sahip olması, onun bellekte kopyalanmasının çok hızlı ve maliyetsiz olduğunu da gösterir. _(Shallow Copy)_ işlemi de diyebiliriz.
  - Copy işlemi, sadece basit ve küçük veri türleri için geçerlidir. String veya Vec gibi büyük veri yapıları tutmaya aday yapılar Copy davranışı sergilemezler, çünkü bunların derin kopyalanması(Deep Copy) gerekir.
- **Clone**
  - Clone, bir veri yapısının derin kopyasını _(Deep Copy)_ oluşturur. Verinin içeriği kopyalanarak yeni bir alan yaratılır.
  - Clone operasyonu daha maliyetlidir çünkü veri yapısının tamamının kopyalanmasını gerektirir.
  - String ve Vec gibi heap üzerinde veri tutan yapılar Clone trait implementasyonu kullanır _(String türünün koduna bakılabilir)_.

Burada Shallow Copy ile Deep Copy kavramları zaman zaman birbirlerine karıştırılır. Shallow copy tekniğinde verinin sadece yüzeysel kopyalanması söz konusuru. Sadece adres işaretçisi _(pointer)_ veya adres bilgisi kopyalanır. Ancak gösterilen asıl veri (heap'teki veri) kopyalanmaz. Bu durumda, hem orijinal hem de kopyalanan veri yapısı aynı heap alanını işaret eder.

Rust dilinde kullanılan i32, f64, bool, gibi basit türler stack alanında depolandığı için kopyalanmaları shallow copy gibidir  keza verinin kendisi doğrudan kopyalanır ve heap'e erişim yoktur.

Deep copy dediğimizde ise verinin tamamının yani hem işaretçilerin hem de heap üzerindeki asıl verinin kopyalanmasını söz konusudur. Bu yeni bir bellek alanı tahsis edilmesi ve orijinal verinin açılan yeni alana tamamen kopyalanması demektir. Çok doğal olarak bunun bir maliyeti vardır. İşlem sonrasında orijinal veri ile kopyalanan veri birbirlerinden bağımsız kullanılabilirler.

Bir String nesnesi oluşturulduğunda, içerik bir vektör olarak karakter seti halinde heap bellek bölgesinde tutulur. Sadece shallow copy yapılırsa birbirine atanan iki string de aynı heap alanını işaret ederdi. Rust'ta String Copy trait'ini bilinçli olarak uygulamaz ve bu tip referans eşleşmelerini zaten istemez. Ancak clone() kullanılarak Deep Copy yapılabilir ve heap üzerindeki karakter serisi yeni bir alana kopyalanır.

S07 Struct konusunu işlerken dikkat edilmesi gereken bir kavram karmaşası var. Fonksiyon mu metot mu? Metotlar, fonksiyonlara benzer ancak metotlar esasında Struct context'ine dahildir ve struct ile ilişkilendirilerler.
