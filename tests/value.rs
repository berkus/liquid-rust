extern crate liquid;
extern crate serde_yaml;

#[macro_use]
extern crate difference;

use std::f32;

#[test]
pub fn serialize_num() {
    let actual = liquid::Value::scalar(1f32);
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\n1", "", 0);

    let actual = liquid::Value::scalar(-100f32);
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\n-100", "", 0);

    let actual = liquid::Value::scalar(3.14e_10f32);
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\n31399999488", "", 0);

    let actual = liquid::Value::scalar(f32::NAN);
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\nNaN", "", 0);

    let actual = liquid::Value::scalar(f32::INFINITY);
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\ninf", "", 0);
}

#[test]
pub fn deserialize_num() {
    let actual: liquid::Value = serde_yaml::from_str("---\n1").unwrap();
    assert_eq!(actual, liquid::Value::scalar(1f32));

    let actual: liquid::Value = serde_yaml::from_str("---\n-100").unwrap();
    assert_eq!(actual, liquid::Value::scalar(-100f32));

    let actual: liquid::Value = serde_yaml::from_str("---\n31399999488").unwrap();
    assert_eq!(actual, liquid::Value::scalar(3.14e_10f32));

    // Skipping NaN since equality fails

    let actual: liquid::Value = serde_yaml::from_str("---\ninf").unwrap();
    assert_eq!(actual, liquid::Value::scalar(f32::INFINITY));
}

#[test]
pub fn serialize_bool() {
    let actual = liquid::Value::scalar(true);
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\ntrue", "", 0);

    let actual = liquid::Value::scalar(false);
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\nfalse", "", 0);
}

#[test]
pub fn deserialize_bool() {
    let actual: liquid::Value = serde_yaml::from_str("---\ntrue").unwrap();
    assert_eq!(actual, liquid::Value::scalar(true));

    let actual: liquid::Value = serde_yaml::from_str("---\nfalse").unwrap();
    assert_eq!(actual, liquid::Value::scalar(false));
}

#[test]
pub fn serialize_nil() {
    let actual = liquid::Value::Nil;
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\n~", "", 0);
}

#[test]
pub fn deserialize_nil() {
    let actual: liquid::Value = serde_yaml::from_str("---\n~").unwrap();
    assert_eq!(actual, liquid::Value::Nil);

    let actual: liquid::Value = serde_yaml::from_str("---\n- ").unwrap();
    assert_eq!(actual, liquid::Value::Array(vec![liquid::Value::Nil]));
}

#[test]
pub fn serialize_str() {
    let actual = liquid::Value::scalar("Hello");
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\nHello", "", 0);

    let actual = liquid::Value::scalar("10");
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\n\"10\"", "", 0);

    let actual = liquid::Value::scalar("false");
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\n\"false\"", "", 0);
}

#[test]
pub fn deserialize_str() {
    let actual: liquid::Value = serde_yaml::from_str("---\nHello").unwrap();
    assert_eq!(actual, liquid::Value::scalar("Hello"));

    let actual: liquid::Value = serde_yaml::from_str("\"10\"\n").unwrap();
    assert_eq!(actual, liquid::Value::scalar("10"));

    let actual: liquid::Value = serde_yaml::from_str("---\n\"false\"").unwrap();
    assert_eq!(actual, liquid::Value::scalar("false"));
}

#[test]
pub fn serialize_array() {
    let actual = vec![
        liquid::Value::scalar(1f32),
        liquid::Value::scalar(true),
        liquid::Value::scalar("true"),
    ];
    let actual = liquid::Value::Array(actual);
    let actual = serde_yaml::to_string(&actual).unwrap();
    assert_diff!(&actual, "---\n- 1\n- true\n- \"true\"", "", 0);
}

#[test]
pub fn deserialize_array() {
    let actual: liquid::Value = serde_yaml::from_str("---\n- 1\n- true\n- \"true\"").unwrap();
    let expected = vec![
        liquid::Value::scalar(1f32),
        liquid::Value::scalar(true),
        liquid::Value::scalar("true"),
    ];
    let expected = liquid::Value::Array(expected);
    assert_eq!(actual, expected);
}

#[test]
pub fn serialize_object() {
    // Skipping due to HashMap ordering issues
}

#[test]
pub fn deserialize_object() {
    let actual: liquid::Value =
        serde_yaml::from_str("---\nNum: 1\nBool: true\nStr: \"true\"").unwrap();
    let expected: liquid::Object = [
        ("Num".to_owned(), liquid::Value::scalar(1f32)),
        ("Bool".to_owned(), liquid::Value::scalar(true)),
        ("Str".to_owned(), liquid::Value::scalar("true")),
    ].iter()
        .cloned()
        .collect();
    let expected = liquid::Value::Object(expected);
    assert_eq!(actual, expected);
}
