use std::io;

mod basics;

fn main() -> io::Result<()> {
    basics::run()?;

    Ok(())
}
