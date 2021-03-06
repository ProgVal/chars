extern crate getopts;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quick_error;
extern crate regex;
extern crate fst;

use std::env;
use std::path::Path;

mod fst_generator;
mod unicode;
mod ascii;


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m, Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") || matches.free.len() != 2 {
        println!("{}", opts.usage(&format!(
            "USAGE: {} [options] data-dir output-src-dir\n -- generate character name table",
            program)));
        return
    }
    let data_dirname = matches.free[0].clone();
    let data_dir = Path::new(data_dirname.as_str());

    let src_dirname = matches.free[1].clone();
    let src_dir = Path::new(src_dirname.as_str());

    let mut sorted_names = fst_generator::Names::new();

    ascii::write_ascii_name_data(&data_dir.join("ascii/nametable"), &src_dir.join("ascii/names.rs"),
                                 &mut sorted_names);
    unicode::read_names(&mut sorted_names, &data_dir.join("unicode/NameAliases.txt")).unwrap();
    unicode::read_names(&mut sorted_names, &data_dir.join("unicode/UnicodeData.txt")).unwrap();
    unicode::write_name_data(&sorted_names, &src_dir.join("unicode/")).unwrap();
}
