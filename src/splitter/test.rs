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
