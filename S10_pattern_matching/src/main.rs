/*
   Pattern Matching kabiliyeti rust tarafında da sıklıkla kullanılır.
   Option , Result gibi built-in gelen veya kendi tasarladığım Enum yapılarında karar yapılarını
   inşa etmek için ideal ve temiz bir kod pratiği sunar.
*/
use rand::Rng;

fn main() {
    check_status();

    check_exam(Student {
        id: 1,
        full_name: String::from("Burak De La Fuante Dos Selimos"),
        grade: 44,
    });

    let mut balance = 1000.0;
    process_transaction(&mut balance, CustomerTransaction::Deposit(400.0));
    process_transaction(&mut balance, CustomerTransaction::Withdraw(50.0));

    let klyde = User::new(19, String::from("Jhony Klyde"), None);
    println!("{}", klyde.info()); // İlerde info yerinde Display Trait implementasyonu kullanılır
    let zee = User::new(
        23,
        String::from("Zee"),
        Some(String::from("zee@somewhere.abc")),
    );
    println!("{}", zee.info());

    let accounts = load_accounts();
    let result = find_account(&accounts, 1003);
    match result {
        Some(account) => println!(
            "Account for '{}' found: {} with balance ${}",
            account.id, account.holder_name, account.balance
        ),
        None => println!("Account not found."),
    }
    // Bazen yukarıdaki gibi bir match ifadesi yerine 'if let' söz dizimi de kullanılabilir
    if let Some(account) = find_account(&accounts, 1002) {
        println!(
            "Account for '{}' found: {} with balance ${}",
            account.id, account.holder_name, account.balance
        );
    }
}
fn check_status() {
    /*
       ping, parametre olarak gelen servis adresini için bir health check yapıyor diye düşünelim.
       dönen HttpStatus enum değerine göre ekrana bilgilendirme yapılmakta
    */
    let call_response = ping("http://localhost:3456/ping");
    match call_response {
        HttpStatus::Ok => {
            println!("Http Status is OK(200)");
        }
        HttpStatus::Accepted => {
            println!("Http Status is ACCEPTED(201)");
        }
        HttpStatus::NotFound => {
            println!("Http Status is NOT FOUND(404)");
        }
        HttpStatus::BadRequest => {
            println!("Http Status is BAD REQUEST(400)");
        }
        HttpStatus::InternalServerError => {
            println!("Http Status INTERNAL ERROR(500)");
        }
    }
}
// Sembolik olarak bir servise gidipte rastgele HttpStatus bilgisi döndüren fonksiyon
fn ping(service_address: &str) -> HttpStatus {
    let mut generator = rand::thread_rng();
    let lucy_number = generator.gen_range(0..=10);
    println!("Pinging the {service_address}");
    // luck_number değerine göre bir Status değeri üretilir
    match lucy_number {
        1 => HttpStatus::Ok,                  // 1 ise
        2..=4 => HttpStatus::Accepted,        // 2, 3 ve 4 dahil ise
        5 => HttpStatus::BadRequest,          // 5 ise
        8 | 10 => HttpStatus::NotFound,       // 8 veya 10 ise
        _ => HttpStatus::InternalServerError, // 1,2,3,4 dışındaki tüm durumlar için
    }
}
enum HttpStatus {
    Ok,
    Accepted,
    NotFound,
    BadRequest,
    InternalServerError,
}

struct Student {
    id: u32,
    full_name: String,
    grade: u8,
}

fn check_exam(student: Student) {
    // pattner mathcing'de aşağıdaki gibi değer aralıkları da kullanılabilir
    match student.grade {
        0..=49 => println!("[{}]{} failed.", student.id, student.full_name),
        50..=79 => println!("[{}]{} passed.", student.id, student.full_name),
        80..=100 => println!(
            "[{}]{} passed with congrats.",
            student.id, student.full_name
        ),
        _ => println!("Invalid grade score"),
    }
}

enum CustomerTransaction {
    Deposit(f64),
    Withdraw(f64),
}

fn process_transaction(balance: &mut f64, transaction: CustomerTransaction) {
    match transaction {
        CustomerTransaction::Deposit(amount) => {
            *balance += amount;
            println!("Deposited ${}\nNew balance: ${}.", amount, balance);
        }
        CustomerTransaction::Withdraw(amount) => {
            if *balance >= amount {
                *balance -= amount;
                println!("Withdraw ${}\nNew balance: ${}.", amount, balance);
            } else {
                println!("Insufficient funds.");
            }
        }
    }
}

struct User {
    id: u32,
    title: String,
    email: Option<String>, // Some(String) veya None değerlerinden birisini alabilir
                           // Yani User için bir email adresi atama zorunluluğu olmadığı varsayılmaktadır
}

impl User {
    fn new(id: u32, title: String, email: Option<String>) -> Self {
        User { id, title, email }
    }

    fn info(&self) -> String {
        /*
           User bilgisini string olarak dönerken email bilgisine sahip olup olmamasına
           göre farklı bir içerik oluşturmak için pattern matching'den yararlanabiliriz
        */
        match self.email {
            Some(ref em) => format!("{}-{} ({})", self.id, self.title, em),
            None => format!("{} ({})", self.id, self.title),
        }
    }
}

struct Account {
    id: u32,
    holder_name: String,
    balance: f64,
}
/*
   Referans olarak gelen vektör içerisinde id bilgisine sahip Account değişkenini arayan
   bu fonksiyon Option<&T> dönmektedir.
   Metodun kullanıldığı yerde pattern matching'den yararlanılır.
*/
fn find_account(accounts: &Vec<Account>, id: u32) -> Option<&Account> {
    accounts.iter().find(|acc| acc.id == id)
}

fn load_accounts() -> Vec<Account> {
    vec![
        Account {
            id: 1001,
            holder_name: "Nora Min".to_string(),
            balance: 1000.0,
        },
        Account {
            id: 1002,
            holder_name: "Agnis Yang".to_string(),
            balance: 750.0,
        },
        Account {
            id: 1003,
            holder_name: "Valeri Mora".to_string(),
            balance: 850.0,
        },
        Account {
            id: 1004,
            holder_name: "Monti Konti".to_string(),
            balance: 275.0,
        },
    ]
}
