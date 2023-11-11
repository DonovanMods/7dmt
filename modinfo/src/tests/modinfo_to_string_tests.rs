use super::*;

fn strip_ws(s: &str) -> String {
    s.split_whitespace().collect()
}

#[test]
fn to_string_v1_test() {
    let xml = fixtures::xml_string_v1();
    let result = Modinfo::from_str(&xml).unwrap().to_string();

    assert_eq!(strip_ws(&result), strip_ws(&xml));
}

#[test]
fn to_string_v1_no_compat_test() {
    let xml = fixtures::xml_string_v1_no_compat();
    let result = Modinfo::from_str(&xml).unwrap().to_string();

    assert_eq!(strip_ws(&result), strip_ws(&xml));
}

#[test]
fn to_string_v2_test() {
    let xml = fixtures::xml_string_v2();
    let result = Modinfo::from_str(&xml).unwrap().to_string();

    assert_eq!(strip_ws(&result), strip_ws(&xml));
}

#[test]
fn to_string_v2_no_compat_test() {
    let xml = fixtures::xml_string_v2_no_compat();
    let result = Modinfo::from_str(&xml).unwrap().to_string();

    assert_eq!(strip_ws(&result), strip_ws(&xml));
}
