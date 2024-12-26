use std::io;

mod advanced;
mod basics;
mod data_generator;
mod model;
/*
    Pek çok programlama dilinde olduğu gibi Rust tarafında da kullanışlı File I/O
    modüller ve fonksiyonellikleri vardır. Bu bölümde fiziki dosyalar ile ilgili Input/Output
    işlemlerine ait temel seviyede örneklere yer verilmektedir.
*/
fn main() -> io::Result<()> {
    // basics::run()?;
    data_generator::run()?;
    advanced::run()?;

    Ok(())
}
