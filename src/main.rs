// use std::fs::rename;
// use std::os::unix::fs::symlink;

// use std::fs::read_dir;
//use std::fs::ReadDir;

use hashbrown::HashMap;
use std::hash::Hash;
use walkdir::WalkDir;
use std::path::PathBuf;

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
    let (map1,vec1) = dir_entries_to_hashmap_vector(dir1)?;
    let (map2,vec2) = dir_entries_to_hashmap_vector(dir2)?;

    let (same,diffrent) = check_map_for_elements(&map1,&vec2);
    println!("entries in dir2 that arent in dir1:");
    diffrent.iter().for_each(|entry| println!("{}",entry.display()));
    println!("entries in dir1 that are in dir2:");
    same.iter().for_each(|entry| println!("{}",entry.display()));

Ok(())


}

fn check_map_for_elements<K:Eq+Hash+Clone,V>(map_to_check: &HashMap<K,V>, vec_to_iter:&Vec<K>) -> (Vec<K>,Vec<K>) { //Return Same+Diffrent
    let mut same = Vec::new();
    let mut diffrent = Vec::new();

    vec_to_iter.iter().for_each(|entry|
        {
            if map_to_check.contains_key(entry) {
                same.push(entry.clone())
            }
            else {diffrent.push(entry.clone())}

        }
    );
    (same,diffrent)

}

fn dir_entries_to_hashmap_vector<P:AsRef<Path>>(dir1:P) -> Result< (HashMap<PathBuf,usize>,Vec<PathBuf>) ,std::io::Error >{
    let dir1 = dir1.as_ref();
    let mut vec1 = Vec::new();
    let mut map1 = HashMap::new();

    for entry in WalkDir::new(dir1).into_iter() { 
        let entry = entry?;
        let relative_path = entry.path().strip_prefix(dir1).unwrap().to_path_buf();
        vec1.push(relative_path.clone());
        map1.insert(relative_path,vec1.len());
    }
    Ok((map1,vec1))
}