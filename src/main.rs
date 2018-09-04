// given a directory iterate through its contents and 
// remove any spaces
extern crate rayon;
use std::env;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry, rename};
use rayon::prelude::*;

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
        fs::read_dir(dir)?
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect::<Vec<DirEntry>>()
        .par_iter()
        .for_each(|entry| {
            let mut path = entry.path();
           
                // the heart of it
            if let Some(pathstr) = path.clone().to_str() {
                let replacement = pathstr.replace(" ", "_");
                let replacement = Path::new(replacement.as_str());
                if rename(path.clone(),replacement).is_err() {
                    println!("renaming {} failed", pathstr);
                } else {
                    if pathstr.contains(" ") { 
                        println!("renamed {}",pathstr);
                        path = replacement.to_path_buf();
                    }
                };
            }
            if path.is_dir() {
                // recurse
                if fix_dir(path.clone().to_str().expect("unable to convert")).is_err() {
                    println!("fixdir failed on {}", path.to_str().expect("unable to convert"));
                }
            }
        });
    }
    Ok(())
}

fn main() {
    fix_dirs();
}
