use std::env;
use std::io::Error;

pub fn run_cli() -> Result<(), Error> {
    let pwd = match env::current_dir() {
        Ok(path) => path,
        Err(e) => return Err(e),
    };
    print!("{}> ", pwd.display());

    Ok(())
}
