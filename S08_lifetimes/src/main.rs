/*
   Her referansın bir yaşam ömrü vardır(lifetime).
   Çoğunlukla implicit olarak var olurlar ve bazı durumlarda açıkça belirtmek gerekir.
   lifetimes, daha çok Dangling References durumunun oluşmasını engellemek içindir.
*/
fn main() {
    /*
       Öncelikle basit bir danling reference durumu oluşturalım.
       Bunun için aşağıdaki kod parçasını göz önüne alalım.

       value değişkenine scope içerisinde yaşayabilen point isimli değişkenin referansı veriliyor.
       Kasıtlı olarak açtığımız blok kapatıldığında ise point değişkeni bellekten drop ediliyor.
       Dolayısıyla value değişkeni artık var olmayan bir değere referans etmeye çalışıyor.
       Rust bu durumu algılar ve kodu derlemez.
       Çalışma zamanı çıktısı aşağıdaki gibi olacaktır.

       error[E0597]: `point` does not live long enough
      --> S08_lifetimes\src/main.rs:13:17
       |
    12 |         let point = 2.25;
       |             ----- binding `point` declared here
    13 |         value = &point;
       |                 ^^^^^^ borrowed value does not live long enough
    14 |     }
       |     - `point` dropped here while still borrowed
    15 |     println!("{}", value);
       |                    ----- borrow later used here

    For more information about this error, try `rustc --explain E0597`.


    */
    // let value;
    // {
    //     let point = 2.25;
    //     value = &point;
    // } // point değişkeni scope sonlandığı için drop edilir
    // println!("{}", value); // value artık olmayan bir değeri referans etmeye çalışıyordur.

    /*
        Bazı durumlarda referansların yaşam ömürlerinin açıkça belirtilmesi gerekir.
        Bunun için lifetime annotation özelliği kullanılır.
        Örneğin referans içeren struct'lar da bu açıkça belirtilmelidir.

        Aşağıdaki örneği ele alalım.

        Account veri yapısında customer_name isimli bir string literal kullanımı söz konusudur.
        Bu bir referanstır.
        Aşağıdaki kod parçasında da name isimli String bir değişken oluşturulur ve
        van_dam_account nesnesi örneklenirken customer_name alanına name referansı eklenir.
        Eğer, herhangi bir sebeple name değişkeni van_dam_account değişkeninden daha az yaşarsa,
        (yani daha önceden bir şekilde drop olursa), van_dam_account isimli değişkendeki customer_name,
        olmayan bir referansı taşımaya çalışacaktır. Bu da Dangling Pointer durumu olarak bilinir.
        Bu nedenle Account struct tanımında &str şeklinde bir kullanıma derleyici izin vermez.
        Lifetime annotation kullanılmazsa şöyle bir derleme zamanı hatası oluşur.

        error[E0106]: missing lifetime specifier
          --> S08_lifetimes\src/main.rs:66:20
           |
        66 |     customer_name: &str,
           |                    ^ expected named lifetime parameter
           |
        help: consider introducing a named lifetime parameter
           |
        65 ~ struct Account<'a> {
        66 ~     customer_name: &'a str,
           |
    */

    let name = String::from("Jan Klot Van Dam");
    /*
       Bulunduğumuz scope'da name değişkeninin implicit lifetime değeri,
       Account nesne örneğine de aktarılır ve customer_name'in aynı süre yaşaması garanti edilir.
    */
    let van_dam_account = Account {
        customer_name: &name,
        balance: 1000f32,
    };
    println!(
        "{} has a {} coin",
        van_dam_account.customer_name, van_dam_account.balance
    );

    /*
        Bütün string literal referansları static lifetime bilgisine sahiptir ancak açıkça yazılmazlar
        Tüm program döngüsü boyunca çalışacak lifetime bildirimleri için 'static kullanılabilir ancak önerilmez
        Mümkün mertebe nesnelerin ömürlerini gerektiği kadar sürelerde tutmak optimizasyon açısından daha doğrudur.
    */
    let settings = get_app_settings();
    println!("Server address is {}", settings.server_address);
    println!("Connection string is {}", settings.connection_string);
}

struct Account<'a> {
    customer_name: &'a str, // customer_name için lifetime annotation bildirimi
    balance: f32,
}

/*
   Aşağıdaki örnekte yer alan ApplicationSettings isimli struct
   lifetime annotation kullanıyor.
   Bunu tip bir yapıyı programın çalışma zamanı boyunca tutacağı bazı ayarlar
   için kullanabiliriz. Sunucu adresi, veritabanı bilgisi gibi.

   get_app_settings fonksiyonun ApplicationSettings örneğini dönerken dikkat edileceği üzere
   <'static> bildirimini ele almıştır.
*/
struct ApplicationSettings<'a> {
    server_address: &'a str,
    connection_string: &'a str,
}

fn get_app_settings() -> ApplicationSettings<'static> {
    ApplicationSettings {
        server_address: "localhost",
        connection_string: "data source=Northwind;server:127.0.0.1;",
    }
}
