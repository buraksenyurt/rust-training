use std::io;

mod basics;
mod low_level_server;
mod ping_server;

fn main() -> io::Result<()> {
    // basics::run()?;
    // low_level_server::run()?;
    ping_server::run();

    Ok(())
}
