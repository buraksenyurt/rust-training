/*
   Heap üzerindeki alokasyonlar Multiple Ownership gerektiği durumlarda Rc kullanılabilir.
   RC referansları sayan bir kabiliyete sahiptir.
   Birde Arc<T> vardır yani Atomic Referans Counting. Bunu daha çok paylaşımın thread'ler arasında
   güvenli şekilde sağlanmasını istediğimiz durumlarda ele alırız.
*/
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub fn hello_rc() {
    /*
       p1, p2 ve p3 aynı veriyi işaret eden ve stack' de duran pointer'lardır.
    */
    let p1 = Rc::new(String::from("Some values on the heap"));
    let p2 = p1.clone();
    let p3 = p2.clone();

    println!("p1={:?}", p1);
    println!("p2={:?}", p2);
    println!("p3={:?}", p3);
}

/*
   İlk olarak aşağıdaki senaryoyu göz önüne alalım.
   Burada Player veri modeli yine kendi türünden bir vektör kullanarak arkadaş listesini tutuyor.
*/
// #[derive(Debug)]
// struct Player {
//     id: u32,
//     name: String,
//     friends: Vec<Player>,
// }
//
// impl Player {
//     fn new(id: u32, name: &str) -> Self {
//         Player {
//             id,
//             name: name.to_string(),
//             friends: Vec::new(),
//         }
//     }
//
//     fn add_friend(&mut self, friend: Player) {
//         self.friends.push(friend);
//     }
//
//     fn print(&self) {
//         println!("{}'s friends:", self.name);
//         for friend in &self.friends {
//             println!("  {} (ID: {})", friend.name, friend.id);
//         }
//     }
// }
//
// pub fn run_rc_with_error() {
//     let mut steve = Player::new(1, "Stivi Vondır");
//     let lord = Player::new(2, "Lord veyda");
//     let anakin = Player::new(3, "Anakin");
//
//     steve.add_friend(lord); // lord' un sahipliği add_friend sonrası steve' e taşındı
//     steve.add_friend(anakin);
//
//     steve.print();
//
//     println!("Lord veyda's ID: {}", lord.id); // Dolayısıyla burada value moved here hatası alınır
// }

/*
   İkinci olarak Rc kullandığımız senaryoya bakalım ama bu seferde kendimiz
   error[E0596]: cannot borrow data in an `Rc` as mutable
   hatasına götüreceğiz.
*/
// #[derive(Debug)]
// struct Player {
//     id: u32,
//     name: String,
//     friends: Vec<Rc<Player>>,
// }
//
// impl Player {
//     fn new(id: u32, name: &str) -> Rc<Self> {
//         Rc::new(Player {
//             id,
//             name: name.to_string(),
//             friends: Vec::new(),
//         })
//     }
//
//     fn add_friend(self: &Rc<Self>, friend: Rc<Player>) {
//         self.friends.push(friend);
//     }
//
//     fn print(&self) {
//         println!("{}'s friends:", self.name);
//         for friend in self.friends.iter() {
//             println!("  {} (ID: {})", friend.name, friend.id);
//         }
//     }
// }
//
// pub fn run_rc_with_error_2() {
//     let steve = Player::new(1, "Stivi Vondır");
//     let lord = Player::new(2, "Lord veyda");
//     let anakin = Player::new(3, "Anakin");
//
//     steve.add_friend(Rc::clone(&lord));
//     steve.add_friend(Rc::clone(&anakin));
//
//     steve.print();
//
//     println!("Lord Veyda's ID: {}", lord.id);
// }

/*
   Yukarıdaki senaryoda bir Player'ın kendisini tutan bir Vector nesnesine ihtiyacı olmuştur.
   Bunun için Vec'ün Rc<Player> kullanılması yolu tercih edilmiştir. Ancak bu
   özellikle add_friends metodunda vektör içeriğine mutable erişmeyi gerektirir. Bu nedenle
   vektöre referansının da mutable olarak ele alınabilmesi gerekir. Normalde birden fazla sahip
   varken mutable olma hali derleme hatasına yol açabilir. RefCell kullanımı ile bunu runtime'a
   taşırız. Yani ownership kontrolünü runtime tarafında işleriz.

   Bu kullanımda borrow_mut ve borrow metot çağrımları söz konusudur. Bu çağrımlar esasında
   RefCell türevli Ref<T> ve RefMut<T> trait'lerini kullanır.
*/

#[derive(Debug)]
struct Player {
    id: u32,
    name: String,
    friends: RefCell<Vec<Rc<Player>>>,
}

impl Player {
    fn new(id: u32, name: &str) -> Rc<Self> {
        Rc::new(Player {
            id,
            name: name.to_string(),
            friends: RefCell::new(Vec::new()),
        })
    }

    fn add_friend(self: &Rc<Self>, friend: Rc<Player>) {
        self.friends.borrow_mut().push(friend);
    }

    fn print(&self) {
        println!("{}'s friends:", self.name);
        for friend in self.friends.borrow().iter() {
            println!("  {} (ID: {})", friend.name, friend.id);
        }
    }
}

pub fn run_rc() {
    let steve = Player::new(1, "Stivi Vondır");
    let lord = Player::new(2, "Lord veyda");
    let anakin = Player::new(3, "Anakin");

    steve.add_friend(Rc::clone(&lord));
    steve.add_friend(Rc::clone(&anakin));

    steve.print();

    println!("Lord Veyda's ID: {}", lord.id);
}
