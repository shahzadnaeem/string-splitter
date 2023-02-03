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

    let mut input: String = String::from("Hello😊 😊🐡😊😊🐡🐡🐡 😊😊😊🐡 ");
    let mut delimiter = '🐡';

    if matches.free.len() >= 2 {
        input = matches.free[0].clone();

        if matches.free[1].len() > 0 {
            delimiter = matches.free[1].chars().nth(0).unwrap();
        }
    }

    if matches.opt_present("u") {
        let res = splitter::until_char(&input, delimiter);
        println!("{:?}", res);
    } else {
        let res: Vec<&str> = splitter::StrSplit::new(&input, delimiter).collect();
        println!("{:?}", res);
    }
}
