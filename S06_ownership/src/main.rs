fn main() {
    {
        // Kasıtlı bir scope başlangıcı
        let player_score = 90;
        println!("Player Score is {}", player_score);
        let _title = String::from("Programming with Rust");
        let _size = (640_f32, 480_f32);
    } // Scope sonu

    // println!("Player Score is {}",player_score); // Scope dışından owner_score'a erişilemez(dropped)

    move_rule();
    move_rule_with_clone();

    /*
       i32 gibi türler Copy trait'ini uygularlar. Bu tip boyutça küçük veri türleri
       kopyalama ile taşındıklarından bir move ihlali söz konusu olmaz.
       Copy ve Clone aslında birer Trait' tir ve belli veri türlerine uygulanır.
       Clone, deep copy işlemi uyguladığından Copy operasyonuna göre maliyetlidir.
    */
    let number = 32u8;
    let other_number = number;
    println!("Number is {}", number);
    println!("Other number is {}", other_number);

    /*
       Örneğin String türü otomatik bir kopyalama işlemi yapmaz. Açıkça klonlamak gerekir.
       String veri türünün Rust tarafındaki implementasyonuna bakmakta yarar var.
       Clone trait'ini uyguladığını açıkça görebiliriz.

       Eğer first_name'i full_name'i oluşturmak için kullanırken clone'larsak move sorunu giderilir.
       Yine de Clone operasyonunun maliyetli olduğunu unutmamamlıyız.
    */
    let first_name = String::from("Burak");
    let full_name = first_name + " Selim Şenyurt";
    println!("Full name is {}", full_name);
    // println!("First name is {}", first_name); // Value used after being moved [E0382]

    let book_title = String::from("Programming with Rust");
    search_book(book_title); // sahiplik(ownership) metota geçer
                             // Bu nedenle aşağıdaki satırda artık book_title kullanılamaz.
                             // println!("Book title is {}", book_title); // Value used after being moved [E0382]

    /*
       Aşağıdak örnekte de, bir String değişken check_department fonksiyonuna gönderilmekte.
       Fonksiyon söz konusu String değişkenin sahipliğini alıp bazı operasyonlar gerçekleştirdikten
       sonra aynen geriye dönebilir.
    */
    let department = String::from("IT");
    let department = check_department(department); // Sahiplik fonksiyondan geri alınıyor
    println!("The department is {}", department);

    /*
        Birde Shallo Copy'yi baz alan değer türlerine bakalım.
        f32'de stack bazlı bir veri türüdür ve Deep Copy yerine basit veri kopyalamasını kullanır (Copy)
        Bu nedenle metoda geçen değer otomatik olarak kopyalanarak gider.
        Dönüşünde tekrardan kullanılabilir. Özetle move ihlali olmaz.
    */
    let amount = 955.50f32;
    send_money(amount);
    println!("{} coin sent", amount);

    /*
       Aşağıdaki kod örneğinde String veri türünün referans yolu ile bir metoda parametre olarak
       geçilmesi söz konusudur. & ile level_name referansı taşındığından bir sonraki satırda
       move ihlali söz konusu olmayacaktır.
    */
    let level_name = String::from("Casino Royal");
    load_level(&level_name);
    println!("Now game at level {}", level_name);

    /*
       Aşağıdaki kullanım da mümkündür.
       some_words değişkeninin sahipliği mutable referans olarak trim_last metoduna taşınır.
    */
    let mut some_words = String::from("Some words !.");
    trim_last(&mut some_words);
    println!("{}", some_words);
    trim_last(&mut some_words);
    println!("{}", some_words);

    /*
       Ownership kurallarına göre bir veri aynı anda yalnızca bir mutable referansa sahip olabilir
       ya da aynı anda birden fazla shared referansa mümkündür.
    */
    let mut motto = String::from("It's a Beautiful Day !.");
    let upper_counts = get_upper_counts(&motto);
    load_level(&motto);
    trim_last(&mut motto);
    println!("Upper Letter counts: {}", upper_counts);
}

fn get_upper_counts(context: &String) -> usize {
    let mut count = 0;
    for c in context.chars() {
        if c.is_ascii_uppercase() {
            count += 1;
        }
    }
    count
}

fn load_level(name: &String) {
    println!("Loading level {}", name);
}

fn trim_last(context: &mut String) {
    context.pop();
}

fn send_money(money: f32) {
    println!("{} coin is sending...", money);
}

fn check_department(department: String) -> String {
    println!("Calculating defects for '{}' department", department);
    department
}

/*
   Bu fonksiyon parametre olarak gelen title değişkeninin sahipliğini de alır (ownership)
   Dolayısıyla metottan çıkılırken scope kuralı gereği title değişkeni bellekten düşürülür.
*/
fn search_book(title: String) {
    println!("Starting search for '{}'", title);
}

fn move_rule() {
    // Ownership yasasında move ihlali için Vec tabanlı örnek
    let names = vec![
        "John".to_string(),
        "Beatrice".to_string(),
        "Zee".to_string(),
    ];
    let _names_again = names; // names vektör sahipliği names_again'e geçti. names artık yok.
                              // println!("{:?}", names); // Value used after being moved [E0382]
}

fn move_rule_with_clone() {
    /*
        Ownership yasasında move ihlali clone operasyonu ile aşılabilir ancak;
        Aşağıdaki gibi String türden veri içeren büyük bir vektör için clone operasyonunun
        büyük bir maliyeti vardır.
    */
    let names = vec![
        "John".to_string(),
        "Beatrice".to_string(),
        "Zee".to_string(),
    ];
    let names_again = names.clone(); // names vektörünün bir klonu oluşturulur ve bunun sahipliği names_again değişkenine verilir
    println!("{:?}", names); // Bu durumda names halen kullanılabilirdir.
    println!("{:?}", names_again);
}
