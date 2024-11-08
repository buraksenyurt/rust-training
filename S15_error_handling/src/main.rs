/*
   Rust dilinde hatalar recoverable ve unrecoverable olmak üzere
   iki ana kategoriye ayrılır.

   Recoverable Error için Result türü ele alınırken,
   Unrecoverable Error durumlarında panic! makrosu ön plana çıkar.

*/
use rand::Rng;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    /*
       Unrecoverable Errors

       panic! makrosu çalışmakta olan thread'i bir hata mesajı ile sonlandırır

       Örneğin out of bounds durumunu ele alalım.
       Runtime'da aşağıdaki kod parçası için 'index out of bounds' hatası üretilir.
       Burada istersek Backtrace içeriğine de bakabiliriz. Bunun için terminalden,
       $env:RUST_BACKTRACE = 1
       şeklinde komut çalıştırmak yeterlidir.

       unwinding vs abort ???s
    */
    let numbers = vec![1, 4, 7, 0, 9];
    // let some_number= numbers[99];
    /*
       Recoverable Errors,

       Burada hatalar Result<T,E> türü ile kolaylıkla ele alınabilir.
    */
    // let f = File::open("there_is_no_spoon.dat");
    // let file = match f {
    //     Ok(file) => file,
    //     Err(e) => panic!("{:?}", e),
    // };

    // let file = f.unwrap_or_else(|e| match e.kind() {
    //     std::io::ErrorKind::NotFound => match File::create("there_is_no_spoon.dat") {
    //         Ok(file_created) => file_created,
    //         Err(e) => panic!("{:?}", e),
    //     },
    //     _ => panic!("{:?}", e),
    // });

    /*
       unwrap metodu eğer işlem sonucunda problem varsa bir panik oluşturur
       expect ile bu hata hakkında ekstra bilgi verebiliriz.
    */
    // let file = File::open("there_is_no_spoon.dat").unwrap();
    // let file =
    //     File::open("there_is_no_spoon.dat").expect("Are you sure? There is no file I think.");

    // Error propagation

    match approve_application("10001") {
        Ok(_) => println!("Application approved."),
        Err(e) => println!("Error occurred: {}", e),
    }
}

/*
   Error Propagation

   Oluşan bir hatanın üst katmanlara(fonksiyonlara) taşınması olarak düşünülebilir.

   Kendi hata nesnelerimizi kullanabiliriz. Örneğin kredi başruvu sürecinde kredi
   skoru hesaplaması yapılırken oluşabilecek bazı durumların birer Error olarak
   ele alınmasını sağlayabiliriz.
*/

#[derive(Debug)]
enum CreditScoreError {
    NotFound(String),
    ServiceUnavailable,
    LowScore(i32),
}

impl Display for CreditScoreError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            CreditScoreError::NotFound(ref account_id) => {
                write!(f, "Credit score not found. Account Id {}", account_id)
            }
            CreditScoreError::ServiceUnavailable => {
                write!(f, "Credit service is currently unavailable")
            }
            CreditScoreError::LowScore(score) => write!(f, "Low credit score. Score is {}", score),
        }
    }
}

fn fetch_credit_score(account_owner: &str) -> Result<i32, CreditScoreError> {
    let mut generator = rand::thread_rng();
    let simulation_number = generator.gen_range(0..=3);
    let score = generator.gen_range(300..=700);
    match simulation_number {
        0 => Ok(score),
        1 => Err(CreditScoreError::NotFound(String::from(account_owner))),
        _ => Err(CreditScoreError::ServiceUnavailable),
    }
}

fn approve_application(account_owner: &str) -> Result<(), CreditScoreError> {
    let score = fetch_credit_score(account_owner)?;
    if score < ELIGIBLE_CREDIT_SCORE {
        Err(CreditScoreError::LowScore(score))
    } else {
        Ok(())
    }
}

const ELIGIBLE_CREDIT_SCORE: i32 = 600;
