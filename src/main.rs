extern crate walkdir;
use walkdir::{WalkDir, DirEntry};

extern crate structopt;
use structopt::StructOpt;

use std::error::Error;

#[derive(StructOpt)]
struct Opt {
    dirs: Vec<String>
}

fn main() {
    let opt = Opt::from_args();
    for	dir in opt.dirs {
        visit(&dir)
    }
}

fn visit(dir: &String) {
    for entry in WalkDir::new(dir)
        .follow_links(false)
        .same_file_system(true)
        .contents_first(true)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
    {

        let entry = entry.unwrap();
        let path = entry.path();

        tgabmp_to_png(path).unwrap_or_else(|e| println!("FAIL {:?}, {}", path, e));
    }
}


extern crate imagefmt;
fn tgabmp_to_png(path: &std::path::Path) -> Result<(), Box<Error>> {
    use std::fs;

    match path.extension().and_then(|oss| oss.to_str()) {
        Some("tga")
        | Some("bmp") => {
             use imagefmt::{ColFmt, ColType};

            let img = imagefmt::read(path, ColFmt::Auto)?;
            let png_path = path.with_extension("png");
            if !png_path.exists() {
                println!("CONVERT: {:?}", path);
                imagefmt::write(png_path,img.w, img.h, img.fmt, &img.buf, ColType::Auto)?;
            }
            println!("REMOVE1: {:?}", path);
            fs::remove_file(path)?;
        },
        Some("png") => {
            // png here, check if there is a tga we should remove
            let tga_path = path.with_extension("tga");
            if tga_path.exists() {
                println!("REMOVE2: {:?}", tga_path);
                fs::remove_file(tga_path).unwrap_or(());;
            } else {
                println!("NOP: {:?}", path);
            }
        },
        _ => {
            //println!("SKIP: {:?}", path);
        }
    }
    Ok(())
}

/*****
extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

let mut buffer = Vec::new();
let mut md5 = Md5::new();

fn do_md5(path: &std::path::Path) -> Result<(), Box<Error>> {
    // Do not open symlinks and directories
    let ft = path.metadata().unwrap().file_type();
    if ft.is_dir() {
        return ()
    }
    if ft.is_symlink() {
        return ()
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
            Ok(())
        },
        Err(e) =>
        eprintln!("Could not open {}: {}", path.display(), e)
        Ok(())
    }
}

*****/
fn is_hidden(entry: &DirEntry) -> bool {
    let s = entry.file_name().to_str().unwrap();
    s.starts_with(".")
}
