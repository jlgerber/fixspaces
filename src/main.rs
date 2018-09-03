// given a directory iterate through its contents and 
// remove any spaces
use std::env;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry, rename};

fn fix_dirs() {
    env::args().skip(1).for_each(|x| {
        if fix_dir(x.as_str()).is_err() {
            println!("problem renaming {}", x.as_str());
        };
    });
}

fn fix_dir(pathstr: &str) -> io::Result<()> {
    let dir = Path::new(pathstr);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // recurse
                println!(
                    "skipping directory {:?}", path
                );
            } else {
                // the heart of it
                if let Some(pathstr) = path.to_str() {
                    let replacement = pathstr.replace(" ", "_");
                    let replacement = Path::new(replacement.as_str());
                    rename(path.clone(),replacement)?;
                }
            }
        }

    }
    Ok(())
}
fn main() {
    fix_dirs();
}
