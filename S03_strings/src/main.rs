fn main() {
    let hero_name = "Can Cloud Van Dam".to_string();
    println!("{}", hero_name);
    let position = String::from("Quarter back");
    println!("{}", position);
    let short_name = hero_name.replace("Can Cloud Van Dam", "CCVD");
    println!("{}", &short_name);

    // &str (string slice) bir string değerin belli bir kısmını refeans edebilir ya da
    // string slice (&str) için heap üzerine yer ayrılmaz(String veri türünün aksine)
    // Boyutu bellidir.
    // Genellikle fonksiyonlara argüman taşırken &str daha uygundur (lifetime'a dikkat)
    let greetings = "Greetings dear young and crazy brains".to_string();
    let first_word = greetings.get(0..10).unwrap();
    println!("{}", first_word);

    let first_words = "hello there"; //doğrudan &str
    println!("{}", first_words);

    let word_aloha = "Aloha!";
    let word = word_aloha.to_string();
    let word_ref = &word;
    println!("{}", word_aloha);
    println!("{}", word);
    println!("{}", word_ref);

    println!("Words is equal, {}", word_aloha.to_lowercase() == "aloha!");

    // Elimizde geçerli UTF-8/UNICODE karakterleri olmadığında strings literals kullanılabilir.
    let konnichiwa = "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}";
    println!("{}", konnichiwa);
}
