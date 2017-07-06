use std::collections::HashMap;
use regex::Regex;

pub type JsonObject = HashMap<String, Box<Json + Send>>;
pub type JsonArray = Vec<Box<Json + Send>>;

#[derive(Copy, Clone, PartialEq)]
pub enum JsonType {
    String,
    Number,
    Object,
    Array,
    Boolean,
    Null,
    Error,
}

pub trait Json {
    fn get_value(&self) -> String;
    fn get_type(&self) -> JsonType;
    fn get_path(&self, path: &str) -> Result<&Json, &'static str>;
}

impl Json for String {
    fn get_value(&self) -> String {
        if self.get_type() == JsonType::String {
            format!("{}\"", self.to_string())
        } else {
            self.to_string()
        }
    }

    fn get_type(&self) -> JsonType {
        lazy_static! {
            static ref RE_STRING: Regex = Regex::new(r#"^"(\\.|[^\\"])*"#).unwrap();
            static ref RE_NUM: Regex = Regex::new(r#"^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?"#).unwrap();

        }
        if (self == "true") | (self == "false") {
            JsonType::Boolean
        } else if self == "null" {
            JsonType::Null
        } else if RE_STRING.is_match(self) {
            JsonType::String
        } else if RE_NUM.is_match(self) {
            JsonType::Number
        } else {
            JsonType::Error
        }
    }

    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        println!("Getting {} on a JSON terminal", path);
        let error_string = "Could not get path";
        Err(error_string)
    }
}

impl Json for JsonObject {
    fn get_value<'a>(&self) -> String {
        let object_string = self.iter()
            .map(|(k, v)| format!("\"{}\":{},", k, v.get_value()))
            .fold(String::new(), |acc, s| format!("{}{}", acc, s));

        format!("{{{}}}", object_string.trim_right_matches(','))
    }

    fn get_type(&self) -> JsonType {
        JsonType::Object
    }

    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        println!("Getting key: {} on a JSON Object", path);
        match self.get(&path.to_string()) {
            Some(result) => Ok(&**result),
            None => Err("Could not get path"),
        }
    }
}

impl Json for JsonArray {
    fn get_value(&self) -> String {
        let array_string = self.iter()
            .map(|v| v.get_value())
            .fold(String::new(), |acc, s| format!("{}{},", acc, s));
        format!("[{}]", array_string.trim_right_matches(','))
    }

    fn get_type(&self) -> JsonType {
        JsonType::Array
    }

    fn get_path(&self, path: &str) -> Result<&Json, &'static str> {
        let index = path.parse::<usize>();
        println!("Getting index: {} on a JSON array", path);
        if let Ok(i) = index {
            match self.get(i) {
                Some(result) => Ok(&**result),
                None => Err("Could not get path"),
            }
        } else {
            Err("Could not get path")
        }
    }
}
