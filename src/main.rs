use hashbrown::HashMap;
use walkdir::{WalkDir, DirEntry};
use std::path::{Path, PathBuf};

fn main() -> Result<(), std::io::Error> {
    let args = pargs().expect("arg parse faield");
    cmp_dirs(args.dir1,args.dir2)
}

fn get_relative_path<P:AsRef<Path>>(entry:&DirEntry, prefix:P) -> PathBuf {
    entry.path().strip_prefix(prefix).unwrap().to_path_buf() 
}

fn cmp_dirs<P:AsRef<Path>>(dir1:P,dir2:P) -> Result<(), std::io::Error>  {
   /* 
   Push to vec and hashmap
   Search map for each new value
    if in map push to vector
    if not in map push to another vector
   */ 
    let (dir1,dir2) = (dir1.as_ref(),dir2.as_ref());
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

    println!("entries in {} that arent in {}:", dir2.to_string_lossy(), dir1.to_string_lossy());
    diffrent.iter().for_each(|entry| println!("{}",entry.path().display()));
    println!("entries in {} that are in {}:" ,dir1.to_string_lossy(), dir2.to_string_lossy() );
    same.iter().for_each(|entry| println!("{}",entry.path().display()));
Ok(())
}

    struct Pargs {
        dir1: PathBuf,
        dir2: PathBuf,
        // same: bool,
        // diffrent: bool,
        // all: bool
    }

mod text;
use text::HELP;
use text::VERSION;

fn pargs() -> Result<Pargs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    if pargs.contains(["-v","--version"]) {
        println!("{}", VERSION);
        std::process::exit(0);
    }

    let args = Pargs {
        dir1: pargs.free_from_str()?,
        dir2: pargs.free_from_str()?,
        // same: pargs.contains(["-s","--same"]),
        // diffrent: pargs.contains(["-d","--diffrent"]),
        // all: pargs.contains(["-a","--all"])
    };
    println!("Using Paths: {} , {}",args.dir1.display(),args.dir2.display());



    Ok(args)
}