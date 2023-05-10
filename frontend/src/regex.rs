use regex::Regex;

pub fn email_regex() -> Regex {
    Regex::new(r"[a-zA-Z0-9._%+-]+\@[a-zA-Z0-9.-]+\.[a-zA-Z]{1,}").unwrap()
}
#[test]
fn test_email_regex() {
    let input = "test@test.com";
    let re = email_regex();
    assert_eq!(re.is_match(input), true);
}
#[test]
fn test_email_regex_fail_no_top_level() {
    let input = "test@test.";
    let re = email_regex();
    assert_eq!(re.is_match(input), false);
}

#[test]
fn test_email_regex_fail_no_name() {
    let input = "test.com";
    let re = email_regex();
    assert_eq!(re.is_match(input), false);
}
