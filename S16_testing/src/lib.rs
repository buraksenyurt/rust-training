/*
   Test modüllerini çalıştırmak için

   cargo test

   komutunu kullanabiliriz. Eğer belli bir testi çalıştırmak istersek aynı komutu aşağıdaki gibi
   kullanabiliriz.

   cargo test two_plus_two_is_four

   Örneğin sadece membership modülündeki membership_tests testlerini koşmak için aşağıdaki
   komutu kullanabiliriz.

   cargo test membership_tests

   Testleri paralel çalıştırmak da isteyebiliriz. Bazı durumlarda test metotları uzun süreli
   iş parçacıkları içerebilirler. Bunun için thread sayısı belirtebiliriz. Aşağıdaki komutu
   kullanarak testleri 4 thread'e bölüp koşturmaktayız.

   cargo test -- --test-threads=4 # =4 bitişik yazılmalı yoksa komut anlaşılmaz

    Diğer kullanım komutları ile ilgili bilgi almak için;

    cargo test --help
*/
mod accounting;
mod membership;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_plus_two_is_four() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn just_fails() {
        panic!("Failing...");
    }
}
