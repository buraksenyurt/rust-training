/*
   Burada örnek bir FFI (Foreign Function Interface) kullanımı söz konusudur.
   Bu kullanımlarda unsafe kod blokları ele alınır.
   Örnekte Rust ortamından bir C fonksiyon çağrısı yapılır.
*/
extern "C" {
    fn strlen(s: *const i8) -> usize;
}

pub fn run() {
    let c_string = std::ffi::CString::new("Rust programming language with Ferris").unwrap();
    let ptr = c_string.as_ptr();

    unsafe {
        println!("String length: {}", strlen(ptr));
    }
}
