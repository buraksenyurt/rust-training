# Rust Training

Rust ile ilgili temelden orta seviyeye eğitim vermek istesem hangi konuları hangi örneklerle ele alırdım sorusuna cevap aradığım repodur.

Buradaki konular ile ilgili birde Youtube serimiz var. ["Birlikte Rust Öğrenelim"](https://www.youtube.com/playlist?list=PLY-17mI_rla4zcAQtUsolk6G5bfbQAdYZ)

## Konular

- [x] S00 - Hello World
- [x] S01 - Variables, Mut and Scalar Types
- [x] S02 - Vectors and Slices
- [x] S03 - String and &str
- [x] S04 - Functions
- [x] S05 - Control Flows
- [x] S06 - Ownership and Move
- [x] S07 - Structs
- [x] S08 - Lifetimes
- [x] S09 - Enums
- [x] S10 - Pattern Matching
- [x] S11 - Generics
- [x] S12 - Traits
- [x] S13 - Built-In Traits
- [x] S14 - Modules, Crates, Packages
- [x] S15 - Error Handling
- [x] S16 - Testing
- [x] S17 - Closures
- [x] S18 - Smart Pointers _(Box<T>, Rc<T> ve RefCell<T>)_
- [x] S19 - Concurrency _(Threads)_
- [ ] S20 - 
- [ ] S21 - 
- [ ] S99 - Questions

## Yardımcılar

Rust dilinde **isimlendirme standartları _(Naming Conventions)_** da kod okunurluğu ve genel uyumluluk açısından önemlidir. Aşağıdaki isimlendirme önerilerine ait bilgilerin yer aldığı bir tablo bulunmaktadır.

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

**S03** bölümünde string slice (&str) ve string literal karmaşası için ek bilgi.

| Özellik      | String Literal (`&'static str`)                                 | String Slice (`&str`)                                                                              |
|--------------|-----------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| **Lifetime** | `'static` ömre sahiptir ve programın tamamı boyunca geçerlidir. | Herhangi bir ömür ile kullanılabilir (örneğin bir fonksiyonun yerel değişkenini referans edebilir) |
| **Memory**   | Bellekte statik olarak saklanır, sabit yer kaplar.              | Dinamik `String` veya string literalinin bir dilimini(parçasını) referans edebilir.                |
| **Resource** | Sabit ve değiştirilmez stringlerdir.                            | Bir `String` değişkenin ya da başka bir string'in bir parçası olabilir.                            |
| **Usage**    | Kaynak kodda doğrudan yazılan string değerleri ifade eder.      | Bir `String`'den veya string literalinden belirli bir kısmı almak için kullanılır.                 |

**S06 Ownership** bölümünde **Copy** ve **Clone** trait'lerinden bahsedilmektedir. Burada **Copy** ve **Clone** arasındaki farkı da bilmek gerekir. Bu kavramlar bellekte veri çoğaltma işlemleriyle ilgilidir, ancak işleyişleri ve kullanım alanları farklıdır.

- **Copy**
  - Copy özelliği, i32, f64, bool, char gibi küçük ve sabit boyutlu türlerde kullanılır.
  - Bir türün Copy davranışına sahip olması, onun bellekte kopyalanmasının çok hızlı ve maliyetsiz olduğunu da gösterir. _(Shallow Copy)_ işlemi de diyebiliriz.
  - Copy işlemi, sadece basit ve küçük veri türleri için geçerlidir. String veya Vec gibi büyük veri yapıları tutmaya aday yapılar Copy davranışı sergilemezler, çünkü bunların derin kopyalanması _(Deep Copy)_ gerekir.
- **Clone**
  - Clone, bir veri yapısının derin kopyasını _(Deep Copy)_ oluşturur. Verinin içeriği kopyalanarak yeni bir alan yaratılır.
  - Clone operasyonu _daha maliyetlidir_ çünkü veri yapısının tamamının kopyalanmasını gerektirir.
  - String ve Vec gibi heap üzerinde veri tutan yapılar Clone trait implementasyonu kullanır _(String türünün koduna bakılabilir)_.

Burada **Shallow Copy** ile **Deep Copy** kavramları zaman zaman birbirlerine karıştırılır. Shallow copy tekniğinde verinin sadece yüzeysel kopyalanması söz konusuru. Sadece adres işaretçisi _(pointer)_ veya adres bilgisi kopyalanır. Ancak gösterilen asıl veri _(heap'teki veri)_ kopyalanmaz. Bu durumda, hem orijinal hem de kopyalanan veri yapısı aynı heap alanını işaret eder.

Rust dilinde kullanılan **i32, f64, bool,** gibi basit türler stack alanında depolandığı için kopyalanmaları shallow copy gibidir  keza verinin kendisi doğrudan kopyalanır ve heap'e erişim yoktur.

**Deep copy** dediğimizde ise verinin tamamının yani hem işaretçilerin hem de heap üzerindeki asıl verinin kopyalanmasını söz konusudur. Bu yeni bir bellek alanı tahsis edilmesi ve orijinal verinin açılan yeni alana tamamen kopyalanması demektir. Çok doğal olarak bunun bir maliyeti vardır. İşlem sonrasında orijinal veri ile kopyalanan veri birbirlerinden bağımsız kullanılabilirler.

Bir **String** nesnesi oluşturulduğunda, içerik bir vektör olarak karakter seti halinde heap bellek bölgesinde tutulur. Sadece **shallow copy** yapılırsa birbirine atanan iki string de aynı heap alanını işaret ederdi. Rust dilinde String veri türü Copy trait'ini bilinçli olarak uygulamaz ve bu tip referans eşleşmelerini zaten istemez. Ancak **clone()** metodu kullanılarak **Deep Copy** yapılabilir ve heap üzerindeki karakter serisi yeni bir alana kopyalanır.

**S07 Struct** konusunu işlerken dikkat edilmesi gereken bir kavram karmaşası var. Fonksiyon mu metot mu? Metotlar, fonksiyonlara benzer ancak metotlar esasında bir Struct Context'ine dahildir ve struct ile ilişkilendirilerler.

**S14** isimli bölümde package, crate ve module kavramlarına kısaca değiniyoruz. Her uygulama çözümünde olduğu gibi rust tarafından da projelerin organizasyonu mümkün. Aşağıdaki şema bunu desteklemek amacıyla kullanılabilir.

![image](https://github.com/user-attachments/assets/00f92509-637e-4f4d-b104-cc1f4abf3c7f)

Bir e-commerce sisteminin olduğunu düşünelim. Burada birçok işlevi içeren kütüpahenelerimiz bu kütüphaneleri referans olarak alan ve kullanan başka binary'lerimiz olabilir. Bir backend kütüphaneler topluluğu ve bir web uygulaması ya da api servisi olduğunu düşünelim. Klasik senaryo... En dış bloğu workspace olarak düşünebiliriz. Kendi **Cargo.toml** dosyasını içeren bir klasördür aslında. Diğer projeler _(library ve binary türleri)_ bu workspace altına açılabilir. C# tarafındakiler için Workspace'in bir Solution olarak düşünülebileceğini söylesem sanırım yeterli olacaktır. Workspace aynı zamanda bir Package olarak da yorumlanabilir. Paketler birden fazla kütüphane barındıran toplu çözümlerdir. [Crates.io](https://crates.io) sisteminde bu tip birçok kullanışlı paket yer alır. Bir paket içerisinde yer alan library ve binary'ler birer **Crate** olarak da düşünülebilir. Crate'ler içerisinde modüller _(Modules)_ ve hatta alt modüller _(Sub Modules)_ yer alır. Çalıştığımız projelerin structure iskeletini görebilmek **cargo-modules** aracından yararlanabiliriz.

```bash
cargo install cargo-modules

# ve örneğin
cargo modules structure --package S14_modules
```
Bu kullanım aşağıdakine benzer bir çıktı üretir.

![image](https://github.com/user-attachments/assets/de0db625-0d52-463f-ad82-04b3b70ff999)

**S18**' de ele alınan **Smart Pointer** konusu ile ilgili bazı notlar;

**Pointer** bellek üzerindeki bir veri bölgesini işaret eden adres bilgisini taşıyan değişken olduğunu tanımlanabilir.

Farkında olmadan şu ana kadar kullandığımız bir işaretçi de vardır esasında. Birçok yerde **&** sembolü ile datayı referans ederek kullandığımızı fark etmiş olmalısınız. **Smart Pointer**'lar ise işaretçilerin taşıdığı adresler bilgilerine ek farklı metadata bilgileri veya kabiliyetler içerirler. Aslında bu konu Rust diline özel bir kavram değildir ve esasen **C++** orijinli bir unsurdur. Referanslar _(yani & ile kullandığımız değişkenler)_ veriyi işaret ederken Smart Pointer’ lar genellikle verinin sahipliğini de alır _(Sürpriz! String ve Vec<T> türleri smart pointer olarak da geçer zira belli bir bellek adresindeki verinin sahipliğini alırlar ve onu manipüle etmemize izin verirler)_ **Deref** ve **Drop** trait’lerini implemente eden struct türleri olarak düşünebiliriz. Bir başka deyişle kendi Smart Pointer modellerimizi tasarlayabiliriz. 

Rust standart kütüphanesinde bilinen ve sık kullanılan bazı **Smart Pointer** yapıları ise sırasıya **Box < T >** , **Rc < T >** ve **RefCell < T >** orijinli **Ref < T >** ile **RefMut < T >** türleridir. Box nesnesi, değişken verisi için Heap' te yer ayrılmasına izin verir. Bunu normal şartlarda stack'e alınan bir veri türü için Heap' te yer ayrılmasına ve işaret edilmesine benzetebiliriz. Ne var ki asıl senaryolar daha farklı olabilir. Örneğin çok büyük bir verinin sahipliğinin taşınırken bu işin kopyalanmadan gerçekleştirilmesini istediğimiz durumlarda işe yarar. **Rc** işaretçisi birden fazla sahiplik _(ownership)_ imkanı sağlayan ve bu referansları sayan türdendir. Burada sahiplik kalmayıncaya kadar aynı veriyi sahiplenen n sayıda referans olabileceğini ifade edebiliriz. Rust' ın resmi dokümanı bu konuda Tv odası örneğini verir. Televizyon **Rc < T >** nesnesidir ve odaya gelen biri televizyonu açar. Odaya gelen diğer insanlar da televizyonu izleyebilir. En son kişi odadan çıkınca da televizyonu kapatır _(Ya açık bırakıp giderse :P Olmuyor işte öyle bir şey)_ Burada Tv sahiplikleri de sayar ve kimse kalmayınca da kapanmış olur bir nevi belleten düşer diyebiliriz. 

**RefCell < T >** türünün senaryosu ise biraz karışık gelebilir zira ödünç alma _(borrowing)_ ilkelerinin derleme zamanı yerine çalışma zamanında _(runtime)_ kullanılmasına olanak tanır. Neden böyle bir şeye ihtiyacımız olsun zira Rust' ın ana felsefesini çalışma zamanında niye kıralım gibi sorular aklımıza düşebilir elbette. Esasında bunun için gerekli senaryolar olur. Örneğin derleme zamanında izin verilmeyen ama memory safe olacağı garanti edilen bazı senaryoları çalışma zamanında işletmenin yolunu açar. Ancakkkk burada dikkat edilmesi gereken bir husus vardır. Herhangi bir ihlal çalışma zamanının **panic!** lemesine yol açar. Bu konuda Rust resmi kitabı kaynağı neredeyse 1900 yılına dayanan önemli bir problemi örnek gösterir. Dilimize Sonlanma Problemi olarak çevrilen Halting Problem. Bir programın sonsuz döngü halinde sonlanıp sonlanamayacağına karar verememesi gibi özetlenebilir sanıyorum ki ancak an itibariyle beni aşan bir konu. Zira hesaplanabilirlik kuramının bir parçası. Bolca matematik işte.

Smart Pointer kullanımı ile ilgili şöyle bir özet bilgi de verebiliriz.

![image](https://github.com/user-attachments/assets/9d0fdc34-a201-40dc-b8db-b6399ef6e825)

**S19**' da ele alınmaya başlayan thread kavramı ayrıca **Concurrency** ve **Parallel Programming** gibi modellerle de yakın ilişkilidir. Bu noktada Concurrency _(Eş Zamanlılık)_ ve paralel programlama modelleri arasındaki farkları da bilmek gerekir.

**Concurrency** modelinde temel amaç bir sistemin birden fazla görevi _(task)_ yönetibilmesini veya reaksiyon verebilmesini sağlamaktır. Başlatılan görevler ya da işler aynı anda tamamlanmasa bile sistem sanki hepsini aynı anda çalıştırıyormuş gibi görünebilir. Esasında sistem başlatılan görevler arasında fark edilmeyecek kadar küçük zaman dilimlerinde geçişler yaparak planlı bir işleyişi yerine getirir. Bunu yaparken birden fazla thread kullanımı tekniklerden birisidir ama **Event Loop** yaklaşımının kullanıldığı da görülmektedir.

Parallel Programming modelinde amaç işlemleri _(ya da hesaplamaları)_ aynı anda gerçekleştirmektir. Ancak bu yapılırken gerçekten de aynı anda çalışan fiziki iş parçacıkları söz konusudur. Dolayısıyla çekirdek sayısı ve buna bağlı kaynaklar _(resources)_ bu metodolojide kilit noktadır. Örneğin büyük bir veri kümesinin sekiz parçaya bölünüp her parçanın bir CPU çekirdeği tarafından işlenmesi buna örnek gösterilebilir. Bu modelde thread'ler fiziksel olarak paralel şekilde çalıştırılır.

|                 | **Concurrency**                                           | **Parallel Programming**                                                 |
|-----------------|-----------------------------------------------------------|--------------------------------------------------------------------------|
| **Tanım**       | Birden fazla işin aynı anda başlatılması ve yönetilmesi.  | Birden fazla işin fiziksel olarak aynı anda gerçekleştirilmesi.          |
| **Amaç**        | Görevlerin birbirine karışmadan yönetilmesi.              | İşlerin gerçekten fiziksel kaynaklar arasında bölünerek hızlandırılması. |
| **Araçlar**     | `async/await`, `Future`, `tokio`, `async-std`             | `thread::spawn`, `rayon`, veya işlemci çekirdeği bazlı.                  |
| **Planlama**    | Zamanlayıcı bir araç söz konusudur (`tokio` gibi).        | OS veya kütüphane thread'ler üzerinden paralellik sağlar.                |
| **Eşzamanlılık** | İşler sırayla veya birbirine bağımlı olmadan çalışabilir. | İşler fiziksel olarak aynı anda çalışır.                                 |
| **Örnek**       | Asenkron dosya I/O, web sunucuları.                       | Büyük veri işleme, paralel hesaplama.                                    |

Tabii asenkron çalışma modelleri söz konusu olunca akla Go programlama dili ve ona has kabul edilen Goroutine'ler akla gelmektedir. **Goroutine**'ler esasında **Coroutine** olarak da bilinen bir modelin uyarlamasıdır.  Coroutine'ler görevler _(task)_ arasında hızlı ve kolay geçişler yapılmasını sağlayarak asenkronluğu icra etmek için kullanılır. Golang' de Goroutine çalışma zamanı bunu otomatik olarak yönetir. Rust tarafında benzer bir amaçla Future'lar kullanılır ama genelde harici bir executor' a ihtiyaç duyulur. Bu executor'lardan en popüleri artık defacto haline gelmiş olan tokio paketidir. Asenkron programlamada hafıza yönetimi de önemlidir ki Rust bilindiği üzere bir **Garbage Collector** mekanizması içermez ama bellek güvenliğini önceliklendirir. Burada **ownership** ve **borrow checker** modelleri devreye girer ve hatta eş zamanlı olarak aynı veri kümelerinde çalışılacağı zaman bazı smart pointer'ler ve **Mutex**'ler ele alınır. Performans açısından bir kıyaslama yapmak doğru mudur emin değilim ama Golang tarafındaki Goroutine'ler hafif _(az yer kaplar, çalışmak için az kaynağa ihtiyaç duyar)_ ve hızlıdır. Rust ise daha kontrollü ve memory safe bir alan sağlar.
