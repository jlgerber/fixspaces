// given a directory iterate through its contents and 
// remove any spaces
extern crate rayon;
use std::env;
use std::path::Path;
use std::io;
use std::fs::{self, DirEntry, rename};
use rayon::prelude::*;

fn fix_dirs(pathstr: &str, replace: &str, replace_with: &str) {
    //env::args().skip(1).for_each(|x| {
        if fix_dir(pathstr, replace, replace_with).is_err() {
            println!("problem renaming {}", pathstr);
        };
    //});
}

fn fix_dir(pathstr: &str, replace: &str, replace_with: &str) -> io::Result<()> {
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
                let replacement_string = pathstr.replace(replace, replace_with);
                let replacement = Path::new(replacement_string.as_str());
                if rename(path.clone(),replacement).is_err() {
                    println!("renaming {} failed", pathstr);
                } else {
                    if pathstr.contains(replace) { 
                        println!("mv {} => {}",pathstr, replacement_string);
                        path = replacement.to_path_buf();
                    }
                };
            }
            if path.is_dir() {
                // recurse
                if fix_dir(path.clone().to_str().expect("unable to convert"), replace, replace_with).is_err() {
                    println!("fixdir failed on {}", path.to_str().expect("unable to convert"));
                }
            }
        });
    }
    Ok(())
}
use std::process;
fn main() {
    let mut enviter = env::args().skip(1);
    //enviter.next();
    let path = enviter.next().unwrap_or_else(||{println!("incorrect usage"); process::exit(1); });
    let skipchar = enviter.next().unwrap_or(" ".to_string());
    let withchar = enviter.next().unwrap_or("_".to_string());
    fix_dirs(path.as_str(), skipchar.as_str(), withchar.as_str());
}
