use crate::repository::Game;

mod repository;
mod scenario;
/*
   Closure' lar anonim fonksiyonlar olarak düşünülebilirler. Özellikle fonksiyonel dillerin
   önemli enstrümanlarındandırlar. Bir closure bir değişkene atanıp kullanılabilir dolayısıyla
   fonksiyonlara parametre olarak kod bloğu içeren ifadeler göndermenin yolunu da açar.

   Rust standart kütüphanesinde Fn, FnMut, FnOnce gibi trait'ler ile Closure' ların çalışma
   davranışları belirlenir.

   Fn: Closure değişkenlere salt okunur olarak erişir.
   FnMut: Closure değişkenleri değiştirebilir.
   FnOnce: Closure değişkenleri sahiplenir ve sadece bir defa çalıştırılabilir.
*/

// Klasik bir yöntemler oyunların yıllarına göre sıralanması işlevini ele alalım
fn year_sorter(game: &Game) -> u16 {
    game.year
}

fn main() {
    let mut games = repository::load_games();
    // println!("Games List {:#?}", games);
    // /*
    //    sort_by_key parametre olarak FnMut traitini uygulayan bir fonksiyon kullanır.
    //    Örneğin yıla göre sıralama yapmak için bu bilgiyi geriye döndüren year_sorter fonksiyonu
    //    sort_by_key'e parametre olarak geçebiliriz. Sort By Key bunu ascending sırada yapar.
    // */
    // games.sort_by_key(year_sorter);
    // println!("Games List(Sorted by Year) {:#?}", games);
    // // Ancak closure'lar aşağıdaki örneklerde olduğu gibi daha efektif kullanılabilirler.

    // games.sort_by(|g1, g2| g1.year.cmp(&g2.year));
    // println!("Games by year ascending order");
    // print_games(&games);
    //
    // games.sort_by(|g1, g2| g2.year.cmp(&g1.year));
    // println!("\nGames by year descending order");
    // print_games(&games);

    let popular_games: Vec<Game> = games.into_iter().filter(|g| g.popularity > 2.0).collect();
    print_games(&popular_games);

    scenario::run();
}

fn print_games(games: &Vec<Game>) {
    for game in games {
        println!("{}, {}", game.year, game.title);
    }
}
