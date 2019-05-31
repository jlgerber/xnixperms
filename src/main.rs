use xnixperms::*;
use std::fs::File;
use std::env;

fn main() -> std::io::Result<()> {
    let fpath = env::args().nth(1).unwrap_or(String::from(""));

    let f = File::open(fpath)?;
    let metadata = f.metadata()?;
    let permissions = metadata.permissions();
    println!("sugo: {:#o}", permissions.sugo());
    println!("ascii: {}", permissions.as_ascii());
    println!("is file {}", permissions.is_file());
    println!("is dir {}", permissions.is_dir());
    println!("is link {}", permissions.is_link());
    println!("is sticky {}", permissions.is_sticky());
    println!("is sgid {}", permissions.is_sgid());
    println!("is suid {}", permissions.is_suid());

    Ok(())
}