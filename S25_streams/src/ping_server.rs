use std::fmt::Display;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/*
    Bu örnekte TcpStream ve TcpListener kullanılarak ilkel bir web server işletiliyor.
    İstemciler http://localhost:5001/ping adresine bir talepte bulunduklarında JSON formatında
    Pong! isimli bir cevap alıyorlar. Eğer başka bir route talep edilirse de Not Found cevabı
    alıyorlar.

    TcpStream ve TcpListener kullanılarak low-level'da bir HTTP Server yazmak daha mantıklı elbette.
    Bu ikili kullanılarak farklı server'lar da yazılabilir.

    İstemciden gelen veriyi geriye döndüren echo server,
    File Server,
    Chat uygulaması,
    Redis benzeri key-value store
*/
fn handle_ping(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    let response = if method == HttpMethod::Get.to_string() && path == "/ping" {
        let body = r#"{"message": "Pong!"}"#;
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
    } else {
        let body = r#"{"error": "Not Found"}"#;
        format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn run() {
    /*
        localhost:5001 portu sürekli dinleniyor ve data stream handle_ping metoduna
        gönderilerek değerlendiriliyor.
    */
    let listener = TcpListener::bind("127.0.0.1:5001").unwrap();
    println!("Listening on 127.0.0.1:5001...");

    for stream in listener.incoming() {
        handle_ping(stream.unwrap());
    }
}

enum HttpMethod {
    Post,
    Get,
    Delete,
    Patch,
    Put,
}
impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HttpMethod::Post => "POST".to_string(),
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Delete => "DELETE".to_string(),
            HttpMethod::Patch => "PATCH".to_string(),
            HttpMethod::Put => "PUT".to_string(),
        };
        write!(f, "{}", str)
    }
}
