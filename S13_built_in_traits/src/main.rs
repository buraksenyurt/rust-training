/*
   Rust standart kütüphanesi bir dizi hazır trait sunar.
   Debug, Copy, Clone, Default, PartialEq, Drop gibi bir çok tür vardır.
   Aşağıdaki kod parçalarında bu trait'lerin kullanımlarına ait örnekler yer alıyor
*/
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let max_payne = Player {
        name: String::from("Max Payne"),
        level: 80,
        last_score: 167.58,
    };
    // Debug trait'ini uygulamazsak
    // `Player` does not implement `Debug` (required by `{:#?}`): E0277
    println!("{:#?}", max_payne);

    // Aşağıdaki kullanım ise Display trait'ini uygulamayı gerektirir
    println!("{}", max_payne);

    // Clone trait uygulaması
    let mut red_one = max_payne.clone();
    red_one.name = String::from("Red Leader One");
    println!("{:#?}", red_one);

    // PartialEq ve Copy, Clone örnekleri
    let my_speed = Velocity { x: 10.0, y: 0.0 };
    let your_speed = Velocity { x: 10.0, y: 0.0 };
    if my_speed == your_speed {
        println!("{my_speed:?} is equal to {your_speed:?}")
    }
    accelerate(your_speed);
    // Eğer Velocity için Copy trait kullanmazsak
    // Value used after being moved [E0382] derleme hatası oluşur
    if my_speed == your_speed {
        println!("{my_speed:?} is equal to {your_speed:?}")
    } else {
        println!("{my_speed:?} is not equal to {your_speed:?}")
    }

    // PartialEq örneği
    let log_1 = LogStamp {
        id: 1234,
        time: String::from("2024-10-20T09:56-PM"),
        content: String::from("Check with ping"),
    };
    let log_2 = LogStamp {
        id: 1234,
        time: String::from("2024-10-20T09:58-PM"),
        content: String::from("Check with ping"),
    };
    if log_1 == log_2 {
        println!("Same logs\n{log_1:?}\n{log_2:?}");
    }

    // Default trait örneği
    let my_game = Game::default();
    println!("{:#?}", my_game);
    println!("{}", my_game);

    // FromStr kullanımı
    let valid_color = "125:100:100".parse::<SystemColor>();
    match valid_color {
        Ok(color) => println!("Parsed color: {:?}", color),
        Err(e) => println!("Error parsing color: {:?}", e),
    }

    let invalid_color = "red:100:100".parse::<SystemColor>();
    match invalid_color {
        Ok(color) => println!("Parsed color: {:?}", color),
        Err(e) => println!("Error parsing color: {:?}", e),
    }

    // Drop trait kullanım örneği
    {
        let author_service = Service {
            title: String::from("Author service"),
            address: String::from("https://azon/services/author"),
        };
        println!("{:#?} is ready.", author_service);
        author_service.call();
    } // Drop trait tetiklenir
}

fn accelerate(mut speed: Velocity) {
    speed.x += 1.0;
    speed.y += 1.0;
    println!("Accelerating a little bit. {:#?}", speed);
    println!("Normalizing");
}

/*
   Bir player değişkeni Debug trait sayesinde {:?} veya
   {:#?} gösterimlerini doğrudan destekler.
   Çıktı otomatik olarak JSON formatına döndürülür.

   Player veri yapısı aynı zamanda Clone trait'ini devralır.
   Buna göre klonlanarak (shallow copy) taşıma işlemlerine dahil edilebilir.
*/
#[derive(Debug, Clone)]
struct Player {
    name: String,
    level: u8,
    last_score: f32,
}

/*
   Display trait uygulandığı veri modelinin bir Formatter bileşeni
   ile çalışabilmesini sağlar.
   fmt fonksiyonundan write makrosu ile döndürülen içerik,
   örneğin println! makrosunda {} formasyonunda kullanılabilir.
*/
impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}) last score is {}",
            self.name, self.level, self.last_score
        )
    }
}

/*
   Aşağıdaki örnekte ise Velocity türünden değişkenlerin
   eşitlik operatörü ile kontrol edilebilmeleri için PartialEq
   trait'ini devralması söz konusudur.

   Velocity tamamen f32 değerlerinden oluştuğundan move yerine kopyalanarak
   taşıma özelliği de sağlayabilir. Bunun için Copy trait'ini devralması yeterlidir.
*/
#[derive(Debug, PartialEq, Copy, Clone)]
struct Velocity {
    x: f32,
    y: f32,
}

// impl PartialEq for Velocity {
//    fn eq(&self, other: &Self) -> bool {
//        self.x == other.x && self.y == other.y
//    }
//}

/*
    Bazı durumlarda eşitlik kontrolü için veri modelinin tüm üyelerine bakılmaz.
    Bu gibi bir durumda PartialEq trait açıkça uygulanabilir.
*/
#[derive(Debug)]
struct LogStamp {
    id: i32,
    time: String,
    content: String,
}
impl PartialEq for LogStamp {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/*
   Bir veri yapısının default değerleri ile oluşturmak için
   Default trait kullanılabilir. derive edilebileceği gibi açıkça yazılabilir de.
*/
#[derive(Debug)]
struct Game {
    fps: u8,
    title: String,
    screen_width: u32,
    screen_height: u32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            fps: 30,
            title: String::from("A Simple Game"),
            screen_width: 1280,
            screen_height: 960,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}({}x{}) at {} fps",
            self.title, self.screen_width, self.screen_height, self.fps
        )
    }
}

/*
    Popüler trait'lerden birisi de FromStr'dır.
    Bir String ifadeden dönüşütürme işlemini tariflememizi sağlar.

    FromStr trait Result dönen bir fonksiyon sağlar. Dolayısıyla Result
    değişkeninin Err tipi de programcı tarafından sağlanır.
    Aşağıdaki örnekte 125:100:100 gibi bir metinsel bir içerik geldiğinde
    Red Green Blue değerlerinin çekilmesi için bir uyarlama yapılmıştır.
*/
#[derive(Debug, Copy, Clone)]
struct SystemColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
enum ColorParseError {
    WrongArgumentCount,
    ParseError(ParseIntError),
}

impl FromStr for SystemColor {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
            return Err(ColorParseError::WrongArgumentCount);
        }

        let r = parts[0]
            .parse::<u8>()
            .map_err(ColorParseError::ParseError)?;
        let g = parts[1]
            .parse::<u8>()
            .map_err(ColorParseError::ParseError)?;
        let b = parts[2]
            .parse::<u8>()
            .map_err(ColorParseError::ParseError)?;

        Ok(SystemColor { r, g, b })
    }
}

/*
   Drop trait.
   Bu trait bir değişken drop edileceği sırada yapılması istenenlerin
   gerçekleştirilebileceği bir fonksiyonellik sağlar.
*/
#[derive(Debug)]
struct Service {
    title: String,
    address: String,
}

impl Service {
    fn call(&self) {
        println!("Service call");
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        println!("Closing temp connections and releasing resources.");
    }
}
