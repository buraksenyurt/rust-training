use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

/*
    Bu örnekte ise basit bir key-value store'un stream üzerinden host edilmesi ele alınır.
    Gelen talepler için ayrı birer thread başlatılır.
    Test etmek için yine ncat aracından aşağıdaki gibi yararlanabiliriz.

    echo SET isActive true | ncat localhost 5002
    echo GET isActive" | ncat localhost 5002

    echo SET connectionString datasource=localhost;database=Northwind | ncat localhost 5002
    echo GET connectionString | ncat localhost 5002

    echo LIST | ncat localhost 5002

    Pek tabii bu tip bir sunucuya karşılık gelecek bir istemci de yazılabilir.
*/

fn handle_request(mut stream: TcpStream, value_store: Arc<Mutex<HashMap<String, String>>>) {
    let mut buffer = [0; 1024];
    while let Ok(bytes_read) = stream.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Request: {}", request);
        let mut parts = request.split_whitespace();

        let command = parts.next().unwrap_or("");
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");
        println!("Command: {}\nKey:{}\nValue:{}\n", command, key, value);

        let response = match command {
            "SET" => {
                let mut vs_guard = value_store.lock().unwrap();
                vs_guard.insert(key.to_string(), value.to_string());
                "OK\n".to_string()
            }
            "GET" => {
                let vs_guard = value_store.lock().unwrap();
                vs_guard
                    .get(key)
                    .cloned()
                    .unwrap_or("NOT FOUND\n".to_string())
            }
            "DEL" => {
                let mut vs_guard = value_store.lock().unwrap();
                if vs_guard.remove(key).is_some() {
                    "DELETED\n".to_string()
                } else {
                    "NOT FOUND\n".to_string()
                }
            }
            "LIST" => {
                let vs_guard = value_store.lock().unwrap();
                let keys: Vec<String> = vs_guard.keys().cloned().collect();
                keys.join("\n")
            }
            _ => "ERROR: Unknown command\n".to_string(),
        };

        stream.write_all(response.as_bytes()).unwrap();
    }
}

pub fn run() -> std::io::Result<()> {
    let address = "127.0.0.1:5002";
    let listener = TcpListener::bind(address)?;
    let value_store = Arc::new(Mutex::new(HashMap::new()));

    println!("KV Store is ready on {address}");

    for stream in listener.incoming() {
        let stream = stream?;
        let value_store_clone = Arc::clone(&value_store);

        thread::spawn(move || handle_request(stream, value_store_clone));
    }

    Ok(())
}
