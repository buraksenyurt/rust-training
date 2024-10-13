# Rust Training

Rust ile ilgili temelden orta seviyeye eğitim vermek istesem hangi konuları hangi örneklerle ele alırdım sorusuna cevap aradığım repodur.

## Konular

- [ ] S00 - Hello World
- [ ] S01 - Variables, Mut and Scalar Types
- [ ] S02 - Vectors and Slices
- [ ] S03 - String and &str

## Yardımcılar

S03 bölümünde string slice (&str) ve string literal karmaşası için ek bilgi.

| Özellik      | String Literal (`&'static str`)                                 | String Slice (`&str`)                                                                              |
|--------------|-----------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| **Lifetime** | `'static` ömre sahiptir ve programın tamamı boyunca geçerlidir. | Herhangi bir ömür ile kullanılabilir (örneğin bir fonksiyonun yerel değişkenini referans edebilir) |
| **Memory**   | Bellekte statik olarak saklanır, sabit yer kaplar.              | Dinamik `String` veya string literalinin bir dilimini(parçasını) referans edebilir.                |
| **Resource** | Sabit ve değiştirilmez stringlerdir.                            | Bir `String` değişkenin ya da başka bir string'in bir parçası olabilir.                            |
| **Usage**    | Kaynak kodda doğrudan yazılan string değerleri ifade eder.      | Bir `String`'den veya string literalinden belirli bir kısmı almak için kullanılır.                 |

