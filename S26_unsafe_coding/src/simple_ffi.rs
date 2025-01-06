/*
   Burada örnek bir FFI (Foreign Function Interface) kullanımı söz konusudur.
   Bu kullanımlarda unsafe kod blokları ele alınır.
   Örnekte Rust ortamından bir C fonksiyon çağrısı yapılır.
*/
extern "C" {
    fn strlen(s: *const i8) -> usize;
    fn sin(x: f64) -> f64;
    fn cos(x: f64) -> f64;
}

pub fn run() {
    let c_string = std::ffi::CString::new("Rust programming language with Ferris").unwrap();
    let ptr = c_string.as_ptr();

    unsafe {
        println!("String length: {}", strlen(ptr));
    }

    let angle = 60.0_f64.to_radians();

    unsafe {
        let sin_value = sin(angle);
        let cos_value = cos(angle);

        println!("sin(60°): {}", sin_value);
        println!("cos(60°): {}", cos_value);
    }
}
