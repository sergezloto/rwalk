fn main() {
    println!("Hello, world!");
    visit();
}
extern crate walkdir;
use walkdir::WalkDir;

extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

///////////
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// extern crate memmap;
// use memmap::Mmap;

fn visit() {
    let mut buffer = Vec::new();
    let mut md5 = Md5::new();

    for entry in WalkDir::new(".")
        .follow_links(false)
        .same_file_system(true) {
        let entry = entry.unwrap();
            let path = entry.path();

        // Do not open symlinks and directories
        let ft = path.metadata().unwrap().file_type();
        if ft.is_dir() {
            continue
        }
        if ft.is_symlink() {
            continue
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
        println!("{}  {}", h, path.display())
    }
}
