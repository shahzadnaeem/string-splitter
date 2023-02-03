use super::*;

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
