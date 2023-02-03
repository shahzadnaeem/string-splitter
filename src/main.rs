use std::env;

mod splitter;

fn main() {
    let mut input: String = String::from("Hello😊 😊😊😊🐡🐡🐡 😊😊😊🐡 ");
    let mut delimiter = '🐡';

    if env::args().len() >= 3 {
        // We have been give some args to play with!
        let args: Vec<String> = env::args().collect();

        input = args[1].clone();

        if args[2].len() > 0 {
            delimiter = args[2].chars().nth(0).unwrap();
        }
    }

    let res: Vec<&str> = splitter::StrSplit::new(&input, delimiter).collect();

    println!("{:?}", res);
}
