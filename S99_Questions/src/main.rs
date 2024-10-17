fn main() {
    // Soru 1
    let mut numbers = vec![1, 5, 6, 9, 3, 8];
    let result = is_exist(&numbers, 6);
    numbers.push(23); // Eğer is_exist metoduna numbers vektörü referans olarak geçilmezse
                      // Value used after being moved [E0382]
    println!("{:#?}", result);
    // Soru 1

    // Soru 2
    let mut some_number = 23i8;
    println!("V. The number is now {}", some_number);
    add_someone(some_number, 1i8);
    println!("V. And after add the number is {}", some_number);

    let mut some_number = 23i8;
    println!("R. The number is now {}", some_number);
    add_someone_by_ref(&mut some_number, 1i8);
    println!("R. And after add the number is {}", some_number);
    // Soru 2
}

// Soru 2 metodu
fn add_someone(mut number: i8, acceleration: i8) {
    number += acceleration;
}

fn add_someone_by_ref(number: &mut i8, acceleration: i8) {
    *number += acceleration;
}

// Soru 1 metodu
fn is_exist(numbers: &Vec<i8>, value: i8) -> bool {
    for n in numbers {
        if *n == value {
            return true;
        }
    }
    false
}

/*
   Soru 1:
   Birkaç tam sayıdan oluşan bir vektörü parametre olarak alan bir fonksiyon
   yazılması isteniyor. Bu fonksiyon kendisine gelen vektör içerisine aranan herhangibir
   sayının olup olmadığına göre bool dönüşü yapıyor.
   Bu durum yakalandıktan sonra aynı vektöre herhangibir sayının eklenmesi isteniyor.

   Soru 2:
   Bu sefer bir sayının değerini belli bir oranda artıracak bir fonksiyon yazılması isteniyor.
   Parametre olarak gelen i8 türünden değerin önce normal taşınarak metod içerisinde artırılması
   sonraki sürümde ise referans olarak gönderilip aynı işlemin yapılması isteniyor. Her iki
   durum arasındaki farkların tartışılması bekleniyor.
*/
