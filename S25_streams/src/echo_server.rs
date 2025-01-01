use std::io::{Read, Write};
use std::net::TcpListener;

/*
    Aşağıdaki örnek basit bir echo server. Client server arasında kurulan bağlantıda,
    client'ın gönderdiği mesaj sunucu tarafından client'a geri gönderilir veya sunucu
    client'tan gelen mesaja karşılık ECHO diye yanıt döner.

    Çalıştığım Windows 11 sisteminde bunu denemek için netcat programından yararlandım.

    echo "Hello world" | nc localhost 7001

    ile deneyebiliriz.
*/
pub fn run() -> std::io::Result<()> {
    let address = "0.0.0.0:7001";
    let listener = TcpListener::bind(address)?;
    println!("Listening on {address}");

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        let message = String::from_utf8_lossy(&buffer[..bytes_read])
            .trim()
            .to_owned();
        println!("{bytes_read} bytes received: {message} ");
        stream.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}
