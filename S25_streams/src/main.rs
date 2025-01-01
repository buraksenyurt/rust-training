use std::io;

mod basics;
mod echo_server;
mod file_server;
mod kv_server;
mod low_level_server;
mod ping_server;

fn main() -> io::Result<()> {
    // basics::run()?;
    // low_level_server::run()?;
    // ping_server::run();
    // echo_server::run()?;
    // file_server::run()?;
    kv_server::run()?;
    Ok(())
}
