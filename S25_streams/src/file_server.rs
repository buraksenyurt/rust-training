use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
/*
    Bu örnek kod parçasında ise ilkel bir file server yer almaktadır.
    Örnek png, jpg, gif gibi resim formatları ile html ve txt gibi metinsel olanları ele alır.

    Test etmek için

    localhost:5003 adresine HTTP Get talebi göndermek yeterli. Örneğin;

    http://localhost:5003/profile_woman.png
    http://localhost:5003/profile_man.png
    http://localhost:5003/index.html

    curl ile test etmek içinse aşağıdaki gibi ilerleyebiliriz.

    curl -v http://localhost:5003/profile_woman.png --output woman.png

    gibi
*/

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    if let Err(err) = stream.read(&mut buffer) {
        eprintln!("Error reading stream: {}", err);
        return;
    }

    let request = String::from_utf8_lossy(&buffer);
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    println!("Request: {} {}", method, path);

    if method == "GET" && path != "/favicon.ico" {
        let filename = &path[1..];
        println!("Serving file: {}", filename);

        if let Ok(contents) = fs::read(filename) {
            let content_type = match Path::new(filename).extension().and_then(|ext| ext.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("gif") => "image/gif",
                Some("html") => "text/html",
                Some("txt") => "text/plain",
                _ => "application/octet-stream",
            };

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                content_type,
                contents.len()
            );

            stream.write_all(response.as_bytes()).unwrap();
            stream.write_all(&contents).unwrap();
            stream.flush().unwrap();
        } else {
            let response =
                "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

pub fn run() -> std::io::Result<()> {
    let address = "127.0.0.1:5003";
    let listener = TcpListener::bind(address)?;
    println!("File server is ready on {address}");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_request(stream);
        } else {
            eprintln!("Failed to accept connection");
        }
    }

    Ok(())
}
