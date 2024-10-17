/*
   Rust dilinde 3 tür struct vardır.
   Named Field base. Player struct'ı bu türdendir
   Tuple based. Position struct'ı bu türdendir.
   Unit based.

*/
fn main() {
    let marlon = Player {
        name: String::from("Marlon Singer"),
        is_active: false,
        level: 100,
        position: Position(1.0, 0.0),
    };
    println!("{} level is {}", marlon.name, marlon.level);

    let durden = spawn_random_user(String::from("Tyler Durden"));
    println!(
        "{}({}) on ({}:{})",
        durden.name, durden.is_active, durden.position.0, durden.position.1
    );

    let position = Position(9.0, 12.0);
    println!(
        "Some position at the galaxy ({}:{})",
        position.0, position.1
    );

    let mut rectangle = Rectangle::new(10.0, 20.0);
    println!("Area is {}", rectangle.area());
    println!("Perimeter is {}", rectangle.perimeter());
    rectangle.resize(10.0, 10.0);
    println!("Area is {}", rectangle.area());
    println!("Perimeter is {}", rectangle.perimeter());
}

fn spawn_random_user(name: String) -> Player {
    Player {
        name,
        is_active: true,
        level: 100,
        position: Position(10.0, 20.40),
    }
}

// Field base struct
struct Player {
    name: String,
    is_active: bool,
    level: u8,
    position: Position,
}

struct Position(f32, f32); // Tuple based struct

struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Self {
        Rectangle { width, height }
    }
    fn area(&self) -> f32 {
        self.width * self.height
    }
    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
    fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

struct Entity; // Unit based struct (ECS tarafında sık gördüğümüz bir kavram)
