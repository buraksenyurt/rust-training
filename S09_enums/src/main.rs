/*
   Rust tarafında veriyi ifade etme şeklinin bir yolu da Enum kullanmaktır.
   Enum veri yapısı oldukça zengindir.
   İşte kullanıma ilişkin birkaç örnek.
*/
// Örnekte User enum değişkeninde aktivasyon tarihi için de yardımcı bir crate kullanıldı
use chrono::{DateTime, Utc};
// Bu örnekte rastgelelik gerektiren bir durum olduğu için bu crate eklendi
use rand::Rng;

enum Status {
    Success,
    InProgress,
    Error(String),
}

// Enum'lara da metotlar uygulanabilir
impl Status {
    fn get_info(&self) -> &str {
        match self {
            Status::Success => "Success",
            Status::InProgress => "InProgress",
            Status::Error(detail) => detail,
        }
    }
}

fn create_report(title: String) -> Status {
    if title.is_empty() {
        return Status::Error("Title cannot be empty".to_string());
    }

    // Sembolik olarak 3 ile bölünme durumuna göre geriye InProgress durumu döndürmekteyiz.
    let mut randomizer = rand::thread_rng();
    let value = randomizer.gen_range(0..=3);

    if value % 3 == 0 {
        return Status::InProgress;
    }

    Status::Success
}

/*
   Pek tabi enum türleri başka bir tipin verisinde de anlam kazanır.
   Örneğin bir oyuncunun seviyesi enum türü ile ifade edilebilir.

   Debug trait implementasyonuna şimdilik takılmayalım ancak Player için bu direktif uygulandığında,
   Level için de uygulanması gerekir, nitekim Player veri modeli Level'ı da kullanır.
   Debug trait, uygulandığı tür için {:?} veya {:#?} gibi kullanımlarda kolaylık sağlar.
*/
#[derive(Debug)]
enum Level {
    Rookie,
    Pro,
    Elit(f32),
}

#[derive(Debug)]
struct Player {
    title: String,
    level: Level,
    is_active: bool,
}

impl Player {
    fn new(title: String, level: Level) -> Self {
        Player {
            title,
            level,
            is_active: false,
        }
    }
    fn change_level(&mut self, level: Level) {
        self.level = level;
    }
    fn activate(&mut self) {
        self.is_active = true;
    }
    fn deactivate(&mut self) {
        self.is_active = false;
    }
}

fn main() {
    let result = create_report(String::new());
    println!("{}", result.get_info());

    let result = create_report(String::from("Last Sales"));
    println!("{}", result.get_info());

    let mut mario = Player::new(String::from("mario"), Level::Rookie);
    println!("{:?}", mario);
    mario.activate();
    mario.change_level(Level::Pro);
    println!("{:?}", mario);
    mario.change_level(Level::Elit(300.59));
    /*
       Enum veri yapısının güzel yanlarından birisi de pattern matching bloklarında ele alınabilmesidir.
       Aşağıda mario isimli Player değişkeninin Elit olma hali değerlendirilir.

       Bir enum değişkeni pattern match bloğuna dahil edildiğin olası tüm değerleri ele alınmalıdır.
    */
    match mario.level {
        // Elit değeri f32 türünden birde parametre barındırabiliyor
        Level::Elit(coin) => {
            println!("Now Mario is on elit league with {coin} coin",);
        }
        _ => {} // Diğer durumları ele almak istemiyoruz
    }
    mario.deactivate();
    println!("{:?}", mario);

    let mario = User::Inactive {
        name: "Can Cey Mario".to_string(),
    };
    println!("{:#?}", mario);

    /*
       Built-in gelen Option enum'ı generic bir türdür(Generic kavramı burada kafa karıştırabilir. Çok kısaca değinmek lazım)
       Eğer bir değer söz konusu ise onu temsil eder(örneğin User enum değeri).
       Eğer ortada bir değer yoksa None döner
    */
    if let Some(m) = mario.activate(Utc::now()) {
        println!("Mario etkinleştirildi");
        println!("{:#?}", m);
    }
}

/*
   Rust enum veri yapısının zenginliğini gösteren bir diğer örneği aşağıdaki gibi ele alabiliriz.

   Sistemdeki bir kullanıcı akfit veya pasif olma durumuna göre kullanılmak istenir.
   Klasik bir OOP stilinde User isimli bir sınıf yazıp aktif olup olmama durumunu bir bool özellik ile ele alabiliriz.
   Aşağıdaki kurguda ise kullanıcının Inactive veya Active olma halleri bir veri modeli olarak ifade edilmektedir.
   Kullanıcı inactive ise sadece adını taşıyan bir enum durumu söz konusudur.
   Active olması ise adı haricinde, aktiv olduğu tarih gibi bilgileri de içerir.
*/
#[derive(Debug)]
enum User {
    Inactive {
        name: String,
    },
    Active {
        name: String,
        active: bool,
        activation_date: DateTime<Utc>,
    },
}
impl User {
    /*
       activate metodu dönüş türü olarak da built-in Option enum'ını kullanır.
       Bazı diller null değer olmasın halini karşılamak için Options pattern'i ele alır,
       ya da olası Exception durumlarında bilgilendirme de dönebilmek için Resut pattern kullanır.
       Rust, bu iki durumu built-in karşılar. Option ve Result enum'ları bunun içindir.
    */
    fn activate(&self, activation_date: DateTime<Utc>) -> Option<User> {
        match self {
            // Inactive bir kullanıcı söz konusu ise kullanıcı ile ilgili bazı bilgiler barındıran Active değişkeni döner
            User::Inactive { name } => {
                let created = User::Active {
                    name: name.clone(),
                    active: true,
                    activation_date,
                };
                // Oluşturulan kullanıcı örneğin veri tabanına log olarak bırakılır
                Some(created)
            }
            User::Active { .. } => None, // Kullanıcı zaten aktivse None dönebiliriz
        }
    }
}
