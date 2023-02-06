use quickcheck::TestResult;

use crate::splitter::*;

#[test]
fn check_crate_root() {}

#[test]
fn empty_split() {
    let input = "";
    let delim = '🌵';

    let res: Vec<&str> = StrSplit::new(input, delim).collect();

    assert_eq!(vec![""], res);
}

#[test]
fn delim_only_split1() {
    let input = "🌵";
    let delim = '🌵';

    let res: Vec<&str> = StrSplit::new(input, delim).collect();

    assert_eq!(vec!["", ""], res);
}

#[test]
fn delim_only_split2() {
    let input = "🌵🌵";
    let delim = '🌵';

    let res: Vec<&str> = StrSplit::new(input, delim).collect();

    assert_eq!(vec!["", "", ""], res);
}

#[test]
fn split_some() {
    let input = "😊🌵🐡🐡,🐡🌵🐡more text";
    let delim = '🐡';

    let res: Vec<&str> = StrSplit::new(input, delim).collect();

    assert_eq!(vec!["😊🌵", "", ",", "🌵", "more text"], res);
}

#[test]
fn split_three_trailing() {
    let input = "😊🌵🐡🌵🐡more text🐡";
    let delim = '🐡';

    let res: Vec<&str> = StrSplit::new(input, delim).collect();

    assert_eq!(vec!["😊🌵", "🌵", "more text", ""], res);
}

#[test]
fn split_csv() {
    let input = "1,2,3,4,5,678,9,10,1001";
    let delim = ',';

    let res: Vec<&str> = StrSplit::new(input, delim).collect();

    assert_eq!(vec!["1", "2", "3", "4", "5", "678", "9", "10", "1001"], res);
}

#[test]
fn split_csv_by_string() {
    let input = "1//2//3//10//10,01//";
    let delim = "//";

    let res: Vec<&str> = StrSplit::new(input, delim).collect();

    assert_eq!(vec!["1", "2", "3", "10", "10,01", ""], res);
}

#[test]
fn until_char_empty() {
    let input = "";
    let delim = "🌵";

    let res = until(input, delim);

    assert_eq!("", res);
}

#[test]
fn until_char_one() {
    let input = "one🌵🌵🌵TWO three";
    let delim = "🌵🌵";

    let res = until(input, delim);

    assert_eq!("one", res);
}

// ---- Quicktest - a little example

fn csv3(a: i32, b: i32, c: i32) -> String {
    format!("{},{},{}", a, b, c)
}

#[quickcheck]
fn split_csv3(a: i32, b: i32, c: i32) -> bool {
    let as_csv = csv3(a, b, c);
    let split = as_csv
        .split(",")
        .map(|it| it.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    a == split[0] && b == split[1] && c == split[2]
}

#[quickcheck]
fn str_split(inputs: Vec<String>, delim: String) -> TestResult {
    // Unsupported cases!
    if inputs.len() == 0 || delim.len() == 0 {
        return TestResult::discard();
    }
    // Can't have 'delim' in 'inputs'
    if inputs.iter().any(|it| it.contains(&delim)) {
        return TestResult::discard();
    }

    let line = inputs.join(&delim);
    let res: Vec<&str> = StrSplit::new(&line, &*delim).collect();

    TestResult::from_bool(res == inputs)
}
