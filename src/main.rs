use std::env;

use getopts::Options;

enum FoundFlag {
    Exists,
    Directory,
}

const SUCCESS_EXIT_CODE : i32 = 0;
const FAILURE_EXIT_CODE : i32 = 1;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let program = args.pop().unwrap();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("e", "exists", "tests if a file exists");
    opts.optflag("d", "directory", "tests if a directory exists");
    let mut matches = match opts.parse(&args) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let flags = if matches.opt_present("e") {
        FoundFlag::Exists
    } else if matches.opt_present("d") {
        FoundFlag::Directory
    } else {
        eprintln!("Use either -d or -e!");
        return;
    };

    match matches.free.pop() {
        Some(file) => check_if_file_exists(file, flags),
        None => print_usage(&program, opts),
    }
}

fn check_if_file_exists(file: String, flags: FoundFlag) -> ! {
    let path = std::path::PathBuf::from(file);
    let code = if path.exists() {
        match flags {
            FoundFlag::Exists => SUCCESS_EXIT_CODE,
            FoundFlag::Directory if path.is_dir() => SUCCESS_EXIT_CODE,
            _ => FAILURE_EXIT_CODE,
        }
    } else {
        FAILURE_EXIT_CODE
    };
    std::process::exit(code)
}

fn print_usage(program: &str, opts: Options) {
    eprintln!("{}", opts.usage(&format!("{} [OPTIONS] [FILE]\nIf called with multiple options, the world burns.", program)));
}
