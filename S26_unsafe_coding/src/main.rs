/*
    Rust, bellek güvenliğini sağlama ve eşzamanlılık garantilerini sunmak üzere tasarlanmış bir
    bellek yönetim sistemini benimser. Ancak bazı hallerde bu güvenli alanın dışına çıkılması
    gerekebilir. Bu genellikle unsafe kod blokları ile gerçekleştirilir.

    Unsafe bloklarda dilin standart güvnelik mekanizmaları göz ardı edilir. Unsafe kod tarafına
    çıkmayı gerektiren durumlar kısaca şöyle özetlenebilir.

    - Performans iyileştirmeleri: Rust'un kontrol mekanizmaları performans üzerinde yük oluşturabilir.
    Mesela raw pointer'lar ile çalışmak çok hızlıdır. Bellek erişiminde bu hızlara çıkmak istediğimiz
    durumlarda ele alabiliriz.
    - C/C++ gibi rust dışındaki dillerle FFI(Foreign Function Interface) etkileşim kurmak istediğimiz
    hallerde tercih edilir.
    - Bellek bloklarını doğrudan manupile etmek istediğimiz hallerde.
    - Rust'ın normal veri yapılarından daha büyük boyutlardaki (union yapılar gerektiren haller)
    ihtiyaçlarda bit seviyesinde işlemler gerektiğinde
    - Yazılan kodun mantıksal olarak güvenli olduğu hallerde kullanılan built-in fonksiyonellikleri
    sağlayan api'ler için güvenlik kontrollerini atlamak istediğimizde

    Unsafe kod kullanımında sorumluluk daha çok geliştiriciye kalır. Diğer yandan unsafe kod blokları
    gerekli olduğu yerlerde kullanılır ve kalan kod parçaları rust'ın güvenlik kriterlerine tabi
    olmaya devam eder.

    Unsafe kodlama bellek güvenliği kriterlerinden feragat etmek anlamına gelse de, doğru şekilde
    ve yerlerde kullanıldığında sistem seviyesinde kontrol ve yüksek performans sağlar.
*/
mod advanced;
mod simple_ffi;

fn main() {
    // simple_ffi::run();
    advanced::run();
}
