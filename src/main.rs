// use std::fs::rename;
// use std::os::unix::fs::symlink;

// use std::fs::read_dir;
//use std::fs::ReadDir;

use hashbrown::HashMap;
use std::hash::Hash;
use walkdir::WalkDir;
use std::path::PathBuf;
use walkdir::DirEntry;
use std::path::Path;


fn main() -> Result<(), std::io::Error> {
    let dir1 = "/home/pog";
    let dir2 = "/home/pog/Downloads/newhome";
   /* 
   Push to vec and hashmap
   Search map for each new value
    if in map push to vector
    if not in map push to another vector
   */ 
    let mut map: HashMap<PathBuf,DirEntry> = HashMap::new();
    let mut vec: Vec<DirEntry> = Vec::new();

    for entry in WalkDir::new(dir1).into_iter() { 
        let entry = entry?;
        let relative_path = get_relative_path(&entry, dir1);
        map.insert(relative_path, entry);
    }

    for entry in WalkDir::new(dir2).into_iter() { 
        let entry = entry?;
        vec.push(entry);
    }

    let mut same: Vec<DirEntry> = Vec::new();
    let mut diffrent: Vec<DirEntry> = Vec::new();

    vec.iter().for_each(|entry| {
        let relative_path = get_relative_path(&entry, dir2);
        match map.get(&relative_path) {
            Some(mentry) if mentry.file_type() == entry.file_type() => {
                same.push(entry.clone());
            }
            Some(_) | None => {
                diffrent.push(entry.clone());
            }
        }
    });

    println!("entries in dir2 that arent in dir1:");
    diffrent.iter().for_each(|entry| println!("{}",entry.path().display()));
    println!("entries in dir1 that are in dir2:");
    same.iter().for_each(|entry| println!("{}",entry.path().display()));





Ok(())


}

fn get_relative_path<P:AsRef<Path>>(entry:&DirEntry, prefix:P) -> PathBuf {
    entry.path().strip_prefix(prefix).unwrap().to_path_buf() 
}
