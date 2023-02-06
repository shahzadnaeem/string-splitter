use std::env;

use getopts::Options;

mod splitter;

fn main() {
    let mut opts = Options::new();
    opts.optflag("u", "until", "Until ...");

    let _args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&_args[1..]) {
        Ok(m) => m,
        Err(fail) => panic!("{}", fail.to_string()),
    };

    if matches.free.len() >= 2 {
        let input = matches.free[0].as_str();
        let delimiter = matches.free[1].as_str();

        if matches.opt_present("u") {
            let res = splitter::until(&input, delimiter);
            println!("{}", res);
        } else {
            let res: Vec<&str> = splitter::StrSplit::new(&input, delimiter).collect();
            println!("{:?}", res);
        }
    }
}
