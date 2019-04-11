extern crate walkdir;
use walkdir::{WalkDir, DirEntry};

extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

///////////
//use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::env;

extern crate rayon;
//use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

// extern crate memmap;
// use memmap::Mmap;

fn main() {
    for dir in env::args() {
        visit(&dir)
    }
}

fn visit(dir: &str) {
    let mut buffer = Vec::new();
    let mut md5 = Md5::new();

    let nthreads = 0;  // let rayon set thread number
    let pool = ThreadPoolBuilder::new().num_threads(nthreads).build().unwrap();

    for entry in WalkDir::new(dir)
        .follow_links(false)
        .same_file_system(true)
        .contents_first(true)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {

        let entry = entry.unwrap();
        let path = entry.path();

        pool.install(|| {
            // Do not open symlinks and directories
            let ft = path.metadata().unwrap().file_type();
            if ft.is_dir() {
                return
            }
            if ft.is_symlink() {
                return
            }
            // Open file
            match File::open(path) {
                Ok(f) => {
                    //let mmap = unsafe { Mmap::map(&f)? };

                    let mut reader = BufReader::new(f);
                    if let Ok(_) = reader.read_to_end(&mut buffer) {

                        // Compute content MD5
                        md5.reset();
                        md5.input(&buffer);
                        let h = md5.result_str();

                        // Write out md5< >< >filename
                        println!("{}  {}", h, path.display())
                    }
                },
                Err(e) =>
                eprintln!("Could not open {}: {}", path.display(), e)
            }
        }
        );
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    let s = entry.file_name().to_str().unwrap();
    s.starts_with(".")
}
