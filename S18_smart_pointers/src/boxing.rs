pub fn simple_boxing() {
    // Örnek bir Boxing işlemi
    let value = 23; // Normalde stack' de saklanır
    let boxed_value = Box::new(value); // Şimdi heap'e alındı ama boxed_value hala stack'te zira adres göstermekte
    println!("Boxed value is {}", boxed_value);

    let identity = ("John Smith", 23, true); // tuple veriyi stack'ta saklar
    let boxed_identity = Box::new(identity); // Şimdi heap' te
    println!("Boxed identity is {:?}", boxed_identity);
}

/*
   Aşağıdaki kullanım build aşamasında
   error[E0072]: recursive type `Tree` has infinite size
   hatasını verir. Bu tip recursive veri modellerinde datanın ne kadar yer kaplayacağı bilinemez.
   Zira Rust derleme zamanında nesnelerin ne kadar yer kaplayacağını bilmek ister.
   Bir düğüm kendisine referans verdikçe bu sonsuz bir boyutlamaya gider.
   Bu nedenle ilgili senaryoda Box kullanılarak esasında Heap bazlı bir kullanıma gidilir.
*/
// enum Tree {
//     Node(i32, Tree, Tree),
//     Empty,
// }

//pub  fn recursive_data_model_with_error() {
//     let left_child = Tree::Node(1, Tree::Empty, Tree::Empty);
//     let right_child = Tree::Node(3, Tree::Empty, Tree::Empty);
//     let root = Tree::Node(2, left_child, right_child);
// }

use std::fmt::{Display, Formatter};

enum Server {
    Node(String, Box<Server>, Box<Server>),
    Empty,
}
impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Server::Node(name, primary, backup) => {
                write!(
                    f,
                    "Server: {}\n  Primary: {}\n  Backup: {}",
                    name, primary, backup
                )
            }
            Server::Empty => write!(f, "None"),
        }
    }
}

pub fn recursive_sample() {
    let backup_server_blue = Server::Node(
        String::from("Backup Server Blue"),
        Box::new(Server::Empty),
        Box::new(Server::Empty),
    );

    let primary_server_green = Server::Node(
        String::from("Primary Server Green"),
        Box::new(Server::Empty),
        Box::new(backup_server_blue),
    );

    let root_server = Server::Node(
        String::from("Root Server"),
        Box::new(primary_server_green),
        Box::new(Server::Node(
            String::from("Backup Root"),
            Box::new(Server::Empty),
            Box::new(Server::Empty),
        )),
    );

    println!("{}", root_server);
}
