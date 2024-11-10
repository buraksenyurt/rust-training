use std::thread;

#[derive(Debug)]
struct Player {
    id: u32,
    position: (f32, f32),
    velocity: (f32, f32),
    score: u32,
}

#[derive(Debug)]
struct GameWorld {
    players: Vec<Player>,
}

/*
   update fonksiyonunda kullanılan f değişkeni generic bir tür olarak belirtilmiştir ve
   Fn trait'ini uygulaması beklenmektedir. Kısaca f yerine Fn trait'ine uygun bir closure
   ifadesi gelebilir. Örneğin apply_gravity fonksiyonu gibi ya da anonim şekilde belirtilerek.
*/

fn update_players_system<F>(world: &mut GameWorld, mut f: F)
where
    F: Fn(&mut Player),
{
    for p in &mut world.players {
        f(p);
    }
}

fn update_score_system<F>(world: &GameWorld, mut f: F)
where
    F: FnMut(&Player),
    /*
       Burada FnMut yerine Fn kullanıp oluşan hata mesajını inceleyebiliriz.

       error[E0594]: cannot assign to `total_team_score`, as it is a captured variable in a `Fn` closure
       change this to accept `FnMut` instead of `Fn`
    */
{
    for p in &world.players {
        f(p);
    }
}

pub fn run() {
    let mut world = GameWorld {
        players: vec![
            Player {
                id: 1,
                position: (0.0, 0.0),
                velocity: (2.0, 0.0),
                score: 0,
            },
            Player {
                id: 2,
                position: (100.0, 0.0),
                velocity: (8.0, 0.0),
                score: 0,
            },
        ],
    };

    let apply_gravity = |entity: &mut Player| {
        entity.position.0 += entity.velocity.0 * 0.9;
        entity.position.1 += entity.velocity.1 * 0.9;
    };

    println!("Before Update: {:?}", world.players);
    update_players_system(&mut world, apply_gravity);
    // update_players_system(&mut world, |entity| {
    //     entity.position.0 += entity.velocity.0 * 0.9;
    //     entity.position.1 += entity.velocity.1 * 0.9;
    // });
    println!("After Update: {:?}", world.players);

    // FnMut kullanımı ile ilgili bir örnek
    let mut total_team_score = 0;

    println!("Total score before update: {}", total_team_score);
    update_players_system(&mut world, |p| p.score += 2);
    update_score_system(&world, |p: &Player| {
        total_team_score += p.score;
    });
    println!("Total score after update: {}", total_team_score);

    // FnOnce Örneği
    let message = Some(String::from("You have unlocked a new level!"));
    let show_message = || {
        if let Some(msg) = message {
            println!("{}", msg);
        } else {
            println!("Message already shown.");
        }
    };

    show_message();
    // show_message(); // Burada 'value used here after move' hatası oluşur
    /*
       Henüz erken olsa da thread açmak FnOnce kullanımı için iyi bir örnek olabilir.
       thread::spawn yeni bir thread başlatırken FnOnce türünden bir closure alır. Dışarıdan
       değerler closure içerisine taşınır ve thread sonlanana kadar closure sahip olduğu tüm
       değerleri tüketir. Bu tek sefer çalıştırılması gereken bir closure olarak düşünülebilir.
    */
    let message = String::from("Inside a thread!");
    let handle = thread::spawn(move || {
        println!("{}", message);
    });
    // println!("{}", message); // value borrowed here after move
    handle.join().unwrap();
}
