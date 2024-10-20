/*
   Trait'ler Rust'ın en önemli enstrümanlarından birisidir.
   Özellikle generic olma kabiliyeti ile birlikte ele alınır.
   Bu örneğe kadar çok detaya girmeden gazı trait'leri kullandık.
   Debug, Copy, Clone, Add vb

   Bir trait aslında bir davranış tanımlar. Herhangibir veri modeli bu davranışı
   kendisine alabilir. Böylece sistem içerisinde aynı davranışlara sahip fonksiyonellikleri
   bulmak ve organize etmek kolaylaşır.

   C# tarafında interface tipinin kullanım amacı ile benzetilebilir.
*/
fn main() {
    let mut redis_instance = Redis;
    let mut doctor = HealthCheck { is_online: false };

    start_sample(&mut redis_instance);
    start_sample(&mut doctor);
    start_sample2(&mut redis_instance);

    // default trait kullanımı için örnek
    let debit_payment = DebitCard;
    let company_payment = CompanyAccount;
    let card_payment = CreditCard;

    debit_payment.pay(100.0);
    card_payment.pay(100.0);
    company_payment.pay(100.0);

    // Trait Object kullanımı
    let red_ball = Circle;
    let blue_ball = Circle;
    let wall = Square;
    let warrior = Player;
    let level_1: Vec<&dyn Draw> = vec![&red_ball, &blue_ball, &wall, &warrior];
    draw_shapes(&level_1);
}

/*
   Aşağıdaki fonksiyon parametre olarak Service trait' ini
   uygulayan değişkenler ile çalışabilmektedir.
*/
fn start_sample(service: &mut impl Service) {
    service.activate();
    println!("{:?}", service.status());
    service.deactivate();
    println!("{:?}", service.status());
}
// Bazı durumlarda Trait bildirimi generic tanımlama üzerinden de yapılabilir
fn start_sample2<T: Service>(service: &mut T) {
    service.activate();
    println!("{:?}", service.status());
    service.deactivate();
    println!("{:?}", service.status());
}

/*
   # Bir trait nasıl tanımlanır, nasıl uygulanır?

   Service isimli trait activate ve deactivate isimli iki fonksiyon tanımlar.
   Devam eden kob bloğunda yer alan Redis ve HealthCheck isimli veri yapıları
   bu davranışları kendilerine göre özelleştirir.
*/
trait Service {
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn status(&self) {
        println!("Service status");
    }
}

struct Redis;
struct HealthCheck {
    is_online: bool,
}
/*
    Service davranışları Redis veri modeli için aşağıdaki gibi uygulanabilir.
    Bir Trait bir veri yapısı ile ilişkilendirildiğinde tanımlı tüm trait'ler uygulanmalıdır.
    Sadece Default trait için farklı bir durum söz konusudur.
    Örneğin status isimli fonksiyon Service trait'i içerisinde bir kod fonksiyonuna sahiptir.
    Yani Service trait'inin uygulandığı veri yapısında ezilmek zorunda değildir.
*/
impl Service for Redis {
    fn activate(&mut self) {
        println!("Activating Redis");
    }
    fn deactivate(&mut self) {
        println!("Deactivating Redis");
    }
}

impl Service for HealthCheck {
    fn activate(&mut self) {
        self.is_online = true;
        println!("Activating HealthCheck");
    }
    fn deactivate(&mut self) {
        println!("Deactivating HealthCheck");
    }
    fn status(&self) {
        println!("Service status {}", self.is_online);
    }
}

/*
    varsayılan trait kullanımı için bir başka örneği de şöyle yazabiliriz.
    Bir ödeme davranışımız var. Varsayılan olarak tutar ne ise onu kullanır.
    Ancak örneğin kredi kartı ödemesinde ek komisyon söz konusudur.
    Diğer ödeme biçimlerinden farklı bir davranışın uygulanması gerekir.
*/
trait Payment {
    fn pay(&self, amount: f64) {
        println!("Paid amount {:.2} with cash.", amount);
    }
}
struct DebitCard;
impl Payment for DebitCard {}
struct CompanyAccount;
impl Payment for CompanyAccount {}
struct CreditCard;
impl Payment for CreditCard {
    fn pay(&self, amount: f64) {
        let amount_with_commision = amount * 1.1;
        println!("Paid {:.2} with credit card.", amount_with_commision);
    }
}

/*
   Aşağıdaki örnekte ise Trait Object kullanımı söz konusudur.
   Bazı durumlarda belli bir Trait'in uygulayıcılarını nesne olarak fonksiyonlara almak isteyebiliriz.
   Burada sıkıntı Trait implemente eden tipin bellekte ne kadar yer kaplayacağını kestirememektir.
   Kullanım formasyonlarından birisi dinamik nesne tanımlamasıdır.
   dyn Trait
*/
// Standart bir grafik kütüphanesinde veya oyun motorunda draw davranışı olduğunu varsayalım
trait Draw {
    fn draw(&self);
}

struct Circle;
struct Square;
struct Player;

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

impl Draw for Player {
    fn draw(&self) {
        println!("Drawing a player");
    }
}

/*
   draw_shape fonksiyonu dikkat edileceği üzre &dyn Draw türünden bir parametre almaktadır.
   Buna göre bu fonksiyona Draw trait'ini implemente eden herhangibir nesneyi gönderebilir
   ve bu değişken üzerinen draw metodunu işlettirebiliriz.
*/
fn draw_shapes(shapes: &Vec<&dyn Draw>) {
    for shape in shapes.iter() {
        shape.draw();
    }
}
