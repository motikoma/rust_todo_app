#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullName {
    first_name: Name,
    last_name: Name,
}

impl FullName {
    fn new(first_name: Name, last_name: Name) -> Self {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

impl FromStr for Name {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        let re = Regex::new(r#"^[a-zA-Z]+$"#).unwrap();
        if re.is_match(s){
            Ok(Name(s.to_string()))
        } else {
            Err(MyError::type_error("許可されていない文字が使用されています。"))
        }
    }
}

#[test]
fn show_full_name(){
    let first_name = "taro".parse().unwrap();
    let last_name = "tanaka".parse().unwrap();
    let full_name = FullName::new(firstName, lastName);
    println!("{:?}", full_name);
}

#[test]
fn test_parse_name(){
    let valid_name = "taro".parse::<Name>();
    let invalid_name_with_num = "taro1".parse::<Name>();
    let invalid_name_with_jpn = "太郎".parse::<Name>();
    assert!(valid_name.is_ok());
    assert!(invalid_name_with_num.is_err());
}