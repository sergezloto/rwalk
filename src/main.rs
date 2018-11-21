fn main() {
    visit();
}
extern crate walkdir;
use walkdir::{WalkDir, DirEntry};

extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

///////////
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

extern crate rayon;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

// extern crate memmap;
// use memmap::Mmap;

fn visit() {
    let mut buffer = Vec::new();
    let mut md5 = Md5::new();

    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    for entry in WalkDir::new(".")
        .follow_links(false)
        .same_file_system(true)
        .contents_first(true)
    //.into_iter()
    //.filter_entry(|e| !is_hidden(e))
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
            let f = File::open(path).unwrap();

            //let mmap = unsafe { Mmap::map(&f)? };

            let mut reader = BufReader::new(f);
            reader.read_to_end(&mut buffer).unwrap();

            // Compute content MD5
            md5.reset();
            md5.input(&buffer);
            let h = md5.result_str();

            // Write out md5< >< >filename
            println!("{}  {}", h, path.display());
        }
        );
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    let s = entry.file_name().to_str().unwrap();
    s.starts_with(".")
}
