use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub fn hello_channels() {
    let (transmitter, reciever) = channel();
    let message = String::from("Sample content");
    // let number = 3.14;
    thread::spawn(move || {
        // message değişkeninin sahipliği send metoduna geçer ve sonrasında ilgili veri drop olur
        transmitter.send(message).unwrap();
        // transmitter.send(number).unwrap();
    });

    let data = reciever.recv().unwrap();
    /*
         Örneğimizde String türünü kullandığımızdan message değerini
         aşağıdaki gibi kullanmak istersek ownership ihlali oluşmasına sebep oluruz.
         Bu compile-time ile tespit edilir.
         error[E0382]: borrow of moved value: `message`

         10 |         transmitter.send(message).unwrap();
    |                          ------- variable moved due to use in closure

        Ancak örneğin Copy trait implementasyonu yapmış bir değer kullansak bu hataya takılmayız.
        Shallow ve Deep Copy davranışlarını hatırlayalım.
      */
    // println!("{}", message);
    // println!("{}", number);
    println!("{}", data);
}

/*
   Aşağıdaki örnekte birden fazla producer kullanımı denenmektedir.
   Bu kodun ilginç yanı ikinci for döngüsünde transmitter'lardan gelen tüm mesajlar yakalansa
   dahi döngünün devam ediyor olmasıdır. Zira transmitter_clone'ları halen daha yaşamaktadır
   ve tamamı drop edilene kadar for döngüsü devam eder.

   Buradaki basit çözüm yöntemlerinden birisi ana göndericiyi, yani diğer klonların referans
   aldığı göndericiyi drop etmektir.
*/
pub fn multi_producer() {
    let (transmitter, receiver) = channel();

    for i in 0..10 {
        let transmitter_clone = transmitter.clone();
        thread::spawn(move || {
            transmitter_clone
                .send(format!("Sending message is {}", i))
                .unwrap();
            thread::sleep(Duration::from_secs(2));
        });
    }

    // İlk denemede aşağıdaki gibi transmitter'ı drop etmeden deneyelim.
    // drop(transmitter);

    for received in receiver {
        println!("Incoming message is '{}'", received);
    }

    println!("End of program");
}

/*
   Aşağıdaki örnek ile yukardaki arasındaki en önemli fark transmitter nesnelerinin
   yaşam ömürler. for döngüsü olan örnekte transmitter nesnesinin clone referansları oluşuyor.
   main fonksiyonunda son satıra gelindiğinde bunlardan hiçbiri drop olmamışsa
   receiver dinlemede kalıyor.

   Ancak aşağıdaki örnekte ilk thread ana transmitter nesnesi baz alarak çalışıyor ve ikinci
   thread'de onun clone referansını kullanıyor. Dolayısıyla ilk transmitter nesnesi
   ilk closure sonunda drop edileceğinden kalan klonuda işlemini bitirdiyse receiver sonlanabiliyor.
*/

pub fn multi_producer_2() {
    let (transmitter, receiver) = channel();
    let transmitter_clone = transmitter.clone();
    thread::spawn(move || {
        transmitter.send(String::from("Hello there!")).unwrap();
        thread::sleep(Duration::from_secs(2));
    });

    thread::spawn(move || {
        transmitter_clone
            .send(String::from("Why are you so serious!"))
            .unwrap();
    });

    for received in receiver {
        println!("Incoming message: {}", received);
    }

    println!("End of program");
}
