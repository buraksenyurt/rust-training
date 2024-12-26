use std::io;

mod basics;
mod model;

fn main() -> io::Result<()> {
    basics::run()?;

    Ok(())
}
