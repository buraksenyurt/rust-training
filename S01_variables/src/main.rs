// Referans doküman https://doc.rust-lang.org/beta/book/ch03-02-data-types.html

fn main() {
    let last_score = 90;
    println!("Last score is {}", last_score);
    // last_score=80; // Default immutable olduğundan çalışmaz.

    let mut last_score = 80;
    println!("Last score is {}", last_score);
    last_score = 60;
    println!("Last score is {}", last_score);

    let _delta_time = 1.25; // default f64
    let _delta_time = 1.25_f32;
    let delta_time: f32 = 1.25;
    println!("Delta time is {}", delta_time);

    let _total_values = 1 + 2 + 3;
    let total_values: u8 = 1 + 2 + 3;
    println!("Total values is {}", total_values);

    /*
       Genellikle renk değerlerini temsil etmek veya bellek adreslerini ifade etmek için kullanılır.
       16'lık sistem, ikilik sisteme (binary) göre daha büyük bir aralık sunar ve sayıları daha
       kısa formatta ifade edebilir.
    */
    let color_in_hex = 0xFF0032;
    println!("Color is {color_in_hex} or {:x}", color_in_hex);

    /*
       Octal(8lik sistem), genellikle dosya izinlerini ve diğer POSIX sistemlerinde kullanılan izinleri göstermek için kullanılır.
       Unix ve Linux tabanlı sistemlerde dosya izinleri,Örneğin, chmod 755 komutu, dosya izinlerini 8'lik sistemde ayarlayan bir komuttur
    */
    let file_permission: u32 = 0o755;
    println!(
        "File permission is {file_permission} or {:o}",
        file_permission
    );

    /*
        Düşük seviye programlama ve bit manipülasyonunda kullanılabilir.
        Örneğin bit seviyesinden flag'leri kontrol etmek, belli bitleri açık kapamak gibi
    */
    let some_flags: u8 = 0b1010_0110;
    println!("Flags is {some_flags} or {:b}", some_flags);

    let is_valid = true;
    println!("Is valid? {}", is_valid);

    let is_even = 10 % 2 == 0;
    println!("Is even? {}", is_even);

    let up_button = 'w';
    println!("Button is {:}", up_button);

    println!("Max Height is {MAX_HEIGHT}");
    println!("Background color is {:?}", BACKGROUND_COLOR);

    // Tuple, compound type olarak bilinen türlerndendir.
    // Farklı tür değerlerini tek bir çatı altında toplayan türler Compound Type olarak adlandırılır
    let arguments = (640, 480, String::from("Game Title"), false);
    println!("Arguments is {:#?}", arguments);
    println!("Screen width and height {}x{}", arguments.0, arguments.1);

    let (title, is_active) = (arguments.1, arguments.2);
    println!("Title is {}", title);
    println!("Is active? {}", is_active);

    // Array türü sadece aynı türden verileri taşır
    let points = [9.0, 8.5, 7.2, 8.4, 9.1];
    println!("Points is {:#?}", points);
    println!("First value is {}", points[0]);
    println!("Second value is {}", points[1]);
    // Aşağıdaki satırda ısrar etsek bile derleme zamanında hata alırız.
    // println!("None value is {}", points[999]); //Index out of bounds: array length is 5 but the index is 999

    let mut numbers = [1, 2, 3, 4, 5];
    numbers[2] = 8;
    println!("Mutated second value is {}", numbers[2]);
}

const MAX_HEIGHT: u16 = 1280;
const BACKGROUND_COLOR: (u8, u8, u8) = (255, 255, 0);
