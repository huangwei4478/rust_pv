use std::env;
use std::io::{self, Result, ErrorKind, Read, Write};
const CHUNK_SIZE: usize = 16 * 1024;
fn main() -> Result<()> {
    let mut total_bytes = 0;
    let silent = !env::var("PV_SILENT").unwrap_or_default().is_empty();
    let mut buffer = [0; CHUNK_SIZE];
    loop {
         let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        // io::stdout().write_all(&buffer[..num_read]).unwrap();
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    if !silent {
        eprintln!("{}", total_bytes);
    }
    eprintln!("{}", total_bytes);
    Ok(())
}
