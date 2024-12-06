/*
   Aşağıdaki senaryoda iki sayıdan hangisi büyükse o geriye dönüyor.
   Ancak üç sayı kıyaslamak istediğimiz bir durum olursa ne yaparız?
   Bu durumda Vec<i32> parametre alan bir versiyon yazabiliriz ya da
   bunu bir makro ile çözebiliriz.
*/
pub fn max_of_two(a: i32, b: i32) -> i32 {
    if a >= b {
        return a;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_of_two() {
        assert_eq!(max_of_two(10, 20), 20);
        assert_eq!(max_of_two(30, 20), 30);
    }
}
