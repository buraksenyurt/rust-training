use std::io;
use std::io::BufRead;
use std::io::Write;

/*
    Aşağıdaki kod parçasında stdin ve stdout kullanılarak çok düşük seviyede bir http server
    fonksiyonelliği ele alınmaktadır. Bu tabii çok düşük seviye bir kodlamadır ve açıkçası çok
    pratik değildir. Bu tip bir web sunucusu hizmeti için std::net modülündeki TcpListener ve
    TcpStream türleri kullanılabilir.

    Terminalden denemek için;

    Linux tarafında aşağıdaki şekilde ilerleyebilir.

    echo -e "GET /ping HTTP/1.1\r\n" | cargo run

    Windows tarafında ise

    cargo run ile projeyi çalıştırdıktan sonra
    terminale
    GET /ping HTTP/1.1
    yazılıp ilerlenebilir.
*/
pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut reader = stdin.lock();
    let mut writer = stdout.lock();

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");
    println!("{:?} {:?}", method, path);

    if method == "GET" && path == "/ping" {
        writeln!(writer, "HTTP/1.1 200 OK")?;
        writeln!(writer, "Content-Type: application/json")?;
        writeln!(writer, "Content-Length: 6")?;
        writeln!(writer, "")?;
        writeln!(writer, "Pong!")?;
    } else {
        writeln!(writer, "HTTP/1.1 404 Not Found")?;
        writeln!(writer, "Content-Type: application/json")?;
        writeln!(writer, "Content-Length: 9")?;
        writeln!(writer, "")?;
        writeln!(writer, "Not Found")?;
    }

    Ok(())
}
