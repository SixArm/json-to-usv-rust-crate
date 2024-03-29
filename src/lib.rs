pub mod app {
    pub mod args;
    pub mod clap;
    pub mod log;
}
pub mod examples;
pub mod leaf;
pub mod nest;

use crate::nest::Nest;
use usv::style::Style;

pub fn json_to_usv<
    S: AsRef<str> + Sized,
>(
    json: S,
    style: &Style,
) -> Result<String, serde_json::Error> {
    let mut s = String::new();
    match serde_json::from_str::<Nest>(json.as_ref())? {
        Nest::Leaf(x) => {
            s += &format!("{}", x);
            s += &style.unit_separator;
        },
        Nest::V1(units) => {
            for unit in units {
                s += &format!("{}", unit);
                s += &style.unit_separator;
            }
        },
        Nest::V2(records) => {
            for record in records {
                for unit in record {
                    s += &format!("{}", unit);
                    s += &style.unit_separator;
                }
                s += &style.record_separator;
            }
        },
        Nest::V3(groups) => {
            for group in groups {
                for record in group {
                    for unit in record {
                        s += &format!("{}", unit);
                        s += &style.unit_separator;
                    }
                    s += &style.record_separator;
                }
                s += &style.group_separator;
            }
        },
        Nest::V4(files) => {
            for file in files {
                for group in file {
                    for record in group {
                        for unit in record {
                            s += &format!("{}", unit);
                            s += &style.unit_separator;
                        }
                        s += &style.record_separator;
                    }
                    s += &style.group_separator;
                }
                s += &style.file_separator;
            }
        },
        _ => {}
    }
    Ok(s)
}


#[cfg(test)]
mod tests {
    use super::*;
    use usv::style::Style;

    #[test]
    fn json_boolean() {
        let json = String::from(r#"true"#);
        let usv = String::from("true␟");
        assert_eq!(json_to_usv(&json, &Style::default()).unwrap(), usv);
    }

    #[test]
    fn json_number() {
        let json = String::from(r#"123"#);
        let usv = String::from("123␟");
        assert_eq!(json_to_usv(&json, &Style::default()).unwrap(), usv);
    }

    #[test]
    fn json_string() {
        let json = String::from(r#""a""#);
        let usv = String::from("a␟");
        assert_eq!(json_to_usv(&json, &Style::default()).unwrap(), usv);
    }

    #[test]
    fn json_array_dimension_1() {
        let json = String::from(r#"["a","b"]"#);
        let usv = String::from("a␟b␟");
        assert_eq!(json_to_usv(&json, &Style::default()).unwrap(), usv);
    }

    #[test]
    fn json_array_dimension_2() {
        let json = String::from(r#"[["a","b"],["c","d"]]"#);
        let usv = String::from("a␟b␟␞c␟d␟␞");
        assert_eq!(json_to_usv(&json, &Style::default()).unwrap(), usv);
    }

    #[test]
    fn json_array_dimension_3() {
        let json = String::from(r#"[[["a","b"],["c","d"]],[["e","f"],["g","h"]]]"#);
        let usv = String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝");
        assert_eq!(json_to_usv(&json, &Style::default()).unwrap(), usv);
    }

    #[test]
    fn json_array_dimension_4() {
        let json = String::from(r#"[[[["a","b"],["c","d"]],[["e","f"],["g","h"]]],[[["i","j"],["k","l"]],[["m","n"],["o","p"]]]]"#);
        let usv = String::from("a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜");
        assert_eq!(json_to_usv(&json, &Style::default()).unwrap(), usv);
    }

}
